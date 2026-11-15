use super::*;
use prost::Message;
use sha2::{Digest, Sha256};
use yofc_management_protocol::v1::SystemControlAction;

fn plan(operation: i32) -> Value {
    json!({"id":"system-11111111-1111-4111-8111-111111111111","state":"PREPARED",
        "action":if operation==24{"service.restart"}else{"system.reboot"},"unit":"kura.service",
        "issued":false,"requestAccepted":false,
        "before":{"unit":"kura.service","activeState":"active","pid":123,"invocationId":"a".repeat(32),"controlGroup":"/system.slice/kura.service","unitFileState":"enabled"},
        "beforeBootId":"22222222-2222-4222-8222-222222222222","after":null,"afterBootId":null,
        "expiresAtUnixMs":1234,"applicationVerificationConfigured":false,"scope":"system-process-only",
        "kuraRecoveryVerified":false,"normalBootVerified":false,
        "confirmToken":"33333333-3333-4333-8333-333333333333","grantSha256":"b".repeat(64),"unitDefinitionSha256":"c".repeat(64)})
}
fn completed(operation: i32, application: bool) -> Value {
    let mut v = plan(if operation == 25 { 24 } else { 26 });
    v["state"] = json!("COMPLETED");
    v["issued"] = json!(true);
    v["requestAccepted"] = json!(true);
    v["stage"] = json!(if operation == 25 {
        "SERVICE_RESTART_OBSERVED"
    } else {
        "HOST_REBOOT_OBSERVED"
    });
    v["after"] = v["before"].clone();
    v["after"]["pid"] = json!(234);
    v["after"]["invocationId"] = json!("d".repeat(32));
    v["afterBootId"] = if operation == 25 {
        v["beforeBootId"].clone()
    } else {
        json!("44444444-4444-4444-8444-444444444444")
    };
    v["processRestartVerified"] = json!(true);
    v["hostRebootVerified"] = json!(operation == 27);
    v["applicationVerificationConfigured"] = json!(application);
    v["applicationVerified"] = json!(application);
    v["scope"] = json!(if application {
        "administrator-selected-checks"
    } else {
        "system-process-only"
    });
    v["application"] =
        json!({"configured":application,"verified":application,"error":null,"report":null});
    if application {
        v["profileId"] = json!("profile-55555555-5555-4555-8555-555555555555");
        v["profileDigest"] = json!("e".repeat(64));
        v["approvedSnapshotSha256"] = json!("f".repeat(64));
        v["componentPids"] = json!(["test.pid"]);
        v["checkIds"] = json!(["test.business"]);
        v["application"]["report"] = json!({"schema":2,"handoffId":"handoff-66666666-6666-4666-8666-666666666666",
            "profileId":v["profileId"],"nonce":"a".repeat(64),"approvedSnapshotSha256":v["approvedSnapshotSha256"],
            "observerInstanceId":"77777777-7777-4777-8777-777777777777","processId":234,"frameworkStartLevel":5,"ordinaryLoad":true,"ready":true,
            "components":[{"pid":"test.pid","registered":true}],"providers":[{"id":"test.business","available":true,"unique":true}],"checks":[{"id":"test.business","passed":true}]});
    }
    v
}

#[test]
fn system_plan_cancel_and_status_never_certify_a_transition() {
    for op in [24, 26] {
        let mut v = plan(op);
        assert!(system_result(&v, op, TaskState::Succeeded));
        assert!(!system_result(
            &v,
            if op == 24 { 25 } else { 27 },
            TaskState::Succeeded
        ));
        v["issued"] = json!(true);
        assert!(!system_result(&v, op, TaskState::Succeeded));
    }
    let mut v = plan(24);
    v["state"] = json!("CANCELLED");
    assert!(system_result(&v, 28, TaskState::Succeeded));
    v["requestAccepted"] = json!(true);
    assert!(!system_result(&v, 28, TaskState::Succeeded));
    let status = json!({"configured":true,"configurationState":"RECOVERY_REQUIRED","kuraRecoveryVerified":false,"normalBootVerified":false});
    assert!(system_result(&status, 23, TaskState::Succeeded));
}

#[test]
fn reboot_acceptance_is_unknown_until_actual_new_boot_and_mapped_process() {
    let mut v = plan(26);
    v["state"] = json!("AWAITING_BOOT");
    v["issued"] = json!(true);
    v["requestAccepted"] = json!(true);
    assert!(system_result(&v, 27, TaskState::Unknown));
    assert!(!system_result(&v, 27, TaskState::Succeeded));
    let mut v = completed(27, false);
    assert!(system_result(&v, 27, TaskState::Succeeded));
    v["afterBootId"] = v["beforeBootId"].clone();
    assert!(!system_result(&v, 27, TaskState::Succeeded));
    let mut v = completed(27, false);
    v["after"]["pid"] = json!(0);
    v["after"]["activeState"] = json!("failed");
    v["processRestartVerified"] = json!(false);
    assert!(system_result(&v, 27, TaskState::Failed));
    assert!(!system_result(&v, 27, TaskState::Succeeded));
}

#[test]
fn system_completion_rejects_wrong_process_boot_scope_and_selected_provider_identity() {
    for op in [25, 27] {
        for application in [false, true] {
            let v = completed(op, application);
            assert!(system_result(&v, op, TaskState::Succeeded));
            for field in ["kuraRecoveryVerified", "normalBootVerified"] {
                let mut wrong = v.clone();
                wrong[field] = json!(true);
                assert!(!system_result(&wrong, op, TaskState::Succeeded));
            }
            let mut wrong = v.clone();
            wrong["after"]["controlGroup"] = json!("/user.slice/foreign.service");
            assert!(!system_result(&wrong, op, TaskState::Succeeded));
            let mut wrong = v.clone();
            wrong["after"]["invocationId"] = wrong["before"]["invocationId"].clone();
            assert!(!system_result(&wrong, op, TaskState::Succeeded));
            if application {
                let mut fully_started = v.clone();
                fully_started["application"]["report"]["frameworkStartLevel"] = json!(6);
                assert!(system_result(&fully_started, op, TaskState::Succeeded));
                for (field, value) in [
                    ("processId", json!(123)),
                    ("profileId", json!("profile-other")),
                    ("approvedSnapshotSha256", json!("b".repeat(64))),
                    ("frameworkStartLevel", json!(4)),
                    ("frameworkStartLevel", json!("6")),
                    ("frameworkStartLevel", json!(6.5)),
                    ("frameworkStartLevel", json!(u64::MAX)),
                ] {
                    let mut wrong = v.clone();
                    wrong["application"]["report"][field] = value;
                    assert!(!system_result(&wrong, op, TaskState::Succeeded), "{field}");
                }
                for field in ["providers", "checks"] {
                    let mut wrong = v.clone();
                    wrong["application"]["report"][field][0]["id"] = json!("unreviewed.check");
                    assert!(!system_result(&wrong, op, TaskState::Succeeded));
                }
                let mut failed = v.clone();
                failed["applicationVerified"] = json!(false);
                failed["application"]["verified"] = json!(false);
                assert!(system_result(&failed, op, TaskState::Failed));
                assert!(!system_result(&failed, op, TaskState::Succeeded));
            }
        }
    }
}

#[test]
fn system_reconciliation_requires_complete_original_wire_and_native_stage_proof() {
    let mut value = completed(25, true);
    let command = ReadCommand {
        command_id: "88888888-8888-4888-8888-888888888888".into(),
        operation: 25,
        idempotency_key: "original-system-command".into(),
        expires_at_unix_ms: 12345,
        system_control_action: Some(SystemControlAction {
            transaction_id: value["id"].as_str().unwrap().into(),
            confirmation_token: value["confirmToken"].as_str().unwrap().into(),
        }),
        ..Default::default()
    };
    value["reconciliation"] = json!({"source":"system.control.journal.v1","commandId":command.command_id,"operation":command.operation,
        "requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"transactionId":value["id"],"journalSha256":"a".repeat(64),"decisionSha256":"b".repeat(64)});
    assert!(system_proof(&value, &command, TaskState::Succeeded));
    for changed in 0..6 {
        let mut wrong = command.clone();
        match changed {
            0 => wrong.command_id = Uuid::new_v4().to_string(),
            1 => wrong.operation = 27,
            2 => wrong.expires_at_unix_ms += 1,
            3 => wrong.idempotency_key.push('x'),
            4 => {
                wrong
                    .system_control_action
                    .as_mut()
                    .unwrap()
                    .confirmation_token = Uuid::new_v4().to_string()
            }
            _ => {
                wrong.system_control_action.as_mut().unwrap().transaction_id =
                    format!("system-{}", Uuid::new_v4())
            }
        }
        assert!(!system_proof(&value, &wrong, TaskState::Succeeded));
    }
    assert!(!system_proof(
        &json!({"code":"OUTCOME_UNKNOWN"}),
        &command,
        TaskState::Failed
    ));
    value["application"]["report"]["providers"][0]["unique"] = json!(false);
    assert!(!system_proof(&value, &command, TaskState::Succeeded));
}

async fn isolated_database() -> (PgPool, PgPool, String, TaskCipher, Uuid) {
    let url = std::env::var("DATABASE_URL").unwrap();
    let admin = PgPool::connect(&url).await.unwrap();
    let schema = format!("system_control_{}", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let options = url
        .parse::<sqlx::postgres::PgConnectOptions>()
        .unwrap()
        .options([("search_path", schema.as_str())]);
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&db).await.unwrap();
    let actor = Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'system-test','operator','unused','admin')").bind(actor).execute(&db).await.unwrap();
    let dir = tempfile::tempdir().unwrap().keep();
    std::fs::set_permissions(&dir, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
    let key = dir.join("task-data.key");
    crate::task_crypto::create_key(&key).unwrap();
    let cipher = TaskCipher::load(&key).unwrap();
    println!("retained schema {schema}; task key {}", key.display());
    (admin, db, schema, cipher, actor)
}

#[tokio::test]
#[ignore = "requires an explicit isolated PostgreSQL database"]
async fn postgres_system_unleased_queue_expires_on_connection_change_but_issued_work_only_reconciles()
 {
    let (admin, db, _, _, actor) = isolated_database().await;
    let old = Uuid::new_v4();
    let current = Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('system','system-test','system','unused',$1,1)").bind(current).execute(&db).await.unwrap();
    let mut stale = Vec::new();
    for op in 24..=29 {
        let id = Uuid::new_v4();
        stale.push(id);
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,dispatch_epoch) VALUES($1,'system','system-test',$2,$3,$4,'QUEUED',1,now()+interval '5 minutes',$5)").bind(id).bind(actor).bind(op).bind(id.to_string()).bind(old).execute(&db).await.unwrap();
    }
    let fresh = Uuid::new_v4();
    let issued = Uuid::new_v4();
    let read = Uuid::new_v4();
    for (id, op, state, epoch) in [
        (fresh, 29, "QUEUED", current),
        (issued, 29, "RUNNING", old),
        (read, 23, "RUNNING", old),
    ] {
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,lease_until,dispatch_epoch) VALUES($1,'system','system-test',$2,$3,$4,$5,1,now()+interval '5 minutes',now()+interval '5 minutes',$6)").bind(id).bind(actor).bind(op).bind(id.to_string()).bind(state).bind(epoch).execute(&db).await.unwrap();
    }
    let mut tx = db.begin().await.unwrap();
    on_connect(&mut tx, "system", 1).await.unwrap();
    tx.commit().await.unwrap();
    for id in stale {
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(id)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "EXPIRED");
    }
    let states: Vec<(Uuid, String)> =
        sqlx::query_as("SELECT id,state FROM mgmt_tasks WHERE id IN ($1,$2,$3)")
            .bind(fresh)
            .bind(issued)
            .bind(read)
            .fetch_all(&db)
            .await
            .unwrap();
    assert!(states.contains(&(fresh, "QUEUED".into())));
    assert!(states.contains(&(issued, "UNKNOWN".into())));
    assert!(states.contains(&(read, "RUNNING".into())));
    let ids = reconciliation_batch(&db, "system", current, 1)
        .await
        .unwrap();
    assert_eq!(ids, vec![issued.to_string()]);
    let epoch: Uuid = sqlx::query_scalar("SELECT dispatch_epoch FROM mgmt_tasks WHERE id=$1")
        .bind(issued)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(epoch, current);
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM mgmt_audit WHERE operation='task.system.connection.expire'",
    )
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(count, 6);
    db.close().await;
    admin.close().await;
}

#[tokio::test]
#[ignore = "requires an explicit isolated PostgreSQL database"]
async fn postgres_system_expired_original_receipt_is_encrypted_bound_and_audited_once() {
    let (admin, db, _, cipher, actor) = isolated_database().await;
    let epoch = Uuid::new_v4();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('system','system-test','system','unused',$1,1)").bind(epoch).execute(&db).await.unwrap();
    for (op, application) in [(25, false), (27, true)] {
        let id = Uuid::new_v4();
        let mut value = completed(op, application);
        if application {
            value["applicationVerified"] = json!(false);
            value["application"]["verified"] = json!(false);
            value["application"]["error"] = json!("APPLICATION_CHECK_FAILED");
        }
        let expected = if application {
            TaskState::Failed
        } else {
            TaskState::Succeeded
        };
        let context = TaskContext {
            id,
            tenant: "system-test".into(),
            gateway: "system".into(),
            operation: op,
        };
        let action = SystemControlAction {
            transaction_id: value["id"].as_str().unwrap().into(),
            confirmation_token: Uuid::new_v4().to_string(),
        };
        let input = cipher
            .seal(&context, "input", &json!({"systemControlAction":action}))
            .unwrap();
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,lease_until,dispatch_epoch,input) VALUES($1,'system','system-test',$2,$3,$4,'RUNNING',1,now()-interval '30 seconds',now()-interval '20 seconds',$5,$6)").bind(id).bind(actor).bind(op).bind(id.to_string()).bind(epoch).bind(input).execute(&db).await.unwrap();
        let expires: chrono::DateTime<chrono::Utc> =
            sqlx::query_scalar("SELECT expires_at FROM mgmt_tasks WHERE id=$1")
                .bind(id)
                .fetch_one(&db)
                .await
                .unwrap();
        let command = ReadCommand {
            command_id: id.to_string(),
            operation: op,
            idempotency_key: id.to_string(),
            expires_at_unix_ms: expires.timestamp_millis(),
            system_control_action: Some(action),
            ..Default::default()
        };
        value["reconciliation"] = json!({"source":"system.control.journal.v1","commandId":id.to_string(),"operation":op,"requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"transactionId":value["id"],"journalSha256":"a".repeat(64),"decisionSha256":"b".repeat(64)});
        let mut tx = db.begin().await.unwrap();
        expire_tasks(&mut tx).await.unwrap();
        tx.commit().await.unwrap();
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(id)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "UNKNOWN");
        let make = |v: &Value| TaskResult {
            command_id: id.to_string(),
            state: expected as i32,
            error: if application {
                ErrorCode::KuraUnavailable as i32
            } else {
                0
            },
            result_json: serde_json::to_vec(v).unwrap(),
        };
        for field in [
            "source",
            "requestSha256",
            "decisionSha256",
            "journalSha256",
            "commandId",
            "operation",
            "transactionId",
        ] {
            let mut bad = value.clone();
            bad["reconciliation"][field] = json!("wrong");
            assert!(
                apply_result(&db, &cipher, "system", "system-test", epoch, make(&bad))
                    .await
                    .is_err(),
                "{field}"
            );
        }
        for _ in 0..2 {
            apply_result(&db, &cipher, "system", "system-test", epoch, make(&value))
                .await
                .unwrap();
        }
        let row=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,input,result FROM mgmt_tasks WHERE id=$1").bind(id).fetch_one(&db).await.unwrap();
        assert_eq!(row.get::<String, _>("state"), expected.as_str_name());
        assert_eq!(cipher.open_row(&row, "result").unwrap(), value);
        assert!(
            !row.get::<Value, _>("result")
                .to_string()
                .contains("SERVICE_RESTART_OBSERVED")
        );
        assert!(
            !row.get::<Value, _>("input")
                .to_string()
                .contains("confirmationToken")
        );
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE target=$1 AND operation='task.reconcile.system_control'").bind(id.to_string()).fetch_one(&db).await.unwrap();
        assert_eq!(count, 1);
    }
    db.close().await;
    admin.close().await;
}
