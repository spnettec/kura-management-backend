use super::*;

#[tokio::test]
async fn server_csr_upload_original_receipt_http_rejection_and_signed_result_are_read_only() {
    let (_dir, setup) = fixture().await;
    let h = authenticated(&setup).await;
    let id = Uuid::new_v4();
    let base = format!("/api/setup/server-csr/{id}");
    async fn send(setup: Arc<Setup>, headers: &HeaderMap, method: &str, uri: &str, body: Value) -> axum::response::Response {
        use tower::ServiceExt;
        let mut request = axum::http::Request::builder().method(method).uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(body.to_string())).unwrap();
        request.headers_mut().extend(headers.clone());
        router(setup).oneshot(request).await.unwrap()
    }
    let response = send(setup.clone(), &h, "POST", "/api/setup/server-csr",
        json!({"requestId":id,"origin":"https://localhost:18443"})).await;
    assert_eq!(response.status(), StatusCode::OK);
    let key = setup.root.join(format!("server-csr-requests/{id}/server.key"));
    let original_key = files::read_private(&key).unwrap();
    let first = Uuid::new_v4();
    let response = send(setup.clone(), &headers(), "GET", &format!("{base}/imports/{first}"), Value::Null).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let mut without_csrf = h.clone(); without_csrf.remove("x-setup-csrf");
    assert_eq!(send(setup.clone(), &without_csrf, "PUT", &format!("{base}/certificate"),
        json!({"operationId":first,"serverPem":"invalid certificate"})).await.status(), StatusCode::FORBIDDEN);
    let response = send(setup.clone(), &h, "PUT", &format!("{base}/certificate"),
        json!({"operationId":first,"serverPem":"invalid certificate"})).await;
    assert_eq!(response.status(), StatusCode::OK); drop(response);
    let response = send(setup.clone(), &h, "GET", &format!("{base}/imports/{first}"), Value::Null).await;
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), 65536).await.unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["state"], "READY"); assert_eq!(value["importOperation"]["state"], "REJECTED");
    let certificate = crate::server_csr::tests::sign(&setup.root, id, crate::server_csr::tests::EXTENSIONS, "1");
    let second = Uuid::new_v4();
    let response = send(setup.clone(), &h, "PUT", &format!("{base}/certificate"),
        json!({"operationId":second,"serverPem":certificate})).await;
    assert_eq!(response.status(), StatusCode::OK); drop(response);
    let response = send(setup.clone(), &h, "GET", &format!("{base}/imports/{second}"), Value::Null).await;
    let bytes = axum::body::to_bytes(response.into_body(), 65536).await.unwrap();
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["state"], "SIGNED"); assert_eq!(value["importOperation"]["state"], "APPLIED");
    assert!(!String::from_utf8_lossy(&bytes).contains("PRIVATE KEY"));
    assert_eq!(files::read_private(&key).unwrap(), original_key);
    assert_eq!(fs::read_dir(setup.root.join("setup-operations")).unwrap().count(), 0);
    assert!(!setup.root.join("console-config-active.json").exists());
    assert!(!setup.root.join("pending/tls-update.json").exists());
}

#[tokio::test]
async fn platform_status_is_readonly_and_never_exports_the_installer_token() {
    let empty=tempfile::tempdir().unwrap();
    assert!(installation_status(empty.path()).is_err());
    assert_eq!(fs::read_dir(empty.path()).unwrap().count(),0);
    let (_dir,setup)=fixture().await;
    let before=fs::read(setup.root.join("setup/installation-credential.json")).unwrap();
    let observed=installation_status(&setup.root).unwrap();
    assert_eq!(observed["schemaVersion"],1);assert_eq!(observed["configured"],false);
    assert_eq!(observed["certificateFingerprint"],setup.credential.certificate_fingerprint);
    assert_eq!(observed.as_object().unwrap().len(),5);
    assert!(!observed.to_string().contains(&setup.credential.token));
    assert_eq!(before,fs::read(setup.root.join("setup/installation-credential.json")).unwrap());
    assert_eq!(fs::read_dir(setup.root.join("setup-operations")).unwrap().count(),0);
    assert!(!setup.root.join("tls-runtime-active.json").exists());
    assert!(!setup.root.join("console-config-active.json").exists());
}

#[tokio::test]
async fn platform_installer_export_reads_existing_identity_without_creating_or_changing_material() {
    let empty = tempfile::tempdir().unwrap();
    assert!(installer_credential(empty.path()).is_err());
    assert_eq!(fs::read_dir(empty.path()).unwrap().count(), 0);
    let (_dir, setup) = fixture().await;
    let names = [
        "installation-credential.json",
        "bootstrap.pem",
        "bootstrap.key",
        "bootstrap-request.cnf",
    ];
    let before: Vec<_> = names
        .iter()
        .map(|name| files::read_private(&setup.root.join("setup").join(name)).unwrap())
        .collect();
    let exported = installer_credential(&setup.root).unwrap();
    assert_eq!(
        exported["installationId"],
        setup.credential.installation_id.to_string()
    );
    assert_eq!(
        exported["certificateFingerprint"],
        setup.credential.certificate_fingerprint
    );
    assert_eq!(exported["token"], setup.credential.token);
    let after: Vec<_> = names
        .iter()
        .map(|name| files::read_private(&setup.root.join("setup").join(name)).unwrap())
        .collect();
    assert_eq!(before, after);
    assert_eq!(
        fs::read_dir(setup.root.join("setup-operations"))
            .unwrap()
            .count(),
        0
    );
}

#[tokio::test]
async fn platform_setup_liveness_is_success_and_readiness_is_unavailable() {
    use tower::ServiceExt;
    let (_dir, setup) = fixture().await;
    for (path, expected) in [
        ("/health/live", StatusCode::OK),
        ("/health/ready", StatusCode::SERVICE_UNAVAILABLE),
    ] {
        let response = router(setup.clone())
            .oneshot(
                axum::http::Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), expected);
        let body = axum::body::to_bytes(response.into_body(), 2048)
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<Value>(&body).unwrap()["status"],
            "setup-required"
        );
        assert!(!String::from_utf8_lossy(&body).contains(&setup.credential.token));
    }
}

#[tokio::test]
async fn platform_maintenance_refuses_original_accepted_record_without_mutation() {
    let (_dir, setup) = fixture().await;
    assert!(maintenance_records(&setup.root).is_ok());
    let body = input(Uuid::new_v4());
    let accepted = Intent {
        schema_version: 1,
        operation_id: body.operation_id,
        request_digest: setup.digest(&body).unwrap(),
        origin: body.origin,
        username: body.username,
        selection: None,
    };
    let path = operation_path(&setup.root, accepted.operation_id, "accepted");
    files::publish_new(&path, &serde_json::to_vec(&accepted).unwrap()).unwrap();
    let before = files::read_private(&path).unwrap();
    assert!(
        maintenance_records(&setup.root)
            .unwrap_err()
            .to_string()
            .contains("unsettled installation")
    );
    assert_eq!(files::read_private(&path).unwrap(), before);
    assert_eq!(
        operation(&setup.root, accepted.operation_id, false)
            .unwrap()
            .unwrap()["state"],
        "UNKNOWN"
    );
}
use std::os::unix::fs::{MetadataExt, PermissionsExt};

async fn fixture() -> (tempfile::TempDir, Arc<Setup>) {
    let dir = tempfile::tempdir().unwrap();
    fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
    for name in ["setup-operations", "installation-generations"] {
        files::directory(&dir.path().join(name)).unwrap();
    }
    let credential = credential(dir.path()).await.unwrap();
    let (ready, _) = watch::channel(false);
    let setup = Arc::new(Setup {
        root: dir.path().into(),
        gate: Arc::new(crate::maintenance::Gate::for_state(dir.path()).unwrap()),
        credential,
        listen: "127.0.0.1:18443".parse().unwrap(),
        web: "unused".into(),
        changes: Arc::new(AsyncMutex::new(())),
        slots: Semaphore::new(2),
        job: Mutex::new(None),
        ready,
        pairing_attempts: Mutex::new(Default::default()),
        uncertain_receipt: Mutex::new(None),
    });
    (dir, setup)
}
fn headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(header::HOST, "localhost:18443".parse().unwrap());
    h.insert(header::ORIGIN, "https://localhost:18443".parse().unwrap());
    h
}
fn success<T>(result: Result<T, ApiError>) -> T {
    result.map_err(|e| e.into_response().status()).unwrap()
}
async fn authenticated(setup: &Arc<Setup>) -> HeaderMap {
    let response = success(
        pairing(
            State(setup.clone()),
            headers(),
            Ok(Json(PairInput {
                installation_id: setup.credential.installation_id,
                token: setup.credential.token.clone(),
            })),
        )
        .await,
    );
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(cookie.contains("HttpOnly; Secure; SameSite=Strict; Path=/api/setup"));
    let mut h = headers();
    h.insert(
        header::COOKIE,
        cookie.split(';').next().unwrap().parse().unwrap(),
    );
    let s = setup
        .session(&h, false)
        .map_err(|e| e.into_response().status())
        .unwrap();
    h.insert("x-setup-csrf", s.csrf.parse().unwrap());
    h
}
fn input(id: Uuid) -> SetupInput {
    SetupInput {
        operation_id: id,
        origin: "https://localhost:18443".into(),
        grpc_address: "127.0.0.1:18444".parse().unwrap(),
        database_url: "postgres://never-connect@127.0.0.1:1/unused".into(),
        username: "organization-admin".into(),
        tenant_id: "organization".into(),
        password: "A-strong-initial-password".into(),
        server_pem: include_str!("../tests/fixtures/tls-live/server-one.pem").into(),
        server_key: include_str!("../tests/fixtures/tls-live/server-one.key").into(),
        server_csr_id: None,
        client_ca_pem: include_str!("../tests/fixtures/tls-live/ca-one.pem").into(),
    }
}

#[tokio::test]
async fn initial_setup_credential_is_private_leaf_identity_and_never_regenerated() {
    let (dir, setup) = fixture().await;
    let file = dir.path().join("setup/installation-credential.json");
    let before = fs::read(&file).unwrap();
    let restored = credential(dir.path()).await.unwrap();
    assert_eq!(restored.token, setup.credential.token);
    assert_eq!(before, fs::read(&file).unwrap());
    for path in [
        file,
        dir.path().join("setup/bootstrap.key"),
        dir.path().join("setup/bootstrap.pem"),
    ] {
        let m = path.metadata().unwrap();
        assert_eq!(m.mode() & 0o777, 0o600);
        assert_eq!(m.uid(), unsafe { libc::geteuid() });
    }
    assert!(
        crate::client_ca_store::validate(
            &fs::read_to_string(dir.path().join("setup/bootstrap.pem")).unwrap()
        )
        .is_err()
    );
    let key = fs::read(dir.path().join("setup/bootstrap.key")).unwrap();
    fs::remove_file(dir.path().join("setup/installation-credential.json")).unwrap();
    assert!(credential(dir.path()).await.is_err());
    assert_eq!(
        key,
        fs::read(dir.path().join("setup/bootstrap.key")).unwrap()
    );
}

#[tokio::test]
async fn initial_setup_pairing_requires_installation_identity_origin_and_csrf() {
    let (_dir, setup) = fixture().await;
    let bad = pairing(
        State(setup.clone()),
        headers(),
        Ok(Json(PairInput {
            installation_id: Uuid::new_v4(),
            token: setup.credential.token.clone(),
        })),
    )
    .await;
    assert_eq!(
        bad.err().unwrap().into_response().status(),
        StatusCode::UNAUTHORIZED
    );
    let mut cross = headers();
    cross.insert(header::ORIGIN, "https://attacker.invalid".parse().unwrap());
    let bad = pairing(
        State(setup.clone()),
        cross,
        Ok(Json(PairInput {
            installation_id: setup.credential.installation_id,
            token: setup.credential.token.clone(),
        })),
    )
    .await;
    assert_eq!(
        bad.err().unwrap().into_response().status(),
        StatusCode::FORBIDDEN
    );
    let mut h = authenticated(&setup).await;
    h.remove("x-setup-csrf");
    let rejected = submission(State(setup.clone()), h, Ok(Json(input(Uuid::new_v4())))).await;
    assert_eq!(
        rejected.err().unwrap().into_response().status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        fs::read_dir(setup.root.join("setup-operations"))
            .unwrap()
            .count(),
        0
    );
    let Json(info) = success(info(State(setup.clone()), HeaderMap::new()).await);
    assert_eq!(info["authenticated"], false);
    assert!(info.get("csrf").is_none());
    assert!(!info.to_string().contains(&setup.credential.token));
}

#[tokio::test]
async fn initial_setup_accepted_without_live_handle_is_unknown_and_blocks_another_id() {
    let (_dir, setup) = fixture().await;
    let h = authenticated(&setup).await;
    let id = Uuid::new_v4();
    let body = input(id);
    let accepted = Intent {
        schema_version: 1,
        operation_id: id,
        request_digest: setup.digest(&body).unwrap(),
        origin: body.origin.clone(),
        username: body.username.clone(),
        selection: None,
    };
    files::publish_new(
        &operation_path(&setup.root, id, "accepted"),
        &serde_json::to_vec(&accepted).unwrap(),
    )
    .unwrap();
    assert_eq!(setup.read(id).unwrap().unwrap()["state"], "UNKNOWN");
    assert_eq!(setup.blocking().unwrap(), Some(id));
    let Json(same) = success(submission(State(setup.clone()), h.clone(), Ok(Json(body))).await);
    assert_eq!(same["state"], "UNKNOWN");
    let alternative = Uuid::new_v4();
    let Json(rejected) = success(
        submission(
            State(setup.clone()),
            h.clone(),
            Ok(Json(input(alternative))),
        )
        .await,
    );
    assert_eq!(rejected["state"], "REJECTED");
    let mut altered = input(id);
    altered.password.push('x');
    assert_eq!(
        submission(State(setup.clone()), h, Ok(Json(altered)))
            .await
            .err()
            .unwrap()
            .into_response()
            .status(),
        StatusCode::CONFLICT
    );
    assert!(!operation_path(&setup.root, id, "intent").exists());
    assert!(!console_settings::generation(&setup.root, id).exists());
}

#[tokio::test]
async fn initial_setup_pending_requires_a_live_job_and_never_a_lock_or_timestamp() {
    let (_dir, setup) = fixture().await;
    let id = Uuid::new_v4();
    let body = input(id);
    let accepted = Intent {
        schema_version: 1,
        operation_id: id,
        request_digest: setup.digest(&body).unwrap(),
        origin: body.origin,
        username: body.username,
        selection: None,
    };
    files::publish_new(
        &operation_path(&setup.root, id, "accepted"),
        &serde_json::to_vec(&accepted).unwrap(),
    )
    .unwrap();
    let handle = tokio::spawn(std::future::pending::<()>());
    *setup.job.lock().unwrap() = Some(Job { id, handle });
    assert_eq!(setup.read(id).unwrap().unwrap()["state"], "PENDING");
    setup.job.lock().unwrap().as_ref().unwrap().handle.abort();
    tokio::task::yield_now().await;
    assert_eq!(setup.read(id).unwrap().unwrap()["state"], "UNKNOWN");
}

#[tokio::test]
async fn initial_setup_invalid_material_is_durably_rejected_without_database_writes() {
    let (_dir, setup) = fixture().await;
    let h = authenticated(&setup).await;
    let id = Uuid::new_v4();
    let mut body = input(id);
    body.server_key = "not a key".into();
    let Json(accepted) = success(submission(State(setup.clone()), h, Ok(Json(body))).await);
    assert_eq!(accepted["state"], "PENDING");
    let handle = setup.job.lock().unwrap().take().unwrap().handle;
    handle.await.unwrap();
    assert_eq!(setup.read(id).unwrap().unwrap()["state"], "REJECTED");
    assert_eq!(setup.blocking().unwrap(), None);
    assert!(!operation_path(&setup.root, id, "intent").exists());
    assert!(!setup.root.join("console-config-active.json").exists());
}

fn configured_files(setup: &Setup) -> (SavedSettings, Selection) {
    let id = Uuid::new_v4();
    let dir = console_settings::generation(&setup.root, id);
    files::directory(&dir).unwrap();
    let body = input(id);
    for (name, value) in [
        ("server.pem", body.server_pem),
        ("server.key", body.server_key),
        ("client-ca.pem", body.client_ca_pem),
    ] {
        files::write_new(&dir.join(name), value.as_bytes()).unwrap();
    }
    crate::task_crypto::create_key(&dir.join("task-data.key")).unwrap();
    let config = Config {
        http_addr: setup.listen,
        grpc_addr: body.grpc_address,
        origin: body.origin,
        tls_cert: dir.join("server.pem").to_string_lossy().into(),
        tls_key: dir.join("server.key").to_string_lossy().into(),
        client_ca: dir.join("client-ca.pem").to_string_lossy().into(),
        web_dir: setup.web.clone(),
        state_dir: setup.root.to_string_lossy().into(),
    };
    let settings = SavedSettings {
        schema_version: 1,
        installation_id: setup.credential.installation_id,
        operation_id: id,
        database_url: body.database_url,
        data_key_file: dir.join("task-data.key").to_string_lossy().into(),
        config,
        material_sha256: ["server.pem", "server.key", "client-ca.pem", "task-data.key"]
            .iter()
            .map(|n| auth::hash(&files::read_private(&dir.join(n)).unwrap()))
            .collect(),
    };
    let bytes = serde_json::to_vec(&settings).unwrap();
    files::write_new(&dir.join("settings.json"), &bytes).unwrap();
    let selected = Selection {
        schema_version: 1,
        operation_id: id,
        digest: auth::hash(&bytes),
    };
    files::publish_new(
        &setup.root.join("console-config-active.json"),
        &serde_json::to_vec(&selected).unwrap(),
    )
    .unwrap();
    (settings, selected)
}

#[tokio::test]
async fn initial_setup_saved_configuration_rejects_tampering_and_paths_outside_generation() {
    let (_dir, setup) = fixture().await;
    let (mut saved, mut selected) = configured_files(&setup);
    assert!(console_settings::load_at(&setup.root).unwrap().is_some());
    let path = console_settings::generation(&setup.root, saved.operation_id).join("settings.json");
    let before = fs::read(&path).unwrap();
    fs::write(&path, b"corrupted").unwrap();
    assert!(console_settings::load_at(&setup.root).is_err());
    fs::write(&path, &before).unwrap();
    saved.data_key_file = setup.root.join("outside.key").to_string_lossy().into();
    let bytes = serde_json::to_vec(&saved).unwrap();
    fs::write(&path, &bytes).unwrap();
    selected.digest = auth::hash(&bytes);
    files::publish_replace(
        &setup.root.join("console-config-active.json"),
        &serde_json::to_vec(&selected).unwrap(),
    )
    .unwrap();
    assert!(console_settings::load_at(&setup.root).is_err());
}

#[tokio::test]
async fn initial_setup_client_only_certificate_cannot_be_used_as_server_identity() {
    let (_dir, setup) = fixture().await;
    let (saved, _) = configured_files(&setup);
    fs::write(
        &saved.config.tls_cert,
        include_str!("../tests/fixtures/tls-live/client-one.pem"),
    )
    .unwrap();
    fs::write(
        &saved.config.tls_key,
        include_str!("../tests/fixtures/tls-live/client-one.key"),
    )
    .unwrap();
    let result = crate::tls_runtime::validate_config(&saved.config);
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("extended key usage")
    );
}

#[tokio::test]
async fn initial_setup_public_original_record_never_discloses_credentials() {
    let (_dir, setup) = fixture().await;
    let id = Uuid::new_v4();
    let body = input(id);
    let accepted = Intent {
        schema_version: 1,
        operation_id: id,
        request_digest: setup.digest(&body).unwrap(),
        origin: body.origin.clone(),
        username: body.username.clone(),
        selection: None,
    };
    files::publish_new(
        &operation_path(&setup.root, id, "accepted"),
        &serde_json::to_vec(&accepted).unwrap(),
    )
    .unwrap();
    let public = setup.read(id).unwrap().unwrap().to_string();
    for secret in [
        &body.password,
        &body.database_url,
        &body.server_key,
        &setup.credential.token,
        &accepted.request_digest,
    ] {
        assert!(!public.contains(secret));
    }
}

#[test]
fn initial_setup_native_key_encoding_is_accepted_by_the_actual_tls_provider() {
    for _ in 0..128 {
        let pem = bootstrap_key().unwrap();
        let key = rustls_pemfile::private_key(&mut pem.as_slice())
            .unwrap()
            .unwrap();
        rustls::crypto::ring::sign::any_supported_type(&key).unwrap();
    }
}

#[tokio::test]
async fn initial_setup_started_ui_journal_cannot_be_bypassed_by_legacy_fallback() {
    let (_dir, setup) = fixture().await;
    assert!(console_settings::ensure_legacy_allowed(&setup.root, false).is_ok());
    let body = input(Uuid::new_v4());
    let accepted = Intent {
        schema_version: 1,
        operation_id: body.operation_id,
        request_digest: setup.digest(&body).unwrap(),
        origin: body.origin,
        username: body.username,
        selection: None,
    };
    files::publish_new(
        &operation_path(&setup.root, accepted.operation_id, "accepted"),
        &serde_json::to_vec(&accepted).unwrap(),
    )
    .unwrap();
    assert!(console_settings::ensure_legacy_allowed(&setup.root, false).is_err());
    assert_eq!(
        setup.read(accepted.operation_id).unwrap().unwrap()["state"],
        "UNKNOWN"
    );
}

#[tokio::test]
#[ignore = "requires a distinct private first-setup PostgreSQL fixture"]
async fn initial_setup_existing_database_is_rejected_before_any_migration_or_admin_creation() {
    let url = std::env::var("YOFC_SETUP_REVIEW_PG_URL").expect("new private PostgreSQL required");
    let admin = sqlx::PgPool::connect(&url).await.unwrap();
    let schema = format!("setup_existing_{}", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    sqlx::query(&format!(
        "CREATE TABLE {schema}.retained_business(value text NOT NULL)"
    ))
    .execute(&admin)
    .await
    .unwrap();
    sqlx::query(&format!(
        "INSERT INTO {schema}.retained_business VALUES('original retained value')"
    ))
    .execute(&admin)
    .await
    .unwrap();
    let (_dir, setup) = fixture().await;
    let id = Uuid::new_v4();
    let mut body = input(id);
    assert!(!url.contains('?'));
    body.database_url = format!("{url}?options=-csearch_path%3D{schema}");
    let failure = setup
        .prepare(body)
        .await
        .err()
        .expect("non-empty database must be refused");
    assert!(failure.to_string().contains("dedicated empty database"));
    assert!(!operation_path(&setup.root, id, "intent").exists());
    assert!(!setup.root.join("console-config-active.json").exists());
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname=$1 AND c.relkind IN ('r','p','v','m','S','f')")
        .bind(&schema).fetch_one(&admin).await.unwrap();
    assert_eq!(count, 1);
    let value: String =
        sqlx::query_scalar(&format!("SELECT value FROM {schema}.retained_business"))
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(value, "original retained value");
    println!(
        "SETUP_EXISTING_DB schema={schema} originalRelations=1 finalRelations=1 originalValueRetained=true noMigrationOrAdminCreation=true"
    );
}

async fn pairing(
    state: State<Arc<Setup>>,
    headers: HeaderMap,
    input: Result<Json<PairInput>, JsonRejection>,
) -> Result<Response, ApiError> {
    super::pair(
        state,
        axum::extract::OriginalUri("https://localhost:18443/api/setup/pair".parse().unwrap()),
        headers,
        input,
    )
    .await
}
async fn submission(
    state: State<Arc<Setup>>,
    headers: HeaderMap,
    input: Result<Json<SetupInput>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    super::submit(
        state,
        axum::extract::OriginalUri(
            "https://localhost:18443/api/setup/operations"
                .parse()
                .unwrap(),
        ),
        headers,
        input,
    )
    .await
}
#[tokio::test]
async fn initial_setup_http2_authority_authenticates_without_a_host_header() {
    let (_dir, setup) = fixture().await;
    let mut h = headers();
    h.remove(header::HOST);
    let response = super::pair(
        State(setup.clone()),
        axum::extract::OriginalUri("https://localhost:18443/api/setup/pair".parse().unwrap()),
        h,
        Ok(Json(PairInput {
            installation_id: setup.credential.installation_id,
            token: setup.credential.token.clone(),
        })),
    )
    .await;
    assert_eq!(success(response).status(), StatusCode::OK);
}


#[tokio::test]
async fn server_csr_setup_http_requires_pairing_origin_and_csrf_and_never_exports_a_key() {
    use tower::ServiceExt;
    let (_dir,setup)=fixture().await;
    let id=Uuid::new_v4();
    let payload=json!({"requestId":id,"origin":"https://localhost:18443"});
    for h in [headers(), {
        let mut h=authenticated(&setup).await;h.remove("x-setup-csrf");h
    }, {
        let mut h=authenticated(&setup).await;h.insert(header::ORIGIN,"https://attacker.invalid".parse().unwrap());h
    }] {
        let mut request=axum::http::Request::builder().method("POST").uri("/api/setup/server-csr").header(header::CONTENT_TYPE,"application/json").body(axum::body::Body::from(payload.to_string())).unwrap();
        request.headers_mut().extend(h);
        let response=router(setup.clone()).oneshot(request).await.unwrap();
        assert!(matches!(response.status(),StatusCode::UNAUTHORIZED|StatusCode::FORBIDDEN));
        assert!(!setup.root.join("server-csr-requests").exists());
    }
    let h=authenticated(&setup).await;
    let mut request=axum::http::Request::builder().method("POST").uri("/api/setup/server-csr").header(header::CONTENT_TYPE,"application/json").body(axum::body::Body::from(payload.to_string())).unwrap();
    request.headers_mut().extend(h.clone());
    let response=router(setup.clone()).oneshot(request).await.unwrap();
    assert_eq!(response.status(),StatusCode::OK);
    let bytes=axum::body::to_bytes(response.into_body(),65536).await.unwrap();
    let value:Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(value["state"],"READY");
    assert!(!String::from_utf8_lossy(&bytes).contains("PRIVATE KEY"));
    let before=files::read_private(&setup.root.join(format!("server-csr-requests/{id}/server.key"))).unwrap();
    let mut request=axum::http::Request::builder().uri(format!("/api/setup/server-csr/{id}")).body(axum::body::Body::empty()).unwrap();request.headers_mut().extend(h);
    let response=router(setup.clone()).oneshot(request).await.unwrap();assert_eq!(response.status(),StatusCode::OK);
    assert_eq!(before,files::read_private(&setup.root.join(format!("server-csr-requests/{id}/server.key"))).unwrap());
    assert_eq!(fs::read_dir(setup.root.join("setup-operations")).unwrap().count(),0);
    assert!(!setup.root.join("console-config-active.json").exists());
}

#[tokio::test]
async fn server_csr_setup_input_preserves_exact_legacy_serialization_and_digest() {
    let (_dir,setup)=fixture().await;let body=input(Uuid::new_v4());
    let fields=vec![
        ("operationId",json!(body.operation_id)),("origin",json!(body.origin)),("grpcAddress",json!(body.grpc_address)),
        ("databaseUrl",json!(body.database_url)),("username",json!(body.username)),("tenantId",json!(body.tenant_id)),
        ("password",json!(body.password)),("serverPem",json!(body.server_pem)),("serverKey",json!(body.server_key)),("clientCaPem",json!(body.client_ca_pem)),
    ];
    let legacy=format!("{{{}}}",fields.into_iter().map(|(k,v)|format!("{}:{}",serde_json::to_string(k).unwrap(),v)).collect::<Vec<_>>().join(","));
    assert_eq!(serde_json::to_string(&body).unwrap(),legacy);
    let decoded:SetupInput=serde_json::from_str(&legacy).unwrap();assert!(decoded.server_csr_id.is_none());
    assert_eq!(setup.digest(&body).unwrap(),setup.digest(&decoded).unwrap());
    let mut selected=decoded;selected.server_csr_id=Some(Uuid::new_v4());
    assert_ne!(setup.digest(&body).unwrap(),setup.digest(&selected).unwrap());
}

#[tokio::test]
async fn server_csr_setup_refuses_unsettled_original_and_a_different_manual_identity() {
    use crate::server_csr;
    let (_dir,setup)=fixture().await;
    let request=server_csr::Request{request_id:Uuid::new_v4(),origin:"https://localhost:18443".into()};
    server_csr::create(&setup.root,request.clone(),"installation").unwrap();
    let body=input(Uuid::new_v4());let generation=console_settings::generation(&setup.root,body.operation_id);
    let error=setup.prepare(body).await.err().unwrap();assert!(error.to_string().contains("original CSR"));assert!(!generation.exists());
    let mut body=input(Uuid::new_v4());body.server_csr_id=Some(request.request_id);
    assert!(setup.prepare(body).await.err().unwrap().to_string().contains("generated CSR or an uploaded key"));
    let cert=server_csr::tests::sign(&setup.root,request.request_id,server_csr::tests::EXTENSIONS,"1");
    assert_eq!(server_csr::import_certificate(&setup.root,request.request_id,&cert).unwrap()["state"],"SIGNED");
    let mut body=input(Uuid::new_v4());body.server_key.clear();body.server_csr_id=Some(request.request_id);
    assert!(setup.prepare(body).await.err().unwrap().to_string().contains("selection changed"));
    // No setup operation or database initialization was dispatched.
    assert_eq!(fs::read_dir(setup.root.join("setup-operations")).unwrap().count(),0);
}


#[tokio::test]
async fn server_csr_installation_requires_its_original_receipt_before_permitting_renewal() {
    use crate::server_csr;
    let (_dir,setup)=fixture().await;
    let csr=server_csr::Request{request_id:Uuid::new_v4(),origin:"https://localhost:18443".into()};
    server_csr::create(&setup.root,csr.clone(),"installation").unwrap();
    let cert=server_csr::tests::sign(&setup.root,csr.request_id,server_csr::tests::EXTENSIONS,"1");
    server_csr::import_certificate(&setup.root,csr.request_id,&cert).unwrap();
    let (_,key)=server_csr::identity(&setup.root,csr.request_id,&csr.origin).unwrap();
    // New filesystem-only selection fixture, without database initialization.
    let (mut saved,_) = configured_files(&setup);
    files::publish_replace(FsPath::new(&saved.config.tls_cert),cert.as_bytes()).unwrap();
    files::publish_replace(FsPath::new(&saved.config.tls_key),key.as_bytes()).unwrap();
    saved.material_sha256[0]=auth::hash(cert.as_bytes());saved.material_sha256[1]=auth::hash(key.as_bytes());
    let bytes=serde_json::to_vec(&saved).unwrap();
    files::publish_replace(&console_settings::generation(&setup.root,saved.operation_id).join("settings.json"),&bytes).unwrap();
    let selected=Selection{schema_version:1,operation_id:saved.operation_id,digest:auth::hash(&bytes)};
    files::publish_replace(&setup.root.join("console-config-active.json"),&serde_json::to_vec(&selected).unwrap()).unwrap();
    let original=Intent{schema_version:1,operation_id:saved.operation_id,request_digest:"fixture-only".into(),origin:csr.origin.clone(),username:"admin".into(),selection:Some(selected.clone())};
    files::publish_new(&operation_path(&setup.root,saved.operation_id,"intent"),&serde_json::to_vec(&original).unwrap()).unwrap();
    let before=files::read_private(&setup.root.join("console-config-active.json")).unwrap();
    assert_eq!(server_csr::observe(&setup.root,csr.request_id).unwrap().unwrap()["state"],"UNKNOWN");
    assert!(server_csr::retire(&setup.root,csr.request_id).is_err());
    assert_eq!(server_csr::create(&setup.root,server_csr::Request{request_id:Uuid::new_v4(),origin:csr.origin.clone()},"admin").unwrap()["state"],"REJECTED");
    files::publish_new(&operation_path(&setup.root,saved.operation_id,"receipt"),&serde_json::to_vec(&selected).unwrap()).unwrap();
    let observed=server_csr::observe(&setup.root,csr.request_id).unwrap().unwrap();
    assert_eq!(observed["state"],"INSTALLED");assert_eq!(observed["installationOperationId"],saved.operation_id.to_string());assert!(!observed.to_string().contains("PRIVATE KEY"));
    assert!(server_csr::current(&setup.root).unwrap().is_none());assert!(server_csr::retire(&setup.root,csr.request_id).is_err());
    assert_eq!(server_csr::create(&setup.root,server_csr::Request{request_id:Uuid::new_v4(),origin:csr.origin},"admin").unwrap()["state"],"READY");
    assert_eq!(before,files::read_private(&setup.root.join("console-config-active.json")).unwrap());
}
