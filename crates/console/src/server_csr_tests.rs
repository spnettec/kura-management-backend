//! Fresh cryptographic requests, independent from sealed TLS acceptance fixtures.
use super::*;
use std::os::unix::fs::PermissionsExt;

pub(crate) fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
    dir
}
fn request_input() -> Request {
    Request {
        request_id: Uuid::new_v4(),
        origin: "https://localhost:18443".into(),
    }
}
fn run(command: &mut Command) {
    let output = command.env("OPENSSL_CONF", "/dev/null").output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
// The external authority lives outside the application's state tree. Its key
// is never supplied to create/import/identity/stage or an HTTP request.
pub(crate) fn sign(state: &Path, id: Uuid, extensions: &str, days: &str) -> String {
    let ca = fixture();
    let ca_cert = ca.path().join("ca.pem");
    let ca_key = ca.path().join("ca.key");
    let ca_config = ca.path().join("ca.cnf");
    fs::write(&ca_config,"[req]\nprompt=no\ndistinguished_name=dn\nx509_extensions=ca\n[dn]\nCN=External Review CA\n[ca]\nbasicConstraints=critical,CA:TRUE\nkeyUsage=critical,keyCertSign,cRLSign\n").unwrap();
    run(Command::new("openssl")
        .args([
            "req",
            "-x509",
            "-newkey",
            "ec",
            "-pkeyopt",
            "ec_paramgen_curve:P-256",
            "-nodes",
            "-days",
            "2",
            "-config",
        ])
        .arg(ca_config)
        .arg("-keyout")
        .arg(&ca_key)
        .arg("-out")
        .arg(&ca_cert));
    let extension_file = ca.path().join("leaf.cnf");
    fs::write(&extension_file, extensions).unwrap();
    let certificate = ca.path().join("leaf.pem");
    if days == "-1" {
        fs::write(ca.path().join("index"), "").unwrap();
        fs::write(ca.path().join("serial"), "01\n").unwrap();
        let config = ca.path().join("sign.cnf");
        fs::write(&config,format!("[ca]\ndefault_ca=review\n[review]\ndatabase={}\nserial={}\nnew_certs_dir={}\ncertificate={}\nprivate_key={}\ndefault_md=sha256\npolicy=names\n[names]\ncommonName=supplied\n[leaf]\n{}",ca.path().join("index").display(),ca.path().join("serial").display(),ca.path().display(),ca_cert.display(),ca_key.display(),extensions)).unwrap();
        run(Command::new("openssl")
            .args(["ca", "-batch", "-notext", "-config"])
            .arg(config)
            .args([
                "-startdate",
                "20200101000000Z",
                "-enddate",
                "20200102000000Z",
                "-extensions",
                "leaf",
                "-in",
            ])
            .arg(directory(state, id).join("request.csr"))
            .arg("-out")
            .arg(&certificate));
    } else {
        run(Command::new("openssl")
            .args(["x509", "-req", "-in"])
            .arg(directory(state, id).join("request.csr"))
            .arg("-CA")
            .arg(&ca_cert)
            .arg("-CAkey")
            .arg(&ca_key)
            .args(["-set_serial", "1", "-days", days, "-sha256", "-extfile"])
            .arg(extension_file)
            .arg("-out")
            .arg(&certificate));
    }
    if days == "1" {
        run(Command::new("openssl")
            .arg("verify")
            .arg("-CAfile")
            .arg(ca_cert)
            .arg(&certificate));
    }
    fs::read_to_string(certificate).unwrap()
}
pub(crate) const EXTENSIONS: &str = "basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:localhost\n";
fn config(state: &Path) -> crate::config::Config {
    for (name, bytes) in [
        (
            "active.pem",
            include_str!("../tests/fixtures/tls-live/server-one.pem"),
        ),
        (
            "active.key",
            include_str!("../tests/fixtures/tls-live/server-one.key"),
        ),
        (
            "ca.pem",
            include_str!("../tests/fixtures/tls-live/ca-one.pem"),
        ),
    ] {
        files::write_new(&state.join(name), bytes.as_bytes()).unwrap();
    }
    crate::config::Config {
        http_addr: "127.0.0.1:18443".parse().unwrap(),
        grpc_addr: "127.0.0.1:18444".parse().unwrap(),
        origin: "https://localhost:18443".into(),
        tls_cert: state.join("active.pem").to_string_lossy().into(),
        tls_key: state.join("active.key").to_string_lossy().into(),
        client_ca: state.join("ca.pem").to_string_lossy().into(),
        web_dir: String::new(),
        state_dir: state.to_string_lossy().into(),
    }
}
#[test]
fn server_csr_reads_do_not_create_material() {
    let dir = fixture();
    assert!(current(dir.path()).unwrap().is_none());
    assert!(observe(dir.path(), Uuid::new_v4()).unwrap().is_none());
    assert!(uncertain_stage(dir.path()).unwrap().is_none());
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0);
}
#[test]
fn server_csr_generates_verified_public_csr_and_never_regenerates_same_identity() {
    let dir = fixture();
    let input = request_input();
    let id = input.request_id;
    let ready = create(dir.path(), input.clone(), "admin").unwrap();
    assert_eq!(ready["state"], "READY");
    assert!(
        ready["csrPem"]
            .as_str()
            .unwrap()
            .starts_with("-----BEGIN CERTIFICATE REQUEST-----")
    );
    assert!(!ready.to_string().contains("PRIVATE KEY"));
    let key = files::read_private(&directory(dir.path(), id).join("server.key")).unwrap();
    run(Command::new("openssl")
        .args(["req", "-verify", "-noout", "-in"])
        .arg(directory(dir.path(), id).join("request.csr")));
    let description = Command::new("openssl")
        .args(["req", "-text", "-noout", "-in"])
        .arg(directory(dir.path(), id).join("request.csr"))
        .output()
        .unwrap();
    let text = String::from_utf8(description.stdout).unwrap();
    for required in [
        "DNS:localhost",
        "CA:FALSE",
        "TLS Web Server Authentication",
        "Digital Signature",
    ] {
        assert!(text.contains(required), "{required}");
    }
    assert_eq!(create(dir.path(), input.clone(), "admin").unwrap(), ready);
    assert_eq!(
        key,
        files::read_private(&directory(dir.path(), id).join("server.key")).unwrap()
    );
    let changed = Request {
        origin: "https://elsewhere.invalid".into(),
        ..input
    };
    assert!(create(dir.path(), changed, "admin").is_err());
    let second = request_input();
    let rejected = create(dir.path(), second.clone(), "admin").unwrap();
    assert_eq!(rejected["state"], "REJECTED");
    assert_eq!(rejected["blockingRequestId"], id.to_string());
    assert!(
        !directory(dir.path(), second.request_id)
            .join("server.key")
            .exists()
    );
    assert_eq!(
        current(dir.path()).unwrap().unwrap()["requestId"],
        id.to_string()
    );
    for name in ["server.key", "request.csr", "accepted.json", "ready.json"] {
        assert_eq!(
            fs::metadata(directory(dir.path(), id).join(name))
                .unwrap()
                .mode()
                & 0o777,
            0o600
        );
    }
}
#[test]
fn server_csr_accepts_external_signature_and_stages_without_changing_active_tls() {
    let dir = fixture();
    let cfg = config(dir.path());
    let runtime = crate::tls_runtime::TlsRuntime::open(cfg.clone()).unwrap();
    let active = runtime.status().unwrap();
    let input = request_input();
    let id = input.request_id;
    create(dir.path(), input, "admin").unwrap();
    let cert = sign(dir.path(), id, EXTENSIONS, "1");
    let signed = import_certificate(dir.path(), id, &cert).unwrap();
    assert_eq!(signed["state"], "SIGNED");
    assert_eq!(signed["certificatePem"], cert);
    assert!(!signed.to_string().contains("PRIVATE KEY"));
    assert_eq!(import_certificate(dir.path(), id, &cert).unwrap(), signed);
    assert!(import_certificate(dir.path(), id, &format!("{cert}\n")).is_err());
    assert!(identity(dir.path(), id, "https://changed.invalid").is_err());
    assert_eq!(stage(&cfg, id, "admin").unwrap()["state"], "STAGED");
    assert_eq!(stage(&cfg, id, "admin").unwrap()["state"], "STAGED");
    let after = runtime.status().unwrap();
    assert_eq!(after["generation"], active["generation"]);
    assert_eq!(
        after["certificateFingerprint"],
        active["certificateFingerprint"]
    );
    assert_eq!(
        after["pendingTlsFingerprint"],
        signed["certificateFingerprint"]
    );
    assert_eq!(
        fs::read_to_string(&cfg.tls_cert).unwrap(),
        include_str!("../tests/fixtures/tls-live/server-one.pem")
    );
    assert!(current(dir.path()).unwrap().is_none());
}
#[test]
fn server_csr_rejects_wrong_key_hostname_usage_ca_and_expiration_before_import_intent() {
    let dir = fixture();
    let input = request_input();
    let id = input.request_id;
    create(dir.path(), input, "admin").unwrap();
    let other = fixture();
    let other_input = request_input();
    create(other.path(), other_input.clone(), "admin").unwrap();
    let bad_key = sign(other.path(), other_input.request_id, EXTENSIONS, "1");
    assert_eq!(
        import_certificate(dir.path(), id, &bad_key).unwrap()["certificateRejected"],
        true
    );
    for extensions in [
        EXTENSIONS.replace("DNS:localhost", "DNS:wrong.invalid"),
        EXTENSIONS.replace("serverAuth", "clientAuth"),
        EXTENSIONS.replace("CA:FALSE", "CA:TRUE"),
    ] {
        let cert = sign(dir.path(), id, &extensions, "1");
        assert_eq!(
            import_certificate(dir.path(), id, &cert).unwrap()["certificateRejected"],
            true
        );
    }
    let expired = sign(dir.path(), id, EXTENSIONS, "-1");
    assert_eq!(
        import_certificate(dir.path(), id, &expired).unwrap()["certificateRejected"],
        true
    );
    assert!(!directory(dir.path(), id).join("sign-intent.json").exists());
    assert_eq!(observe(dir.path(), id).unwrap().unwrap()["state"], "READY");
}
#[test]
fn server_csr_missing_generation_receipt_is_unknown_and_cannot_be_replaced() {
    let dir = fixture();
    let input = request_input();
    let id = input.request_id;
    create(dir.path(), input.clone(), "admin").unwrap();
    let key = files::read_private(&directory(dir.path(), id).join("server.key")).unwrap();
    fs::remove_file(directory(dir.path(), id).join("ready.json")).unwrap();
    assert_eq!(
        create(dir.path(), input, "admin").unwrap()["state"],
        "UNKNOWN"
    );
    assert_eq!(
        import_certificate(dir.path(), id, "unused").unwrap()["state"],
        "UNKNOWN"
    );
    assert!(retire(dir.path(), id).is_err());
    assert_eq!(
        create(dir.path(), request_input(), "admin").unwrap()["state"],
        "REJECTED"
    );
    assert_eq!(
        key,
        files::read_private(&directory(dir.path(), id).join("server.key")).unwrap()
    );
}
#[test]
fn server_csr_missing_sign_receipt_never_reimports_or_retires() {
    let dir = fixture();
    let input = request_input();
    let id = input.request_id;
    create(dir.path(), input, "admin").unwrap();
    let cert = sign(dir.path(), id, EXTENSIONS, "1");
    import_certificate(dir.path(), id, &cert).unwrap();
    fs::remove_file(directory(dir.path(), id).join("signed.json")).unwrap();
    assert_eq!(
        import_certificate(dir.path(), id, &cert).unwrap()["state"],
        "UNKNOWN"
    );
    assert!(!directory(dir.path(), id).join("signed.json").exists());
    assert!(retire(dir.path(), id).is_err());
    assert_eq!(
        create(dir.path(), request_input(), "admin").unwrap()["state"],
        "REJECTED"
    );
}
#[test]
fn server_csr_missing_stage_receipt_blocks_legacy_upload_and_activation() {
    let dir = fixture();
    let cfg = config(dir.path());
    let runtime = crate::tls_runtime::TlsRuntime::open(cfg.clone()).unwrap();
    let input = request_input();
    let id = input.request_id;
    create(dir.path(), input, "admin").unwrap();
    let cert = sign(dir.path(), id, EXTENSIONS, "1");
    import_certificate(dir.path(), id, &cert).unwrap();
    stage(&cfg, id, "admin").unwrap();
    fs::remove_file(directory(dir.path(), id).join("staged.json")).unwrap();
    let before = crate::tls_store::pending_fingerprint(&cfg).unwrap();
    assert_eq!(stage(&cfg, id, "admin").unwrap()["state"], "UNKNOWN");
    assert!(!directory(dir.path(), id).join("staged.json").exists());
    assert!(runtime.prepare_stage().is_err());
    let status = runtime.status().unwrap();
    let activation_id = Uuid::new_v4();
    let activation = runtime
        .activate(
            crate::tls_runtime::ActivationRequest {
                operation_id: activation_id,
                expected_generation: serde_json::from_value(status["generation"].clone()).unwrap(),
                pending_revision: status["pendingRevision"].as_str().unwrap().into(),
            },
            "admin",
        )
        .unwrap();
    assert_eq!(activation["state"], "REJECTED");
    assert_eq!(
        runtime.status().unwrap()["generation"],
        status["generation"]
    );
    assert!(
        !dir.path()
            .join(format!("tls-operations/{activation_id}.intent.json"))
            .exists()
    );

    assert!(ensure_no_uncertain_stage(dir.path()).is_err());
    assert_eq!(
        runtime.status().unwrap()["blockingCsrRequestId"],
        id.to_string()
    );
    assert_eq!(before, crate::tls_store::pending_fingerprint(&cfg).unwrap());
    assert!(retire(dir.path(), id).is_err());
}
#[test]
fn server_csr_retirement_preserves_material_and_requires_a_new_identity() {
    let dir = fixture();
    let input = request_input();
    let id = input.request_id;
    let ready = create(dir.path(), input.clone(), "admin").unwrap();
    let key = files::read_private(&directory(dir.path(), id).join("server.key")).unwrap();
    assert_eq!(retire(dir.path(), id).unwrap()["state"], "RETIRED");
    assert_eq!(
        create(dir.path(), input, "admin").unwrap()["state"],
        "RETIRED"
    );
    assert_eq!(
        key,
        files::read_private(&directory(dir.path(), id).join("server.key")).unwrap()
    );
    assert!(current(dir.path()).unwrap().is_none());
    let next = create(dir.path(), request_input(), "admin").unwrap();
    assert_eq!(next["state"], "READY");
    assert_ne!(next["csrSha256"], ready["csrSha256"]);
}
#[test]
fn server_csr_concurrent_requests_generate_one_key_and_durably_reject_the_other() {
    let dir = fixture();
    let state = dir.path().to_path_buf();
    let a = request_input();
    let b = request_input();
    let start = std::sync::Arc::new(std::sync::Barrier::new(3));
    let handles: Vec<_> = [a, b]
        .into_iter()
        .map(|input| {
            let start = start.clone();
            let state = state.clone();
            std::thread::spawn(move || {
                start.wait();
                create(&state, input, "admin").unwrap()
            })
        })
        .collect();
    start.wait();
    let mut states: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap()["state"].as_str().unwrap().to_owned())
        .collect();
    states.sort();
    assert_eq!(states, vec!["READY", "REJECTED"]);
    assert_eq!(
        fs::read_dir(root(dir.path()))
            .unwrap()
            .filter(|entry| entry.as_ref().unwrap().path().join("server.key").exists())
            .count(),
        1
    );
}
#[test]
fn server_csr_invalid_origins_and_symlinks_never_generate_keys() {
    for origin in [
        "http://localhost",
        "https://localhost/path",
        "https://localhost?x=1",
        "https://user@localhost",
        "https://localhost:0",
        "https://localhost:65536",
        "https://localhost/#x",
        "https://localhost/",
        "https://LOCALHOST",
        "https://localhost:443",
    ] {
        let dir = fixture();
        assert!(
            create(
                dir.path(),
                Request {
                    origin: origin.into(),
                    ..request_input()
                },
                "admin"
            )
            .is_err()
        );
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 0, "{origin}");
    }
    let dir = fixture();
    let other = fixture();
    std::os::unix::fs::symlink(other.path(), root(dir.path())).unwrap();
    assert!(create(dir.path(), request_input(), "admin").is_err());
    assert_eq!(fs::read_dir(other.path()).unwrap().count(), 0);
}

#[test]
fn server_csr_ipv4_and_ipv6_sans_are_accepted_by_the_real_tls_provider() {
    for (origin, ip) in [
        ("https://127.0.0.1:18443", "127.0.0.1"),
        ("https://[::1]:18443", "::1"),
    ] {
        let dir = fixture();
        let input = Request {
            request_id: Uuid::new_v4(),
            origin: origin.into(),
        };
        let id = input.request_id;
        assert_eq!(
            create(dir.path(), input, "admin").unwrap()["state"],
            "READY"
        );
        let extensions = EXTENSIONS.replace("DNS:localhost", &format!("IP:{ip}"));
        let certificate = sign(dir.path(), id, &extensions, "1");
        assert_eq!(
            import_certificate(dir.path(), id, &certificate).unwrap()["state"],
            "SIGNED"
        );
        assert_eq!(identity(dir.path(), id, origin).unwrap().0, certificate);
    }
}
