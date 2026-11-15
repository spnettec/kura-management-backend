//! Actual loopback HTTP and isolated PostgreSQL; no native system actions.
use super::*;

#[tokio::test]
#[ignore = "requires an explicit isolated PostgreSQL database"]
async fn postgres_system_http_authority_caps_and_original_intent_are_enforced() {
    let url = std::env::var("DATABASE_URL").unwrap();
    let admin = PgPool::connect(&url).await.unwrap();
    let schema = format!("system_http_{}", Uuid::new_v4().simple());
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
    let key = directory.join("key");
    crate::task_crypto::create_key(&key).unwrap();
    let cipher = TaskCipher::load(&key).unwrap();
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
    let server = tokio::spawn(async move {
        axum::serve(
            listener,
            router(app).into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .unwrap();
    });
    struct Stop(tokio::task::JoinHandle<()>);
    impl Drop for Stop {
        fn drop(&mut self) {
            self.0.abort();
        }
    }
    let server = Stop(server);
    let mut actors = Vec::new();
    for (n, (role, tenant, initial)) in [
        ("admin", "a", false),
        ("operator", "a", false),
        ("viewer", "a", false),
        ("recovery", "a", false),
        ("admin", "b", false),
        ("admin", "a", true),
        ("admin", "a", false),
    ]
    .into_iter()
    .enumerate()
    {
        let id = Uuid::new_v4();
        let token = auth::random_token();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,$2,$3,'unused',$4,$5)").bind(id).bind(tenant).bind(format!("system-{n}")).bind(role).bind(initial).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(id).execute(&pool).await.unwrap();
        actors.push((id, token));
    }
    let epoch = Uuid::new_v4();
    let caps = json!([
        "system.control.v1",
        "service.restart.v1",
        "system.reboot.v1",
        "recovery.ordinary.v1",
        "recovery.ordinary.cancel.v1"
    ]);
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation,online_reported,last_seen,capabilities) VALUES('power','a','power','unused',$1,1,true,now(),$2)").bind(epoch).bind(&caps).execute(&pool).await.unwrap();
    let client = reqwest::Client::new();
    let action = json!({"transactionId":format!("system-{}",Uuid::new_v4()),"confirmationToken":Uuid::new_v4().to_string()});
    let handoff = json!({"transactionId":format!("handoff-{}",Uuid::new_v4()),"confirmationToken":Uuid::new_v4().to_string()});
    let make = |operation: &str| {
        let mut v = json!({"operation":operation,"idempotencyKey":Uuid::new_v4().to_string()});
        if ["service.restart", "system.reboot", "system.control.cancel"].contains(&operation) {
            v["systemControlAction"] = action.clone();
        }
        if operation == "recovery.ordinary.cancel" {
            v["ordinaryHandoffAction"] = handoff.clone();
        }
        v
    };
    let request = |method: reqwest::Method, path: &str, index: usize| {
        client
            .request(method, format!("{base}{path}"))
            .header("cookie", format!("kura_session={}", actors[index].1))
            .header("origin", "https://fixture.invalid")
            .header("x-csrf-token", "csrf")
    };
    let operations = [
        "service.status",
        "service.restart.prepare",
        "service.restart",
        "system.reboot.prepare",
        "system.reboot",
        "system.control.cancel",
        "recovery.ordinary.cancel",
    ];
    // Every operation is administrator-only, including the status used to inspect
    // private plan identity. Tenant and initial-password gates precede admission.
    for operation in operations {
        for index in [1, 2, 3, 4, 5] {
            let response = request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", index)
                .json(&make(operation))
                .send()
                .await
                .unwrap();
            assert_eq!(
                response.status().as_u16(),
                if index == 4 { 404 } else { 403 },
                "{operation}, actor {index}"
            );
        }
    }
    for (cookie, origin, csrf, expected) in [
        (false, true, true, 401),
        (true, false, true, 403),
        (true, true, false, 403),
    ] {
        let mut r = client.post(format!("{base}/api/v1/gateways/power/tasks"));
        if cookie {
            r = r.header("cookie", format!("kura_session={}", actors[0].1));
        }
        if origin {
            r = r.header("origin", "https://fixture.invalid");
        }
        if csrf {
            r = r.header("x-csrf-token", "csrf");
        }
        assert_eq!(
            r.json(&make("service.restart.prepare"))
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            expected
        );
    }
    for bad in [
        json!({"operation":"service.restart.prepare","idempotencyKey":"bad-1","unit":"ssh.service"}),
        json!({"operation":"service.restart","idempotencyKey":"bad-2"}),
        json!({"operation":"service.status","idempotencyKey":"bad-3","systemControlAction":action}),
        json!({"operation":"system.reboot","idempotencyKey":"bad-4","systemControlAction":{"transactionId":"kura.service","confirmationToken":"invalid"}}),
        json!({"operation":"system.reboot","idempotencyKey":"bad-5","systemControlAction":{"transactionId":action["transactionId"],"confirmationToken":action["confirmationToken"],"command":"reboot"}}),
    ] {
        assert!(matches!(
            request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
                .json(&bad)
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            400 | 422
        ));
    }
    // Fresh, live capability and the distinct installation permissions are
    // necessary even when the authenticated center user is an administrator.
    for granted in [json!([]), json!(["system.control.v1"])] {
        sqlx::query("UPDATE mgmt_gateways SET capabilities=$1,last_seen=now() WHERE id='power'")
            .bind(granted)
            .execute(&pool)
            .await
            .unwrap();
        for operation in ["service.restart.prepare", "system.reboot.prepare"] {
            assert_eq!(
                request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
                    .json(&make(operation))
                    .send()
                    .await
                    .unwrap()
                    .status()
                    .as_u16(),
                422
            );
        }
    }
    for granted in [
        json!(["recovery.ordinary.v1"]),
        json!(["recovery.ordinary.cancel.v1"]),
    ] {
        sqlx::query("UPDATE mgmt_gateways SET capabilities=$1,last_seen=now() WHERE id='power'")
            .bind(granted)
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(
            request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
                .json(&make("recovery.ordinary.cancel"))
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            422
        );
    }
    for (online, valid_epoch, seen) in [
        (false, true, "0 seconds"),
        (true, false, "0 seconds"),
        (true, true, "60 seconds"),
    ] {
        sqlx::query("UPDATE mgmt_gateways SET capabilities=$1,online_reported=$2,connection_epoch=$3,last_seen=now()-$4::interval WHERE id='power'").bind(&caps).bind(online).bind(if valid_epoch{Some(epoch)}else{None}).bind(seen).execute(&pool).await.unwrap();
        let status = request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
            .json(&make("recovery.ordinary.cancel"))
            .send()
            .await
            .unwrap()
            .status()
            .as_u16();
        assert!(matches!(status, 409 | 422));
    }
    sqlx::query("UPDATE mgmt_gateways SET capabilities=$1,online_reported=true,connection_epoch=$2,last_seen=now() WHERE id='power'").bind(&caps).bind(epoch).execute(&pool).await.unwrap();
    let mut accepted = Vec::new();
    for (n, operation) in operations.iter().enumerate() {
        let body = make(operation);
        let response = request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), 202);
        let value: Value = response.json().await.unwrap();
        let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();
        let row=sqlx::query("SELECT id,tenant_id,gateway_id,operation,input,dispatch_epoch,extract(epoch from (expires_at-created_at))::double precision AS ttl FROM mgmt_tasks WHERE id=$1").bind(id).fetch_one(&pool).await.unwrap();
        assert_eq!(row.get::<i32, _>("operation"), n as i32 + 23);
        assert_eq!(
            row.get::<Option<Uuid>, _>("dispatch_epoch"),
            if n == 0 { None } else { Some(epoch) }
        );
        assert!(
            (row.get::<f64, _>("ttl") - if [2, 4].contains(&n) { 300.0 } else { 30.0 }).abs() < 1.0
        );
        let plain = cipher.open_row(&row, "input").unwrap();
        if [2, 4, 5].contains(&n) {
            assert_eq!(plain["systemControlAction"], action);
        }
        assert!(
            !row.get::<Value, _>("input")
                .to_string()
                .contains("confirmationToken")
        );
        if n == 6 {
            assert_eq!(plain["ordinaryHandoffAction"], handoff);
        }
        accepted.push((id, body));
    }
    // Repeat only the same original command after the capability is gone. This
    // returns its ID; it neither admits another task nor changes its deadline.
    sqlx::query(
        "UPDATE mgmt_gateways SET capabilities='[]',online_reported=false WHERE id='power'",
    )
    .execute(&pool)
    .await
    .unwrap();
    let (cancel_id, cancel_body) = &accepted[6];
    let original: Value = request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
        .json(cancel_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(original["id"], cancel_id.to_string());
    let (id, body) = &accepted[1];
    let repeat: Value = request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
        .json(body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(repeat["id"], id.to_string());
    let path = format!(
        "/api/v1/gateways/power/task-intents/{}",
        body["idempotencyKey"].as_str().unwrap()
    );
    assert_eq!(
        request(reqwest::Method::GET, &path, 0)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap()["id"],
        id.to_string()
    );
    for index in [1, 4, 6] {
        assert!(matches!(
            request(reqwest::Method::GET, &path, index)
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            403 | 404
        ));
    }
    let mut changed = body.clone();
    changed["operation"] = json!("system.reboot.prepare");
    assert_eq!(
        request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
            .json(&changed)
            .send()
            .await
            .unwrap()
            .status()
            .as_u16(),
        409
    );
    let result = json!({"state":"PREPARED","confirmToken":action["confirmationToken"],"id":action["transactionId"]});
    let context = TaskContext {
        id: *id,
        tenant: "a".into(),
        gateway: "power".into(),
        operation: 24,
    };
    let result = cipher.seal(&context, "result", &result).unwrap();
    sqlx::query("UPDATE mgmt_tasks SET state='SUCCEEDED',result=$2 WHERE id=$1")
        .bind(id)
        .bind(result)
        .execute(&pool)
        .await
        .unwrap();
    let detail = format!("/api/v1/tasks/{id}");
    let admin_result: Value = request(reqwest::Method::GET, &detail, 0)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        admin_result["result"]["confirmToken"],
        action["confirmationToken"]
    );
    let operator_result: Value = request(reqwest::Method::GET, &detail, 1)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(operator_result["result"]["confirmToken"].is_null());
    assert!(
        !operator_result
            .to_string()
            .contains(action["confirmationToken"].as_str().unwrap())
    );
    sqlx::query("UPDATE mgmt_gateways SET capabilities=$1,online_reported=true,last_seen=now() WHERE id='power'").bind(&caps).execute(&pool).await.unwrap();
    sqlx::query("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN' WHERE id=$1")
        .bind(accepted[2].0)
        .execute(&pool)
        .await
        .unwrap();
    for operation in [
        "service.restart.prepare",
        "service.restart",
        "system.reboot.prepare",
        "system.reboot",
        "recovery.ordinary.cancel",
    ] {
        let r = request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
            .json(&make(operation))
            .send()
            .await
            .unwrap();
        assert_eq!(r.status().as_u16(), 409);
        assert_eq!(
            r.json::<Value>().await.unwrap()["error"]["code"],
            "OUTCOME_UNKNOWN"
        );
    }
    for operation in ["service.status", "system.control.cancel"] {
        assert_eq!(
            request(reqwest::Method::POST, "/api/v1/gateways/power/tasks", 0)
                .json(&make(operation))
                .send()
                .await
                .unwrap()
                .status()
                .as_u16(),
            202
        );
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM mgmt_tasks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 9);
    println!(
        "actual HTTP: seven operations including handoff cancellation; role/tenant/password/CSRF/payload/capability/epoch/TTL/encryption/redaction/intent/UNKNOWN gates; retained schema {schema}; key {}",
        key.display()
    );
    drop(server);
    pool.close().await;
    admin.close().await;
}
