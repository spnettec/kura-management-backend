use super::*;
use crate::server_csr::tests::{EXTENSIONS, fixture, sign};
use std::collections::BTreeMap;

fn ready(state: &Path) -> Uuid {
    let id = Uuid::new_v4();
    create(
        state,
        Request {
            request_id: id,
            origin: "https://localhost:18443".into(),
        },
        "review",
    )
    .unwrap();
    id
}
fn snapshot(state: &Path) -> BTreeMap<PathBuf, String> {
    fn walk(base: &Path, path: &Path, rows: &mut BTreeMap<PathBuf, String>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, rows)
            } else {
                rows.insert(
                    path.strip_prefix(base).unwrap().to_path_buf(),
                    auth::hash(&fs::read(path).unwrap()),
                );
            }
        }
    }
    let mut rows = BTreeMap::new();
    walk(state, state, &mut rows);
    rows
}
#[test]
fn upload_query_does_not_create_or_adopt_missing_records() {
    let state = fixture();
    let before = snapshot(state.path());
    assert!(
        import_operation(state.path(), Uuid::new_v4(), Uuid::new_v4())
            .unwrap()
            .is_none()
    );
    assert_eq!(snapshot(state.path()), before);
}
#[test]
fn rejected_upload_has_an_immutable_original_receipt_and_allows_a_corrected_attempt() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let first = Uuid::new_v4();
    let response = import_with_receipt(root, id, first, "invalid certificate", "review").unwrap();
    assert_eq!(response["state"], "READY");
    assert_eq!(response["importOperation"]["state"], "REJECTED");
    assert_eq!(response["importOperation"]["code"], "INVALID_ARGUMENT");
    assert!(!directory(root, id).join("sign-intent.json").exists());
    let before = snapshot(root);
    assert_eq!(
        import_operation(root, id, first).unwrap().unwrap(),
        response
    );
    assert_eq!(snapshot(root), before);
    let rejection = operation_dir(root, id, first).join("rejected.json");
    let rejected_bytes = fs::read(&rejection).unwrap();
    let certificate = sign(root, id, EXTENSIONS, "1");
    let second = Uuid::new_v4();
    let applied = import_with_receipt(root, id, second, &certificate, "another-admin").unwrap();
    assert_eq!(applied["state"], "SIGNED");
    assert_eq!(applied["importOperation"]["state"], "APPLIED");
    assert_eq!(fs::read(rejection).unwrap(), rejected_bytes);
    assert_eq!(
        import_operation(root, id, first).unwrap().unwrap()["importOperation"]["state"],
        "REJECTED"
    );
}
#[test]
fn signed_business_receipt_settles_the_original_without_importing_again() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let operation = Uuid::new_v4();
    let certificate = sign(root, id, EXTENSIONS, "1");
    import_with_receipt(root, id, operation, &certificate, "review").unwrap();
    let before = snapshot(root);
    let result = import_operation(root, id, operation).unwrap().unwrap();
    assert_eq!(
        result["importOperation"]["operationId"],
        operation.to_string()
    );
    assert_eq!(
        result["importOperation"]["certificateSha256"],
        auth::hash(certificate.as_bytes())
    );
    assert_eq!(
        result["importOperation"]["receiptSource"],
        "SIGNED_CERTIFICATE_RECEIPT"
    );
    assert_eq!(result["importOperation"]["state"], "APPLIED");
    assert!(!result.to_string().contains("PRIVATE KEY"));
    assert_eq!(snapshot(root), before);
}
#[test]
fn existing_upload_identity_is_only_observed_and_cannot_change_material() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let operation = Uuid::new_v4();
    let first = import_with_receipt(root, id, operation, "invalid certificate", "review").unwrap();
    let before = snapshot(root);
    assert_eq!(
        import_with_receipt(root, id, operation, "invalid certificate", "another-admin").unwrap(),
        first
    );
    assert!(import_with_receipt(root, id, operation, "a different certificate", "review").is_err());
    assert_eq!(snapshot(root), before);
}
#[test]
fn missing_upload_rejection_receipt_fences_new_and_legacy_imports_and_retirement() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let operation = Uuid::new_v4();
    import_with_receipt(root, id, operation, "invalid certificate", "review").unwrap();
    fs::rename(
        operation_dir(root, id, operation).join("rejected.json"),
        root.join("retained-rejection.json"),
    )
    .unwrap();
    let before = snapshot(root);
    assert_eq!(
        import_operation(root, id, operation).unwrap().unwrap()["importOperation"]["state"],
        "UNKNOWN"
    );
    assert_eq!(current(root).unwrap().unwrap()["state"], "UNKNOWN");
    assert!(ensure_no_uncertain_stage(root).is_err());
    assert_eq!(
        import_with_receipt(root, id, operation, "invalid certificate", "review").unwrap()["state"],
        "UNKNOWN"
    );
    assert!(import_with_receipt(root, id, Uuid::new_v4(), "other certificate", "review").is_err());
    assert_eq!(
        import_certificate(root, id, "other certificate").unwrap()["state"],
        "UNKNOWN"
    );
    assert!(retire(root, id).is_err());
    assert_eq!(snapshot(root), before);
}
#[test]
fn missing_signed_receipt_is_not_replaced_by_current_certificate_files() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let operation = Uuid::new_v4();
    let certificate = sign(root, id, EXTENSIONS, "1");
    import_with_receipt(root, id, operation, &certificate, "review").unwrap();
    fs::rename(
        directory(root, id).join("signed.json"),
        root.join("retained-signed.json"),
    )
    .unwrap();
    let before = snapshot(root);
    assert!(directory(root, id).join("certificate.pem").exists());
    assert_eq!(
        import_operation(root, id, operation).unwrap().unwrap()["importOperation"]["state"],
        "UNKNOWN"
    );
    assert!(import_with_receipt(root, id, Uuid::new_v4(), &certificate, "review").is_err());
    assert_eq!(snapshot(root), before);
}
#[test]
fn missing_registered_upload_and_registry_restore_cannot_create_a_fresh_attempt() {
    for missing in ["operation", "registry"] {
        let state = fixture();
        let root = state.path();
        let id = ready(root);
        let operation = Uuid::new_v4();
        import_with_receipt(root, id, operation, "invalid certificate", "review").unwrap();
        let retained = root.join("retained-upload-history");
        fs::rename(
            if missing == "operation" {
                operation_dir(root, id, operation)
            } else {
                import_root(root, id)
            },
            retained,
        )
        .unwrap();
        let before = snapshot(root);
        assert_eq!(observe(root, id).unwrap().unwrap()["state"], "UNKNOWN");
        assert!(
            import_with_receipt(root, id, Uuid::new_v4(), "other certificate", "review").is_err()
        );
        assert_eq!(snapshot(root), before);
    }
}
#[test]
fn rejection_receipt_is_bound_to_the_exact_original_accepted_upload() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let operation = Uuid::new_v4();
    import_with_receipt(root, id, operation, "invalid certificate", "review").unwrap();
    let path = operation_dir(root, id, operation).join("rejected.json");
    let mut receipt: Rejection = load(&path).unwrap();
    receipt.accepted_sha256 = "0".repeat(64);
    fs::write(path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    assert_eq!(observe(root, id).unwrap().unwrap()["state"], "UNKNOWN");
    assert!(import_operation(root, id, operation).is_err());
}
#[test]
fn a_visible_receipt_after_failed_durability_is_not_a_success_ack_in_this_process() {
    let state = fixture();
    let root = state.path();
    let id = ready(root);
    let operation = Uuid::new_v4();
    import_with_receipt(root, id, operation, "invalid certificate", "review").unwrap();
    uncertain()
        .lock()
        .unwrap()
        .insert((directory(root, id), operation));
    let before = snapshot(root);
    assert_eq!(
        import_operation(root, id, operation).unwrap().unwrap()["importOperation"]["state"],
        "UNKNOWN"
    );
    assert!(import_with_receipt(root, id, Uuid::new_v4(), "other certificate", "review").is_err());
    assert_eq!(snapshot(root), before);
}
#[test]
fn legacy_signed_receipt_serialization_stays_exact_and_is_not_an_upload_operation_receipt() {
    let id = Uuid::new_v4();
    let original = format!(
        "{{\"requestId\":\"{id}\",\"certificateSha256\":\"{}\",\"fingerprint\":\"{}\"}}",
        "a".repeat(64),
        "b".repeat(64)
    );
    let signed: Signed = serde_json::from_str(&original).unwrap();
    assert_eq!(serde_json::to_string(&signed).unwrap(), original);
    assert!(signed.import_operation_id.is_none());
    assert!(signed.import_accepted_sha256.is_none());
}
