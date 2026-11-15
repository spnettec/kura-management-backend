//! Pending-settings staging for TLS material, the client CA and the task key.
//!
//! Uploaded material is validated into immutable generations and pending
//! pointers. The unprivileged server activates complete versions on restart;
//! the root entrypoint never publishes user-staged credential files.
//! Until applied, every read reflects both the live and pending state.
use crate::config::Config;
use anyhow::{Context, Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

const PENDING: &str = "pending";
const MAX_PEM: usize = 65536;

pub fn certificate_fingerprint(pem: &str) -> Result<String> {
    ensure!(pem.len() <= MAX_PEM, "certificate too large");
    let trimmed = pem.trim();
    let encoded = trimmed
        .strip_prefix("-----BEGIN CERTIFICATE-----")
        .and_then(|v| v.strip_suffix("-----END CERTIFICATE-----"))
        .ok_or_else(|| anyhow::anyhow!("one public certificate required"))?;
    let encoded: String = encoded
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let der = STANDARD.decode(&encoded)?;
    ensure!(STANDARD.encode(&der) == encoded, "noncanonical certificate");
    let cert = rustls::pki_types::CertificateDer::from(der.as_slice());
    rustls::server::ParsedCertificate::try_from(&cert)?;
    Ok(crate::auth::hash(&der))
}

fn pending_dir(config: &Config) -> PathBuf {
    Path::new(&config.state_dir).join(PENDING)
}

fn pending_path(config: &Config, name: &str) -> PathBuf {
    pending_dir(config).join(name)
}

fn read_live(path: &str) -> Option<String> {
    let mut value = Vec::new();
    File::open(path)
        .ok()?
        .take((MAX_PEM + 1) as u64)
        .read_to_end(&mut value)
        .ok()?;
    if value.len() > MAX_PEM {
        return None;
    }
    String::from_utf8(value).ok()
}

pub fn tls_status(config: &Config) -> Result<Value> {
    let live = read_live(&config.tls_cert)
        .map(|pem| certificate_fingerprint(&pem))
        .transpose()?;
    let pending = crate::tls_store::pending_fingerprint(config)?;
    Ok(json!({
        "certPath": config.tls_cert,
        "keyPath": config.tls_key,
        "certificateFingerprint": live,
        "pendingFingerprint": pending,
        "restartRequired": pending.is_some(),
    }))
}

pub fn client_ca_status(config: &Config) -> Result<Value> {
    let live = read_live(&config.client_ca)
        .map(|pem| certificate_fingerprint(&pem))
        .transpose()?;
    let pending = crate::client_ca_store::pending_fingerprint(config)?;
    Ok(json!({
        "caPath": config.client_ca,
        "caFingerprint": live,
        "pendingFingerprint": pending,
        "restartRequired": pending.is_some(),
    }))
}

pub fn task_key_status(config: &Config, cipher: &crate::task_crypto::TaskCipher) -> Result<Value> {
    let configured =
        std::env::var("CONSOLE_DATA_KEY_FILE").context("CONSOLE_DATA_KEY_FILE required")?;
    let slot = Path::new(&config.state_dir)
        .join("pending/keys")
        .join(format!("{}.key", cipher.fingerprint()));
    let path = if slot.exists() {
        slot
    } else {
        PathBuf::from(configured)
    };
    ensure!(
        crate::task_crypto::TaskCipher::load(&path)?.fingerprint() == cipher.fingerprint(),
        "active task key recovery material unavailable"
    );
    let pending = pending_path(config, "task-data.key").exists();
    Ok(json!({
        "keyPath": path,
        "keyFingerprint": cipher.fingerprint(),
        "pendingRestoreStaged": pending,
        "restartRequired": pending,
    }))
}

/// Read the live client CA for identity-bundle packaging.
pub fn live_client_ca(config: &Config) -> Result<String> {
    read_live(&config.client_ca).context("client CA unavailable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_rejects_non_certificate() {
        assert!(
            certificate_fingerprint("-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----")
                .is_err()
        );
        assert!(
            certificate_fingerprint("-----BEGIN CERTIFICATE-----\nAAAA\n-----END CERTIFICATE-----")
                .is_err()
        );
    }
}
