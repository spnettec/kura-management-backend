//! Admission contract of the generic task API for the joint NM/firewall
//! operations, mirroring `firewall_http_tests.rs`. Requires an explicit
//! isolated PostgreSQL fixture; retains all data.
use super::*;
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
#[ignore = "requires explicit new isolated PostgreSQL fixture; retains all data"]
async fn postgres_nm_http_admission_contract_mirrors_the_firewall_family() {
    let url = std::env::var("YOFC_TEST_NM_POSTGRES_URL").unwrap();
    let admin = PgPool::connect(&url).await.unwrap();
    let schema = format!("nm_http_{}", Uuid::new_v4().simple());
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
    let directory = tempfile::tempdir().unwrap().keep();
    std::fs::set_permissions(
        &directory,
        std::os::unix::fs::PermissionsExt::from_mode(0o700),
    )
    .unwrap();
    let key = directory.join("task.key");
    crate::task_crypto::create_key(&key).unwrap();
    let cipher = TaskCipher::load(&key).unwrap();
    assert_eq!(cipher.prepare_storage(&pool).await.unwrap(), 0);
    let config = Config {
        http_addr: "127.0.0.1:0".parse().unwrap(),
        grpc_addr: "127.0.0.1:0".parse().unwrap(),
        origin: "https://fixture.invalid".into(),
        tls_cert: String::new(),
        tls_key: String::new(),
        client_ca: String::new(),
        web_dir: String::new(),
        state_dir: String::new(),
    };
    let app = App::new(pool.clone(), config, cipher.clone()).unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    struct Stop(tokio::task::JoinHandle<()>);
    impl Drop for Stop {
        fn drop(&mut self) {
            self.0.abort();
        }
    }
    let server = Stop(tokio::spawn(async move {
        axum::serve(
            listener,
            router(app).into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .unwrap();
    }));
    let mut actors = Vec::new();
    for (index, (role, tenant, initial)) in [
        ("admin", "a", false),
        ("operator", "a", false),
        ("viewer", "a", false),
        ("admin", "b", false),
    ]
    .into_iter()
    .enumerate()
    {
        let id = Uuid::new_v4();
        let token = auth::random_token();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,$2,$3,'unused',$4,$5)").bind(id).bind(tenant).bind(format!("nm-{index}")).bind(role).bind(initial).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(id).execute(&pool).await.unwrap();
        actors.push(token);
    }
    let epoch = Uuid::new_v4();
    let caps = json!(["nm.remote.v1", "task.reconcile.v1"]);
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation,online_reported,last_seen,capabilities) VALUES('nm','a','nm','fixture',$1,1,true,now(),$2)").bind(epoch).bind(&caps).execute(&pool).await.unwrap();
    let client = reqwest::Client::new();
    let path = "/api/v1/gateways/nm/tasks";
    let request = |method: reqwest::Method, path: &str, actor: usize| {
        client
            .request(method, format!("{base}{path}"))
            .header("cookie", format!("kura_session={}", actors[actor]))
            .header("origin", "https://fixture.invalid")
            .header("x-csrf-token", "csrf")
    };
    let change = json!({"sourceDigest":"a".repeat(64),"confirmSeconds":60,
        "changes":[{"interface":"eth0","ipv4":{"method":"manual",
        "addresses":[{"address":"192.0.2.10","prefix":24}],"gateway":"192.0.2.1",
        "dns":["192.0.2.53"],"ignoreAutoDns":true,"ignoreAutoRoutes":false,"wanPriority":30}}]});
    let action = json!({"operationId":Uuid::new_v4().to_string(),"confirmationToken":"b".repeat(32)});
    let make = |operation: &str| {
        let mut v = json!({"operation":operation,"idempotencyKey":Uuid::new_v4().to_string()});
        if operation == "nm.prepare" {
            v["nmChange"] = change.clone();
        }
        if matches!(operation, "nm.confirm" | "nm.rollback") {
            v["nmAction"] = action.clone();
        }
        v
    };
    // Viewer/operator and the foreign tenant never reach write admission;
    // reads are admitted for the admin of the owning tenant only here.
    for operation in ["nm.view", "nm.prepare", "nm.confirm", "nm.rollback"] {
        for actor in 1..4 {
            // actor 3 is the foreign-tenant admin: the gateway is invisible.
            assert_eq!(
                request(reqwest::Method::POST, path, actor)
                    .json(&make(operation))
                    .send()
                    .await
                    .unwrap()
                    .status()
                    .as_u16(),
                if actor == 3 { 404 } else { 403 }
            );
        }
    }
    assert_eq!(
        client
            .post(format!("{base}{path}"))
            .json(&make("nm.view"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        401
    );
    for bad in [
        json!({"operation":"nm.prepare","idempotencyKey":"no-draft"}),
        json!({"operation":"nm.view","idempotencyKey":"wrong-action","nmAction":action}),
        // The interface grammar is the agent's preview validation; the center
        // rejects wire-invalid digests and windows before admission.
        json!({"operation":"nm.prepare","idempotencyKey":"bad-digest","nmChange":{"sourceDigest":"zz","confirmSeconds":60,"changes":[{"interface":"eth0"}]}}),
        json!({"operation":"nm.confirm","idempotencyKey":"wrong-token","nmAction":{"operationId":Uuid::new_v4().to_string(),"confirmationToken":"not-hex"}}),
    ] {
        assert!(matches!(
            request(reqwest::Method::POST, path, 0)
                .json(&bad)
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            400 | 422
        ));
    }
    // Without the capability the writer is refused before admission.
    sqlx::query("UPDATE mgmt_gateways SET capabilities='[]' WHERE id='nm'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&make("nm.prepare"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        422
    );
    // Offline gateways refuse writes with 409, reads still queue.
    sqlx::query("UPDATE mgmt_gateways SET capabilities=$1,online_reported=false WHERE id='nm'")
        .bind(&caps)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&make("nm.prepare"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        409
    );
    sqlx::query("UPDATE mgmt_gateways SET online_reported=true,last_seen=now() WHERE id='nm'")
        .execute(&pool)
        .await
        .unwrap();
    // Admin admission succeeds for the whole family with the capability.
    for operation in ["nm.view", "nm.prepare", "nm.confirm", "nm.rollback"] {
        let response = request(reqwest::Method::POST, path, 0)
            .json(&make(operation))
            .send()
            .await
            .unwrap();
        assert_eq!(
            response.status().as_u16(),
            202,
            "{operation}: {}",
            response.text().await.unwrap()
        );
    }
    server.0.abort();
    let _ = sqlx::query(&format!("DROP SCHEMA {schema} CASCADE")).execute(&admin).await;
}
