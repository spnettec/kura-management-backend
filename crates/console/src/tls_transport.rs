//! Per-connection TLS snapshots share one runtime generation. An activation
//! closes old handshakes and established streams, including long-lived gRPC.
use crate::tls_runtime::TlsRuntime;
use std::{
    future::Future,
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::{TcpListener, TcpStream},
};
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use tokio_util::sync::CancellationToken;
use tonic::transport::server::{Connected, TcpConnectInfo};

#[derive(Clone)]
pub(crate) struct GatewayTls {
    pub tls: tonic::transport::server::TlsConnectInfo<TcpConnectInfo>,
    pub epoch: CancellationToken,
}

pub(crate) struct GatewayIo(TlsStream<EpochIo>);
impl Connected for GatewayIo {
    type ConnectInfo = GatewayTls;
    fn connect_info(&self) -> Self::ConnectInfo {
        GatewayTls {
            tls: self.0.connect_info(),
            epoch: self.0.get_ref().0.epoch.clone(),
        }
    }
}
impl AsyncRead for GatewayIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if self.0.get_ref().0.epoch.is_cancelled() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "TLS generation changed",
            )));
        }
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}
impl AsyncWrite for GatewayIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.0.get_ref().0.epoch.is_cancelled() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "TLS generation changed",
            )));
        }
        Pin::new(&mut self.0).poll_write(cx, bytes)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

pub(crate) struct EpochIo {
    stream: TcpStream,
    epoch: CancellationToken,
    cancelled: Pin<Box<dyn Future<Output = ()> + Send + Sync>>,
}
impl EpochIo {
    fn new(stream: TcpStream, epoch: CancellationToken) -> Self {
        Self {
            stream,
            epoch: epoch.clone(),
            cancelled: Box::pin(epoch.cancelled_owned()),
        }
    }
    fn check(&mut self, cx: &mut Context<'_>) -> io::Result<()> {
        if self.cancelled.as_mut().poll(cx).is_ready() {
            Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "TLS generation changed",
            ))
        } else {
            Ok(())
        }
    }
}
impl Connected for EpochIo {
    type ConnectInfo = TcpConnectInfo;
    fn connect_info(&self) -> Self::ConnectInfo {
        self.stream.connect_info()
    }
}
impl AsyncRead for EpochIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.check(cx)?;
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
impl AsyncWrite for EpochIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.check(cx)?;
        Pin::new(&mut self.stream).poll_write(cx, bytes)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.check(cx)?;
        Pin::new(&mut self.stream).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}

async fn handshake(
    runtime: Arc<TlsRuntime>,
    stream: TcpStream,
    grpc: bool,
) -> io::Result<TlsStream<EpochIo>> {
    let (config, epoch) = runtime.transport(grpc)?;
    stream.set_nodelay(true)?;
    tokio::time::timeout(
        Duration::from_secs(10),
        TlsAcceptor::from(config).accept(EpochIo::new(stream, epoch)),
    )
    .await
    .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "TLS handshake timeout"))?
}

#[derive(Clone)]
pub(crate) struct WebAcceptor(pub Arc<TlsRuntime>);
impl<S: Send + 'static> axum_server::accept::Accept<TcpStream, S> for WebAcceptor {
    type Stream = TlsStream<EpochIo>;
    type Service = S;
    type Future = Pin<Box<dyn Future<Output = io::Result<(Self::Stream, S)>> + Send>>;
    fn accept(&self, stream: TcpStream, service: S) -> Self::Future {
        let runtime = self.0.clone();
        Box::pin(async move { Ok((handshake(runtime, stream, false).await?, service)) })
    }
}

pub(crate) fn grpc_incoming(
    listener: TcpListener,
    runtime: Arc<TlsRuntime>,
) -> impl tokio_stream::Stream<Item = io::Result<GatewayIo>> + Send {
    async_stream::stream! {
        let mut pending = tokio::task::JoinSet::new();
        loop {
            tokio::select! {
                accepted = listener.accept(), if pending.len() < 128 => {
                    match accepted {
                        Ok((stream, _)) => { pending.spawn(handshake(runtime.clone(), stream, true)); }
                        Err(error) => { yield Err(error); break; }
                    }
                }
                finished = pending.join_next(), if !pending.is_empty() => {
                    // Untrusted handshake failures do not terminate the listener.
                    if let Some(Ok(Ok(stream))) = finished { yield Ok(GatewayIo(stream)); }
                }
            }
        }
    }
}
