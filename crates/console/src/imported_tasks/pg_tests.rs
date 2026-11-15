//! Explicit private-PostgreSQL integration; retained schemas and encrypted keys.
use super::*;
use crate::task_crypto::{TaskCipher, TaskContext};
use chrono::{DateTime, Duration, Utc};
use serde_json::json;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::os::unix::fs::PermissionsExt;

struct Fixture {
    pool: PgPool,
    cipher: TaskCipher,
    actor: Uuid,
    other: Uuid,
    epoch: Uuid,
}
async fn fixture() -> Fixture {
    let url = std::env::var("YOFC_IMPORTED_PG_URL").expect("explicit private PG required");
    assert!(url.contains("@127.0.0.1:") && url.ends_with("/imported_v51"));
    let admin = PgPool::connect(&url).await.unwrap();
    let schema = format!("imported_v51_{}", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let options = url
        .parse::<PgConnectOptions>()
        .unwrap()
        .options([("search_path", schema.as_str())]);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    let directory = tempfile::tempdir().unwrap().keep();
    std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700)).unwrap();
    crate::task_crypto::create_key(&directory.join("task.key")).unwrap();
    let cipher = TaskCipher::load(&directory.join("task.key")).unwrap();
    cipher.prepare_storage(&pool).await.unwrap();
    let actor = Uuid::new_v4();
    let other = Uuid::new_v4();
    let epoch = Uuid::new_v4();
    for (user, name) in [(actor, "original-owner"), (other, "other-owner")] {
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'imported',$2,'unused','admin')")
            .bind(user).bind(name).execute(&pool).await.unwrap();
    }
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('imported','imported','private test','unused',$1,1)")
        .bind(epoch).execute(&pool).await.unwrap();
    if let Ok(evidence) = std::env::var("YOFC_IMPORTED_EVIDENCE_DIR") {
        std::fs::create_dir_all(&evidence).unwrap();
        std::fs::write(std::path::Path::new(&evidence).join(format!("{schema}.json")),serde_json::to_vec_pretty(&json!({"schema":schema,"keyDirectory":directory,"actor":actor,"other":other,"epoch":epoch})).unwrap()).unwrap();
    }
    admin.close().await;
    Fixture {
        pool,
        cipher,
        actor,
        other,
        epoch,
    }
}
impl Fixture {
    async fn task(
        &self,
        request: Request,
        state: &str,
        actor: Uuid,
        generation: i64,
        expired: bool,
    ) -> ReadCommand {
        let id = Uuid::new_v4();
        let operation = request.operation();
        let expires = Utc::now() + Duration::seconds(if expired { -60 } else { 300 });
        let mut payload = json!({"importedFirewall":{"request":request}});
        materialize(&mut payload, None).unwrap();
        let encrypted = self
            .cipher
            .seal(
                &TaskContext {
                    id,
                    tenant: "imported".into(),
                    gateway: "imported".into(),
                    operation,
                },
                "input",
                &payload,
            )
            .unwrap();
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,dispatch_epoch,input,lease_until) VALUES($1,'imported','imported',$2,$3,$4,$5,$6,$7,$8,$9,now()-interval '1 second')")
            .bind(id).bind(actor).bind(operation).bind(id.to_string()).bind(state).bind(generation).bind(expires).bind(self.epoch).bind(encrypted)
            .execute(&self.pool).await.unwrap();
        command(
            id.to_string(),
            operation,
            id.to_string(),
            expires.timestamp_millis(),
            &payload,
        )
        .unwrap()
    }
    async fn state(&self, id: &str) -> String {
        sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(Uuid::parse_str(id).unwrap())
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
    async fn audits(&self, id: &str) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE target=$1 AND operation='task.reconcile.imported'").bind(id).fetch_one(&self.pool).await.unwrap()
    }
    async fn result(&self, query: &ReadCommand, data: &Value) -> Result<(), tonic::Status> {
        crate::task_lifecycle::apply_result(
            &self.pool,
            &self.cipher,
            "imported",
            "imported",
            self.epoch,
            TaskResult {
                command_id: query.command_id.clone(),
                state: TaskState::Succeeded as i32,
                error: 0,
                result_json: serde_json::to_vec(data).unwrap(),
            },
        )
        .await
    }
}
fn proposal(id: Uuid) -> Request {
    Request::Prepare {
        proposal: yofc_management_protocol::imported::Proposal {
            operation_id: id,
            expected_revision: 7,
            source_digest: "a".repeat(64),
            cohort_digest: "b".repeat(64),
            confirm_seconds: 120,
            candidate: json!({}),
            destination_prefixes: Default::default(),
        },
    }
}
fn query_receipt(
    query: &ReadCommand,
    original: &ReadCommand,
    prepared: &ReadCommand,
    id: Uuid,
    ack: bool,
) -> Value {
    json!({"schemaVersion":1,"operation":query.operation,"originalTaskId":original.command_id,
        "originalCommandSha256":format!("{:x}",Sha256::digest(original.encode_to_vec())),"writeEnabled":false,"confirmationReady":false,"newManagementConnectionVerified":false,
        "receipt":{"id":id,"requestId":"remote-original-fixture","state":"WAIT_CONFIRM","deadlineUnixMs":Utc::now().timestamp_millis()+120000,
            "deadlineExpired":false,"ownerGrant":false},
        "decisionEvidence":DecisionEvidence {record_sha256:"c".repeat(64),historical:false,source_observation:false,
            forward_intent:true,forward_acknowledged:ack,inverse_intent:false,inverse_acknowledged:false,
            negative_acknowledged:false,confirmed_decision:false,commands:Some(yofc_management_protocol::imported::OriginalCommands {
                original_sha256:format!("{:x}",Sha256::digest(prepared.encode_to_vec())),
                forward_sha256:Some(format!("{:x}",Sha256::digest(original.encode_to_vec()))),inverse_sha256:None,
                confirmation_sha256:None,cancellation_sha256:None,expiry_decision:false})}})
}

#[tokio::test]
#[ignore = "requires explicit private YOFC_IMPORTED_PG_URL"]
async fn postgres_imported_query_resolves_only_exact_owned_original_with_ack_and_one_audit() {
    let f = fixture().await;
    let id = Uuid::new_v4();
    let prepare = f.task(proposal(id), "SUCCEEDED", f.actor, 1, true).await;
    let apply = f
        .task(
            Request::Apply {
                original_task_id: Uuid::parse_str(&prepare.command_id).unwrap(),
            },
            "UNKNOWN",
            f.actor,
            1,
            true,
        )
        .await;
    let target = Uuid::parse_str(&apply.command_id).unwrap();
    let query = f
        .task(
            Request::Outcome {
                original_task_id: target,
            },
            "RUNNING",
            f.actor,
            1,
            false,
        )
        .await;
    let data = query_receipt(&query, &apply, &prepare, id, false);
    f.result(&query, &data).await.unwrap();
    assert_eq!(f.state(&apply.command_id).await, "UNKNOWN");
    assert_eq!(f.audits(&apply.command_id).await, 0);
    // A valid gateway cannot swap hashes, proposal identities, actors or generations.
    for variant in ["hash", "proposal", "actor", "generation", "parentHash"] {
        let query = f
            .task(
                Request::Outcome {
                    original_task_id: target,
                },
                "RUNNING",
                if variant == "actor" { f.other } else { f.actor },
                if variant == "generation" { 2 } else { 1 },
                false,
            )
            .await;
        let mut data = query_receipt(&query, &apply, &prepare, id, true);
        if variant == "hash" {
            data["originalCommandSha256"] = json!("d".repeat(64));
        }
        if variant == "parentHash" {
            data["decisionEvidence"]["commands"]["originalSha256"] = json!("d".repeat(64));
        }
        if variant == "proposal" {
            data["receipt"]["id"] = json!(Uuid::new_v4());
        }
        assert!(f.result(&query, &data).await.is_err(), "{variant}");
        assert_eq!(f.state(&apply.command_id).await, "UNKNOWN");
        assert_eq!(f.audits(&apply.command_id).await, 0);
    }
    let query = f
        .task(
            Request::Outcome {
                original_task_id: target,
            },
            "RUNNING",
            f.actor,
            1,
            false,
        )
        .await;
    let data = query_receipt(&query, &apply, &prepare, id, true);
    f.result(&query, &data).await.unwrap();
    assert_eq!(f.state(&apply.command_id).await, "SUCCEEDED");
    assert_eq!(f.audits(&apply.command_id).await, 1);
    f.result(&query, &data).await.unwrap();
    assert_eq!(f.audits(&apply.command_id).await, 1);
    let row =
        sqlx::query("SELECT id,tenant_id,gateway_id,operation,result FROM mgmt_tasks WHERE id=$1")
            .bind(target)
            .fetch_one(&f.pool)
            .await
            .unwrap();
    let stored: Value = row.get("result");
    assert!(!stored.to_string().contains("recordSha256"));
    let opened = f.cipher.open_row(&row, "result").unwrap();
    assert_eq!(
        opened["reconciliation"]["source"],
        "firewall.imported.original.v1"
    );
    assert_eq!(opened["reconciliation"]["queryTaskId"], query.command_id);
    assert_eq!(opened["originalTaskId"], prepare.command_id);
    f.pool.close().await;
}

#[tokio::test]
#[ignore = "requires explicit private YOFC_IMPORTED_PG_URL"]
async fn postgres_confirmation_needs_its_exact_command_connection_proof_and_does_not_inherit_another_confirmation()
 {
    let f = fixture().await;
    let id = Uuid::new_v4();
    let prepare = f.task(proposal(id), "SUCCEEDED", f.actor, 1, true).await;
    let parent = Uuid::parse_str(&prepare.command_id).unwrap();
    let apply = f
        .task(
            Request::Apply {
                original_task_id: parent,
            },
            "SUCCEEDED",
            f.actor,
            1,
            true,
        )
        .await;
    let confirm = f
        .task(
            Request::Confirm {
                original_task_id: parent,
            },
            "UNKNOWN",
            f.actor,
            1,
            true,
        )
        .await;
    for verified in [false, true] {
        let query = f
            .task(
                Request::Outcome {
                    original_task_id: Uuid::parse_str(&confirm.command_id).unwrap(),
                },
                "RUNNING",
                f.actor,
                1,
                false,
            )
            .await;
        let mut data = query_receipt(&query, &confirm, &prepare, id, true);
        data["receipt"]["state"] = json!("COMMITTED");
        data["receipt"]["historical"] = json!(true);
        data["decisionEvidence"]["historical"] = json!(true);
        data["decisionEvidence"]["confirmedDecision"] = json!(true);
        data["decisionEvidence"]["commands"]["forwardSha256"] =
            json!(format!("{:x}", Sha256::digest(apply.encode_to_vec())));
        data["decisionEvidence"]["commands"]["confirmationSha256"] =
            json!(format!("{:x}", Sha256::digest(confirm.encode_to_vec())));
        data["newManagementConnectionVerified"] = json!(verified);
        f.result(&query, &data).await.unwrap();
        assert_eq!(
            f.state(&confirm.command_id).await,
            if verified { "SUCCEEDED" } else { "UNKNOWN" }
        );
        assert_eq!(f.audits(&confirm.command_id).await, i64::from(verified));
    }
    let other = f
        .task(
            Request::Confirm {
                original_task_id: parent,
            },
            "UNKNOWN",
            f.actor,
            1,
            true,
        )
        .await;
    let query = f
        .task(
            Request::Outcome {
                original_task_id: Uuid::parse_str(&other.command_id).unwrap(),
            },
            "RUNNING",
            f.actor,
            1,
            false,
        )
        .await;
    let mut data = query_receipt(&query, &other, &prepare, id, true);
    data["receipt"]["state"] = json!("COMMITTED");
    data["decisionEvidence"]["historical"] = json!(true);
    data["decisionEvidence"]["confirmedDecision"] = json!(true);
    data["newManagementConnectionVerified"] = json!(true);
    data["decisionEvidence"]["commands"]["forwardSha256"] =
        json!(format!("{:x}", Sha256::digest(apply.encode_to_vec())));
    data["decisionEvidence"]["commands"]["confirmationSha256"] =
        json!(format!("{:x}", Sha256::digest(confirm.encode_to_vec())));
    f.result(&query, &data).await.unwrap();
    assert_eq!(f.state(&other.command_id).await, "FAILED");
    assert_eq!(f.audits(&other.command_id).await, 1);
    f.result(&query, &data).await.unwrap();
    assert_eq!(f.audits(&other.command_id).await, 1);
    f.pool.close().await;
}

#[tokio::test]
#[ignore = "requires explicit private YOFC_IMPORTED_PG_URL"]
async fn postgres_imported_reconnect_and_expiry_keep_reads_and_unissued_writes_distinct() {
    let f = fixture().await;
    let mut reads = Vec::new();
    for request in [
        Request::View,
        Request::Outcome {
            original_task_id: Uuid::new_v4(),
        },
        Request::SourceOutcome {
            original_task_id: Uuid::new_v4(),
        },
    ] {
        reads.push(f.task(request, "RUNNING", f.actor, 1, true).await);
    }
    let running = f
        .task(proposal(Uuid::new_v4()), "RUNNING", f.actor, 1, true)
        .await;
    let queued = f
        .task(proposal(Uuid::new_v4()), "QUEUED", f.actor, 1, false)
        .await;
    let fresh = Uuid::new_v4();
    sqlx::query("UPDATE mgmt_gateways SET connection_epoch=$1")
        .bind(fresh)
        .execute(&f.pool)
        .await
        .unwrap();
    let mut tx = f.pool.begin().await.unwrap();
    crate::task_lifecycle::on_connect(&mut tx, "imported", 1)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(f.state(&running.command_id).await, "UNKNOWN");
    assert_eq!(f.state(&queued.command_id).await, "EXPIRED");
    for read in &reads {
        assert_eq!(f.state(&read.command_id).await, "RUNNING");
    }
    let mut tx = f.pool.begin().await.unwrap();
    crate::task_lifecycle::expire_tasks(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
    for read in &reads {
        assert_eq!(f.state(&read.command_id).await, "EXPIRED");
    }
    assert_eq!(f.state(&running.command_id).await, "UNKNOWN");
    let expires: DateTime<Utc> =
        sqlx::query_scalar("SELECT expires_at FROM mgmt_tasks WHERE id=$1")
            .bind(Uuid::parse_str(&queued.command_id).unwrap())
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert!(
        expires > Utc::now(),
        "connection change, not elapsed time, expired unissued work"
    );
    f.pool.close().await;
}
