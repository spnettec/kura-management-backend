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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Generation {
    id: uuid::Uuid,
    fingerprint: String,
    actor: String,
}

/// Read a selection without activating pending material. Runtime activation owns
/// the joint HTTPS/mTLS manifest; the legacy startup helper remains for migration.
pub(crate) fn selection(config: &Config, staged: bool) -> Result<Option<Generation>> {
    reject_legacy(config)?;
    let path = if staged { pending(config) } else {
        Path::new(&config.state_dir).join("tls-active.json")
    };
    if !path.try_exists()? { return Ok(None); }
    let generation: Generation = serde_json::from_slice(&read_private(&path)?)?;
    generation_paths(config, &generation)?;
    Ok(Some(generation))
}

pub(crate) fn reject_legacy(config: &Config) -> Result<()> {
    for name in ["tls-server.pem", "tls-server.key"] {
        ensure!(!Path::new(&config.state_dir).join("pending").join(name).exists(),
            "legacy TLS pending files require explicit review");
    }
    Ok(())
}

pub(crate) fn select(config: &mut Config, generation: &Generation) -> Result<()> {
    let (cert, key) = generation_paths(config, generation)?;
    config.tls_cert = cert.to_str().context("non-UTF8 TLS path")?.into();
    config.tls_key = key.to_str().context("non-UTF8 TLS path")?.into();
    Ok(())
}

pub(crate) fn consume(config: &Config, selected: &Option<Generation>) -> Result<()> {
    if selected.is_some() && &selection(config, true)? == selected {
        fs::remove_file(pending(config))?;
        sync_parent(&pending(config))?;
    }
    Ok(())
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

/// Validate a server leaf against its intended origin before importing or staging it.
pub(crate) fn validate_server_identity(cert: &str, key: &str, origin: &str) -> Result<String> {
    let fingerprint = validate_pair(cert, key)?;
    let certs = rustls_pemfile::certs(&mut cert.as_bytes()).collect::<std::io::Result<Vec<_>>>()?;
    let origin: axum::http::Uri = origin.parse()?;
    ensure!(
        origin.scheme_str() == Some("https") && origin.path() == "/" && origin.query().is_none(),
        "console origin must be an HTTPS origin"
    );
    let host = origin
        .host()
        .context("console origin host required")?
        .trim_start_matches('[')
        .trim_end_matches(']');
    let name = rustls::pki_types::ServerName::try_from(host.to_owned())?;
    let leaf = certs.first().context("server certificate required")?;
    let parsed = rustls::server::ParsedCertificate::try_from(leaf)?;
    rustls::client::verify_server_name(&parsed, &name)?;
    {
        use x509_cert::der::Decode;
        let leaf = x509_cert::Certificate::from_der(leaf.as_ref())?;
        if let Some((_, usage)) = leaf
            .tbs_certificate()
            .get_extension::<x509_cert::ext::pkix::ExtendedKeyUsage>()?
        {
            ensure!(
                usage
                    .0
                    .iter()
                    .any(|oid| oid.to_string() == "1.3.6.1.5.5.7.3.1"),
                "server certificate extended key usage does not allow server authentication"
            );
        }
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?;
        let validity = leaf.tbs_certificate().validity();
        ensure!(
            validity.not_before.to_unix_duration() <= now
                && now <= validity.not_after.to_unix_duration(),
            "server certificate is outside its validity period"
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

#[cfg(test)]
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
