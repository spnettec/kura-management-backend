//! Public client trust anchors use immutable generations, never a root copy
//! over bootstrap mounts. No CA private key is accepted by this store.
use crate::{
    config::Config,
    settings_files::{
        directory, publish_new, publish_replace, read_private, sync_parent, write_new,
    },
};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use x509_cert::{
    der::Decode,
    ext::pkix::{BasicConstraints, KeyUsage},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Generation {
    schema_version: u8,
    id: uuid::Uuid,
    fingerprint: String,
    previous_fingerprint: String,
    actor: String,
}

pub fn validate(pem: &str) -> Result<String> {
    validate_at(pem, SystemTime::now().duration_since(UNIX_EPOCH)?)
}

fn validate_at(pem: &str, now: std::time::Duration) -> Result<String> {
    let fingerprint = crate::settings::certificate_fingerprint(pem)?;
    let der = rustls_pemfile::certs(&mut pem.as_bytes())
        .next()
        .context("CA certificate required")??;
    let cert = x509_cert::Certificate::from_der(der.as_ref())?;
    let tbs = cert.tbs_certificate();
    let (_, constraints) = tbs
        .get_extension::<BasicConstraints>()?
        .context("CA basic constraints required")?;
    ensure!(
        constraints.ca,
        "client trust anchor must be a CA certificate"
    );
    if let Some((_, usage)) = tbs.get_extension::<KeyUsage>()? {
        ensure!(
            usage.key_cert_sign(),
            "CA key usage must allow certificate signing"
        );
    }
    let validity = tbs.validity();
    ensure!(
        validity.not_before.to_unix_duration() <= now
            && now <= validity.not_after.to_unix_duration(),
        "CA certificate is outside its validity period"
    );
    let mut roots = rustls::RootCertStore::empty();
    roots.add(der)?;
    Ok(fingerprint)
}

#[cfg(test)]
#[path = "client_ca_tests.rs"]
mod client_ca_tests;

fn pending(config: &Config) -> PathBuf {
    Path::new(&config.state_dir).join("pending/client-ca-update.json")
}
fn active(config: &Config) -> PathBuf {
    Path::new(&config.state_dir).join("client-ca-active.json")
}
fn legacy(config: &Config) -> PathBuf {
    Path::new(&config.state_dir).join("pending/client-ca.pem")
}

fn generation_path(config: &Config, generation: &Generation) -> Result<PathBuf> {
    ensure!(
        generation.schema_version == 1,
        "unsupported CA generation schema"
    );
    let root = Path::new(&config.state_dir).join("client-ca-generations");
    let folder = root.join(generation.id.to_string());
    // These directories must already exist: reading must not create state.
    for path in [&root, &folder] {
        let m = fs::symlink_metadata(path)?;
        use std::os::unix::fs::MetadataExt;
        ensure!(
            m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
            "invalid CA generation directory"
        );
    }
    let certificate = folder.join("client-ca.pem");
    let fingerprint = validate(std::str::from_utf8(&read_private(&certificate)?)?)?;
    ensure!(
        fingerprint == generation.fingerprint,
        "CA generation fingerprint mismatch"
    );
    Ok(certificate)
}

pub fn stage(config: &Config, pem: &str, actor: &str) -> Result<String> {
    let fingerprint = validate(pem)?;
    ensure!(
        !legacy(config).exists(),
        "legacy CA pending entry requires explicit review"
    );
    let previous_fingerprint =
        crate::settings::certificate_fingerprint(&crate::settings::live_client_ca(config)?)?;
    ensure!(
        previous_fingerprint != fingerprint,
        "CA certificate is already active"
    );
    let state = Path::new(&config.state_dir);
    directory(state)?;
    directory(&state.join("pending"))?;
    ensure!(!pending(config).exists(), "CA update already pending");
    let root = state.join("client-ca-generations");
    directory(&root)?;
    let generation = Generation {
        schema_version: 1,
        id: uuid::Uuid::new_v4(),
        fingerprint: fingerprint.clone(),
        previous_fingerprint,
        actor: actor.into(),
    };
    let folder = root.join(generation.id.to_string());
    directory(&folder)?;
    write_new(&folder.join("client-ca.pem"), pem.as_bytes())?;
    let bytes = serde_json::to_vec(&generation)?;
    write_new(&folder.join("generation.json"), &bytes)?;
    File::open(&folder)?.sync_all()?;
    File::open(&root)?.sync_all()?;
    generation_path(config, &generation)?;
    publish_new(&pending(config), &bytes)?;
    Ok(fingerprint)
}

pub fn pending_fingerprint(config: &Config) -> Result<Option<String>> {
    ensure!(
        !legacy(config).exists(),
        "legacy CA pending entry requires explicit review"
    );
    if !pending(config).exists() {
        return Ok(None);
    }
    let generation: Generation = serde_json::from_slice(&read_private(&pending(config))?)?;
    generation_path(config, &generation)?;
    Ok(Some(generation.fingerprint))
}

pub fn activate(config: &mut Config) -> Result<()> {
    ensure!(
        !legacy(config).exists(),
        "legacy CA pending entry requires explicit review"
    );
    let mut active_id = None;
    let live = active(config);
    if live.exists() {
        let generation: Generation = serde_json::from_slice(&read_private(&live)?)?;
        config.client_ca = generation_path(config, &generation)?
            .to_str()
            .context("non-UTF8 CA path")?
            .into();
        active_id = Some(generation.id);
    }
    let pending = pending(config);
    if !pending.exists() {
        return Ok(());
    }
    let bytes = read_private(&pending)?;
    let generation: Generation = serde_json::from_slice(&bytes)?;
    let certificate = generation_path(config, &generation)?;
    if active_id != Some(generation.id) {
        let current =
            crate::settings::certificate_fingerprint(&crate::settings::live_client_ca(config)?)?;
        ensure!(
            current == generation.previous_fingerprint,
            "CA update was approved against a different active trust anchor"
        );
        publish_replace(&live, &bytes)?;
    }
    fs::remove_file(&pending)?;
    sync_parent(&pending)?;
    config.client_ca = certificate.to_str().context("non-UTF8 CA path")?.into();
    Ok(())
}
