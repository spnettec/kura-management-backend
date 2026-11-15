use super::*;
use std::os::unix::fs::PermissionsExt;

const FIRST: &str = include_str!("../tests/fixtures/review-ca-one.pem");
const SECOND: &str = include_str!("../tests/fixtures/review-ca-two.pem");
const LEAF: &str = include_str!("../tests/fixtures/review-not-ca.pem");
const BAD_USAGE: &str = include_str!("../tests/fixtures/review-ca-no-sign.pem");

fn fixture() -> (tempfile::TempDir, Config) {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("bootstrap-ca.pem");
    fs::write(&original, FIRST).unwrap();
    fs::set_permissions(&original, fs::Permissions::from_mode(0o400)).unwrap();
    let config = Config {
        http_addr: "127.0.0.1:0".parse().unwrap(),
        grpc_addr: "127.0.0.1:0".parse().unwrap(),
        origin: "https://fixture.invalid".into(),
        tls_cert: String::new(),
        tls_key: String::new(),
        web_dir: String::new(),
        client_ca: original.to_str().unwrap().into(),
        state_dir: temp.path().join("state").to_str().unwrap().into(),
    };
    (temp, config)
}

#[test]
fn ca_upload_rejects_leaf_non_signing_expired_future_and_private_material() {
    assert!(validate(FIRST).is_ok());
    assert!(validate(SECOND).is_ok());
    assert!(crate::tls_store::validate_leaf_certificate(FIRST).is_err());
    assert!(crate::gateway_registration::certificate_sha256(FIRST).is_err());
    assert!(crate::tls_store::validate_leaf_certificate(LEAF).is_ok());
    for value in [
        LEAF,
        BAD_USAGE,
        "-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----",
    ] {
        assert!(validate(value).is_err());
    }
    assert!(validate(&format!("{FIRST}{SECOND}")).is_err());
    assert!(validate_at(FIRST, std::time::Duration::ZERO).is_err());
    assert!(validate_at(FIRST, std::time::Duration::from_secs(10_000_000_000)).is_err());
}

#[test]
fn activation_uses_private_state_and_preserves_read_only_bootstrap_and_old_versions() {
    let (_temp, mut config) = fixture();
    let base = config.client_ca.clone();
    let original_config = config.clone();
    let fingerprint = stage(&config, SECOND, "fixture-admin").unwrap();
    assert_eq!(fs::read_to_string(&base).unwrap(), FIRST);
    assert_eq!(
        pending_fingerprint(&config).unwrap(),
        Some(fingerprint.clone())
    );
    activate(&mut config).unwrap();
    assert_ne!(config.client_ca, base);
    assert_eq!(
        validate(&fs::read_to_string(&config.client_ca).unwrap()).unwrap(),
        fingerprint
    );
    assert_eq!(fs::read_to_string(&base).unwrap(), FIRST);
    assert!(pending_fingerprint(&config).unwrap().is_none());
    let first_version = config.client_ca.clone();
    let mut restarted = original_config;
    activate(&mut restarted).unwrap();
    assert_eq!(restarted.client_ca, first_version);
    stage(&restarted, FIRST, "fixture-admin").unwrap();
    activate(&mut restarted).unwrap();
    assert_eq!(fs::read_to_string(first_version).unwrap(), SECOND);
}

#[test]
fn crash_after_active_pointer_publication_can_finish_without_reissuing_ca_update() {
    let (_temp, mut config) = fixture();
    stage(&config, SECOND, "fixture-admin").unwrap();
    fs::copy(pending(&config), active(&config)).unwrap();
    activate(&mut config).unwrap();
    assert_eq!(
        validate(&fs::read_to_string(&config.client_ca).unwrap()).unwrap(),
        validate(SECOND).unwrap()
    );
    assert!(!pending(&config).exists());
    activate(&mut config).unwrap();
}

#[test]
fn unavailable_staging_keeps_bootstrap_trust_unchanged() {
    let (_temp, config) = fixture();
    directory(Path::new(&config.state_dir)).unwrap();
    fs::write(Path::new(&config.state_dir).join("pending"), b"fault").unwrap();
    assert!(stage(&config, SECOND, "fixture-admin").is_err());
    assert_eq!(fs::read_to_string(&config.client_ca).unwrap(), FIRST);
    assert!(!active(&config).exists());
}

#[test]
fn corrupted_generation_never_publishes_active_pointer() {
    let (_temp, mut config) = fixture();
    stage(&config, SECOND, "fixture-admin").unwrap();
    let pointer = pending(&config);
    let mut generation: Generation = serde_json::from_slice(&fs::read(&pointer).unwrap()).unwrap();
    generation.fingerprint = validate(FIRST).unwrap();
    fs::write(&pointer, serde_json::to_vec(&generation).unwrap()).unwrap();
    assert!(activate(&mut config).is_err());
    assert!(!active(&config).exists());
    assert!(pointer.exists());
    assert_eq!(fs::read_to_string(&config.client_ca).unwrap(), FIRST);
}

#[test]
fn active_trust_drift_rejects_previously_approved_ca_change() {
    let (_temp, mut config) = fixture();
    stage(&config, SECOND, "fixture-admin").unwrap();
    fs::set_permissions(&config.client_ca, fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(&config.client_ca, SECOND).unwrap();
    assert!(activate(&mut config).is_err());
    assert!(!active(&config).exists());
    assert!(pending(&config).exists());
}

#[test]
fn second_request_cannot_replace_an_existing_pending_ca_change() {
    let (_temp, config) = fixture();
    stage(&config, SECOND, "first-admin").unwrap();
    let bytes = fs::read(pending(&config)).unwrap();
    assert!(stage(&config, SECOND, "second-admin").is_err());
    assert_eq!(fs::read(pending(&config)).unwrap(), bytes);
}

#[test]
fn legacy_ca_pending_is_preserved_and_never_copied_over_bootstrap() {
    let (_temp, mut config) = fixture();
    directory(Path::new(&config.state_dir)).unwrap();
    directory(&Path::new(&config.state_dir).join("pending")).unwrap();
    write_new(&legacy(&config), SECOND.as_bytes()).unwrap();
    assert!(activate(&mut config).is_err());
    assert_eq!(fs::read_to_string(&config.client_ca).unwrap(), FIRST);
    assert_eq!(fs::read_to_string(legacy(&config)).unwrap(), SECOND);
}
