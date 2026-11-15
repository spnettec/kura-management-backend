//! Server leaf keys stay in the installation. Only a public CSR leaves it.
//! Accepted IDs are immutable; an incomplete record never regenerates a key.
use crate::{auth, settings_files as files};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    fs,
    fs::File,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use uuid::Uuid;

#[path = "server_csr_import.rs"]
mod imports;
pub(crate) use imports::{import_operation, import_with_receipt};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    pub request_id: Uuid,
    pub origin: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Accepted {
    schema_version: u8,
    request: Request,
    actor: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Ready {
    request_id: Uuid,
    csr_sha256: String,
    key_sha256: String,
}
#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Signed {
    request_id: Uuid,
    certificate_sha256: String,
    fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    import_operation_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    import_accepted_sha256: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Stage {
    request_id: Uuid,
    certificate_sha256: String,
    actor: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Marker {
    request_id: Uuid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Rejected {
    request_id: Uuid,
    blocking_request_id: Uuid,
}

fn root(state: &Path) -> PathBuf {
    state.join("server-csr-requests")
}
fn directory(state: &Path, id: Uuid) -> PathBuf {
    root(state).join(id.to_string())
}
fn load<T: DeserializeOwned>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&files::read_private(path)?)?)
}
fn publish<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    files::publish_new(path, &serde_json::to_vec(value)?)
}
fn check_directory(path: &Path) -> Result<()> {
    let meta = fs::symlink_metadata(path)?;
    ensure!(
        meta.is_dir() && meta.uid() == unsafe { libc::geteuid() } && meta.mode() & 0o077 == 0,
        "invalid CSR directory"
    );
    Ok(())
}
fn lock(state: &Path) -> Result<File> {
    files::directory(state)?;
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
        .open(state.join("server-csr.lock"))?;
    let meta = file.metadata()?;
    ensure!(
        meta.is_file() && meta.uid() == unsafe { libc::geteuid() } && meta.mode() & 0o077 == 0,
        "invalid CSR lock"
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if unsafe {
            libc::flock(
                std::os::fd::AsRawFd::as_raw_fd(&file),
                libc::LOCK_EX | libc::LOCK_NB,
            )
        } == 0
        {
            return Ok(file);
        }
        let error = std::io::Error::last_os_error();
        ensure!(
            error.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline,
            "CSR writer unavailable"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}
pub(crate) fn origin_host(origin: &str) -> Result<String> {
    ensure!(
        origin.len() <= 256
            && !origin.chars().any(char::is_control)
            && !origin.contains('@')
            && !origin.contains('#'),
        "invalid CSR origin"
    );
    let uri: axum::http::Uri = origin.parse()?;
    ensure!(
        uri.scheme_str() == Some("https") && uri.path() == "/" && uri.query().is_none(),
        "HTTPS origin required"
    );
    let host = uri
        .host()
        .context("origin host required")?
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_ascii_lowercase();
    rustls::pki_types::ServerName::try_from(host.clone())?;
    let authority = uri.authority().context("origin authority required")?;
    let port = authority
        .as_str()
        .strip_prefix(authority.host())
        .context("invalid origin authority")?;
    ensure!(
        host.len() <= 253
            && (port.is_empty()
                || port
                    .strip_prefix(':')
                    .is_some_and(|port| port.parse::<u16>().is_ok_and(|p| p > 0))),
        "invalid origin port"
    );
    let canonical_host = match host.parse::<std::net::IpAddr>() {
        Ok(std::net::IpAddr::V6(value)) => format!("[{value}]"),
        Ok(value) => value.to_string(),
        Err(_) => host.clone(),
    };
    let canonical_port = if port.is_empty() || port == ":443" {
        String::new()
    } else {
        format!(":{}", port.trim_start_matches(':').parse::<u16>()?)
    };
    ensure!(
        origin == format!("https://{canonical_host}{canonical_port}"),
        "canonical HTTPS origin required"
    );
    Ok(host)
}
fn request(state: &Path, id: Uuid) -> Result<Accepted> {
    ensure!(!id.is_nil(), "invalid CSR identity");
    check_directory(&root(state))?;
    check_directory(&directory(state, id))?;
    let record: Accepted = load(&directory(state, id).join("accepted.json"))?;
    ensure!(
        record.schema_version == 1 && record.request.request_id == id,
        "CSR identity mismatch"
    );
    origin_host(&record.request.origin)?;
    Ok(record)
}
fn material(state: &Path, id: Uuid) -> Result<(Accepted, String, String)> {
    let accepted = request(state, id)?;
    let dir = directory(state, id);
    let ready: Ready = load(&dir.join("ready.json"))?;
    let key = files::read_private(&dir.join("server.key"))?;
    let csr = files::read_private(&dir.join("request.csr"))?;
    ensure!(
        ready.request_id == id
            && ready.key_sha256 == auth::hash(&key)
            && ready.csr_sha256 == auth::hash(&csr),
        "CSR material changed"
    );
    crate::provisioning::validate_private_key_pem(std::str::from_utf8(&key)?)?;
    Ok((accepted, String::from_utf8(key)?, String::from_utf8(csr)?))
}
fn inspect(state: &Path, id: Uuid) -> Result<Value> {
    let (accepted, key, csr) = material(state, id)?;
    let dir = directory(state, id);
    let mut value = json!({"requestId":id,"origin":accepted.request.origin,"state":"READY","csrPem":csr,"csrSha256":auth::hash(csr.as_bytes())});
    if dir.join("retired.json").try_exists()? {
        let marker: Marker = load(&dir.join("retired.json"))?;
        ensure!(
            marker.request_id == id && !dir.join("stage-intent.json").try_exists()?,
            "invalid CSR retirement"
        );
        value["state"] = json!("RETIRED");
        imports::decorate(state, id, &mut value)?;
        return Ok(value);
    }
    if dir.join("signed.json").try_exists()? {
        let signed: Signed = load(&dir.join("signed.json"))?;
        let intent: Signed = load(&dir.join("sign-intent.json"))?;
        let certificate = String::from_utf8(files::read_private(&dir.join("certificate.pem"))?)?;
        ensure!(
            intent == signed
                && signed.request_id == id
                && signed.certificate_sha256 == auth::hash(certificate.as_bytes())
                && crate::tls_store::validate_pair(&certificate, &key)? == signed.fingerprint,
            "signed certificate changed"
        );
        imports::validate_signed(state, id, &signed)?;
        value["state"] = json!("SIGNED");
        value["certificatePem"] = json!(certificate);
        value["certificateFingerprint"] = json!(signed.fingerprint);
        if dir.join("stage-intent.json").try_exists()? {
            let intent: Stage = load(&dir.join("stage-intent.json"))?;
            let receipt: Stage = load(&dir.join("staged.json"))?;
            ensure!(
                intent.request_id == id
                    && receipt.request_id == id
                    && intent.certificate_sha256 == signed.certificate_sha256
                    && receipt.certificate_sha256 == intent.certificate_sha256
                    && receipt.actor == intent.actor,
                "invalid CSR stage receipt"
            );
            value["state"] = json!("STAGED");
        } else if let Some(selected) = crate::console_settings::load_at(state)? {
            // Derive initial consumption from the original applied receipt,
            // without adding a write to that once-only installation transaction.
            if selected.settings.config.origin == accepted.request.origin
                && selected.settings.material_sha256[0] == signed.certificate_sha256
                && selected.settings.material_sha256[1] == auth::hash(key.as_bytes())
            {
                let original =
                    crate::initial_setup::operation(state, selected.selection.operation_id, false)?
                        .context("original installation receipt missing")?;
                ensure!(
                    original["state"] == "APPLIED" && original["observedConfiguration"] == true,
                    "original CSR installation outcome requires review"
                );
                value["state"] = json!("INSTALLED");
                value["installationOperationId"] = json!(selected.selection.operation_id);
            }
        }
    } else {
        ensure!(
            !dir.join("certificate.pem").try_exists()?
                && !dir.join("sign-intent.json").try_exists()?
                && !dir.join("stage-intent.json").try_exists()?,
            "incomplete certificate import"
        );
    }
    imports::decorate(state, id, &mut value)?;
    Ok(value)
}
pub(crate) fn observe(state: &Path, id: Uuid) -> Result<Option<Value>> {
    ensure!(!id.is_nil(), "invalid CSR identity");
    if !root(state).try_exists()? {
        return Ok(None);
    }
    check_directory(&root(state))?;
    if !directory(state, id).try_exists()? {
        return Ok(None);
    }
    check_directory(&directory(state, id))?;
    if let Ok(rejected) = load::<Rejected>(&directory(state, id).join("rejected.json")) {
        ensure!(
            rejected.request_id == id && !directory(state, id).join("server.key").try_exists()?,
            "invalid rejected request"
        );
        return Ok(Some(
            json!({"requestId":id,"state":"REJECTED","blockingRequestId":rejected.blocking_request_id}),
        ));
    }
    // A missing/invalid receipt is unknown, even when files are currently present.
    Ok(Some(inspect(state, id).unwrap_or_else(
        |_| json!({"requestId":id,"state":"UNKNOWN"}),
    )))
}
pub(crate) fn current(state: &Path) -> Result<Option<Value>> {
    if !root(state).try_exists()? {
        return Ok(None);
    }
    check_directory(&root(state))?;
    let mut active = None;
    for entry in fs::read_dir(root(state))? {
        let entry = entry?;
        let id = Uuid::parse_str(
            entry
                .file_name()
                .to_str()
                .context("invalid CSR directory name")?,
        )?;
        let value = observe(state, id)?.context("CSR record disappeared")?;
        if value["state"] != "RETIRED"
            && value["state"] != "STAGED"
            && value["state"] != "INSTALLED"
            && value["state"] != "REJECTED"
        {
            ensure!(active.is_none(), "multiple unresolved CSR requests");
            active = Some(value);
        }
    }
    Ok(active)
}
fn openssl(command: &mut Command) -> Result<()> {
    let mut child = command
        .env("OPENSSL_CONF", "/dev/null")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = child.try_wait()? {
            if !status.success() {
                use std::io::Read;
                let mut diagnostic = String::new();
                if let Some(stderr) = child.stderr.take() {
                    let _ = stderr.take(4096).read_to_string(&mut diagnostic);
                }
                anyhow::bail!("CSR cryptographic operation failed: {}", diagnostic.trim());
            }
            return Ok(());
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!("CSR cryptographic operation expired");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
pub(crate) fn create(state: &Path, input: Request, actor: &str) -> Result<Value> {
    ensure!(
        !input.request_id.is_nil() && !actor.is_empty() && actor.len() <= 512,
        "invalid CSR request"
    );
    let host = origin_host(&input.origin)?;
    let _lock = lock(state)?;
    files::directory(&root(state))?;
    if let Some(value) = observe(state, input.request_id)? {
        if let Ok(old) = request(state, input.request_id) {
            ensure!(old.request == input, "CSR request identity changed");
        }
        return Ok(value);
    }
    let blocking = current(state)?;
    let dir = directory(state, input.request_id);
    // Existing orphan directories are never adopted or cleared.
    fs::create_dir(&dir)?;
    fs::set_permissions(&dir, std::os::unix::fs::PermissionsExt::from_mode(0o700))?;
    files::sync_parent(&dir)?;
    publish(
        &dir.join("accepted.json"),
        &Accepted {
            schema_version: 1,
            request: input.clone(),
            actor: actor.into(),
        },
    )?;
    if let Some(blocking) = blocking {
        publish(
            &dir.join("rejected.json"),
            &Rejected {
                request_id: input.request_id,
                blocking_request_id: Uuid::parse_str(
                    blocking["requestId"]
                        .as_str()
                        .context("blocking CSR identity missing")?,
                )?,
            },
        )?;
        return observe(state, input.request_id)?.context("CSR rejection missing");
    }
    files::write_new(&dir.join("server.key"), b"")?;
    files::write_new(&dir.join("request.csr"), b"")?;
    let san = if host.parse::<std::net::IpAddr>().is_ok() {
        format!("IP:{host}")
    } else {
        format!("DNS:{host}")
    };
    files::write_new(&dir.join("request.cnf"),format!("[req]\nprompt=no\ndistinguished_name=dn\nreq_extensions=leaf\n[dn]\nCN=YOFC Kura management center\n[leaf]\nsubjectAltName={san}\nbasicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=serverAuth\n").as_bytes())?;
    openssl(
        Command::new("openssl")
            .args([
                "genpkey",
                "-algorithm",
                "EC",
                "-pkeyopt",
                "ec_paramgen_curve:P-256",
                "-pkeyopt",
                "ec_param_enc:named_curve",
                "-out",
            ])
            .arg(dir.join("server.key")),
    )?;
    openssl(
        Command::new("openssl")
            .args(["req", "-new", "-batch", "-config"])
            .arg(dir.join("request.cnf"))
            .arg("-key")
            .arg(dir.join("server.key"))
            .arg("-out")
            .arg(dir.join("request.csr")),
    )?;
    openssl(
        Command::new("openssl")
            .args(["req", "-verify", "-noout", "-in"])
            .arg(dir.join("request.csr")),
    )?;
    let key = files::read_private(&dir.join("server.key"))?;
    let csr = files::read_private(&dir.join("request.csr"))?;
    crate::provisioning::validate_private_key_pem(std::str::from_utf8(&key)?)?;
    ensure!(
        std::str::from_utf8(&csr)?.starts_with("-----BEGIN CERTIFICATE REQUEST-----"),
        "CSR required"
    );
    File::open(dir.join("server.key"))?.sync_all()?;
    File::open(dir.join("request.csr"))?.sync_all()?;
    publish(
        &dir.join("ready.json"),
        &Ready {
            request_id: input.request_id,
            csr_sha256: auth::hash(&csr),
            key_sha256: auth::hash(&key),
        },
    )?;
    observe(state, input.request_id)?.context("CSR receipt missing")
}
pub(crate) fn import_certificate(state: &Path, id: Uuid, certificate: &str) -> Result<Value> {
    let _lock = lock(state)?;
    import_certificate_locked(state, id, certificate, None)
}
fn import_certificate_locked(
    state: &Path,
    id: Uuid,
    certificate: &str,
    binding: Option<&imports::Binding>,
) -> Result<Value> {
    let value = observe(state, id)?.context("CSR request missing")?;
    // This invocation's newly accepted upload is intentionally unresolved until
    // validation completes. Every other unresolved operation remains fenced.
    let own_pending =
        binding.is_some_and(|b| value["blockingImportOperationId"] == b.operation_id.to_string());
    if value["state"] != "READY" && !own_pending {
        if directory(state, id).join("sign-intent.json").try_exists()? {
            let intent: Signed = load(&directory(state, id).join("sign-intent.json"))?;
            ensure!(
                intent.request_id == id
                    && intent.certificate_sha256 == auth::hash(certificate.as_bytes()),
                "certificate import changed"
            );
        }
        return Ok(value);
    }
    let (accepted, key, _) = material(state, id)?;
    let fingerprint = match crate::tls_store::validate_server_identity(
        certificate,
        &key,
        &accepted.request.origin,
    ) {
        Ok(value) => value,
        Err(_) => {
            // A completed validation rejection has no import intent. It is
            // distinct from a missing response/receipt after a durable write.
            let mut rejected = value;
            rejected["certificateRejected"] = json!(true);
            return Ok(rejected);
        }
    };
    let signed = Signed {
        request_id: id,
        certificate_sha256: auth::hash(certificate.as_bytes()),
        fingerprint,
        import_operation_id: binding.map(|b| b.operation_id),
        import_accepted_sha256: binding.map(|b| b.accepted_sha256.clone()),
    };
    let dir = directory(state, id);
    publish(&dir.join("sign-intent.json"), &signed)?;
    files::write_new(&dir.join("certificate.pem"), certificate.as_bytes())?;
    publish(&dir.join("signed.json"), &signed)?;
    observe(state, id)?.context("certificate import receipt missing")
}
pub(crate) fn identity(state: &Path, id: Uuid, origin: &str) -> Result<(String, String)> {
    let value = observe(state, id)?.context("CSR request missing")?;
    ensure!(
        value["state"] == "SIGNED" || value["state"] == "STAGED",
        "CSR signed receipt required"
    );
    let (accepted, key, _) = material(state, id)?;
    ensure!(accepted.request.origin == origin, "CSR origin changed");
    let certificate = String::from_utf8(files::read_private(
        &directory(state, id).join("certificate.pem"),
    )?)?;
    crate::tls_store::validate_server_identity(&certificate, &key, origin)?;
    Ok((certificate, key))
}
pub(crate) fn stage(config: &crate::config::Config, id: Uuid, actor: &str) -> Result<Value> {
    let state = Path::new(&config.state_dir);
    let _lock = lock(state)?;
    let value = observe(state, id)?.context("CSR request missing")?;
    if value["state"] != "SIGNED" {
        return Ok(value);
    }
    let (certificate, key) = identity(state, id, &config.origin)?;
    // All validation precedes the once-only staging intent.
    ensure!(
        crate::tls_store::pending_fingerprint(config)?.is_none(),
        "TLS material already pending"
    );
    let intent = Stage {
        request_id: id,
        certificate_sha256: auth::hash(certificate.as_bytes()),
        actor: actor.into(),
    };
    publish(&directory(state, id).join("stage-intent.json"), &intent)?;
    crate::tls_store::stage(config, &certificate, &key, actor)?;
    publish(&directory(state, id).join("staged.json"), &intent)?;
    observe(state, id)?.context("CSR staging receipt missing")
}
pub(crate) fn retire(state: &Path, id: Uuid) -> Result<Value> {
    let _lock = lock(state)?;
    let value = observe(state, id)?.context("CSR request missing")?;
    ensure!(
        value["state"] == "READY" || value["state"] == "SIGNED" || value["state"] == "RETIRED",
        "uncertain CSR cannot be retired"
    );
    if value["state"] != "RETIRED" {
        publish(
            &directory(state, id).join("retired.json"),
            &Marker { request_id: id },
        )?;
    }
    observe(state, id)?.context("CSR retirement receipt missing")
}

/// A lost staging receipt cannot be bypassed through the older upload/activation APIs.
pub(crate) fn uncertain_stage(state: &Path) -> Result<Option<Uuid>> {
    if !root(state).try_exists()? {
        return Ok(None);
    }
    check_directory(&root(state))?;
    for entry in fs::read_dir(root(state))? {
        let entry = entry?;
        let id = Uuid::parse_str(
            entry
                .file_name()
                .to_str()
                .context("invalid CSR directory name")?,
        )?;
        check_directory(&entry.path())?;
        if imports::unresolved(state, id)? {
            return Ok(Some(id));
        }
        if entry.path().join("stage-intent.json").try_exists()? {
            if observe(state, id)?.context("CSR disappeared")?["state"] != "STAGED" {
                return Ok(Some(id));
            }
        }
    }
    Ok(None)
}
pub(crate) fn ensure_no_uncertain_stage(state: &Path) -> Result<()> {
    ensure!(
        uncertain_stage(state)?.is_none(),
        "original CSR staging outcome requires review"
    );
    Ok(())
}

#[cfg(test)]
#[path = "server_csr_tests.rs"]
pub(crate) mod tests;
