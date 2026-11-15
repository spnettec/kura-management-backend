use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tonic::transport::server::Connected;

const CA1: &str = include_str!("../tests/fixtures/tls-live/ca-one.pem");
const CA2: &str = include_str!("../tests/fixtures/tls-live/ca-two.pem");
const CERT1: &str = include_str!("../tests/fixtures/tls-live/server-one.pem");
const KEY1: &str = include_str!("../tests/fixtures/tls-live/server-one.key");
const CERT2: &str = include_str!("../tests/fixtures/tls-live/server-two.pem");
const KEY2: &str = include_str!("../tests/fixtures/tls-live/server-two.key");

fn fixture() -> (tempfile::TempDir, Config) {
    let dir = tempfile::tempdir().unwrap();
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
    for (name, value) in [("server.pem", CERT1), ("server.key", KEY1), ("ca.pem", CA1)] {
        files::write_new(&dir.path().join(name), value.as_bytes()).unwrap();
    }
    let state = dir.path().join("state");
    let config = Config {
        http_addr: "127.0.0.1:0".parse().unwrap(),
        grpc_addr: "127.0.0.1:0".parse().unwrap(),
        origin: "https://localhost".into(),
        tls_cert: dir.path().join("server.pem").to_str().unwrap().into(),
        tls_key: dir.path().join("server.key").to_str().unwrap().into(),
        client_ca: dir.path().join("ca.pem").to_str().unwrap().into(),
        web_dir: String::new(),
        state_dir: state.to_str().unwrap().into(),
    };
    (dir, config)
}

#[test]
fn platform_readiness_observes_only_selected_tls_without_publication_or_lease() {
    let (_dir,cfg)=fixture();
    let state=std::path::Path::new(&cfg.state_dir);
    let first=crate::settings::certificate_fingerprint(CERT1).unwrap();
    assert_eq!(readiness_fingerprint(&cfg).unwrap(),first);
    assert!(!state.exists());
    let runtime=TlsRuntime::open(cfg.clone()).unwrap();
    let request=stage(&runtime);
    let pointer=state.join("tls-runtime-active.json");
    let before=fs::read(&pointer).unwrap();
    assert_eq!(readiness_fingerprint(&cfg).unwrap(),first);
    assert_eq!(fs::read(&pointer).unwrap(),before);
    assert!(runtime.operation(request.operation_id).unwrap().is_none());
    assert_eq!(runtime.activate(request,"unit-readiness-admin").unwrap()["state"],"APPLIED");
    let after=fs::read(&pointer).unwrap();
    assert_eq!(readiness_fingerprint(&cfg).unwrap(),crate::settings::certificate_fingerprint(CERT2).unwrap());
    assert_eq!(fs::read(&pointer).unwrap(),after);
}

fn stage(runtime: &TlsRuntime) -> ActivationRequest {
    let cfg = runtime.prepare_stage().unwrap();
    tls_store::stage(&cfg, CERT2, KEY2, "test-admin").unwrap();
    client_ca_store::stage(&cfg, CA2, "test-admin").unwrap();
    request(runtime)
}

fn request(runtime: &TlsRuntime) -> ActivationRequest {
    let status = runtime.status().unwrap();
    ActivationRequest {
        operation_id: Uuid::new_v4(),
        expected_generation: serde_json::from_value(status["generation"].clone()).unwrap(),
        pending_revision: status["pendingRevision"].as_str().unwrap().into(),
    }
}

#[test]
fn tls_runtime_restart_does_not_approve_staged_tls_or_ca() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    let original = runtime.status().unwrap();
    let req = stage(&runtime);
    drop(runtime);
    let runtime = TlsRuntime::open(cfg).unwrap();
    let status = runtime.status().unwrap();
    assert_eq!(status["generation"], original["generation"]);
    assert_eq!(
        status["certificateFingerprint"],
        original["certificateFingerprint"]
    );
    assert_eq!(status["caFingerprint"], original["caFingerprint"]);
    assert_eq!(status["activationRequired"], true);
    assert!(runtime.operation(req.operation_id).unwrap().is_none());
}

#[test]
fn tls_runtime_joint_activation_closes_both_old_epochs_and_reloads_exact_cohort() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    let old_web = runtime.transport(false).unwrap().1;
    let old_grpc = runtime.transport(true).unwrap().1;
    let req = stage(&runtime);
    let operation = runtime.activate(req.clone(), "test-admin").unwrap();
    assert_eq!(operation["state"], "APPLIED");
    assert_eq!(operation["observedActive"], true);
    assert!(old_web.is_cancelled() && old_grpc.is_cancelled());
    let status = runtime.status().unwrap();
    assert_eq!(
        status["certificateFingerprint"],
        crate::settings::certificate_fingerprint(CERT2).unwrap()
    );
    assert_eq!(
        status["caFingerprint"],
        crate::settings::certificate_fingerprint(CA2).unwrap()
    );
    assert_eq!(status["activationRequired"], false);
    drop(runtime);
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    assert_eq!(
        runtime.status().unwrap()["generation"],
        req.operation_id.to_string()
    );
    assert_eq!(fs::read_to_string(&cfg.tls_cert).unwrap(), CERT1);
    assert_eq!(fs::read_to_string(&cfg.client_ca).unwrap(), CA1);
    // Consumed pending entries can be retired for a subsequent upload without
    // touching immutable generations or bootstrap mounts.
    let effective = runtime.prepare_stage().unwrap();
    client_ca_store::stage(&effective, CA1, "test-admin").unwrap();
}

#[test]
fn tls_runtime_missing_receipt_stays_unknown_on_duplicate_and_restart() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    let req = stage(&runtime);
    runtime.activate(req.clone(), "test-admin").unwrap();
    let receipt = runtime.operation_path(req.operation_id, "receipt");
    fs::remove_file(&receipt).unwrap(); // Test-only lost durable acknowledgement.
    let pointer = fs::read(Path::new(&cfg.state_dir).join("tls-runtime-active.json")).unwrap();
    let observed = runtime.activate(req.clone(), "test-admin").unwrap();
    assert_eq!(observed["state"], "UNKNOWN");
    assert_eq!(observed["observedActive"], true);
    assert_eq!(
        runtime.status().unwrap()["blockingOperationId"],
        req.operation_id.to_string()
    );
    let alternative = request(&runtime);
    assert_eq!(
        runtime
            .activate(alternative.clone(), "another-admin")
            .unwrap()["state"],
        "REJECTED"
    );
    assert!(
        !runtime
            .operation_path(alternative.operation_id, "intent")
            .exists()
    );
    assert!(!receipt.exists());
    assert_eq!(
        pointer,
        fs::read(Path::new(&cfg.state_dir).join("tls-runtime-active.json")).unwrap()
    );
    drop(runtime);
    let runtime = TlsRuntime::open(cfg).unwrap();
    assert_eq!(
        runtime.activate(req, "test-admin").unwrap()["state"],
        "UNKNOWN"
    );
    assert!(!receipt.exists());
}

#[test]
fn tls_runtime_stale_review_or_changed_material_cannot_publish_intent() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    let mut req = stage(&runtime);
    req.pending_revision = "stale".into();
    assert_eq!(
        runtime.activate(req.clone(), "test-admin").unwrap()["state"],
        "REJECTED"
    );
    assert!(!runtime.operation_path(req.operation_id, "intent").exists());
    req = request(&runtime);
    fs::write(&cfg.tls_key, KEY2).unwrap();
    assert_eq!(
        runtime.activate(req.clone(), "test-admin").unwrap()["state"],
        "REJECTED"
    );
    assert!(!runtime.operation_path(req.operation_id, "intent").exists());
}

#[test]
fn tls_runtime_exclusive_lease_and_original_request_binding() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    assert!(TlsRuntime::open(cfg).is_err());
    let req = stage(&runtime);
    runtime.activate(req.clone(), "test-admin").unwrap();
    let mut altered = req;
    altered.pending_revision = "different".into();
    assert!(runtime.activate(altered, "test-admin").is_err());
}

#[test]
fn tls_runtime_wrong_server_name_cannot_publish_initial_generation() {
    let (_dir, mut cfg) = fixture();
    cfg.origin = "https://wrong.invalid".into();
    let pointer = Path::new(&cfg.state_dir).join("tls-runtime-active.json");
    assert!(TlsRuntime::open(cfg).is_err());
    assert!(!pointer.exists());
}

#[test]
fn tls_runtime_joint_pointer_is_authoritative_after_legacy_migration() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    let req = stage(&runtime);
    runtime.activate(req.clone(), "test-admin").unwrap();
    drop(runtime);
    // Legacy active pointers are retained history, not selected generations.
    files::write_new(
        &Path::new(&cfg.state_dir).join("tls-active.json"),
        b"retained obsolete metadata",
    )
    .unwrap();
    let runtime = TlsRuntime::open(cfg).unwrap();
    assert_eq!(
        runtime.status().unwrap()["generation"],
        req.operation_id.to_string()
    );
}

#[test]
fn tls_runtime_receipt_publication_failure_stays_unknown_and_blocks_another_id() {
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg).unwrap();
    let req = stage(&runtime);
    let receipt = runtime.operation_path(req.operation_id, "receipt");
    fs::create_dir(&receipt).unwrap(); // Test-only EEXIST during receipt publication.
    let result = runtime.activate(req.clone(), "test-admin").unwrap();
    assert_eq!(result["state"], "UNKNOWN");
    assert_eq!(result["observedActive"], true);
    assert_eq!(
        runtime.status().unwrap()["blockingOperationId"],
        req.operation_id.to_string()
    );
    assert!(runtime.prepare_stage().is_err());
    assert_eq!(
        runtime.activate(req, "test-admin").unwrap()["state"],
        "UNKNOWN"
    );
    assert!(receipt.is_dir());
}

fn client_config(ca: &str, cert: Option<(&str, &str)>) -> Arc<rustls::ClientConfig> {
    let mut roots = rustls::RootCertStore::empty();
    for der in rustls_pemfile::certs(&mut ca.as_bytes()) {
        roots.add(der.unwrap()).unwrap();
    }
    let builder = rustls::ClientConfig::builder().with_root_certificates(roots);
    let mut client = if let Some((cert, key)) = cert {
        let cert = rustls_pemfile::certs(&mut cert.as_bytes())
            .collect::<std::io::Result<Vec<_>>>()
            .unwrap();
        let key = rustls_pemfile::private_key(&mut key.as_bytes())
            .unwrap()
            .unwrap();
        builder.with_client_auth_cert(cert, key).unwrap()
    } else {
        builder.with_no_client_auth()
    };
    client.alpn_protocols = vec![b"h2".to_vec()];
    Arc::new(client)
}

#[tokio::test]
async fn tls_runtime_real_mtls_preserves_peer_identity_and_closes_old_streams() {
    use tokio_stream::StreamExt;
    let (_dir, cfg) = fixture();
    let runtime = TlsRuntime::open(cfg).unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let incoming = crate::tls_transport::grpc_incoming(listener, runtime.clone());
    let (send, mut receive) = tokio::sync::mpsc::channel(4);
    let server = tokio::spawn(async move {
        tokio::pin!(incoming);
        while let Some(Ok(mut stream)) = incoming.next().await {
            let identity = auth::hash(stream.connect_info().tls.peer_certs().unwrap()[0].as_ref());
            send.send(identity).await.unwrap();
            tokio::spawn(async move {
                let mut byte = [0];
                while stream.read_exact(&mut byte).await.is_ok() {
                    if stream.write_all(&byte).await.is_err() {
                        break;
                    }
                }
            });
        }
    });
    let cert1 = include_str!("../tests/fixtures/tls-live/client-one.pem");
    let key1 = include_str!("../tests/fixtures/tls-live/client-one.key");
    let cert2 = include_str!("../tests/fixtures/tls-live/client-two.pem");
    let key2 = include_str!("../tests/fixtures/tls-live/client-two.key");
    let connect = |config| async move {
        tokio_rustls::TlsConnector::from(config)
            .connect(
                rustls::pki_types::ServerName::try_from("localhost").unwrap(),
                tokio::net::TcpStream::connect(address).await.unwrap(),
            )
            .await
    };
    let mut old = connect(client_config(CA1, Some((cert1, key1))))
        .await
        .unwrap();
    assert_eq!(
        receive.recv().await.unwrap(),
        crate::settings::certificate_fingerprint(cert1).unwrap()
    );
    old.write_all(b"a").await.unwrap();
    assert_eq!(old.read_u8().await.unwrap(), b'a');
    let req = stage(&runtime);
    runtime.activate(req, "test-admin").unwrap();
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(2), old.read_u8())
            .await
            .unwrap()
            .is_err()
    );
    let mut new = connect(client_config(CA2, Some((cert2, key2))))
        .await
        .unwrap();
    assert_eq!(
        receive.recv().await.unwrap(),
        crate::settings::certificate_fingerprint(cert2).unwrap()
    );
    new.write_all(b"b").await.unwrap();
    assert_eq!(new.read_u8().await.unwrap(), b'b');
    // Current server trust accepts neither an old client nor an absent client.
    for identity in [Some((cert1, key1)), None] {
        match connect(client_config(CA2, identity)).await {
            Err(_) => {}
            Ok(mut rejected) => {
                let _ = rejected.write_all(b"c").await;
                assert!(
                    tokio::time::timeout(std::time::Duration::from_secs(2), rejected.read_u8())
                        .await
                        .unwrap()
                        .is_err()
                );
            }
        }
    }
    assert!(receive.try_recv().is_err());
    server.abort();
}
