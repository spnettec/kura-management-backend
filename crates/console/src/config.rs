use anyhow::{Context, Result, ensure};
use std::{env, net::SocketAddr};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    pub http_addr: SocketAddr,
    pub grpc_addr: SocketAddr,
    pub origin: String,
    pub tls_cert: String,
    pub tls_key: String,
    pub client_ca: String,
    pub web_dir: String,
    // Writable state directory for pending settings (TLS/client CA/task key)
    // and other runtime state; deployed under /var/lib/kura-management-console.
    pub state_dir: String,
}
pub(crate) fn setting(name: &str) -> Result<Option<String>> {
    normalize_setting(env::var(name)).with_context(|| format!("invalid {name}"))
}
fn normalize_setting(value: std::result::Result<String, env::VarError>) -> Result<Option<String>> {
    match value {
        Ok(value) => Ok((!value.trim().is_empty()).then_some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => anyhow::bail!("environment value must be valid UTF-8"),
    }
}
fn required(name: &str) -> Result<String> {
    setting(name)?.with_context(|| format!("missing {name}"))
}
impl Config {
    pub fn load() -> Result<Self> {
        if let Some(saved) = crate::console_settings::load()? {
            return Self::overrides(saved.settings.config);
        }
        let origin = required("CONSOLE_ORIGIN")?;
        ensure!(
            origin.starts_with("https://") && !origin.ends_with('/'),
            "CONSOLE_ORIGIN must be an HTTPS origin without trailing slash"
        );
        Ok(Self {
            http_addr: setting("CONSOLE_HTTP_ADDR")?
                .unwrap_or("127.0.0.1:18443".into())
                .parse()?,
            grpc_addr: setting("CONSOLE_GRPC_ADDR")?
                .unwrap_or("127.0.0.1:18444".into())
                .parse()?,
            origin,
            tls_cert: required("CONSOLE_TLS_CERT")?,
            tls_key: required("CONSOLE_TLS_KEY")?,
            client_ca: required("CONSOLE_CLIENT_CA")?,
            web_dir: setting("CONSOLE_WEB_DIR")?.unwrap_or("frontend/react/dist".into()),
            state_dir: setting("CONSOLE_STATE_DIR")?
                .unwrap_or("/var/lib/kura-management-console".into()),
        })
    }

    pub(crate) fn overrides(mut cfg: Self) -> Result<Self> {
        for (name, value) in [
            ("CONSOLE_ORIGIN", &mut cfg.origin),
            ("CONSOLE_TLS_CERT", &mut cfg.tls_cert),
            ("CONSOLE_TLS_KEY", &mut cfg.tls_key),
            ("CONSOLE_CLIENT_CA", &mut cfg.client_ca),
            ("CONSOLE_WEB_DIR", &mut cfg.web_dir),
        ] {
            if let Some(override_value) = setting(name)? {
                *value = override_value;
            }
        }
        if let Some(address) = setting("CONSOLE_HTTP_ADDR")? {
            cfg.http_addr = address.parse()?;
        }
        if let Some(address) = setting("CONSOLE_GRPC_ADDR")? {
            cfg.grpc_addr = address.parse()?;
        }
        ensure!(
            cfg.origin.starts_with("https://") && !cfg.origin.ends_with('/'),
            "invalid HTTPS origin"
        );
        Ok(cfg)
    }
}

#[test]
fn platform_empty_settings_are_absent_without_trimming_nonempty_credentials() {
    for value in ["", "  ", "\t\r\n"] {
        assert_eq!(normalize_setting(Ok(value.into())).unwrap(), None);
    }
    let value = "postgres://user:a b@host/database";
    assert_eq!(
        normalize_setting(Ok(value.into())).unwrap().as_deref(),
        Some(value)
    );
    assert_eq!(
        normalize_setting(Err(env::VarError::NotPresent)).unwrap(),
        None
    );
    let error=normalize_setting(Err(env::VarError::NotUnicode(std::ffi::OsString::from("private-test-value")))).unwrap_err();
    assert!(!error.to_string().contains("private-test-value"));
}
