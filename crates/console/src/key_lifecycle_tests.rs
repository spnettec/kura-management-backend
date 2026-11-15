//! Adversarial key/TLS lifecycle contracts. Only a fresh private review DB is supported.
use super::*;
use std::os::unix::fs::PermissionsExt;

struct Fixture {
    app: App,
    headers: HeaderMap,
    dir: std::path::PathBuf,
}
async fn fixture() -> Fixture {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .with_test_writer()
        .try_init();
    let url = std::env::var("YOFC_REVIEW_PG_URL").expect("private review PG required");
    let admin = PgPool::connect(&url).await.unwrap();
    let schema = format!("review_{}", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let options = url
        .parse::<sqlx::postgres::PgConnectOptions>()
        .unwrap()
        .options([("search_path", schema.as_str())]);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    let dir = tempfile::tempdir().unwrap().keep();
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).unwrap();
    crate::task_crypto::create_key(&dir.join("task.key")).unwrap();
    let cipher = TaskCipher::load(&dir.join("task.key")).unwrap();
    cipher.prepare_storage(&pool).await.unwrap();
    let actor = Uuid::new_v4();
    let token = auth::random_token();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,'review','review-owner','unused','admin',false)").bind(actor).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'review-csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(actor).execute(&pool).await.unwrap();
    let config = Config {
        http_addr: "127.0.0.1:0".parse().unwrap(),
        grpc_addr: "127.0.0.1:0".parse().unwrap(),
        origin: "https://review.invalid".into(),
        tls_cert: std::env::var("YOFC_REVIEW_TLS_CERT").unwrap(),
        tls_key: std::env::var("YOFC_REVIEW_TLS_KEY").unwrap(),
        client_ca: std::env::var("YOFC_REVIEW_TLS_CERT").unwrap(),
        web_dir: String::new(),
        state_dir: dir.to_string_lossy().into(),
    };
    let app = App::new(pool, config, cipher);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        format!("kura_session={token}").parse().unwrap(),
    );
    headers.insert(header::ORIGIN, "https://review.invalid".parse().unwrap());
    headers.insert("x-csrf-token", "review-csrf".parse().unwrap());
    Fixture { app, headers, dir }
}
async fn rotate(f: &Fixture) -> Result<Response, ApiError> {
    settings_task_key_rotate(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TaskKeyRotate {
            passphrase: "review-fixture-passphrase-only".into(),
            confirmed: true,
        })),
    )
    .await
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn failed_key_staging_must_leave_the_active_database_binding_readable() {
    let f = fixture().await;
    std::fs::write(f.dir.join("pending"), b"fault: pending is not a directory").unwrap();
    assert!(
        rotate(&f).await.is_err(),
        "staging fault must fail the request"
    );
    let live = f.app.cipher.read().await;
    assert!(
        live.prepare_storage(&f.app.pool).await.is_ok(),
        "DB was committed under a new key before durable staging; active key can no longer open it"
    );
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn rotating_task_key_must_preserve_undelivered_identity_bundle_ciphertext() {
    let f = fixture().await;
    let issued = Utc::now();
    let sealed = f
        .app
        .cipher
        .read()
        .await
        .seal_identity_bundle(
            "review-gateway",
            issued.timestamp_millis(),
            b"fixture-only-undelivered-identity",
        )
        .unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,identity_bundle_ciphertext,identity_bundle_issued_at,identity_bundle_token_hash) VALUES('review-gateway','review','review','fixture-fingerprint',$1,$2,'fixture-token-hash')").bind(serde_json::to_string(&sealed).unwrap()).bind(issued).execute(&f.app.pool).await.unwrap();
    assert!(
        rotate(&f).await.is_ok(),
        "first rotation fixture must complete"
    );
    let stored: String = sqlx::query_scalar(
        "SELECT identity_bundle_ciphertext FROM mgmt_gateways WHERE id='review-gateway'",
    )
    .fetch_one(&f.app.pool)
    .await
    .unwrap();
    let stored: Value = serde_json::from_str(&stored).unwrap();
    assert!(
        f.app
            .cipher
            .read()
            .await
            .open_identity_bundle("review-gateway", issued.timestamp_millis(), &stored)
            .is_ok(),
        "live replacement key cannot open an undelivered bundle retained by rotation"
    );
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn a_second_key_rotation_must_remain_supported_after_successful_first_rotation() {
    let f = fixture().await;
    assert!(rotate(&f).await.is_ok());
    assert!(
        rotate(&f).await.is_ok(),
        "first rotation leaves version=2, but rotation accepts only version=1"
    );
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn tls_upload_must_reject_structural_pem_that_is_not_a_private_key() {
    let f = fixture().await;
    crate::settings::certificate_fingerprint(
        &std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
    )
    .expect("TLS fixture must be a parsed v3 certificate");
    let result = settings_tls_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TlsUpload {
            server_pem: std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
            server_key: "-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----\n".into(),
        })),
    )
    .await;
    assert!(
        result.is_err(),
        "base64 PEM shell was accepted as a server private key"
    );
    assert!(
        !f.dir.join("pending/tls-server.pem").exists(),
        "invalid pair left staged certificate"
    );
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn second_tls_file_failure_must_not_publish_the_first_half_of_the_pair() {
    let f = fixture().await;
    crate::settings::certificate_fingerprint(
        &std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
    )
    .expect("TLS fixture must be a parsed v3 certificate");
    crate::provisioning::validate_private_key_pem(
        &std::fs::read_to_string(&f.app.config.tls_key).unwrap(),
    )
    .expect("TLS fixture must carry a valid PEM key");
    std::fs::create_dir(f.dir.join("pending")).unwrap();
    std::fs::set_permissions(
        f.dir.join("pending"),
        std::fs::Permissions::from_mode(0o700),
    )
    .unwrap();
    std::fs::write(
        f.dir.join("pending/tls-server.key"),
        b"pre-existing-pending-key",
    )
    .unwrap();
    let result = settings_tls_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TlsUpload {
            server_pem: std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
            server_key: std::fs::read_to_string(&f.app.config.tls_key).unwrap(),
        })),
    )
    .await;
    assert!(result.is_err());
    assert!(
        !f.dir.join("pending/tls-server.pem").exists(),
        "first half of a TLS pair was published before the second half failed"
    );
}

#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn valid_tls_pair_control_reaches_the_staging_path() {
    let f = fixture().await;
    crate::settings::certificate_fingerprint(
        &std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
    )
    .expect("TLS fixture must be parsed");
    crate::tls_store::validate_pair(
        &std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
        &std::fs::read_to_string(&f.app.config.tls_key).unwrap(),
    )
    .expect("valid fixture must parse and match");
    let result = settings_tls_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TlsUpload {
            server_pem: std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
            server_key: std::fs::read_to_string(&f.app.config.tls_key).unwrap(),
        })),
    )
    .await;
    assert!(result.is_ok(), "valid fixture must reach both TLS stages");
    assert!(f.dir.join("pending/tls-update.json").is_file());
    assert!(crate::settings::tls_status(&f.app.config).unwrap()["pendingFingerprint"].is_string());
    let mut restarted = f.app.config.clone();
    crate::tls_store::activate(&mut restarted).unwrap();
    assert!(!f.dir.join("pending/tls-update.json").exists());
    crate::tls_store::validate_pair(
        &std::fs::read_to_string(&restarted.tls_cert).unwrap(),
        &std::fs::read_to_string(&restarted.tls_key).unwrap(),
    )
    .unwrap();
    assert_ne!(restarted.tls_cert, f.app.config.tls_cert);
    let first = restarted.tls_cert.clone();
    crate::tls_store::activate(&mut restarted).unwrap();
    assert_eq!(restarted.tls_cert, first);
}

#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn restart_after_db_commit_selects_the_durable_new_key_without_replaying_rotation() {
    let f = fixture().await;
    let old = f.app.cipher.read().await.clone();
    let new = TaskCipher::from_key_bytes(&[23; 32]);
    crate::task_key_store::prepare(&f.dir, &old, &new).unwrap();
    old.rotate_storage(&new, &f.app.pool).await.unwrap();
    // Simulate loss before swapping the in-memory cipher or delivering backup.
    let recovered = crate::task_key_store::recover(&f.app.pool, &f.dir.join("task.key"), &f.dir)
        .await
        .unwrap();
    assert_eq!(recovered.fingerprint(), new.fingerprint());
    assert_eq!(
        TaskCipher::load(&f.dir.join("task.key"))
            .unwrap()
            .fingerprint(),
        old.fingerprint()
    );
    assert!(!old.matches_storage(&f.app.pool).await.unwrap());
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn preparation_without_db_commit_keeps_the_original_key_authoritative() {
    let f = fixture().await;
    let old = f.app.cipher.read().await.clone();
    let new = TaskCipher::from_key_bytes(&[24; 32]);
    crate::task_key_store::prepare(&f.dir, &old, &new).unwrap();
    let recovered = crate::task_key_store::recover(&f.app.pool, &f.dir.join("task.key"), &f.dir)
        .await
        .unwrap();
    assert_eq!(recovered.fingerprint(), old.fingerprint());
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn stale_process_key_is_fenced_before_it_can_lock_or_write_a_task() {
    let f = fixture().await;
    let old = f.app.cipher.read().await.clone();
    assert!(rotate(&f).await.is_ok());
    let mut tx = f.app.pool.begin().await.unwrap();
    assert!(old.lock_storage(&mut tx).await.is_err());
    tx.rollback().await.unwrap();
    let mut tx = f.app.pool.begin().await.unwrap();
    f.app
        .cipher
        .read()
        .await
        .lock_storage(&mut tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn in_flight_writer_binding_lock_must_delay_db_rekey_until_its_commit() {
    let f = fixture().await;
    let old = f.app.cipher.read().await.clone();
    let new = TaskCipher::from_key_bytes(&[25; 32]);
    let mut tx = f.app.pool.begin().await.unwrap();
    old.lock_storage(&mut tx).await.unwrap();
    let change = old.rotate_storage(&new, &f.app.pool);
    tokio::pin!(change);
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(150), &mut change)
            .await
            .is_err()
    );
    tx.commit().await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(3), &mut change)
        .await
        .unwrap()
        .unwrap();
    assert!(new.matches_storage(&f.app.pool).await.unwrap());
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn mismatching_pending_restore_never_overwrites_the_active_key() {
    let f = fixture().await;
    let old = f.app.cipher.read().await.clone();
    crate::task_key_store::retain(&f.dir, &old).unwrap();
    TaskCipher::from_key_bytes(&[26; 32])
        .persist_new(&f.dir.join("pending/task-data.key"))
        .unwrap();
    assert!(
        crate::task_key_store::recover(&f.app.pool, &f.dir.join("task.key"), &f.dir)
            .await
            .is_err()
    );
    assert_eq!(
        TaskCipher::load(&f.dir.join("task.key"))
            .unwrap()
            .fingerprint(),
        old.fingerprint()
    );
    assert!(old.matches_storage(&f.app.pool).await.unwrap());
    assert!(f.dir.join("pending/task-data.key").exists());
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn current_cipher_backup_after_rotation_contains_the_new_key_not_the_base_file_key() {
    let f = fixture().await;
    let old = f.app.cipher.read().await.fingerprint();
    assert!(rotate(&f).await.is_ok());
    let cipher = f.app.cipher.read().await;
    let response = settings_task_key_backup(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TaskKeyBackup {
            passphrase: "review-fixture-passphrase-only".into(),
        })),
    )
    .await
    .unwrap_or_else(|_| panic!("active key backup must succeed"));
    let bytes = axum::body::to_bytes(response.into_body(), 8192)
        .await
        .unwrap();
    let backup = f.dir.join("download.backup");
    let pass = f.dir.join("download.pass");
    let out = f.dir.join("restored.key");
    for (path, bytes) in [
        (&backup, &bytes[..]),
        (&pass, b"review-fixture-passphrase-only".as_slice()),
    ] {
        std::fs::write(path, bytes).unwrap();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    crate::task_crypto::restore_key(&backup, &out, &pass).unwrap();
    let restored = TaskCipher::load(&out).unwrap();
    assert_eq!(restored.fingerprint(), cipher.fingerprint());
    assert_ne!(restored.fingerprint(), old);
}

#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn restore_api_rejects_an_authenticated_backup_for_another_database_key() {
    let f = fixture().await;
    let wrong = TaskCipher::from_key_bytes(&[27; 32]);
    let bytes = cipher_backup(&f.dir, &wrong, b"review-fixture-passphrase-only").unwrap();
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
    let result = settings_task_key_restore(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TaskKeyRestore {
            backup_base64: encoded,
            passphrase: "review-fixture-passphrase-only".into(),
        })),
    )
    .await;
    assert!(result.is_err());
    assert!(!f.dir.join("pending/task-data.key").exists());
    assert!(
        f.app
            .cipher
            .read()
            .await
            .matches_storage(&f.app.pool)
            .await
            .unwrap()
    );
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn task_inputs_and_results_remain_readable_and_unchanged_after_rekey() {
    let f = fixture().await;
    let id = Uuid::new_v4();
    let context = crate::task_crypto::TaskContext {
        id,
        tenant: "review".into(),
        gateway: "review-task-gateway".into(),
        operation: 1,
    };
    let input = json!({"fixture":"input"});
    let result = json!({"fixture":"result"});
    let cipher = f.app.cipher.read().await;
    let sealed_input = cipher.seal(&context, "input", &input).unwrap();
    let sealed_result = cipher.seal(&context, "result", &result).unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES('review-task-gateway','review','review','task-fixture-fingerprint')").execute(&f.app.pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,binding_generation,expires_at,input,result,state) SELECT $1,'review-task-gateway','review',id,1,'review-task',1,now()+interval '30 seconds',$2,$3,'SUCCEEDED' FROM mgmt_users WHERE username='review-owner'").bind(id).bind(sealed_input.clone()).bind(sealed_result.clone()).execute(&f.app.pool).await.unwrap();
    drop(cipher);
    assert!(rotate(&f).await.is_ok());
    let row = sqlx::query(
        "SELECT id,tenant_id,gateway_id,operation,input,result FROM mgmt_tasks WHERE id=$1",
    )
    .bind(id)
    .fetch_one(&f.app.pool)
    .await
    .unwrap();
    let cipher = f.app.cipher.read().await;
    assert_eq!(cipher.open_row(&row, "input").unwrap(), input);
    assert_eq!(cipher.open_row(&row, "result").unwrap(), result);
    assert_ne!(row.get::<Value, _>("input"), sealed_input);
    assert_ne!(row.get::<Value, _>("result"), sealed_result);
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn tls_pair_with_another_valid_private_key_is_rejected_before_staging() {
    let f = fixture().await;
    let rng = ring::rand::SystemRandom::new();
    let document = ring::signature::EcdsaKeyPair::generate_pkcs8(
        &ring::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        &rng,
    )
    .unwrap();
    let encoded = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        document.as_ref(),
    );
    let key = format!("-----BEGIN PRIVATE KEY-----\n{encoded}\n-----END PRIVATE KEY-----\n");
    assert!(crate::provisioning::validate_private_key_pem(&key).is_ok());
    let result = settings_tls_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(TlsUpload {
            server_pem: std::fs::read_to_string(&f.app.config.tls_cert).unwrap(),
            server_key: key,
        })),
    )
    .await;
    assert!(result.is_err());
    assert!(!f.dir.join("pending/tls-update.json").exists());
}

#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn client_ca_upload_is_pending_until_validated_restart_activation() {
    let mut f = fixture().await;
    f.app.config.client_ca = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/review-ca-one.pem"
    )
    .into();
    let uploaded = include_str!("../tests/fixtures/review-ca-two.pem");
    let result = settings_client_ca_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(ClientCaUpload {
            client_ca_pem: uploaded.into(),
        })),
    )
    .await;
    assert!(result.is_ok());
    let before = crate::settings::client_ca_status(&f.app.config).unwrap();
    assert_ne!(before["caFingerprint"], before["pendingFingerprint"]);
    let mut config = f.app.config.clone();
    crate::client_ca_store::activate(&mut config).unwrap();
    let after = crate::settings::client_ca_status(&config).unwrap();
    assert_eq!(after["caFingerprint"], before["pendingFingerprint"]);
    assert!(after["pendingFingerprint"].is_null());
    assert_eq!(crate::settings::live_client_ca(&config).unwrap(), uploaded);
}
#[tokio::test]
#[ignore = "new private review PostgreSQL fixture only"]
async fn client_ca_http_rejects_leaf_and_unauthorized_settings_writes() {
    let mut f = fixture().await;
    f.app.config.client_ca = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/review-ca-one.pem"
    )
    .into();
    let result = settings_client_ca_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(ClientCaUpload {
            client_ca_pem: include_str!("../tests/fixtures/review-not-ca.pem").into(),
        })),
    )
    .await;
    assert!(matches!(
        result,
        Err(ApiError(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))
    ));
    let mut headers = f.headers.clone();
    headers.remove("x-csrf-token");
    let result = settings_client_ca_put(
        State(f.app.clone()),
        headers,
        Ok(Json(ClientCaUpload {
            client_ca_pem: include_str!("../tests/fixtures/review-ca-two.pem").into(),
        })),
    )
    .await;
    assert!(matches!(result, Err(ApiError(StatusCode::FORBIDDEN, _))));
    sqlx::query("UPDATE mgmt_users SET role='viewer' WHERE username='review-owner'")
        .execute(&f.app.pool)
        .await
        .unwrap();
    let result = settings_client_ca_put(
        State(f.app.clone()),
        f.headers.clone(),
        Ok(Json(ClientCaUpload {
            client_ca_pem: include_str!("../tests/fixtures/review-ca-two.pem").into(),
        })),
    )
    .await;
    assert!(matches!(result, Err(ApiError(StatusCode::FORBIDDEN, _))));
    assert!(!f.dir.join("pending/client-ca-update.json").exists());
}
