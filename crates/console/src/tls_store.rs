//! A TLS update publishes a complete immutable generation and a single pointer.
//! Startup activates that pointer atomically; configured base files are retained.
use crate::config::Config;
use crate::settings_files::{directory, read_private, sync_parent, write_new};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Generation {
    id: uuid::Uuid,
    fingerprint: String,
    actor: String,
}

pub fn validate_pair(cert: &str, key: &str) -> Result<String> {
    let fingerprint = validate_leaf_certificate(cert)?;
    crate::provisioning::validate_private_key_pem(key)?;
    let certs = rustls_pemfile::certs(&mut cert.as_bytes()).collect::<Result<Vec<_>, _>>()?;
    let key = rustls_pemfile::private_key(&mut key.as_bytes())?.context("private key required")?;
    let pair = rustls::sign::CertifiedKey::from_der(
        certs,
        key,
        &rustls::crypto::ring::default_provider(),
    )?;
    pair.keys_match()?;
    Ok(fingerprint)
}

/// Server and gateway identities are end entities. A CA key pair must not be
/// imported accidentally through an ordinary identity/private-key form.
pub(crate) fn validate_leaf_certificate(pem: &str) -> Result<String> {
    use x509_cert::{der::Decode, ext::pkix::BasicConstraints};
    let fingerprint = crate::settings::certificate_fingerprint(pem)?;
    let der = rustls_pemfile::certs(&mut pem.as_bytes())
        .next()
        .context("certificate required")??;
    let cert = x509_cert::Certificate::from_der(der.as_ref())?;
    if let Some((_, constraints)) = cert.tbs_certificate().get_extension::<BasicConstraints>()? {
        ensure!(
            !constraints.ca,
            "a CA certificate is not a server or gateway identity"
        );
    }
    Ok(fingerprint)
}
fn generation_paths(config: &Config, g: &Generation) -> Result<(PathBuf, PathBuf)> {
    let root = Path::new(&config.state_dir).join("tls-generations");
    for path in [&root, &root.join(g.id.to_string())] {
        let m = fs::symlink_metadata(path)?;
        ensure!(
            m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
            "invalid TLS generation directory"
        );
    }
    let directory = root.join(g.id.to_string());
    let cert = directory.join("server.pem");
    let key = directory.join("server.key");
    let fingerprint = validate_pair(
        std::str::from_utf8(&read_private(&cert)?)?,
        std::str::from_utf8(&read_private(&key)?)?,
    )?;
    ensure!(
        fingerprint == g.fingerprint,
        "TLS generation fingerprint mismatch"
    );
    Ok((cert, key))
}
fn pending(config: &Config) -> PathBuf {
    Path::new(&config.state_dir).join("pending/tls-update.json")
}

pub fn stage(config: &Config, cert: &str, key: &str, actor: &str) -> Result<String> {
    let fingerprint = validate_pair(cert, key)?;
    let state = Path::new(&config.state_dir);
    directory(state)?;
    directory(&state.join("pending"))?;
    ensure!(!pending(config).exists(), "TLS update already pending");
    ensure!(
        !state.join("pending/tls-server.pem").exists()
            && !state.join("pending/tls-server.key").exists(),
        "legacy TLS pending entries require explicit review"
    );
    let root = state.join("tls-generations");
    directory(&root)?;
    let g = Generation {
        id: uuid::Uuid::new_v4(),
        fingerprint: fingerprint.clone(),
        actor: actor.into(),
    };
    let target = root.join(g.id.to_string());
    directory(&target)?;
    write_new(&target.join("server.pem"), cert.as_bytes())?;
    write_new(&target.join("server.key"), key.as_bytes())?;
    write_new(&target.join("generation.json"), &serde_json::to_vec(&g)?)?;
    File::open(&target)?.sync_all()?;
    File::open(&root)?.sync_all()?;
    generation_paths(config, &g)?;
    let temporary = state.join("pending").join(format!(".tls-update-{}", g.id));
    write_new(&temporary, &serde_json::to_vec(&g)?)?;
    // Atomic create-only publication. Concurrent requests cannot replace an
    // existing pending pointer. Unpublished generations remain as evidence.
    fs::hard_link(&temporary, pending(config))?;
    sync_parent(&pending(config))?;
    fs::remove_file(&temporary)?;
    sync_parent(&temporary)?;
    Ok(fingerprint)
}

pub fn pending_fingerprint(config: &Config) -> Result<Option<String>> {
    if !pending(config).exists() {
        return Ok(None);
    }
    let g: Generation = serde_json::from_slice(&read_private(&pending(config))?)?;
    generation_paths(config, &g)?;
    Ok(Some(g.fingerprint))
}

pub fn activate(config: &mut Config) -> Result<()> {
    let state = PathBuf::from(&config.state_dir);
    // Do not replay old, potentially half-staged updates from the shell path.
    ensure!(
        !state.join("pending/tls-server.pem").exists()
            && !state.join("pending/tls-server.key").exists(),
        "legacy TLS pending entries require explicit review"
    );
    let active = state.join("tls-active.json");
    let pending = pending(config);
    let source = if pending.exists() {
        &pending
    } else if active.exists() {
        &active
    } else {
        return Ok(());
    };
    let bytes = read_private(source)?;
    let g: Generation = serde_json::from_slice(&bytes)?;
    let (cert, key) = generation_paths(config, &g)?;
    if source == &pending {
        let temporary = state.join(format!(".tls-active-{}", uuid::Uuid::new_v4()));
        write_new(&temporary, &bytes)?;
        fs::rename(&temporary, &active)?;
        File::open(&state)?.sync_all()?;
        fs::remove_file(&pending)?;
        sync_parent(&pending)?;
    }
    config.tls_cert = cert.to_str().context("non-UTF8 TLS path")?.into();
    config.tls_key = key.to_str().context("non-UTF8 TLS path")?.into();
    Ok(())
}
