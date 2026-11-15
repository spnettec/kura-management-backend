//! Explicit IPv6 wildcard listeners accept both families. Only an automatic
//! first-setup bind may fall back when IPv6 is unavailable; conflicts never do.
use socket2::{Domain, Protocol, Socket, Type};
use std::{io, net::{SocketAddr, TcpListener}};

pub(crate) fn bind(address: SocketAddr) -> io::Result<TcpListener> {
    let socket=Socket::new(Domain::for_address(address),Type::STREAM,Some(Protocol::TCP))?;
    if address.is_ipv6() && address.ip().is_unspecified() {
        socket.set_only_v6(false)?;
    }
    socket.set_reuse_address(true)?;
    socket.bind(&address.into())?;
    socket.listen(128)?;
    socket.set_nonblocking(true)?;
    Ok(socket.into())
}

fn automatic_with(port:u16,mut bind:impl FnMut(SocketAddr)->io::Result<TcpListener>)->io::Result<TcpListener>{
    match bind(SocketAddr::from(([0u16;8],port))) {
        Err(error) if matches!(error.raw_os_error(),Some(libc::EAFNOSUPPORT|libc::EPROTONOSUPPORT|libc::EADDRNOTAVAIL))=>
            bind(SocketAddr::from(([0u8;4],port))),
        result=>result,
    }
}

pub(crate) fn automatic(port:u16)->io::Result<TcpListener>{ automatic_with(port,bind) }

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    #[test]
    fn platform_ipv6_wildcard_accepts_both_families(){
        let listener=bind("[::]:0".parse().unwrap()).unwrap();
        let port=listener.local_addr().unwrap().port();
        let _v4=TcpStream::connect((std::net::Ipv4Addr::LOCALHOST,port)).unwrap();
        let _v6=TcpStream::connect((std::net::Ipv6Addr::LOCALHOST,port)).unwrap();
    }
    #[test]
    fn platform_automatic_ipv6_unavailable_falls_back_but_conflict_does_not(){
        let mut addresses=Vec::new();
        let listener=automatic_with(0,|address|{
            addresses.push(address);
            if address.is_ipv6(){Err(io::Error::from_raw_os_error(libc::EAFNOSUPPORT))}else{bind(address)}
        }).unwrap();
        assert!(listener.local_addr().unwrap().is_ipv4());assert_eq!(addresses.len(),2);
        let mut attempts=0;
        let result=automatic_with(18443,|_|{attempts+=1;Err(io::Error::from_raw_os_error(libc::EADDRINUSE))});
        assert!(result.is_err());assert_eq!(attempts,1);
    }
    #[test]
    fn platform_explicit_ipv4_does_not_expand_to_ipv6(){
        let listener=bind("127.0.0.1:0".parse().unwrap()).unwrap();
        assert!(listener.local_addr().unwrap().is_ipv4());
    }
}
