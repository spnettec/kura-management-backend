//! Per-upload validation outcomes. Reads never retry an accepted import.
use super::*;
use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AcceptedImport {
    schema_version: u8,
    request_id: Uuid,
    operation_id: Uuid,
    certificate_sha256: String,
    actor: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Registry {
    schema_version: u8,
    request_id: Uuid,
    operations: Vec<Uuid>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Rejection {
    schema_version: u8,
    operation_id: Uuid,
    accepted_sha256: String,
    code: String,
}
pub(super) struct Binding {
    pub operation_id: Uuid,
    pub accepted_sha256: String,
}
type Uncertain = HashSet<(PathBuf, Uuid)>;
static UNCERTAIN: OnceLock<Mutex<Uncertain>> = OnceLock::new();
fn uncertain() -> &'static Mutex<Uncertain> {
    UNCERTAIN.get_or_init(Mutex::default)
}
fn import_root(state: &Path, id: Uuid) -> PathBuf {
    directory(state, id).join("imports")
}
fn marker(state: &Path, id: Uuid) -> PathBuf {
    directory(state, id).join("imports-seen.json")
}
fn operation_dir(state: &Path, id: Uuid, operation: Uuid) -> PathBuf {
    import_root(state, id)
        .join("operations")
        .join(operation.to_string())
}
fn present(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}
fn entries(state: &Path, id: Uuid) -> Result<Vec<AcceptedImport>> {
    let root = import_root(state, id);
    if !present(&root)? {
        ensure!(
            !present(&marker(state, id))?,
            "retained upload registry is missing"
        );
        return Ok(Vec::new());
    }
    check_directory(&root)?;
    let seen: Marker = load(&marker(state, id))?;
    ensure!(seen.request_id == id, "invalid upload history marker");
    let registry: Registry = load(&root.join("accepted.json"))?;
    ensure!(
        registry.schema_version == 1
            && registry.request_id == id
            && registry.operations.len() <= 1024
            && registry.operations.iter().all(|id| !id.is_nil())
            && registry.operations.iter().collect::<HashSet<_>>().len()
                == registry.operations.len(),
        "invalid upload registry"
    );
    check_directory(&root.join("operations"))?;
    let actual: HashSet<_> = fs::read_dir(root.join("operations"))?
        .map(|entry| {
            Ok(entry?
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("invalid upload path"))?)
        })
        .collect::<Result<_>>()?;
    let expected: HashSet<_> = registry.operations.iter().map(Uuid::to_string).collect();
    ensure!(
        actual == expected,
        "a retained upload operation is missing or unregistered"
    );
    let mut values = Vec::new();
    for operation in registry.operations {
        let dir = operation_dir(state, id, operation);
        check_directory(&dir)?;
        let accepted: AcceptedImport = load(&dir.join("accepted.json"))?;
        ensure!(
            accepted.schema_version == 1
                && accepted.request_id == id
                && accepted.operation_id == operation
                && !accepted.actor.is_empty()
                && accepted.actor.len() <= 512
                && accepted.certificate_sha256.len() == 64
                && accepted
                    .certificate_sha256
                    .bytes()
                    .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)),
            "invalid original upload intent"
        );
        values.push(accepted);
    }
    Ok(values)
}
fn accepted_hash(state: &Path, accepted: &AcceptedImport) -> Result<String> {
    Ok(auth::hash(&files::read_private(
        &operation_dir(state, accepted.request_id, accepted.operation_id).join("accepted.json"),
    )?))
}
pub(super) fn validate_signed(state: &Path, id: Uuid, signed: &Signed) -> Result<()> {
    match (signed.import_operation_id, &signed.import_accepted_sha256) {
        (None, None) => Ok(()), // Preserve the exact legacy receipt format.
        (Some(operation), Some(hash)) => {
            let accepted = entries(state, id)?
                .into_iter()
                .find(|row| row.operation_id == operation)
                .context("original upload intent missing")?;
            ensure!(
                signed.request_id == id
                    && signed.certificate_sha256 == accepted.certificate_sha256
                    && accepted_hash(state, &accepted)? == *hash,
                "signed upload identity changed"
            );
            Ok(())
        }
        _ => anyhow::bail!("incomplete signed upload identity"),
    }
}
fn outcome(state: &Path, accepted: &AcceptedImport) -> Result<Value> {
    let id = accepted.request_id;
    let operation = accepted.operation_id;
    let mut value = json!({"requestId":id,"operationId":operation,
        "certificateSha256":accepted.certificate_sha256,"state":"UNKNOWN"});
    if uncertain()
        .lock()
        .unwrap()
        .contains(&(directory(state, id), operation))
    {
        return Ok(value);
    }
    let signed_path = directory(state, id).join("signed.json");
    let signed: Option<Signed> = if present(&signed_path)? {
        Some(load(&signed_path)?)
    } else {
        None
    };
    let applied = signed
        .as_ref()
        .is_some_and(|row| row.import_operation_id == Some(operation));
    let rejected = operation_dir(state, id, operation).join("rejected.json");
    if present(&rejected)? {
        let receipt: Rejection = load(&rejected)?;
        ensure!(
            !applied
                && receipt.schema_version == 1
                && receipt.operation_id == operation
                && receipt.accepted_sha256 == accepted_hash(state, accepted)?
                && ["INVALID_ARGUMENT", "CSR_NOT_READY"].contains(&receipt.code.as_str()),
            "invalid original upload rejection receipt"
        );
        value["state"] = json!("REJECTED");
        value["code"] = json!(receipt.code);
    } else if let Some(signed) = signed.filter(|_| applied) {
        let intent: Signed = load(&directory(state, id).join("sign-intent.json"))?;
        validate_signed(state, id, &signed)?;
        let (_, key, _) = material(state, id)?;
        let certificate = files::read_private(&directory(state, id).join("certificate.pem"))?;
        ensure!(
            intent == signed
                && signed.certificate_sha256 == auth::hash(&certificate)
                && crate::tls_store::validate_pair(std::str::from_utf8(&certificate)?, &key)?
                    == signed.fingerprint,
            "original signed upload receipt changed"
        );
        value["state"] = json!("APPLIED");
        value["receiptSource"] = json!("SIGNED_CERTIFICATE_RECEIPT");
    }
    Ok(value)
}
pub(super) fn decorate(state: &Path, id: Uuid, value: &mut Value) -> Result<()> {
    let outcomes = entries(state, id)?
        .iter()
        .map(|row| outcome(state, row))
        .collect::<Result<Vec<_>>>()?;
    if let Some(blocking) = outcomes.iter().find(|row| row["state"] == "UNKNOWN") {
        value["state"] = json!("UNKNOWN");
        value["blockingImportOperationId"] = blocking["operationId"].clone();
    }
    if !outcomes.is_empty() {
        value["importOperations"] = json!(outcomes);
    }
    Ok(())
}
pub(super) fn unresolved(state: &Path, id: Uuid) -> Result<bool> {
    for accepted in entries(state, id)? {
        if outcome(state, &accepted)?["state"] == "UNKNOWN" {
            return Ok(true);
        }
    }
    Ok(false)
}
pub(crate) fn import_operation(state: &Path, id: Uuid, operation: Uuid) -> Result<Option<Value>> {
    ensure!(!operation.is_nil(), "invalid upload identity");
    let Some(mut value) = observe(state, id)? else {
        return Ok(None);
    };
    let Some(accepted) = entries(state, id)?
        .into_iter()
        .find(|row| row.operation_id == operation)
    else {
        return Ok(None);
    };
    let receipt = outcome(state, &accepted)?;
    if receipt["state"] == "REJECTED" && receipt["code"] == "INVALID_ARGUMENT" {
        value["certificateRejected"] = json!(true);
    }
    value["importOperation"] = receipt;
    Ok(Some(value))
}
fn accept(
    state: &Path,
    id: Uuid,
    accepted: &AcceptedImport,
    mut previous: Vec<AcceptedImport>,
) -> Result<()> {
    let root = import_root(state, id);
    if !present(&root)? {
        publish(&marker(state, id), &Marker { request_id: id })?;
        files::directory(&root)?;
        files::directory(&root.join("operations"))?;
        publish(
            &root.join("accepted.json"),
            &Registry {
                schema_version: 1,
                request_id: id,
                operations: Vec::new(),
            },
        )?;
    }
    let dir = operation_dir(state, id, accepted.operation_id);
    fs::create_dir(&dir)?;
    fs::set_permissions(&dir, std::os::unix::fs::PermissionsExt::from_mode(0o700))?;
    files::sync_parent(&dir)?;
    publish(&dir.join("accepted.json"), accepted)?;
    previous.push(accepted.clone());
    files::publish_replace(
        &root.join("accepted.json"),
        &serde_json::to_vec(&Registry {
            schema_version: 1,
            request_id: id,
            operations: previous.iter().map(|row| row.operation_id).collect(),
        })?,
    )?;
    Ok(())
}
pub(crate) fn import_with_receipt(
    state: &Path,
    id: Uuid,
    operation: Uuid,
    certificate: &str,
    actor: &str,
) -> Result<Value> {
    ensure!(
        !operation.is_nil() && !actor.is_empty() && actor.len() <= 512,
        "invalid upload identity"
    );
    let _lock = lock(state)?;
    let current = observe(state, id)?.context("CSR request missing")?;
    let previous = entries(state, id)?;
    if let Some(original) = previous.iter().find(|row| row.operation_id == operation) {
        ensure!(
            original.certificate_sha256 == auth::hash(certificate.as_bytes()),
            "upload identity reused for different material"
        );
        return import_operation(state, id, operation)?.context("original upload missing");
    }
    ensure!(
        current["state"] != "UNKNOWN" && previous.len() < 1024,
        "observe the original unresolved upload; do not submit another identity"
    );
    let accepted = AcceptedImport {
        schema_version: 1,
        request_id: id,
        operation_id: operation,
        certificate_sha256: auth::hash(certificate.as_bytes()),
        actor: actor.into(),
    };
    accept(state, id, &accepted, previous)?;
    let binding = Binding {
        operation_id: operation,
        accepted_sha256: accepted_hash(state, &accepted)?,
    };
    let result = (|| {
        let rejection = if current["state"] != "READY" {
            Some("CSR_NOT_READY")
        } else {
            let result = import_certificate_locked(state, id, certificate, Some(&binding))?;
            if result["certificateRejected"] == true {
                Some("INVALID_ARGUMENT")
            } else {
                None
            }
        };
        if let Some(code) = rejection {
            publish(
                &operation_dir(state, id, operation).join("rejected.json"),
                &Rejection {
                    schema_version: 1,
                    operation_id: operation,
                    accepted_sha256: binding.accepted_sha256,
                    code: code.into(),
                },
            )?;
        }
        Ok(())
    })();
    if let Err(error) = result {
        // A visible receipt after a failed durability step is not an ACK in
        // this process. Only an independently retained durable original can settle it.
        uncertain()
            .lock()
            .unwrap()
            .insert((directory(state, id), operation));
        return Err(error);
    }
    import_operation(state, id, operation)?.context("original upload receipt missing")
}

#[cfg(test)]
#[path = "server_csr_import_tests.rs"]
mod tests;
