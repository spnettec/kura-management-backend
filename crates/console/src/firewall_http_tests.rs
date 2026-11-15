//! Fresh isolated PostgreSQL + real loopback HTTP. No agent/system mutation.
use super::*;
use prost::Message;
use sha2::{Digest, Sha256};
use yofc_management_protocol::v1::{ErrorCode, TaskResult, TaskState};

#[tokio::test]
#[ignore = "requires explicit new isolated PostgreSQL fixture; retains all data"]
async fn postgres_firewall_http_encryption_authority_unknown_and_connection_epoch_contract() {
    let url = std::env::var("YOFC_TEST_FIREWALL_POSTGRES_URL").unwrap();
    let admin = PgPool::connect(&url).await.unwrap();
    let schema = format!("firewall_http_{}", Uuid::new_v4().simple());
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
        ("admin", "a", true),
    ]
    .into_iter()
    .enumerate()
    {
        let id = Uuid::new_v4();
        let token = auth::random_token();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,$2,$3,'unused',$4,$5)").bind(id).bind(tenant).bind(format!("firewall-{index}")).bind(role).bind(initial).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(id).execute(&pool).await.unwrap();
        actors.push(token);
    }
    let epoch = Uuid::new_v4();
    let caps = json!(["firewall.remote.v1", "task.reconcile.v1"]);
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation,online_reported,last_seen,capabilities) VALUES('firewall','a','firewall','fixture',$1,1,true,now(),$2)").bind(epoch).bind(&caps).execute(&pool).await.unwrap();
    let client = reqwest::Client::new();
    let path = "/api/v1/gateways/firewall/tasks";
    let request = |method: reqwest::Method, path: &str, actor: usize| {
        client
            .request(method, format!("{base}{path}"))
            .header("cookie", format!("kura_session={}", actors[actor]))
            .header("origin", "https://fixture.invalid")
            .header("x-csrf-token", "csrf")
    };
    let candidate = json!({"schemaVersion":3,"ipv4":{"inputPolicy":"accept","forwardPolicy":"drop","allowIcmp":false},"ipv6":{"inputPolicy":"accept","forwardPolicy":"drop","allowIcmp":false},"input":[{"family":"ipv4","protocol":"tcp","ports":{"first":18888,"last":18889},"inputInterface":"wan0"}],"portForward":[],"nat":[]});
    let draft = json!({"expectedRevision":0,"sourceDigest":"a".repeat(64),"confirmSeconds":180,"candidate":candidate});
    let action =
        json!({"transactionId":format!("fw-{}",Uuid::new_v4()),"confirmationToken":"b".repeat(32)});
    let make = |operation: &str| {
        let mut v = json!({"operation":operation,"idempotencyKey":Uuid::new_v4().to_string()});
        if operation == "firewall.prepare" {
            v["firewallChange"] = draft.clone();
        }
        if matches!(operation, "firewall.confirm" | "firewall.rollback") {
            v["firewallAction"] = action.clone();
        }
        v
    };
    for operation in [
        "firewall.view",
        "firewall.prepare",
        "firewall.confirm",
        "firewall.rollback",
    ] {
        for actor in 1..5 {
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
            .json(&make("firewall.view"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        401
    );
    assert_eq!(
        client
            .post(format!("{base}{path}"))
            .header("cookie", format!("kura_session={}", actors[0]))
            .json(&make("firewall.view"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        403
    );
    for bad in [
        json!({"operation":"firewall.prepare","idempotencyKey":"no-draft"}),
        json!({"operation":"firewall.view","idempotencyKey":"wrong-action","firewallAction":action}),
        json!({"operation":"firewall.prepare","idempotencyKey":"escape","firewallChange":{"expectedRevision":0,"sourceDigest":"a".repeat(64),"confirmSeconds":180,"candidate":{"schemaVersion":3,"command":"nft flush ruleset"}}}),
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
    sqlx::query("UPDATE mgmt_gateways SET capabilities='[]' WHERE id='firewall'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&make("firewall.prepare"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        422
    );
    sqlx::query(
        "UPDATE mgmt_gateways SET capabilities=$1,online_reported=false WHERE id='firewall'",
    )
    .bind(&caps)
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&make("firewall.prepare"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        409
    );
    sqlx::query(
        "UPDATE mgmt_gateways SET online_reported=true,last_seen=now() WHERE id='firewall'",
    )
    .execute(&pool)
    .await
    .unwrap();
    let input = make("firewall.prepare");
    let r = request(reqwest::Method::POST, path, 0)
        .json(&input)
        .send()
        .await
        .unwrap();
    assert_eq!(r.status().as_u16(), 202);
    let admitted: Value = r.json().await.unwrap();
    let id = Uuid::parse_str(admitted["id"].as_str().unwrap()).unwrap();
    let again: Value = request(reqwest::Method::POST, path, 0)
        .json(&input)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(again["id"], admitted["id"]);
    let mut changed = input.clone();
    changed["firewallChange"]["confirmSeconds"] = json!(181);
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&changed)
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        409
    );
    let encrypted: Value = sqlx::query_scalar("SELECT input FROM mgmt_tasks WHERE id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!encrypted.to_string().contains("sourceDigest"));
    assert!(!encrypted.to_string().contains("wan0"));
    // Simulate a delivered write whose transport reply was lost. No agent or
    // system action runs: this exercises actual center state transitions.
    sqlx::query("UPDATE mgmt_tasks SET state='RUNNING',dispatch_epoch=$2,lease_until=now()-interval '1 second',expires_at=now()-interval '1 second' WHERE id=$1").bind(id).bind(epoch).execute(&pool).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    crate::task_lifecycle::expire_tasks(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
    let next = Uuid::new_v4();
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE mgmt_gateways SET connection_epoch=$1,last_seen=now() WHERE id='firewall'")
        .bind(next)
        .execute(&mut *tx)
        .await
        .unwrap();
    crate::task_lifecycle::on_connect(&mut tx, "firewall", 1)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let queries = crate::task_lifecycle::reconciliation_batch(&pool, "firewall", next, 1)
        .await
        .unwrap();
    assert_eq!(queries, vec![id.to_string()]);
    let row=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,input,idempotency_key,expires_at FROM mgmt_tasks WHERE id=$1").bind(id).fetch_one(&pool).await.unwrap();
    assert_eq!(row.get::<String, _>("state"), "UNKNOWN");
    let payload = cipher.open_row(&row, "input").unwrap();
    let command = crate::firewall_tasks::command(
        id.to_string(),
        31,
        row.get("idempotency_key"),
        row.get::<DateTime<Utc>, _>("expires_at").timestamp_millis(),
        &payload,
    )
    .unwrap();
    let mut value = json!({"id":action["transactionId"],"state":"WAIT_CONFIRM","deadlineUnixMs":123456,"revision":0,"requestId":id.to_string(),"newManagementConnectionRequired":true,"newManagementConnectionVerified":false,"confirmationToken":action["confirmationToken"],"candidate":draft["candidate"],"reconciliation":{"source":"firewall.remote.journal.v1","commandId":id.to_string(),"operation":31,"transactionId":action["transactionId"],"requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"journalSha256":"c".repeat(64),"observedAtUnixMs":123}});
    let result = |data: &Value| TaskResult {
        command_id: id.to_string(),
        state: TaskState::Succeeded as i32,
        error: ErrorCode::Unspecified as i32,
        result_json: serde_json::to_vec(data).unwrap(),
    };
    let mut bad = value.clone();
    bad["reconciliation"]["requestSha256"] = json!("d".repeat(64));
    assert!(
        crate::task_lifecycle::apply_result(&pool, &cipher, "firewall", "a", next, result(&bad))
            .await
            .is_err()
    );
    for _ in 0..2 {
        crate::task_lifecycle::apply_result(&pool, &cipher, "firewall", "a", next, result(&value))
            .await
            .unwrap();
    }
    let stored: Value = sqlx::query_scalar("SELECT result FROM mgmt_tasks WHERE id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!stored.to_string().contains("confirmationToken"));
    assert!(
        !stored
            .to_string()
            .contains(action["confirmationToken"].as_str().unwrap())
    );
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM mgmt_audit WHERE target=$1 AND operation='task.reconcile.firewall'",
    )
    .bind(id.to_string())
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 1);
    for (actor, visible) in [(0, true), (1, false), (2, false)] {
        let v: Value = request(reqwest::Method::GET, &format!("/api/v1/tasks/{id}"), actor)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(v["result"].get("confirmationToken").is_some(), visible);
    }
    let confirm: Value = request(reqwest::Method::POST, path, 0)
        .json(&make("firewall.confirm"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let confirm = Uuid::parse_str(confirm["id"].as_str().unwrap()).unwrap();
    let last = Uuid::new_v4();
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE mgmt_gateways SET connection_epoch=$1,last_seen=now() WHERE id='firewall'")
        .bind(last)
        .execute(&mut *tx)
        .await
        .unwrap();
    crate::task_lifecycle::on_connect(&mut tx, "firewall", 1)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
        .bind(confirm)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(state, "EXPIRED");
    sqlx::query("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN' WHERE id=$1")
        .bind(confirm)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&make("firewall.prepare"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        409
    );
    assert_eq!(
        request(reqwest::Method::POST, path, 0)
            .json(&make("firewall.rollback"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        202
    );
    let duplicates: i64 = sqlx::query_scalar("SELECT count(*) FROM mgmt_tasks WHERE operation=31")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(duplicates, 1);
    value["confirmationToken"] = Value::Null; // no secrets in the retained test summary
    println!(
        "retained schema={schema}, key directory={}, HTTP stopped; no system action",
        directory.display()
    );
    drop(server);
    pool.close().await;
    admin.close().await;
}
