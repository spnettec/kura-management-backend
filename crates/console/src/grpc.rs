use crate::{
    auth, db,
    task_crypto::{TaskCipher, TaskContext},
};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio::sync::{Semaphore, mpsc};
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;
use yofc_management_protocol::{
    MAX_FRAME, PROTOCOL_MAJOR,
    v1::{agent_frame, console_frame, control_server::Control, *},
};

#[derive(Clone)]
pub struct ControlService {
    pub pool: PgPool,
    pub cipher: TaskCipher,
    pub slots: Arc<Semaphore>,
}
fn internal(e: impl std::fmt::Display) -> Status {
    tracing::error!(error=%e,"control operation failed");
    Status::unavailable("INTERNAL")
}
#[tonic::async_trait]
impl Control for ControlService {
    type ConnectStream = Pin<Box<dyn Stream<Item = Result<ConsoleFrame, Status>> + Send>>;
    async fn connect(
        &self,
        request: Request<Streaming<AgentFrame>>,
    ) -> Result<Response<Self::ConnectStream>, Status> {
        let permit = self
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Status::resource_exhausted("RESOURCE_EXHAUSTED"))?;
        let certs = request
            .peer_certs()
            .ok_or(Status::unauthenticated("UNAUTHENTICATED"))?;
        let leaf = certs
            .first()
            .ok_or(Status::unauthenticated("UNAUTHENTICATED"))?;
        let fingerprint = auth::hash(leaf.as_ref());
        let row = sqlx::query(
            "SELECT id,tenant_id,binding_generation FROM mgmt_gateways WHERE certificate_sha256=$1 AND enabled",
        )
        .bind(&fingerprint)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?
        .ok_or(Status::permission_denied("FORBIDDEN"))?;
        let gateway: String = row.get("id");
        let tenant: String = row.get("tenant_id");
        let binding_generation: i64 = row.get("binding_generation");
        let mut input = request.into_inner();
        let frame = tokio::time::timeout(Duration::from_secs(5), input.message())
            .await
            .map_err(|_| Status::deadline_exceeded("TIMEOUT"))??
            .ok_or(Status::invalid_argument("hello required"))?;
        let Some(agent_frame::Payload::Hello(hello)) = frame.payload else {
            return Err(Status::invalid_argument("hello required"));
        };
        if hello.protocol_major != PROTOCOL_MAJOR {
            return Err(Status::failed_precondition("PROTOCOL_MISMATCH"));
        }
        if hello.gateway_id != gateway {
            return Err(Status::permission_denied("FORBIDDEN"));
        }
        // A durable binding generation must be positive and exactly match the
        // registered generation. Zero identifies a pre-handshake agent and is rejected.
        if hello.binding_generation == 0 || hello.binding_generation > i64::MAX as u64
            || hello.binding_generation as i64 != binding_generation
        {
            return Err(Status::permission_denied("FORBIDDEN"));
        }
        let binding_generation = hello.binding_generation as i64;
        if hello.agent_version.len() > 64
            || hello.boot_id.len() > 64
            || hello.capabilities.len() > 32
            || hello.capabilities.iter().any(|s| s.len() > 128)
        {
            return Err(Status::resource_exhausted("RESOURCE_EXHAUSTED"));
        }
        let reconcile = hello.capabilities.iter().any(|s| s == "task.reconcile.v1");
        let epoch = Uuid::new_v4();
        let mut transaction=self.pool.begin().await.map_err(internal)?;
        // Old queued or running work is not leased by a newer binding generation.
        sqlx::query("UPDATE mgmt_tasks SET state='EXPIRED',error='BINDING_GENERATION_CHANGED',lease_until=NULL WHERE gateway_id=$1 AND binding_generation=$2 AND state IN ('QUEUED','RUNNING')").bind(&gateway).bind(binding_generation).execute(&mut *transaction).await.map_err(internal)?;
        let owned=sqlx::query("UPDATE mgmt_gateways SET connection_epoch=$2,agent_version=$3,boot_id=$4,capabilities=$5,sequence=0 WHERE id=$1 AND binding_generation=$6 RETURNING id").bind(&gateway).bind(epoch).bind(hello.agent_version).bind(hello.boot_id).bind(json!(hello.capabilities)).bind(binding_generation).fetch_optional(&mut *transaction).await.map_err(internal)?;
        if owned.is_none() {
            return Err(Status::permission_denied("FORBIDDEN"));
        }
        transaction.commit().await.map_err(internal)?;
        let binding_generation = hello.binding_generation as i64;
        let (tx, rx) = mpsc::channel(8);
        let pool = self.pool.clone();
        let cipher = self.cipher.clone();
        tokio::spawn(async move {
            let _permit = permit;
            let result = run_stream(
                &pool,
                &cipher,
                &gateway,
                &tenant,
                &fingerprint,
                epoch,
                binding_generation,
                reconcile,
                &mut input,
                &tx,
            )
            .await;
            if let Err(error) = result {
                let _ = tx.try_send(Err(error));
            }
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}
async fn run_stream(
    pool: &PgPool,
    cipher: &TaskCipher,
    gateway: &str,
    tenant: &str,
    fingerprint: &str,
    epoch: Uuid,
    binding_generation: i64,
    reconcile: bool,
    input: &mut Streaming<AgentFrame>,
    tx: &mpsc::Sender<Result<ConsoleFrame, Status>>,
) -> Result<(), Status> {
    tx.send(Ok(ConsoleFrame {
        payload: Some(console_frame::Payload::Welcome(Welcome {
            connection_epoch: epoch.to_string(),
            heartbeat_seconds: 5,
        })),
    }))
    .await
    .map_err(|_| Status::cancelled("disconnected"))?;
    let mut tick = tokio::time::interval(Duration::from_secs(1));
    let mut last_heartbeat = tokio::time::Instant::now();
    loop {
        tokio::select! {
            _=tx.closed()=>return Ok(()),
            frame=input.message()=>{
                let Some(frame)=frame? else{return Ok(())};
                // Every message checks current certificate assignment, disablement and connection ownership.
                let current:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND enabled AND connection_epoch=$2 AND certificate_sha256=$3 AND binding_generation=$4)").bind(gateway).bind(epoch).bind(fingerprint).bind(binding_generation).fetch_one(pool).await.map_err(internal)?;
                if !current{return Err(Status::permission_denied("FORBIDDEN"));}
                match frame.payload {
                    Some(agent_frame::Payload::Heartbeat(h))=>{
                        if h.sequence>i64::MAX as u64 || h.uptime_seconds>i64::MAX as u64{return Err(Status::invalid_argument("INVALID_ARGUMENT"));}
                        let mut transaction=pool.begin().await.map_err(internal)?;
                        let updated=sqlx::query("UPDATE mgmt_gateways SET last_seen=now(),kura_available=$3,uptime_seconds=$4,sequence=$5,revision=revision+1,online_reported=true WHERE id=$1 AND connection_epoch=$2 AND sequence<$5 RETURNING revision").bind(gateway).bind(epoch).bind(h.kura_available).bind(h.uptime_seconds as i64).bind(h.sequence as i64).fetch_optional(&mut *transaction).await.map_err(internal)?;
                        let Some(updated)=updated else{return Err(Status::invalid_argument("CONFLICT"))};
                        last_heartbeat=tokio::time::Instant::now();
                        db::event(&mut *transaction,tenant,"gateway",gateway,updated.get("revision"),json!({"online":true,"kuraAvailable":h.kura_available})).await.map_err(internal)?;
                        transaction.commit().await.map_err(internal)?;
                    }
                    Some(agent_frame::Payload::Result(r))=>{
                        apply_result(pool,cipher,gateway,tenant,epoch,r).await?;
                    }
                    _=>return Err(Status::invalid_argument("INVALID_ARGUMENT")),
                }
            }
            _=tick.tick()=>{
                if last_heartbeat.elapsed()>Duration::from_secs(15){return Err(Status::deadline_exceeded("TIMEOUT"));}
                let current:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND enabled AND connection_epoch=$2 AND certificate_sha256=$3)").bind(gateway).bind(epoch).bind(fingerprint).fetch_one(pool).await.map_err(internal)?;
                if !current{return Err(Status::permission_denied("FORBIDDEN"));}
                // Only reads may be leased again. Uncertain writes use receipt queries.
                let mut transaction=pool.begin().await.map_err(internal)?;
                let row=sqlx::query("UPDATE mgmt_tasks SET state='RUNNING',lease_until=now()+interval '10 seconds',dispatch_epoch=$2 WHERE id=(SELECT id FROM mgmt_tasks WHERE gateway_id=$1 AND binding_generation=$3 AND EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.enabled AND g.connection_epoch=$2 AND g.binding_generation=$3) AND (operation<>12 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'kura.configuration.update')) AND (operation NOT IN (13,14) OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'kura.snapshot.write')) AND (state='QUEUED' OR (state='RUNNING' AND operation IN (1,2,3,4,8,9,11))) AND expires_at>now() AND (SELECT count(*) FROM mgmt_tasks current_tasks WHERE current_tasks.gateway_id=$1 AND current_tasks.state='RUNNING' AND current_tasks.lease_until>now() AND current_tasks.dispatch_epoch=$2)<2 AND (lease_until IS NULL OR lease_until<now()) ORDER BY created_at LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id,tenant_id,gateway_id,operation,expires_at,idempotency_key,input").bind(gateway).bind(epoch).bind(binding_generation).fetch_optional(&mut *transaction).await.map_err(internal)?;
                let payload = row.as_ref().map(|r| cipher.open_row(r,"input")).transpose().map_err(internal)?;
                if let Some(ref r)=row {
                    db::event(&mut *transaction,tenant,"task",&r.get::<Uuid,_>("id").to_string(),0,json!({"state":"RUNNING"})).await.map_err(internal)?;
                }
                transaction.commit().await.map_err(internal)?;
                if let (Some(r),Some(payload))=(row,payload) {
                    let command=ReadCommand{kura_snapshot_rollback:payload.get("kuraSnapshotRollback").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_snapshot_save:payload.get("kuraSnapshotSave").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_configuration_update:payload.get("kuraConfigurationUpdate").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_configuration:payload.get("kuraConfiguration").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,network_takeover:payload.get("networkTakeover").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,network_change:payload.get("networkChange").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,network_action:payload.get("networkAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,command_id:r.get::<Uuid,_>("id").to_string(),operation:r.get("operation"),expires_at_unix_ms:r.get::<DateTime<Utc>,_>("expires_at").timestamp_millis(),idempotency_key:r.get("idempotency_key")};
                    tx.try_send(Ok(ConsoleFrame{payload:Some(console_frame::Payload::Command(command))})).map_err(|_|Status::resource_exhausted("RESOURCE_EXHAUSTED"))?;
                }
                if reconcile {
                    let ids=reconciliation_batch(pool,gateway,epoch,binding_generation).await.map_err(internal)?;
                    if !ids.is_empty() {
                        tx.try_send(Ok(ConsoleFrame{payload:Some(console_frame::Payload::TaskQuery(TaskQuery{command_ids:ids}))})).map_err(|_|Status::resource_exhausted("RESOURCE_EXHAUSTED"))?;
                    }
                }
            }
        }
    }
}
fn valid_result(result: &TaskResult) -> Result<(TaskState, ErrorCode, Value), Status> {
    if result.result_json.len() > MAX_FRAME / 4 {
        return Err(Status::resource_exhausted("RESOURCE_EXHAUSTED"));
    }
    let state = TaskState::try_from(result.state)
        .map_err(|_| Status::invalid_argument("INVALID_ARGUMENT"))?;
    let error = ErrorCode::try_from(result.error)
        .map_err(|_| Status::invalid_argument("INVALID_ARGUMENT"))?;
    let valid = match state {
        TaskState::Succeeded | TaskState::Running => error == ErrorCode::Unspecified,
        TaskState::Unknown => error == ErrorCode::OutcomeUnknown,
        TaskState::Failed | TaskState::Expired => {
            !matches!(error, ErrorCode::Unspecified | ErrorCode::OutcomeUnknown)
        }
        _ => false,
    };
    if !valid {
        return Err(Status::invalid_argument("INVALID_ARGUMENT"));
    }
    let data = serde_json::from_slice(&result.result_json)
        .map_err(|_| Status::invalid_argument("INVALID_ARGUMENT"))?;
    Ok((state, error, data))
}

fn configuration_proof(data: &Value, id: Uuid, operation: i32) -> bool {
    let proof = &data["reconciliation"];
    operation == 12
        && proof["source"] == "configuration.journal.v1"
        && proof["commandId"] == id.to_string()
        && proof["operation"] == operation
        && proof["pid"].as_str().is_some_and(|v| !v.is_empty() && v.len() <= 512)
        && proof["previousRevision"]
            .as_str()
            .is_some_and(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
        && ["paramsSha256", "journalSha256"].iter().all(|key| proof[key]
            .as_str()
            .is_some_and(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit())))
        && data["submitted"] == true
        && data.get("properties").is_none()
}

fn outcome_proof(data: &Value, id: Uuid, operation: i32) -> bool {
    network_proof(data, id, operation) || configuration_proof(data, id, operation)
}

fn network_proof(data: &Value, id: Uuid, operation: i32) -> bool {
    let proof = &data["reconciliation"];
    matches!(operation, 5 | 6 | 7 | 10)
        && proof["source"] == "network.journal.v1"
        && proof["commandId"] == id.to_string()
        && proof["operation"] == operation
        && proof["transactionId"]
            .as_str()
            .is_some_and(|v| Uuid::parse_str(v).is_ok())
        && proof["state"]
            .as_str()
            .is_some_and(|s| matches!(s, "COMMITTED" | "ROLLED_BACK"))
        && proof["state"] == data["state"]
        && proof["journalSha256"]
            .as_str()
            .is_some_and(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit()))
}

async fn apply_result(
    pool: &PgPool,
    cipher: &TaskCipher,
    gateway: &str,
    tenant: &str,
    epoch: Uuid,
    result: TaskResult,
) -> Result<(), Status> {
    let id = Uuid::parse_str(&result.command_id)
        .map_err(|_| Status::invalid_argument("INVALID_ARGUMENT"))?;
    let (state, error, data) = valid_result(&result)?;
    let mut tx = pool.begin().await.map_err(internal)?;
    // Hold ownership stable while recording a receipt, including late replies.
    let owned=sqlx::query("SELECT id FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 AND enabled AND connection_epoch=$3 FOR SHARE")
        .bind(gateway).bind(tenant).bind(epoch).fetch_optional(&mut *tx).await.map_err(internal)?;
    if owned.is_none() {
        return Err(Status::permission_denied("FORBIDDEN"));
    }
    let previous=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state FROM mgmt_tasks WHERE id=$1 AND gateway_id=$2 AND tenant_id=$3 AND dispatch_epoch=$4 AND state IN ('RUNNING','UNKNOWN') FOR UPDATE")
        .bind(id).bind(gateway).bind(tenant).bind(epoch).fetch_optional(&mut *tx).await.map_err(internal)?;
    if let Some(previous) = previous {
        let proved = outcome_proof(&data, id, previous.get("operation"));
        let data = cipher
            .seal(
                &TaskContext::from_row(&previous).map_err(internal)?,
                "result",
                &data,
            )
            .map_err(internal)?;
        let previous: String = previous.get("state");
        // An admission still in progress refreshes the query lease without
        // erasing uncertainty after the original deadline.
        let (state, error) = if previous == "UNKNOWN" && state == TaskState::Running {
            (TaskState::Unknown, ErrorCode::OutcomeUnknown)
        } else {
            (state, error)
        };
        // Only a terminal receipt resolves uncertainty. The original TTL cannot
        // turn an already completed side effect into an expiry.
        sqlx::query("UPDATE mgmt_tasks SET state=$2,error=$3,result=$4,lease_until=CASE WHEN $2 IN ('RUNNING','UNKNOWN') THEN now()+interval '10 seconds' ELSE NULL END WHERE id=$1")
            .bind(id).bind(state.as_str_name()).bind(if error==ErrorCode::Unspecified{None}else{Some(error.as_str_name())})    .bind(&data)
            .execute(&mut *tx).await.map_err(internal)?;
        if previous != state.as_str_name() {
            db::event(
                &mut *tx,
                tenant,
                "task",
                &id.to_string(),
                0,
                json!({"state":state.as_str_name()}),
            )
            .await
            .map_err(internal)?;
            db::audit(
                &mut *tx,
                tenant,
                gateway,
                "task.result",
                &id.to_string(),
                state.as_str_name(),
            )
            .await
            .map_err(internal)?;
            if previous == "UNKNOWN" && matches!(state, TaskState::Succeeded | TaskState::Failed) {
                db::audit(
                    &mut *tx,
                    tenant,
                    gateway,
                    if proved {
                        if data["reconciliation"]["source"] == "network.journal.v1" {
                            "task.reconcile.network"
                        } else {
                            "task.reconcile.configuration"
                        }
                    } else {
                        "task.reconcile.receipt"
                    },
                    &id.to_string(),
                    state.as_str_name(),
                )
                .await
                .map_err(internal)?;
            }
        }
    }
    tx.commit().await.map_err(internal)?;
    Ok(())
}

async fn reconciliation_batch(
    pool: &PgPool,
    gateway: &str,
    epoch: Uuid,
    binding_generation: i64,
) -> anyhow::Result<Vec<String>> {
    let rows=sqlx::query("UPDATE mgmt_tasks SET dispatch_epoch=$2,lease_until=now()+interval '10 seconds' WHERE id IN (SELECT id FROM mgmt_tasks WHERE gateway_id=$1 AND binding_generation=$3 AND EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.enabled AND g.connection_epoch=$2 AND g.binding_generation=$3) AND state='UNKNOWN' AND (lease_until IS NULL OR lease_until<now()) ORDER BY lease_until NULLS FIRST,created_at LIMIT 2 FOR UPDATE SKIP LOCKED) RETURNING id")
        .bind(gateway).bind(epoch).bind(binding_generation).fetch_all(pool).await?;
    Ok(rows
        .iter()
        .map(|r| r.get::<Uuid, _>("id").to_string())
        .collect())
}

async fn expire_tasks(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> anyhow::Result<()> {
    let uncertain=sqlx::query("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',lease_until=NULL WHERE state='RUNNING' AND operation NOT IN (1,2,3,4,8,9,11) AND (expires_at<now() OR lease_until<now()) RETURNING id,tenant_id")
        .fetch_all(&mut **tx).await?;
    for r in uncertain {
        let tenant: String = r.get("tenant_id");
        let id = r.get::<Uuid, _>("id").to_string();
        db::event(
            &mut **tx,
            &tenant,
            "task",
            &id,
            0,
            json!({"state":"UNKNOWN"}),
        )
        .await?;
        db::audit(
            &mut **tx,
            &tenant,
            "console",
            "task.uncertain",
            &id,
            "UNKNOWN",
        )
        .await?;
    }
    let expired=sqlx::query("UPDATE mgmt_tasks SET state='EXPIRED',error='TIMEOUT' WHERE state IN ('QUEUED','RUNNING') AND expires_at<now() RETURNING id,tenant_id")
        .fetch_all(&mut **tx).await?;
    for r in expired {
        let tenant: String = r.get("tenant_id");
        let id = r.get::<Uuid, _>("id").to_string();
        db::event(
            &mut **tx,
            &tenant,
            "task",
            &id,
            0,
            json!({"state":"EXPIRED"}),
        )
        .await?;
        db::audit(&mut **tx, &tenant, "console", "task.expire", &id, "EXPIRED").await?;
    }
    Ok(())
}

pub async fn maintenance(pool: PgPool) {
    let mut tick = tokio::time::interval(Duration::from_secs(5));
    loop {
        tick.tick().await;
        let result: anyhow::Result<()>=async {
            let mut transaction=pool.begin().await?;
            let rows=sqlx::query("UPDATE mgmt_gateways SET online_reported=false,revision=revision+1 WHERE online_reported AND (NOT enabled OR last_seen<now()-interval '15 seconds') RETURNING id,tenant_id,revision").fetch_all(&mut *transaction).await?;
            for r in rows{db::event(&mut *transaction,&r.get::<String,_>("tenant_id"),"gateway",&r.get::<String,_>("id"),r.get("revision"),json!({"online":false})).await?;}
            expire_tasks(&mut transaction).await?;
            transaction.commit().await?;
            sqlx::query("DELETE FROM mgmt_events WHERE id<(SELECT coalesce(max(id),0)-10000 FROM mgmt_events)").execute(&pool).await?;
            sqlx::query("DELETE FROM mgmt_sessions WHERE expires_at<now()").execute(&pool).await?;
            Ok(())
        }.await;
        if let Err(e) = result {
            tracing::error!(error=%e,"maintenance failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn network_proof_annotation_requires_matching_task_and_terminal_transaction() {
        let id = Uuid::new_v4();
        let data = json!({"state":"COMMITTED","reconciliation":{
            "source":"network.journal.v1","commandId":id,"operation":6,
            "transactionId":Uuid::new_v4(),"state":"COMMITTED","journalSha256":"ab".repeat(32)
        }});
        assert!(network_proof(&data, id, 6));
        assert!(!network_proof(&data, Uuid::new_v4(), 6));
        assert!(!network_proof(&data, id, 11));
        for (key, value) in [
            ("state", json!("RECOVERY_REQUIRED")),
            ("operation", json!(7)),
            ("transactionId", json!("../invalid")),
            ("journalSha256", json!("bad")),
            ("source", json!("other")),
        ] {
            let mut changed = data.clone();
            changed["reconciliation"][key] = value;
            assert!(!network_proof(&changed, id, 6));
        }
    }
    #[test]
    fn configuration_outcome_annotation_requires_the_recorded_submission_identity() {
        let id = Uuid::new_v4();
        let data = json!({"pid":"fixture","submitted":true,"previousRevision":"a".repeat(64),
            "reconciliation":{"source":"configuration.journal.v1","commandId":id,"operation":12,
                "pid":"fixture","previousRevision":"a".repeat(64),"paramsSha256":"ab".repeat(32),
                "journalSha256":"cd".repeat(32)}});
        assert!(configuration_proof(&data, id, 12));
        assert!(!configuration_proof(&data, Uuid::new_v4(), 12));
        assert!(!configuration_proof(&data, id, 11));
        let mut with_properties = data.clone();
        with_properties["properties"] = json!({"secret":"x"});
        assert!(!configuration_proof(&with_properties, id, 12));
        for (key, value) in [
            ("source", json!("network.journal.v1")),
            ("pid", json!("")),
            ("previousRevision", json!("short")),
            ("paramsSha256", json!("bad")),
            ("journalSha256", json!("bad")),
        ] {
            let mut changed = data.clone();
            changed["reconciliation"][key] = value;
            assert!(!configuration_proof(&changed, id, 12));
        }
    }
    #[test]
    fn receipt_states_cannot_disguise_uncertainty_as_success_or_expiry() {
        for (state, error, valid) in [
            (TaskState::Unknown, ErrorCode::OutcomeUnknown, true),
            (TaskState::Unknown, ErrorCode::Timeout, false),
            (TaskState::Succeeded, ErrorCode::OutcomeUnknown, false),
            (TaskState::Expired, ErrorCode::OutcomeUnknown, false),
            (TaskState::Running, ErrorCode::Unspecified, true),
            (TaskState::Running, ErrorCode::Internal, false),
            (TaskState::Failed, ErrorCode::Unspecified, false),
        ] {
            assert_eq!(
                valid_result(&TaskResult {
                    command_id: Uuid::new_v4().to_string(),
                    state: state as i32,
                    error: error as i32,
                    result_json: b"{}".to_vec()
                })
                .is_ok(),
                valid
            );
        }
    }

    #[tokio::test]
    #[ignore = "requires explicit isolated PostgreSQL URL; creates its own disposable schema"]
    async fn postgres_uncertain_write_reconciles_after_deadline_without_reexecution() {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("task_journal_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let db = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options)
            .await
            .unwrap();
        sqlx::migrate!("../../migrations").run(&db).await.unwrap();
        let user = Uuid::new_v4();
        let epoch = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'test','test','unused','admin')").bind(user).execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('test','test','test','unused',$1,1)").bind(epoch).execute(&db).await.unwrap();
        let write = Uuid::new_v4();
        let read = Uuid::new_v4();
        let queued = Uuid::new_v4();
        for (id, op, state) in [
            (write, 5, "RUNNING"),
            (read, 1, "RUNNING"),
            (queued, 5, "QUEUED"),
        ] {
            sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,lease_until,dispatch_epoch) VALUES($1,'test','test',$2,$3,$4,$5,1,now()-interval '1 second',now()-interval '1 second',$6)")
                .bind(id).bind(user).bind(op).bind(id.to_string()).bind(state).bind(epoch).execute(&db).await.unwrap();
        }
        let keydir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(
            keydir.path(),
            std::os::unix::fs::PermissionsExt::from_mode(0o700),
        )
        .unwrap();
        let key = keydir.path().join("task.key");
        crate::task_crypto::create_key(&key).unwrap();
        let cipher = TaskCipher::load(&key).unwrap();
        assert_eq!(cipher.prepare_storage(&db).await.unwrap(), 3);
        let mut tx = db.begin().await.unwrap();
        expire_tasks(&mut tx).await.unwrap();
        tx.commit().await.unwrap();
        for (id, expected) in [(write, "UNKNOWN"), (read, "EXPIRED"), (queued, "EXPIRED")] {
            let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
                .bind(id)
                .fetch_one(&db)
                .await
                .unwrap();
            assert_eq!(state, expected);
        }
        assert_eq!(
            reconciliation_batch(&db, "test", epoch, 1).await.unwrap(),
            vec![write.to_string()]
        );
        let running = TaskResult {
            command_id: write.to_string(),
            state: TaskState::Running as i32,
            error: 0,
            result_json: b"{}".to_vec(),
        };
        apply_result(&db, &cipher, "test", "test", epoch, running)
            .await
            .unwrap();
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(write)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "UNKNOWN");
        let fresh = Uuid::new_v4();
        sqlx::query("UPDATE mgmt_gateways SET connection_epoch=$1")
            .bind(fresh)
            .execute(&db)
            .await
            .unwrap();
        let completed = TaskResult {
            command_id: write.to_string(),
            state: TaskState::Succeeded as i32,
            error: 0,
            result_json: serde_json::to_vec(&json!({"once":true,"state":"COMMITTED","reconciliation":{
                "source":"network.journal.v1","commandId":write,"operation":5,
                "transactionId":Uuid::new_v4(),"state":"COMMITTED","journalSha256":"ab".repeat(32)
            }})).unwrap(),
        };
        assert!(
            apply_result(&db, &cipher, "test", "test", epoch, completed.clone())
                .await
                .is_err()
        );
        sqlx::query("UPDATE mgmt_tasks SET lease_until=NULL WHERE id=$1")
            .bind(write)
            .execute(&db)
            .await
            .unwrap();
        assert!(
            reconciliation_batch(&db, "test", epoch, 1)
                .await
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            reconciliation_batch(&db, "test", fresh, 1).await.unwrap(),
            vec![write.to_string()]
        );
        assert!(
            apply_result(&db, &cipher, "test", "other", fresh, completed.clone())
                .await
                .is_err()
        );
        apply_result(&db, &cipher, "test", "test", fresh, completed.clone())
            .await
            .unwrap();
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(write)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "SUCCEEDED");
        let failed = TaskResult {
            state: TaskState::Failed as i32,
            error: ErrorCode::Internal as i32,
            ..completed.clone()
        };
        apply_result(&db, &cipher, "test", "test", fresh, failed)
            .await
            .unwrap();
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(write)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "SUCCEEDED");
        sqlx::query("UPDATE mgmt_gateways SET enabled=false")
            .execute(&db)
            .await
            .unwrap();
        assert!(
            apply_result(&db, &cipher, "test", "test", fresh, completed)
                .await
                .is_err()
        );
        let audit: i64 =
            sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE operation='task.uncertain'")
                .fetch_one(&db)
                .await
                .unwrap();
        assert_eq!(audit, 1);
        let resolved: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM mgmt_audit WHERE operation='task.reconcile.network'",
        )
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(resolved, 1);
        db.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}
