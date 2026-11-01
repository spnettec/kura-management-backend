use anyhow::{Context, Result, ensure};
use std::{env, net::SocketAddr};

#[derive(Clone)]
pub struct Config {
    pub http_addr: SocketAddr,
    pub grpc_addr: SocketAddr,
    pub origin: String,
    pub tls_cert: String,
    pub tls_key: String,
    pub client_ca: String,
    pub web_dir: String,
}
fn required(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("missing {name}"))
}
impl Config {
    pub fn load() -> Result<Self> {
        let origin = required("CONSOLE_ORIGIN")?;
        ensure!(
            origin.starts_with("https://") && !origin.ends_with('/'),
            "CONSOLE_ORIGIN must be an HTTPS origin without trailing slash"
        );
        Ok(Self {
            http_addr: env::var("CONSOLE_HTTP_ADDR")
                .unwrap_or("127.0.0.1:18443".into())
                .parse()?,
            grpc_addr: env::var("CONSOLE_GRPC_ADDR")
                .unwrap_or("127.0.0.1:18444".into())
                .parse()?,
            origin,
            tls_cert: required("CONSOLE_TLS_CERT")?,
            tls_key: required("CONSOLE_TLS_KEY")?,
            client_ca: required("CONSOLE_CLIENT_CA")?,
            web_dir: env::var("CONSOLE_WEB_DIR").unwrap_or("frontend/react/dist".into()),
        })
    }
}
