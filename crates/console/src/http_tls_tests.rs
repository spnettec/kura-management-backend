//! New isolated TLS activation acceptance. No old acceptance fixture is reused.
use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn server_csr_upload_receipts_live_rejection_and_original_signed_outcome() {
    let f = fixture().await;
    let base = format!("{}/api/v1/settings/server-csr", f.origin);
    let id = Uuid::new_v4();
    let rejected_operation = Uuid::new_v4();
    let signed_operation = Uuid::new_v4();
    let foreign_token = auth::random_token(); let foreign_user = Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,'tls-review','csr-receipt-operator','unused','operator',false)")
        .bind(foreign_user).execute(&f.app.pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'operator-csrf',now()+interval '1 hour')")
        .bind(auth::hash(foreign_token.as_bytes())).bind(foreign_user).execute(&f.app.pool).await.unwrap();
    for (method, suffix) in [(reqwest::Method::GET, format!("/{id}/imports/{rejected_operation}")),
        (reqwest::Method::PUT, format!("/{id}/certificate"))] {
        let input = json!({"operationId":rejected_operation,"serverPem":CERT2});
        let response = f.client.request(method.clone(), format!("{base}{suffix}"))
            .header(reqwest::header::COOKIE, "").json(&input).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
        let response = f.client.request(method.clone(), format!("{base}{suffix}"))
            .header(reqwest::header::COOKIE, format!("kura_session={foreign_token}"))
            .header("x-csrf-token", "operator-csrf").json(&input).send().await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
        if method != reqwest::Method::GET {
            assert_eq!(f.client.request(method.clone(), format!("{base}{suffix}"))
                .header("x-csrf-token", "wrong").json(&input).send().await.unwrap().status(), reqwest::StatusCode::FORBIDDEN);
            assert_eq!(f.client.request(method, format!("{base}{suffix}"))
                .header(reqwest::header::ORIGIN, "https://attacker.invalid").json(&input).send().await.unwrap().status(), reqwest::StatusCode::FORBIDDEN);
        }
    }
    let state = std::path::Path::new(&f.app.config.state_dir);
    assert!(!state.join("server-csr-requests").exists());
    let original_tls = f.app.tls.as_ref().unwrap().status().unwrap();
    let ready: Value = f.client.post(&base).json(&json!({"requestId":id,"origin":f.origin}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(ready["state"], "READY");
    let dir = state.join(format!("server-csr-requests/{id}"));
    let original_key = crate::settings_files::read_private(&dir.join("server.key")).unwrap();
    assert_eq!(f.client.put(format!("{base}/{id}/certificate"))
        .json(&json!({"operationId":Uuid::nil(),"serverPem":CERT2})).send().await.unwrap().status(), reqwest::StatusCode::BAD_REQUEST);
    assert!(!dir.join("imports").exists());
    let response = f.client.put(format!("{base}/{id}/certificate"))
        .json(&json!({"operationId":rejected_operation,"serverPem":CERT2})).send().await.unwrap().error_for_status().unwrap();
    drop(response); // Deliberately leave the HTTP body unread; use the original GET receipt.
    let rejected = read(&f, &format!("/settings/server-csr/{id}/imports/{rejected_operation}")).await;
    assert_eq!(rejected["state"], "READY");
    assert_eq!(rejected["importOperation"]["state"], "REJECTED");
    assert_eq!(rejected["importOperation"]["certificateSha256"], auth::hash(CERT2.as_bytes()));
    let cert = crate::server_csr::tests::sign(state, id, crate::server_csr::tests::EXTENSIONS, "1");
    let response = f.client.put(format!("{base}/{id}/certificate"))
        .json(&json!({"operationId":signed_operation,"serverPem":cert})).send().await.unwrap().error_for_status().unwrap();
    drop(response);
    let signed = read(&f, &format!("/settings/server-csr/{id}/imports/{signed_operation}")).await;
    assert_eq!(signed["state"], "SIGNED");
    assert_eq!(signed["importOperation"]["state"], "APPLIED");
    assert_eq!(signed["importOperation"]["operationId"], signed_operation.to_string());
    assert!(!signed.to_string().contains("PRIVATE KEY"));
    assert_eq!(read(&f, &format!("/settings/server-csr/{id}/imports/{rejected_operation}")).await["importOperation"]["state"], "REJECTED");
    assert_eq!(crate::settings_files::read_private(&dir.join("server.key")).unwrap(), original_key);
    assert_eq!(fs::read_dir(dir.join("imports/operations")).unwrap().count(), 2);
    assert!(!state.join("pending/tls-update.json").exists());
    let tls = f.app.tls.as_ref().unwrap().status().unwrap();
    assert_eq!(tls["generation"], original_tls["generation"]);
    assert_eq!(tls["certificateFingerprint"], original_tls["certificateFingerprint"]);
    fs::rename(dir.join("signed.json"), f.root.join("retained-original-signed-receipt.json")).unwrap();
    let unknown = read(&f, &format!("/settings/server-csr/{id}/imports/{signed_operation}")).await;
    assert_eq!(unknown["state"], "UNKNOWN"); assert_eq!(unknown["importOperation"]["state"], "UNKNOWN");
    assert_eq!(read(&f, "/settings/tls-activation").await["blockingCsrRequestId"], id.to_string());
    assert!(!dir.join("signed.json").exists());
    assert!(dir.join("certificate.pem").exists());
    println!("CSR_IMPORT_RECEIPTS_HTTP_ACCEPTED request_id={id} rejected_operation={rejected_operation} signed_operation={signed_operation} unread_response_body=true original_query_only=true final_missing_receipt_unknown=true no_tls_activation=true");
}
use crate::{
    tls_runtime::{ActivationRequest, TlsRuntime},
    tls_transport,
};
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};
use yofc_management_protocol::{
    CONFIGURATION_UPDATE_CAPABILITY, CONFIGURATION_UPDATE_READINESS,
    v1::{AgentFrame, Heartbeat, Hello, agent_frame, console_frame, control_client::ControlClient,
        control_server::ControlServer},
};

const CA1: &str = include_str!("../tests/fixtures/tls-live/ca-one.pem");
const CA2: &str = include_str!("../tests/fixtures/tls-live/ca-two.pem");
const CERT1: &str = include_str!("../tests/fixtures/tls-live/server-one.pem");
const KEY1: &str = include_str!("../tests/fixtures/tls-live/server-one.key");
const CERT2: &str = include_str!("../tests/fixtures/tls-live/server-two.pem");
const KEY2: &str = include_str!("../tests/fixtures/tls-live/server-two.key");

struct Fixture {
    app: App,
    root: PathBuf,
    origin: String,
    grpc: std::net::SocketAddr,
    client: reqwest::Client,
    handles: Vec<tokio::task::JoinHandle<()>>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        for handle in &self.handles {
            handle.abort();
        }
    }
}

async fn fixture() -> Fixture {
    let url =
        std::env::var("YOFC_REVIEW_PG_URL").expect("fresh private review PostgreSQL required");
    let schema = format!("tls_review_{}", Uuid::new_v4().simple());
    let admin = PgPool::connect(&url).await.unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let options = url
        .parse::<sqlx::postgres::PgConnectOptions>()
        .unwrap()
        .options([("search_path", schema.as_str())]);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    let root = tempfile::tempdir().unwrap().keep(); // Preserve new DB/files as evidence.
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    for (name, value) in [("server.pem", CERT1), ("server.key", KEY1), ("ca.pem", CA1)] {
        crate::settings_files::write_new(&root.join(name), value.as_bytes()).unwrap();
    }
    crate::task_crypto::create_key(&root.join("task.key")).unwrap();
    let cipher = TaskCipher::load(&root.join("task.key")).unwrap();
    cipher.prepare_storage(&pool).await.unwrap();
    let token = auth::random_token();
    let actor = Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,'tls-review','tls-review-admin','unused','admin',false)").bind(actor).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'tls-review-csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(actor).execute(&pool).await.unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let grpc_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let grpc = grpc_listener.local_addr().unwrap();
    let origin = format!("https://localhost:{}", address.port());
    let cfg = Config {
        http_addr: address,
        grpc_addr: grpc,
        origin: origin.clone(),
        tls_cert: root.join("server.pem").to_str().unwrap().into(),
        tls_key: root.join("server.key").to_str().unwrap().into(),
        client_ca: root.join("ca.pem").to_str().unwrap().into(),
        web_dir: root.to_str().unwrap().into(),
        state_dir: root.join("state").to_str().unwrap().into(),
    };
    let runtime = TlsRuntime::open(cfg.clone()).unwrap();
    let mut app = App::new(pool.clone(), cfg, cipher).unwrap();
    app.tls = Some(runtime.clone());
    let router = router(app.clone()).into_make_service_with_connect_info::<std::net::SocketAddr>();
    let web = tokio::spawn(async move {
        axum_server::from_tcp(listener)
            .unwrap()
            .acceptor(tls_transport::WebAcceptor(runtime))
            .serve(router)
            .await
            .unwrap();
    });
    let control = crate::grpc::ControlService {
        gate: app.gate.clone(),
        pool,
        cipher: app.cipher.clone(),
        slots: Arc::new(Semaphore::new(128)),
    };
    let runtime = app.tls.clone().unwrap();
    let service = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(ControlServer::new(control))
            .serve_with_incoming(tls_transport::grpc_incoming(grpc_listener, runtime))
            .await
            .unwrap();
    });
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        format!("kura_session={token}").parse().unwrap(),
    );
    headers.insert(reqwest::header::ORIGIN, origin.parse().unwrap());
    headers.insert("x-csrf-token", "tls-review-csrf".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .add_root_certificate(reqwest::Certificate::from_pem(CA1.as_bytes()).unwrap())
        .add_root_certificate(reqwest::Certificate::from_pem(CA2.as_bytes()).unwrap())
        .pool_max_idle_per_host(0)
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    println!(
        "TLS_REVIEW_FIXTURE root={} schema={} origin={} grpc={}",
        root.display(),
        schema,
        origin,
        grpc
    );
    Fixture {
        app,
        root,
        origin,
        grpc,
        client,
        handles: vec![web, service],
    }
}

async fn read(f: &Fixture, path: &str) -> Value {
    f.client
        .get(format!("{}/api/v1{path}", f.origin))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap()
}

async fn approval_admin(f: &Fixture, username: &str, tenant: &str) -> (Uuid, String) {
    let id = Uuid::new_v4();
    let token = auth::random_token();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,$2,$3,'unused','admin',false)")
        .bind(id).bind(tenant).bind(username).execute(&f.app.pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'tls-review-csrf',now()+interval '1 hour')")
        .bind(auth::hash(token.as_bytes())).bind(id).execute(&f.app.pool).await.unwrap();
    (id, format!("kura_session={token}"))
}

async fn approve_ca(f: &Fixture, request: &ActivationRequest, session: Option<&str>) -> reqwest::Response {
    let mut call = f.client.post(format!("{}/api/v1/settings/tls-activation/approvals", f.origin));
    if let Some(session) = session {
        call = call.header(reqwest::header::COOKIE, session);
    }
    call.json(request).send().await.unwrap()
}

async fn hello(
    f: &Fixture,
    cohort: &str,
    gateway: &str,
) -> Result<
    (
        tokio::sync::mpsc::Sender<AgentFrame>,
        tonic::Streaming<yofc_management_protocol::v1::ConsoleFrame>,
    ),
    tonic::Status,
> {
    hello_with_capabilities(f, cohort, gateway, &[]).await
}

async fn hello_with_capabilities(
    f: &Fixture,
    cohort: &str,
    gateway: &str,
    capabilities: &[&str],
) -> Result<
    (
        tokio::sync::mpsc::Sender<AgentFrame>,
        tonic::Streaming<yofc_management_protocol::v1::ConsoleFrame>,
    ),
    tonic::Status,
> {
    use tonic::transport::{Certificate, ClientTlsConfig, Endpoint, Identity};
    let (ca, cert, key) = if cohort == "one" {
        (
            CA1,
            include_str!("../tests/fixtures/tls-live/client-one.pem"),
            include_str!("../tests/fixtures/tls-live/client-one.key"),
        )
    } else {
        (
            CA2,
            include_str!("../tests/fixtures/tls-live/client-two.pem"),
            include_str!("../tests/fixtures/tls-live/client-two.key"),
        )
    };
    let endpoint = Endpoint::from_shared(format!("https://localhost:{}", f.grpc.port()))
        .unwrap()
        .tls_config(
            ClientTlsConfig::new()
                .ca_certificate(Certificate::from_pem(ca))
                .identity(Identity::from_pem(cert, key)),
        )
        .unwrap()
        .connect_timeout(Duration::from_secs(3));
    let channel = endpoint
        .connect()
        .await
        .map_err(|_| tonic::Status::unauthenticated("TLS rejected"))?;
    let (tx, rx) = tokio::sync::mpsc::channel(2);
    tx.send(AgentFrame {
        payload: Some(agent_frame::Payload::Hello(Hello {
            gateway_id: gateway.into(),
            protocol_major: 1,
            binding_generation: 1,
            agent_version: "isolated-tls-test".into(),
            boot_id: "tls-test".into(),
            capabilities: capabilities.iter().map(|capability| (*capability).to_owned()).collect(),
        })),
    })
    .await
    .unwrap();
    let response = ControlClient::new(channel)
        .connect(tokio_stream::wrappers::ReceiverStream::new(rx))
        .await?;
    Ok((tx, response.into_inner()))
}

#[tokio::test]
#[ignore = "requires a fresh isolated TLS review PostgreSQL instance"]
async fn configuration_update_readiness_tracks_the_authenticated_stream() {
    let f = fixture().await;
    let gateway = "configuration-readiness";
    let certificate = include_str!("../tests/fixtures/tls-live/client-one.pem");
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,enabled) VALUES($1,'tls-review',$1,$2,true)")
        .bind(gateway).bind(crate::settings::certificate_fingerprint(certificate).unwrap())
        .execute(&f.app.pool).await.unwrap();
    let read_capability = "kura.configuration.get";
    let (negotiated, mut stream) = hello_with_capabilities(
        &f,
        "one",
        gateway,
        &[
            read_capability,
            CONFIGURATION_UPDATE_READINESS,
            CONFIGURATION_UPDATE_CAPABILITY,
        ],
    )
    .await
    .unwrap();
    let Some(console_frame::Payload::Welcome(welcome)) =
        stream.message().await.unwrap().unwrap().payload
    else {
        panic!("expected authenticated welcome");
    };
    async fn verify_state(f: &Fixture, gateway: &str, sequence: i64, expected: bool, epoch: &str) {
        tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                let (seen, caps, current): (i64, Value, Uuid) = sqlx::query_as(
                    "SELECT sequence,capabilities,connection_epoch FROM mgmt_gateways WHERE id=$1",
                )
                .bind(gateway)
                .fetch_one(&f.app.pool)
                .await
                .unwrap();
                if seen == sequence {
                    assert_eq!(current.to_string(), epoch);
                    let list = caps.as_array().unwrap();
                    assert!(list.iter().any(|name| name == "kura.configuration.get"));
                    assert_eq!(
                        list.iter()
                            .filter(|name| *name == CONFIGURATION_UPDATE_CAPABILITY)
                            .count(),
                        usize::from(expected)
                    );
                    return;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .expect("heartbeat not observed in PostgreSQL");
    }
    async fn heartbeat(
        tx: &tokio::sync::mpsc::Sender<AgentFrame>,
        sequence: u64,
        available: bool,
        ready: Option<bool>,
    ) {
        tx.send(AgentFrame {
            payload: Some(agent_frame::Payload::Heartbeat(Heartbeat {
                sequence,
                kura_available: available,
                uptime_seconds: sequence,
                configuration_update_ready: ready,
            })),
        })
        .await
        .unwrap();
    }
    verify_state(&f, gateway, 0, false, &welcome.connection_epoch).await;
    for (sequence, available, reported, expected) in [
        (1, false, Some(true), false),
        (2, true, Some(true), true),
        (3, true, Some(true), true),
        (4, true, None, false),
        (5, true, Some(true), true),
        (6, true, Some(false), false),
    ] {
        heartbeat(&negotiated, sequence, available, reported).await;
        verify_state(
            &f,
            gateway,
            sequence as i64,
            expected,
            &welcome.connection_epoch,
        )
        .await;
    }
    let response = f
        .client
        .post(format!("{}/api/v1/gateways/{gateway}/tasks", f.origin))
        .json(&json!({
            "operation":CONFIGURATION_UPDATE_CAPABILITY,"idempotencyKey":Uuid::new_v4().to_string(),
            "kuraConfigurationUpdate":{"pid":"readiness.fixture","expectedRevision":"a".repeat(64),
                "properties":[{"id":"enabled","value":{"boolean":true}}]}
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM mgmt_tasks WHERE gateway_id=$1")
            .bind(gateway)
            .fetch_one(&f.app.pool)
            .await
            .unwrap(),
        0
    );
    heartbeat(&negotiated, 6, true, Some(true)).await;
    assert_eq!(
        stream.message().await.unwrap_err().code(),
        tonic::Code::InvalidArgument
    );
    verify_state(&f, gateway, 6, false, &welcome.connection_epoch).await;

    for legacy_advertised in [false, true] {
        let mut caps = vec![read_capability];
        if legacy_advertised {
            caps.push(CONFIGURATION_UPDATE_CAPABILITY);
        }
        let (legacy, mut stream) = hello_with_capabilities(&f, "one", gateway, &caps)
            .await
            .unwrap();
        let Some(console_frame::Payload::Welcome(old)) =
            stream.message().await.unwrap().unwrap().payload
        else {
            panic!("expected legacy welcome");
        };
        verify_state(&f, gateway, 0, legacy_advertised, &old.connection_epoch).await;
        heartbeat(&legacy, 1, true, Some(!legacy_advertised)).await;
        verify_state(&f, gateway, 1, legacy_advertised, &old.connection_epoch).await;
    }
}

#[tokio::test]
#[ignore = "requires a fresh isolated TLS review PostgreSQL instance"]
async fn tls_runtime_http_and_real_grpc_activation_are_joint_and_idempotent() {
    let f = fixture().await;
    for (id, pem) in [
        (
            "tls-one",
            include_str!("../tests/fixtures/tls-live/client-one.pem"),
        ),
        (
            "tls-two",
            include_str!("../tests/fixtures/tls-live/client-two.pem"),
        ),
    ] {
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,enabled) VALUES($1,'tls-review',$1,$2,true)")
            .bind(id).bind(crate::settings::certificate_fingerprint(pem).unwrap()).execute(&f.app.pool).await.unwrap();
    }
    let (_old_tx, mut old) = hello(&f, "one", "tls-one").await.unwrap();
    assert!(matches!(
        old.message().await.unwrap().unwrap().payload,
        Some(console_frame::Payload::Welcome(_))
    ));
    for (path, payload) in [
        ("tls", json!({"serverPem":CERT2,"serverKey":KEY2})),
        ("client-ca", json!({"clientCaPem":CA2})),
    ] {
        let response = f
            .client
            .put(format!("{}/api/v1/settings/{path}", f.origin))
            .json(&payload)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
    }
    let status = read(&f, "/settings/tls-activation").await;
    assert_eq!(status["registeredGateways"], 2);
    assert_eq!(status["activationRequired"], true);
    assert_eq!(
        status["certificateFingerprint"],
        crate::settings::certificate_fingerprint(CERT1).unwrap()
    );
    let request = ActivationRequest {
        operation_id: Uuid::new_v4(),
        expected_generation: serde_json::from_value(status["generation"].clone()).unwrap(),
        pending_revision: status["pendingRevision"].as_str().unwrap().into(),
    };
    let activation = format!("{}/api/v1/settings/tls-activation", f.origin);
    for _ in 0..2 {
        let missing = f.client.post(&activation).json(&request).send().await.unwrap();
        assert_eq!(missing.status(), reqwest::StatusCode::CONFLICT);
        assert!(missing.text().await.unwrap().contains("CA_QUORUM_REQUIRED"));
        let approval: Value = approve_ca(&f, &request, None).await.error_for_status().unwrap().json().await.unwrap();
        assert_eq!(approval["approvedCount"], 1);
        assert_eq!(approval["quorumMet"], false);
        assert!(f.app.tls.as_ref().unwrap().operation(request.operation_id).unwrap().is_none());
    }
    let (_, foreign) = approval_admin(&f, "tls-foreign-admin", "other-tenant").await;
    assert_eq!(approve_ca(&f, &request, Some(&foreign)).await.status(), reqwest::StatusCode::CONFLICT);
    let (second_id, second) = approval_admin(&f, "tls-second-admin", "tls-review").await;
    let stale = ActivationRequest { pending_revision: "a".repeat(64), ..request.clone() };
    assert_eq!(approve_ca(&f, &stale, Some(&second)).await.status(), reqwest::StatusCode::CONFLICT);
    let approval: Value = approve_ca(&f, &request, Some(&second)).await.error_for_status().unwrap().json().await.unwrap();
    assert_eq!(approval["approvedCount"], 2);
    assert_eq!(approval["quorumMet"], true);
    let query = format!("/settings/tls-activation/approvals/{}", request.operation_id);
    assert_eq!(read(&f, &query).await, approval);
    sqlx::query("UPDATE mgmt_users SET enabled=false WHERE id=$1").bind(second_id).execute(&f.app.pool).await.unwrap();
    assert_eq!(f.client.post(&activation).json(&request).send().await.unwrap().status(), reqwest::StatusCode::CONFLICT);
    assert!(f.app.tls.as_ref().unwrap().operation(request.operation_id).unwrap().is_none());
    sqlx::query("UPDATE mgmt_users SET enabled=true WHERE id=$1").bind(second_id).execute(&f.app.pool).await.unwrap();
    // The activation closes the very HTTPS connection carrying this POST.
    let _ = f
        .client
        .post(format!("{}/api/v1/settings/tls-activation", f.origin))
        .json(&request)
        .send()
        .await;
    let original = format!("/settings/tls-activation/{}", request.operation_id);
    let receipt = read(&f, &original).await;
    assert_eq!(receipt["state"], "APPLIED");
    assert_eq!(receipt["observedActive"], true);
    assert!(
        tokio::time::timeout(Duration::from_secs(3), old.message())
            .await
            .unwrap()
            .is_err()
    );
    let (_new_tx, mut new) = hello(&f, "two", "tls-two").await.unwrap();
    assert!(matches!(
        new.message().await.unwrap().unwrap().payload,
        Some(console_frame::Payload::Welcome(_))
    ));
    assert!(hello(&f, "one", "tls-one").await.is_err());
    let pointer = fs::read(f.root.join("state/tls-runtime-active.json")).unwrap();
    let duplicate: Value = f
        .client
        .post(format!("{}/api/v1/settings/tls-activation", f.origin))
        .json(&request)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(duplicate, receipt);
    assert_eq!(
        pointer,
        fs::read(f.root.join("state/tls-runtime-active.json")).unwrap()
    );
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM mgmt_audit WHERE operation='settings.tls.activate'",
    )
    .fetch_one(&f.app.pool)
    .await
    .unwrap();
    assert_eq!(count, 1);
    assert_eq!(
        read(&f, "/settings/tls").await["certificateFingerprint"],
        crate::settings::certificate_fingerprint(CERT2).unwrap()
    );
    assert_eq!(
        read(&f, "/settings/client-ca").await["caFingerprint"],
        crate::settings::certificate_fingerprint(CA2).unwrap()
    );
}

#[tokio::test]
#[ignore = "requires a fresh isolated TLS review PostgreSQL instance"]
async fn tls_runtime_http_auth_csrf_and_missing_receipt_do_not_repeat_activation() {
    let f = fixture().await;
    let unauthenticated = reqwest::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(CA1.as_bytes()).unwrap())
        .build()
        .unwrap();
    assert_eq!(
        unauthenticated
            .get(format!("{}/api/v1/settings/tls-activation", f.origin))
            .send()
            .await
            .unwrap()
            .status(),
        reqwest::StatusCode::UNAUTHORIZED
    );
    // A valid session without an Origin/CSRF pair cannot activate.
    let req = ActivationRequest {
        operation_id: Uuid::new_v4(),
        expected_generation: Uuid::new_v4(),
        pending_revision: "a".repeat(64),
    };
    let response = f
        .client
        .post(format!("{}/api/v1/settings/tls-activation", f.origin))
        .header("x-csrf-token", "wrong")
        .json(&req)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
    assert!(
        f.app
            .tls
            .as_ref()
            .unwrap()
            .operation(req.operation_id)
            .unwrap()
            .is_none()
    );
    for role in ["viewer", "operator", "recovery"] {
        let id = Uuid::new_v4();
        let token = auth::random_token();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,'tls-review',$2,'unused',$2,false)").bind(id).bind(role).execute(&f.app.pool).await.unwrap();
        sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'tls-review-csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(id).execute(&f.app.pool).await.unwrap();
        let response = f
            .client
            .get(format!("{}/api/v1/settings/tls-activation", f.origin))
            .header(reqwest::header::COOKIE, format!("kura_session={token}"))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
    }
    let runtime = f.app.tls.as_ref().unwrap();
    let cfg = runtime.prepare_stage().unwrap();
    crate::tls_store::stage(&cfg, CERT2, KEY2, "tls-review-admin").unwrap();
    crate::client_ca_store::stage(&cfg, CA2, "tls-review-admin").unwrap();
    let status = runtime.status().unwrap();
    let req = ActivationRequest {
        operation_id: Uuid::new_v4(),
        expected_generation: serde_json::from_value(status["generation"].clone()).unwrap(),
        pending_revision: status["pendingRevision"].as_str().unwrap().into(),
    };
    let (_, second) = approval_admin(&f, "tls-second-admin", "tls-review").await;
    approve_ca(&f, &req, None).await.error_for_status().unwrap();
    approve_ca(&f, &req, Some(&second)).await.error_for_status().unwrap();
    let _ = f
        .client
        .post(format!("{}/api/v1/settings/tls-activation", f.origin))
        .json(&req)
        .send()
        .await;
    fs::remove_file(f.root.join(format!(
        "state/tls-operations/{}.receipt.json",
        req.operation_id
    )))
    .unwrap();
    let original = format!("/settings/tls-activation/{}", req.operation_id);
    assert_eq!(read(&f, &original).await["state"], "UNKNOWN");
    let observed: Value = f
        .client
        .post(format!("{}/api/v1/settings/tls-activation", f.origin))
        .json(&req)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(observed["state"], "UNKNOWN");
    assert_eq!(observed["observedActive"], true);
    assert_eq!(
        read(&f, "/settings/tls-activation").await["blockingOperationId"],
        req.operation_id.to_string()
    );
    let alternative = ActivationRequest {
        operation_id: Uuid::new_v4(),
        ..req.clone()
    };
    let rejected: Value = f
        .client
        .post(format!("{}/api/v1/settings/tls-activation", f.origin))
        .json(&alternative)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(rejected["state"], "REJECTED");
    assert!(
        !f.root
            .join(format!(
                "state/tls-operations/{}.intent.json",
                alternative.operation_id
            ))
            .exists()
    );
    assert!(
        !f.root
            .join(format!(
                "state/tls-operations/{}.receipt.json",
                req.operation_id
            ))
            .exists()
    );
}


/// A new CSR-specific fixture; existing TLS activation acceptances are not run.
#[tokio::test]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn server_csr_live_admin_permissions_external_signature_and_unknown_stage_fence() {
    let f=fixture().await;
    let base=format!("{}/api/v1/settings/server-csr",f.origin);
    let id=Uuid::new_v4();
    let input=json!({"requestId":id,"origin":f.origin});
    let token=auth::random_token();let user=Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,'tls-review','csr-operator','unused','operator',false)").bind(user).execute(&f.app.pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'operator-csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(user).execute(&f.app.pool).await.unwrap();
    for (method,suffix) in [(reqwest::Method::GET,String::new()),(reqwest::Method::GET,format!("/{id}")),(reqwest::Method::POST,String::new()),(reqwest::Method::PUT,format!("/{id}/certificate")),(reqwest::Method::POST,format!("/{id}/retire")),(reqwest::Method::POST,format!("/{id}/stage"))] {
        let response=f.client.request(method.clone(),format!("{base}{suffix}")).header(reqwest::header::COOKIE,"").json(&input).send().await.unwrap();assert_eq!(response.status(),reqwest::StatusCode::UNAUTHORIZED);
        let response=f.client.request(method.clone(),format!("{base}{suffix}")).header(reqwest::header::COOKIE,format!("kura_session={token}")).header("x-csrf-token","operator-csrf").json(&input).send().await.unwrap();assert_eq!(response.status(),reqwest::StatusCode::FORBIDDEN);
        if method!=reqwest::Method::GET {
            let response=f.client.request(method.clone(),format!("{base}{suffix}")).header("x-csrf-token","wrong").json(&input).send().await.unwrap();assert_eq!(response.status(),reqwest::StatusCode::FORBIDDEN);
            let response=f.client.request(method,format!("{base}{suffix}")).header(reqwest::header::ORIGIN,"https://attacker.invalid").json(&input).send().await.unwrap();assert_eq!(response.status(),reqwest::StatusCode::FORBIDDEN);
        }
    }
    let state=std::path::Path::new(&f.app.config.state_dir);
    assert!(!state.join("server-csr-requests").exists());
    let original=f.app.tls.as_ref().unwrap().status().unwrap();
    let ready:Value=f.client.post(&base).json(&input).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(ready["state"],"READY");assert!(!ready.to_string().contains("PRIVATE KEY"));
    let cert=crate::server_csr::tests::sign(state,id,crate::server_csr::tests::EXTENSIONS,"1");
    let rejected:Value=f.client.put(format!("{base}/{id}/certificate")).json(&json!({"serverPem":CERT2})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(rejected["state"],"READY");assert_eq!(rejected["certificateRejected"],true);
    let signed:Value=f.client.put(format!("{base}/{id}/certificate")).json(&json!({"serverPem":cert})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();assert_eq!(signed["state"],"SIGNED");assert!(!signed.to_string().contains("PRIVATE KEY"));
    let staged:Value=f.client.post(format!("{base}/{id}/stage")).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();assert_eq!(staged["state"],"STAGED");
    let active=f.app.tls.as_ref().unwrap().status().unwrap();assert_eq!(active["generation"],original["generation"]);assert_eq!(active["certificateFingerprint"],original["certificateFingerprint"]);assert_eq!(active["pendingTlsFingerprint"],signed["certificateFingerprint"]);
    let dir=state.join(format!("server-csr-requests/{id}"));fs::remove_file(dir.join("staged.json")).unwrap();
    let observed=read(&f,&format!("/settings/server-csr/{id}")).await;assert_eq!(observed["state"],"UNKNOWN");assert!(!observed.to_string().contains("PRIVATE KEY"));
    assert_eq!(read(&f,"/settings/tls-activation").await["blockingCsrRequestId"],id.to_string());
    let response=f.client.put(format!("{}/api/v1/settings/tls",f.origin)).json(&json!({"serverPem":CERT2,"serverKey":KEY2})).send().await.unwrap();assert!(!response.status().is_success());
    assert!(!dir.join("staged.json").exists());assert_eq!(f.app.tls.as_ref().unwrap().status().unwrap()["generation"],original["generation"]);
    println!("CSR_HTTP_ACCEPTED request_id={id} public_key_only=true no_activation=true permission_cases=20 unknown_stage_retained=true");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_original_receipts_survive_response_loss_and_later_rotation() {
    let f = fixture().await;
    let gateway = "rotation-original-review";
    let old = crate::gateway_registration::certificate_sha256(CERT1).unwrap();
    let new = crate::gateway_registration::certificate_sha256(CERT2).unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','rotation review',$2)")
        .bind(gateway).bind(&old).execute(&f.app.pool).await.unwrap();
    sqlx::query("UPDATE mgmt_gateways SET binding_generation=7 WHERE id=$1").bind(gateway).execute(&f.app.pool).await.unwrap();
    let endpoint = format!("{}/api/v1/gateways/{gateway}/certificate",f.origin);
    let operation = Uuid::new_v4();
    let body = json!({"operationId":operation,"previousCertificateSha256":old,"previousBindingGeneration":7,"clientCertificatePem":CERT2});
    let response = f.client.post(&endpoint).json(&body).send().await.unwrap();
    assert_eq!(response.status(),reqwest::StatusCode::OK);
    drop(response); // Deliberately do not read the original body; this is not a real wire-failure claim.
    let original_path = format!("/gateways/{gateway}/certificate-operations/{operation}");
    let original = read(&f,&original_path).await;
    assert_eq!(original["state"],"APPLIED");
    assert_eq!(original["operationId"],operation.to_string());
    assert_eq!(original["previousCertificateSha256"],old);
    assert_eq!(original["certificateSha256"],new);
    assert_eq!(original["certificatePemSha256"],auth::hash(CERT2.as_bytes()));
    assert_eq!(original["bundleRequested"],false);
    assert_eq!(original["receiptVersion"],2);
    assert_eq!(original["previousBindingGeneration"],7);
    assert_eq!(original["bindingGeneration"],8);
    assert_eq!(read(&f,&format!("/gateways/{gateway}")).await["bindingGeneration"],8);
    assert!(original.get("identityBundleToken").is_none());
    assert_eq!(f.client.post(&endpoint).json(&body).send().await.unwrap().status(),reqwest::StatusCode::CONFLICT);
    let next = Uuid::new_v4();
    let value:Value = f.client.post(&endpoint).json(&json!({"operationId":next,"previousCertificateSha256":new,"previousBindingGeneration":8,"clientCertificatePem":CERT1}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(value["state"],"APPLIED");
    assert_eq!(read(&f,&original_path).await,original); // The current certificate is old again; original proof stays exact.
    let current=read(&f,&format!("/gateways/{gateway}")).await;
    assert_eq!(current["certificateSha256"],old);
    assert_eq!(current["bindingGeneration"],9);
    assert_eq!(value["previousBindingGeneration"],8);
    assert_eq!(value["bindingGeneration"],9);
    let audit:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE target=$1 AND operation='gateway.certificate.rotate' AND outcome='SUCCEEDED'")
        .bind(gateway).fetch_one(&f.app.pool).await.unwrap();assert_eq!(audit,2);
    sqlx::query("DELETE FROM mgmt_gateway_certificate_rotations WHERE operation_id=$1").bind(operation).execute(&f.app.pool).await.unwrap();
    assert_eq!(read(&f,&original_path).await["state"],"UNKNOWN");
    assert_eq!(f.client.post(&endpoint).json(&body).send().await.unwrap().status(),reqwest::StatusCode::CONFLICT);
    let blocked:Value=f.client.post(&endpoint).json(&json!({"operationId":Uuid::new_v4(),"previousCertificateSha256":old,"clientCertificatePem":CERT2}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(blocked["state"],"REJECTED");assert_eq!(blocked["rejectionCode"],"RECOVERY_REQUIRED");
    assert_eq!(read(&f,&format!("/gateways/{gateway}")).await["certificateSha256"],old);
    let mut tx=f.app.pool.begin().await.unwrap();
    assert!(crate::gateway_certificate::maintenance_guard(&mut tx,"tls-review",gateway).await.is_err());
    let missing: i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_gateway_certificate_rotations WHERE operation_id=$1").bind(operation).fetch_one(&f.app.pool).await.unwrap();assert_eq!(missing,0);
    println!("ROTATION_ORIGINAL_FIXTURE gateway={gateway} first={operation} next={next} retainedMissingFirstReceipt=true");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_legacy_rotation_keeps_generation_for_installed_setup_tools() {
    let f = fixture().await;
    let gateway = "rotation-legacy-setup-tool";
    let old = crate::gateway_registration::certificate_sha256(CERT1).unwrap();
    let new = crate::gateway_registration::certificate_sha256(CERT2).unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation) VALUES($1,'tls-review','legacy setup tool',$2,5)")
        .bind(gateway).bind(&old).execute(&f.app.pool).await.unwrap();
    let operation = Uuid::new_v4();
    let receipt: Value = f.client.post(format!("{}/api/v1/gateways/{gateway}/certificate", f.origin))
        .json(&json!({"operationId":operation,"previousCertificateSha256":old,"clientCertificatePem":CERT2}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(receipt["state"], "APPLIED");
    let current = read(&f, &format!("/gateways/{gateway}")).await;
    assert_eq!(current["certificateSha256"], new);
    assert_eq!(current["bindingGeneration"], 5, "Installed legacy setup tools rotate certificates without changing the local generation");
    assert_eq!(receipt["receiptVersion"], 1);
    assert!(receipt["previousBindingGeneration"].is_null());
    assert!(receipt["bindingGeneration"].is_null());
    assert_eq!(read(&f, &format!("/gateways/{gateway}/certificate-operations/{operation}")).await, receipt);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_generation_receipts_reject_stale_races_and_exhaustion() {
    let f = fixture().await;
    let gateway = "rotation-generation-review";
    let old = crate::gateway_registration::certificate_sha256(CERT1).unwrap();
    let new = crate::gateway_registration::certificate_sha256(CERT2).unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation) VALUES($1,'tls-review','generation review',$2,3)")
        .bind(gateway).bind(&old).execute(&f.app.pool).await.unwrap();
    let endpoint = format!("{}/api/v1/gateways/{gateway}/certificate", f.origin);
    let operation = Uuid::new_v4();
    let stale: Value = f.client.post(&endpoint).json(&json!({"operationId":operation,
        "previousCertificateSha256":old,"previousBindingGeneration":2,"clientCertificatePem":CERT2}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(stale["state"], "REJECTED");
    assert_eq!(stale["previousBindingGeneration"], 2);
    assert_eq!(stale["bindingGeneration"], 3);
    let current = read(&f, &format!("/gateways/{gateway}")).await;
    assert_eq!(current["certificateSha256"], old);
    assert_eq!(current["bindingGeneration"], 3);
    for invalid in [0, -1] {
        assert_eq!(f.client.post(&endpoint).json(&json!({"operationId":Uuid::new_v4(),
            "previousCertificateSha256":old,"previousBindingGeneration":invalid,"clientCertificatePem":CERT2}))
            .send().await.unwrap().status(), reqwest::StatusCode::BAD_REQUEST);
    }
    let pending = crate::gateway_certificate::Request { operation: Uuid::new_v4(), gateway: gateway.into(),
        tenant: "tls-review".into(), previous: old.clone(), previous_generation: Some(3),
        certificate: Some(new), pem_hash: auth::hash(CERT2.as_bytes()), bundle: false };
    assert!(matches!(crate::gateway_certificate::reserve(&f.app.pool, &pending, "synthetic-race-review", None).await, Ok(true)));
    // A synthetic concurrent administrative generation change. No task executor runs.
    sqlx::query("UPDATE mgmt_gateways SET binding_generation=4 WHERE id=$1")
        .bind(gateway).execute(&f.app.pool).await.unwrap();
    let mut tx = f.app.pool.begin().await.unwrap();
    assert!(matches!(crate::gateway_certificate::begin_application(&mut tx, &pending).await, Ok(false)));
    tx.commit().await.unwrap();
    let rejected = read(&f, &format!("/gateways/{gateway}/certificate-operations/{}", pending.operation)).await;
    assert_eq!(rejected["state"], "REJECTED");
    assert_eq!(rejected["previousBindingGeneration"], 3);
    assert_eq!(rejected["bindingGeneration"], 4);
    sqlx::query("UPDATE mgmt_gateways SET binding_generation=$2 WHERE id=$1")
        .bind(gateway).bind(i64::MAX).execute(&f.app.pool).await.unwrap();
    let exhausted: Value = f.client.post(&endpoint).json(&json!({"operationId":Uuid::new_v4(),
        "previousCertificateSha256":old,"previousBindingGeneration":i64::MAX,"clientCertificatePem":CERT2}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(exhausted["state"], "REJECTED");
    assert_eq!(exhausted["previousBindingGeneration"], json!(i64::MAX));
    assert!(exhausted["bindingGeneration"].is_null());
    assert_eq!(read(&f, &format!("/gateways/{gateway}")).await["certificateSha256"], old);
    assert_eq!(read(&f, &format!("/gateways/{gateway}")).await["bindingGeneration"], json!(i64::MAX));
    assert_eq!(read(&f, &format!("/gateways/{gateway}/certificate-operations/{operation}")).await, stale);
    println!("ROTATION_GENERATION_REFUSALS gateway={gateway} stale={operation} race={} original_results_retained=true", pending.operation);
}

#[tokio::test]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_generation_migration_preserves_real_v25_original_receipts() {
    let url = std::env::var("YOFC_REVIEW_PG_URL").expect("fresh private review PostgreSQL required");
    let schema = format!("rotation_v25_upgrade_{}", Uuid::new_v4().simple());
    let admin = PgPool::connect(&url).await.unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}")).execute(&admin).await.unwrap();
    let options = url.parse::<sqlx::postgres::PgConnectOptions>().unwrap().options([("search_path", schema.as_str())]);
    let pool = sqlx::postgres::PgPoolOptions::new().max_connections(2).connect_with(options).await.unwrap();
    let mut migrations = sqlx::migrate!("../../migrations");
    let old = migrations.iter().filter(|migration| migration.version <= 25).cloned().collect::<Vec<_>>();
    migrations.migrations = std::borrow::Cow::Owned(old);
    migrations.run(&pool).await.unwrap();
    let gateway = "rotation-legacy-v25";
    let operation = Uuid::new_v4();
    let previous = auth::hash(b"legacy previous");
    let certificate = auth::hash(b"legacy certificate");
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation,revision,certificate_rotation_operation_id) VALUES($1,'tls-review','legacy v25',$2,17,9,$3)")
        .bind(gateway).bind(&certificate).bind(operation).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_gateway_certificate_rotation_ids(operation_id,gateway_id,tenant_id) VALUES($1,$2,'tls-review')")
        .bind(operation).bind(gateway).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_gateway_certificate_rotations(operation_id,gateway_id,tenant_id,previous_certificate_sha256,certificate_sha256,certificate_pem_sha256,bundle_requested,state,accepted_revision,actor,accepted_at) VALUES($1,$2,'tls-review',$3,$4,$5,false,'APPLIED',2,'legacy-admin',now())")
        .bind(operation).bind(gateway).bind(&previous).bind(&certificate).bind(auth::hash(b"legacy PEM")).execute(&pool).await.unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    let receipt = crate::gateway_certificate::read(&pool, "tls-review", gateway, operation).await.ok().unwrap();
    assert_eq!(receipt["state"], "APPLIED");
    assert_eq!(receipt["receiptVersion"], 1);
    assert!(receipt["previousBindingGeneration"].is_null());
    assert!(receipt["bindingGeneration"].is_null());
    assert_eq!(receipt["revision"], 2);
    assert_eq!(receipt["certificateSha256"], certificate);
    let generation: i64 = sqlx::query_scalar("SELECT binding_generation FROM mgmt_gateways WHERE id=$1")
        .bind(gateway).fetch_one(&pool).await.unwrap();
    assert_eq!(generation, 17);
    sqlx::query("UPDATE mgmt_gateways SET binding_generation=19 WHERE id=$1").bind(gateway).execute(&pool).await.unwrap();
    assert_eq!(crate::gateway_certificate::read(&pool, "tls-review", gateway, operation).await.ok().unwrap(), receipt);
    println!("ROTATION_V25_MIGRATION schema={schema} operation={operation} legacy_generations_not_backfilled=true");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_original_receipts_registry_gateway_mismatch_fences_followup_writes() {
    let f = fixture().await;
    let gateway = "rotation-registry-mismatch";
    let other = "rotation-registry-other";
    let old = crate::gateway_registration::certificate_sha256(CERT1).unwrap();
    let new = crate::gateway_registration::certificate_sha256(CERT2).unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','registry mismatch',$2),($3,'tls-review','registry other',$4)")
        .bind(gateway).bind(&old).bind(other).bind("a".repeat(64)).execute(&f.app.pool).await.unwrap();
    let operation = Uuid::new_v4();
    let endpoint = format!("{}/api/v1/gateways/{gateway}/certificate", f.origin);
    let original:Value = f.client.post(&endpoint).json(&json!({"operationId":operation,"previousCertificateSha256":old,"clientCertificatePem":CERT2}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(original["state"], "APPLIED");
    // A restored or damaged registry can disagree while the public receipt still exists.
    // Do not repair it or infer the old outcome from the current credential.
    sqlx::query("UPDATE mgmt_gateway_certificate_rotation_ids SET gateway_id=$2 WHERE operation_id=$1")
        .bind(operation).bind(other).execute(&f.app.pool).await.unwrap();
    let path = format!("/gateways/{gateway}/certificate-operations/{operation}");
    assert_eq!(read(&f, &path).await["state"], "UNKNOWN");
    let mut tx = f.app.pool.begin().await.unwrap();
    assert!(crate::gateway_certificate::maintenance_guard(&mut tx, "tls-review", gateway).await.is_err(),
        "An inconsistent original registry must fence task and credential writers");
    drop(tx);
    let later:Value = f.client.post(&endpoint).json(&json!({"operationId":Uuid::new_v4(),"previousCertificateSha256":new,"clientCertificatePem":CERT1}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(later["state"], "REJECTED");
    assert_eq!(later["rejectionCode"], "RECOVERY_REQUIRED");
    assert_eq!(read(&f, &format!("/gateways/{gateway}")).await["certificateSha256"], new);
    assert_eq!(read(&f, &path).await["state"], "UNKNOWN");
    let retained:String = sqlx::query_scalar("SELECT gateway_id FROM mgmt_gateway_certificate_rotation_ids WHERE operation_id=$1")
        .bind(operation).fetch_one(&f.app.pool).await.unwrap();
    assert_eq!(retained, other);
    println!("ROTATION_REGISTRY_MISMATCH_FIXTURE gateway={gateway} operation={operation} registryMismatchRetained=true laterWritesFenced=true");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_original_receipts_reject_bad_material_and_fence_pending_intent() {
    let f=fixture().await;let gateway="rotation-pending-review";
    let old=crate::gateway_registration::certificate_sha256(CERT1).unwrap();
    let new=crate::gateway_registration::certificate_sha256(CERT2).unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','rotation pending',$2)")
        .bind(gateway).bind(&old).execute(&f.app.pool).await.unwrap();
    let endpoint=format!("{}/api/v1/gateways/{gateway}/certificate",f.origin);
    let rejected=Uuid::new_v4();
    let response:Value=f.client.post(&endpoint).json(&json!({"operationId":rejected,"previousCertificateSha256":old,"clientCertificatePem":CERT2,"clientKeyPem":KEY1}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(response["state"],"REJECTED");assert_eq!(response["rejectionCode"],"INVALID_ARGUMENT");
    assert_eq!(read(&f,&format!("/gateways/{gateway}/certificate-operations/{rejected}")).await,response);
    assert_eq!(read(&f,&format!("/gateways/{gateway}")).await["certificateSha256"],old);
    let pending=crate::gateway_certificate::Request{operation:Uuid::new_v4(),gateway:gateway.into(),tenant:"tls-review".into(),previous:old.clone(),certificate:Some(new.clone()),pem_hash:auth::hash(CERT2.as_bytes()),bundle:false,previous_generation:None};
    assert!(crate::gateway_certificate::reserve(&f.app.pool,&pending,"test-supervisor",None).await.ok().unwrap());
    let path=format!("/gateways/{gateway}/certificate-operations/{}",pending.operation);
    assert_eq!(read(&f,&path).await["state"],"UNKNOWN");
    let mut tx=f.app.pool.begin().await.unwrap();
    assert!(crate::gateway_certificate::maintenance_guard(&mut tx,"tls-review",gateway).await.is_err());drop(tx);
    let later:Value=f.client.post(&endpoint).json(&json!({"operationId":Uuid::new_v4(),"previousCertificateSha256":old,"clientCertificatePem":CERT2}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(later["state"],"REJECTED");assert_eq!(later["rejectionCode"],"RECOVERY_REQUIRED");
    // Receipt loss keeps the independent UUID registry and gateway pending pointer. No backfill or retry.
    sqlx::query("DELETE FROM mgmt_gateway_certificate_rotations WHERE operation_id=$1").bind(pending.operation).execute(&f.app.pool).await.unwrap();
    assert_eq!(read(&f,&path).await["state"],"UNKNOWN");
    assert_eq!(f.client.post(&endpoint).json(&json!({"operationId":pending.operation,"previousCertificateSha256":old,"clientCertificatePem":CERT2}))
        .send().await.unwrap().status(),reqwest::StatusCode::CONFLICT);
    let foreign_token=auth::random_token();let actor=Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,'other-review','rotation-foreign','unused','admin',false)").bind(actor).execute(&f.app.pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'other-csrf',now()+interval '1 hour')").bind(auth::hash(foreign_token.as_bytes())).bind(actor).execute(&f.app.pool).await.unwrap();
    assert_eq!(f.client.get(format!("{}/api/v1{path}",f.origin)).header(reqwest::header::COOKIE,format!("kura_session={foreign_token}")).send().await.unwrap().status(),reqwest::StatusCode::NOT_FOUND);
    sqlx::query("UPDATE mgmt_users SET role='viewer' WHERE id=$1").bind(actor).execute(&f.app.pool).await.unwrap();
    assert_eq!(f.client.get(format!("{}/api/v1{path}",f.origin)).header(reqwest::header::COOKIE,format!("kura_session={foreign_token}")).send().await.unwrap().status(),reqwest::StatusCode::FORBIDDEN);
    assert_eq!(f.client.post(&endpoint).header("x-csrf-token","wrong").json(&json!({"operationId":Uuid::new_v4(),"previousCertificateSha256":old,"clientCertificatePem":CERT2})).send().await.unwrap().status(),reqwest::StatusCode::FORBIDDEN);
    let applied:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_gateway_certificate_rotations WHERE gateway_id=$1 AND state='APPLIED'").bind(gateway).fetch_one(&f.app.pool).await.unwrap();assert_eq!(applied,0);
    println!("ROTATION_PENDING_FIXTURE gateway={gateway} rejected={rejected} pending={} retainedMissingPendingReceipt=true",pending.operation);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_original_receipts_guard_writes_and_never_reexport_bundle() {
    let f=fixture().await;
    let actor:Uuid=sqlx::query_scalar("SELECT id FROM mgmt_users WHERE username='tls-review-admin'").fetch_one(&f.app.pool).await.unwrap();
    for state in ["QUEUED","RUNNING","UNKNOWN"] {
        let gateway=format!("rotation-guard-{}",state.to_ascii_lowercase());
        let old=auth::hash(state.as_bytes());let task=Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','synthetic write admission case',$2)").bind(&gateway).bind(&old).execute(&f.app.pool).await.unwrap();
        // Synthetic encrypted admission rows, not actual remote execution or an UNKNOWN resolution.
        let context=crate::task_crypto::TaskContext{id:task,tenant:"tls-review".into(),gateway:gateway.clone(),operation:12};
        let encrypted=f.app.cipher.read().await.seal(&context,"input",&json!({"fixture":"synthetic non-secret admission"})).unwrap();
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,expires_at,input,binding_generation) VALUES($1,$2,'tls-review',$3,12,$4,$5,now()+interval '1 hour',$6,1)")
            .bind(task).bind(&gateway).bind(actor).bind(task.to_string()).bind(state).bind(&encrypted).execute(&f.app.pool).await.unwrap();
        let value:Value=f.client.post(format!("{}/api/v1/gateways/{gateway}/certificate",f.origin)).json(&json!({"operationId":Uuid::new_v4(),"previousCertificateSha256":old,"clientCertificatePem":CERT2})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
        assert_eq!(value["state"],"REJECTED");assert_eq!(value["rejectionCode"],"CONFLICT");
        let retained:String=sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1").bind(task).fetch_one(&f.app.pool).await.unwrap();assert_eq!(retained,state);
        assert_eq!(read(&f,&format!("/gateways/{gateway}")).await["certificateSha256"],old);
    }
    let gateway="rotation-bundle-review";let old=auth::hash(b"bundle-old-credential");let operation=Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','bundle original',$2)").bind(gateway).bind(&old).execute(&f.app.pool).await.unwrap();
    let response:Value=f.client.post(format!("{}/api/v1/gateways/{gateway}/certificate",f.origin)).json(&json!({"operationId":operation,"previousCertificateSha256":old,"clientCertificatePem":CERT2,"clientKeyPem":KEY2})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(response["state"],"APPLIED");let token=response["identityBundleToken"].as_str().unwrap();
    let original=read(&f,&format!("/gateways/{gateway}/certificate-operations/{operation}")).await;
    assert_eq!(original["state"],"APPLIED");assert_eq!(original["bundleRequested"],true);assert_eq!(original["identityBundleTokenRecoverable"],false);
    assert!(original.get("identityBundleToken").is_none());assert!(!original.to_string().contains("PRIVATE KEY"));assert!(!original.to_string().contains(token));
    let row=sqlx::query("SELECT identity_bundle_token_hash,identity_bundle_ciphertext FROM mgmt_gateways WHERE id=$1").bind(gateway).fetch_one(&f.app.pool).await.unwrap();
    assert_eq!(row.get::<String,_>("identity_bundle_token_hash"),auth::hash(token.as_bytes()));assert!(!row.get::<String,_>("identity_bundle_ciphertext").contains("PRIVATE KEY"));
    let duplicate_gateway="rotation-duplicate-review";let duplicate_old=auth::hash(b"duplicate-old");
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','duplicate public credential',$2)").bind(duplicate_gateway).bind(&duplicate_old).execute(&f.app.pool).await.unwrap();
    let duplicate:Value=f.client.post(format!("{}/api/v1/gateways/{duplicate_gateway}/certificate",f.origin)).json(&json!({"operationId":Uuid::new_v4(),"previousCertificateSha256":duplicate_old,"clientCertificatePem":CERT2})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(duplicate["state"],"REJECTED");assert_eq!(duplicate["rejectionCode"],"CONFLICT");
    let pending_gateway="rotation-task-admission-review";let pending_old=auth::hash(b"pending-old");
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','pending task admission',$2)").bind(pending_gateway).bind(&pending_old).execute(&f.app.pool).await.unwrap();
    let pending=crate::gateway_certificate::Request{operation:Uuid::new_v4(),gateway:pending_gateway.into(),tenant:"tls-review".into(),previous:pending_old,certificate:Some(crate::gateway_registration::certificate_sha256(CERT1).unwrap()),pem_hash:auth::hash(CERT1.as_bytes()),bundle:false,previous_generation:None};
    assert!(crate::gateway_certificate::reserve(&f.app.pool,&pending,"test-supervisor",None).await.ok().unwrap());
    let endpoint=format!("{}/api/v1/gateways/{pending_gateway}/tasks",f.origin);
    let response=f.client.post(&endpoint).json(&json!({"operation":"kura.snapshot.save","idempotencyKey":Uuid::new_v4(),"kuraSnapshotSave":yofc_management_protocol::v1::KuraSnapshotSave{expected_latest_id:"0".into()}})).send().await.unwrap();
    assert_eq!(response.status(),reqwest::StatusCode::CONFLICT);
    let response=f.client.post(&endpoint).json(&json!({"operation":"agent.status","idempotencyKey":Uuid::new_v4()})).send().await.unwrap();
    assert_eq!(response.status(),reqwest::StatusCode::ACCEPTED);
    let writes:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_tasks WHERE gateway_id=$1 AND operation<>1").bind(pending_gateway).fetch_one(&f.app.pool).await.unwrap();assert_eq!(writes,0);
    println!("ROTATION_GUARDS_FIXTURE bundleOperation={operation} pendingOperation={} bundleNeverDownloaded=true syntheticWriteStatesRetained=true",pending.operation);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires a fresh private YOFC_REVIEW_PG_URL"]
async fn gateway_certificate_original_receipts_refuse_late_write_without_legacy_false_success() {
    let f=fixture().await;
    let actor:Uuid=sqlx::query_scalar("SELECT id FROM mgmt_users WHERE username='tls-review-admin'").fetch_one(&f.app.pool).await.unwrap();
    for legacy in [true,false] {
        let gateway=format!("rotation-late-write-{}",if legacy{"legacy"}else{"current"});
        let old=auth::hash(gateway.as_bytes());
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES($1,'tls-review','late write injection',$2)").bind(&gateway).bind(&old).execute(&f.app.pool).await.unwrap();
        // Hold only the later credential phase. The short public intent transaction can commit first.
        let mut barrier=f.app.pool.begin().await.unwrap();
        sqlx::query("SELECT singleton FROM mgmt_task_crypto WHERE singleton FOR UPDATE").execute(&mut *barrier).await.unwrap();
        let mut body=json!({"previousCertificateSha256":old,"clientCertificatePem":CERT2});
        if !legacy {body["operationId"]=json!(Uuid::new_v4());}
        let client=f.client.clone();let url=format!("{}/api/v1/gateways/{gateway}/certificate",f.origin);
        let request=tokio::spawn(async move {client.post(url).json(&body).send().await.unwrap()});
        let mut operation=None;
        for _ in 0..100 {
            operation=sqlx::query_scalar::<_,Option<Uuid>>("SELECT certificate_rotation_pending_id FROM mgmt_gateways WHERE id=$1").bind(&gateway).fetch_one(&f.app.pool).await.unwrap();
            if operation.is_some(){break;}
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        let operation=operation.expect("original rotation intent was not published before credential admission");
        let task=Uuid::new_v4();let context=crate::task_crypto::TaskContext{id:task,tenant:"tls-review".into(),gateway:gateway.clone(),operation:12};
        let encrypted=f.app.cipher.read().await.seal(&context,"input",&json!({"fixture":"late synthetic admission, no remote execution"})).unwrap();
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,expires_at,input,binding_generation) VALUES($1,$2,'tls-review',$3,12,$4,'QUEUED',now()+interval '1 hour',$5,1)")
            .bind(task).bind(&gateway).bind(actor).bind(task.to_string()).bind(encrypted).execute(&f.app.pool).await.unwrap();
        barrier.rollback().await.unwrap();
        let response=request.await.unwrap();assert_eq!(response.status(),reqwest::StatusCode::CONFLICT);
        let original=read(&f,&format!("/gateways/{gateway}/certificate-operations/{operation}")).await;
        assert_eq!(original["state"],"REJECTED");assert_eq!(original["rejectionCode"],"CONFLICT");
        assert_eq!(read(&f,&format!("/gateways/{gateway}")).await["certificateSha256"],old);
        let retained:String=sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1").bind(task).fetch_one(&f.app.pool).await.unwrap();assert_eq!(retained,"QUEUED");
        println!("ROTATION_LATE_ADMISSION_FIXTURE gateway={gateway} operation={operation} knownRejectionNoCredentialWrite=true");
    }
}
