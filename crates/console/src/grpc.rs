use crate::{
    auth, db,
    task_crypto::TaskCipher,
    task_lifecycle::{self, apply_result, expire_tasks, reconciliation_batch},
};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::{PgPool, Row};
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio::sync::{Semaphore, mpsc};
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;
use yofc_management_protocol::{
    CONFIGURATION_UPDATE_CAPABILITY, CONFIGURATION_UPDATE_READINESS, PROTOCOL_MAJOR,
    v1::{agent_frame, console_frame, control_server::Control, *},
};

#[derive(Clone)]
pub struct ControlService {
    pub(crate) gate: Arc<crate::maintenance::Gate>,
    pub pool: PgPool,
    // Shared with HTTP. Each DB operation holds a read guard through commit;
    // rotation takes the exclusive guard and durable DB binding lock.
    pub cipher: std::sync::Arc<tokio::sync::RwLock<TaskCipher>>,
    pub slots: Arc<Semaphore>,
}
struct GatewaySession {
    gateway: String,
    tenant: String,
    fingerprint: String,
    epoch: Uuid,
    binding_generation: i64,
    reconcile: bool,
    configuration_update_readiness: bool,
}
fn internal(e: impl std::fmt::Display) -> Status {
    tracing::error!(error=%e,"control operation failed");
    Status::unavailable("INTERNAL")
}
fn transport_identity<T>(request: &Request<T>) -> Result<(Arc<Vec<rustls::pki_types::CertificateDer<'static>>>, tokio_util::sync::CancellationToken), Status> {
    if let Some(info) = request.extensions().get::<crate::tls_transport::GatewayTls>() {
        if info.epoch.is_cancelled() { return Err(Status::permission_denied("FORBIDDEN")); }
        return Ok((info.tls.peer_certs().ok_or(Status::unauthenticated("UNAUTHENTICATED"))?, info.epoch.clone()));
    }
    // Compatibility for explicit test servers using tonic's static TLS adapter.
    // The production incoming stream always supplies GatewayTls.
    #[cfg(test)]
    if let Some(certs) = request.peer_certs() {
        return Ok((certs, tokio_util::sync::CancellationToken::new()));
    }
    Err(Status::unauthenticated("UNAUTHENTICATED"))
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
        let (certs, transport_epoch) = transport_identity(&request)?;
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
        if hello.binding_generation == 0
            || hello.binding_generation > i64::MAX as u64
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
        let configuration_update_readiness = hello.capabilities.iter()
            .any(|name| name == CONFIGURATION_UPDATE_READINESS);
        let mut capabilities = hello.capabilities;
        if configuration_update_readiness {
            // On negotiated streams the first heartbeat establishes current readiness.
            capabilities.retain(|name| name != CONFIGURATION_UPDATE_CAPABILITY);
        }
        let epoch = Uuid::new_v4();
        let admission = self.gate.enter().map_err(|_|Status::unavailable("MAINTENANCE"))?;
        let mut transaction = self.pool.begin().await.map_err(internal)?;
        // Old queued or running work is not leased by a newer binding generation.
        let owned=sqlx::query("UPDATE mgmt_gateways SET connection_epoch=$2,agent_version=$3,boot_id=$4,capabilities=$5,sequence=0 WHERE id=$1 AND binding_generation=$6 RETURNING id").bind(&gateway).bind(epoch).bind(hello.agent_version).bind(hello.boot_id).bind(json!(capabilities)).bind(binding_generation).fetch_optional(&mut *transaction).await.map_err(internal)?;
        if owned.is_none() {
            return Err(Status::permission_denied("FORBIDDEN"));
        }
        task_lifecycle::on_connect(&mut transaction, &gateway, binding_generation)
            .await
            .map_err(internal)?;
        transaction.commit().await.map_err(internal)?;
        drop(admission);
        let binding_generation = hello.binding_generation as i64;
        let (tx, rx) = mpsc::channel(8);
        let pool = self.pool.clone();
        let cipher = self.cipher.clone();
        let gate = self.gate.clone();
        tokio::spawn(async move {
            let _permit = permit;
            let work = run_stream(
                &pool,
                &cipher,
                &gate,
                GatewaySession {
                    gateway,
                    tenant,
                    fingerprint,
                    epoch,
                    binding_generation,
                    reconcile,
                    configuration_update_readiness,
                },
                &mut input,
                &tx,
            );
            let result = tokio::select! {
                biased;
                _ = transport_epoch.cancelled() => Err(Status::permission_denied("FORBIDDEN")),
                result = work => result,
            };
            if let Err(error) = result {
                let _ = tx.try_send(Err(error));
            }
        });
        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn check_transfer_revocation(
        &self,
        request: Request<TransferRevocationCheck>,
    ) -> Result<Response<TransferRevocationStatus>, Status> {
        // Gateway identity comes from the mTLS leaf certificate, never from the payload.
        let (certs, _transport_epoch) = transport_identity(&request)?;
        let leaf = certs
            .first()
            .ok_or(Status::unauthenticated("UNAUTHENTICATED"))?;
        let fingerprint = auth::hash(leaf.as_ref());
        let row = sqlx::query(
            "SELECT id,tenant_id FROM mgmt_gateways WHERE certificate_sha256=$1 AND enabled",
        )
        .bind(&fingerprint)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?
        .ok_or(Status::permission_denied("FORBIDDEN"))?;
        let gateway: String = row.get("id");
        let tenant: String = row.get("tenant_id");
        let transfer_id = Uuid::parse_str(request.get_ref().transfer_id.trim())
            .map_err(|_| Status::invalid_argument("INVALID_ARGUMENT"))?;
        let issuance = sqlx::query(
            "SELECT i.revoked_at FROM mgmt_transfer_issuances i \
             JOIN mgmt_transfer_approvals a ON a.id=i.approval_id \
             WHERE i.transfer_id=$1 AND a.gateway_id=$2",
        )
        .bind(transfer_id)
        .bind(&gateway)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?;
        let status = match issuance {
            Some(row) => {
                if row.get::<Option<DateTime<Utc>>, _>("revoked_at").is_some() {
                    "REVOKED"
                } else {
                    "ACTIVE"
                }
            }
            // Unknown ids stay unknown: a consumer must fail closed.
            None => return Err(Status::not_found("NOT_FOUND")),
        };
        db::audit(
            &self.pool,
            &tenant,
            &gateway,
            "transfer.revocation.check",
            &transfer_id.to_string(),
            status,
        )
        .await
        .map_err(internal)?;
        Ok(Response::new(TransferRevocationStatus {
            status: status.into(),
        }))
    }
}
async fn run_stream(
    pool: &PgPool,
    ciphers: &Arc<tokio::sync::RwLock<TaskCipher>>,
    gate: &crate::maintenance::Gate,
    session: GatewaySession,
    input: &mut Streaming<AgentFrame>,
    tx: &mpsc::Sender<Result<ConsoleFrame, Status>>,
) -> Result<(), Status> {
    let gateway = session.gateway.as_str();
    let tenant = session.tenant.as_str();
    let fingerprint = session.fingerprint.as_str();
    let epoch = session.epoch;
    let binding_generation = session.binding_generation;
    let reconcile = session.reconcile;
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
                let _admission=gate.wait_enter().await.map_err(|_|Status::unavailable("MAINTENANCE"))?;
                // Every message checks current certificate assignment, disablement and connection ownership.
                let current:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND enabled AND connection_epoch=$2 AND certificate_sha256=$3 AND binding_generation=$4)").bind(gateway).bind(epoch).bind(fingerprint).bind(binding_generation).fetch_one(pool).await.map_err(internal)?;
                if !current{return Err(Status::permission_denied("FORBIDDEN"));}
                match frame.payload {
                    Some(agent_frame::Payload::Heartbeat(h))=>{
                        if h.sequence>i64::MAX as u64 || h.uptime_seconds>i64::MAX as u64{return Err(Status::invalid_argument("INVALID_ARGUMENT"));}
                        let mut transaction=pool.begin().await.map_err(internal)?;
                        let configuration_ready = session.configuration_update_readiness
                            .then_some(h.kura_available && h.configuration_update_ready == Some(true));
                        let updated = sqlx::query(
                            "UPDATE mgmt_gateways SET last_seen=now(), kura_available=$3, uptime_seconds=$4,
                                sequence=$5, revision=revision+1, online_reported=true,
                                capabilities=CASE
                                    WHEN NOT $6::boolean THEN capabilities - ($7::text)
                                    WHEN $6::boolean AND NOT (capabilities ? $7::text)
                                        THEN capabilities || jsonb_build_array($7::text)
                                    ELSE capabilities
                                END
                             WHERE id=$1 AND connection_epoch=$2 AND sequence<$5
                               AND enabled AND certificate_sha256=$8 AND binding_generation=$9
                             RETURNING revision")
                            .bind(gateway).bind(epoch).bind(h.kura_available).bind(h.uptime_seconds as i64)
                            .bind(h.sequence as i64).bind(configuration_ready).bind(CONFIGURATION_UPDATE_CAPABILITY)
                            .bind(fingerprint).bind(binding_generation)
                            .fetch_optional(&mut *transaction).await.map_err(internal)?;
                        let Some(updated)=updated else{return Err(Status::invalid_argument("CONFLICT"))};
                        last_heartbeat=tokio::time::Instant::now();
                        db::event(&mut *transaction,tenant,"gateway",gateway,updated.get("revision"),json!({"online":true,"kuraAvailable":h.kura_available})).await.map_err(internal)?;
                        transaction.commit().await.map_err(internal)?;
                    }
                    Some(agent_frame::Payload::Result(r))=>{
                        let cipher = ciphers.read().await;
                        apply_result(pool,&cipher,gateway,tenant,epoch,r).await?;
                    }
                    _=>return Err(Status::invalid_argument("INVALID_ARGUMENT")),
                }
            }
            _=tick.tick()=>{
                let _admission=match gate.enter(){
                    Ok(permit)=>permit,
                    Err(error) if matches!(error.downcast_ref::<crate::maintenance::Blocked>(),Some(crate::maintenance::Blocked::Busy))=>continue,
                    Err(_)=>return Err(Status::unavailable("MAINTENANCE")),
                };
                if last_heartbeat.elapsed()>Duration::from_secs(15){return Err(Status::deadline_exceeded("TIMEOUT"));}
                let current:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND enabled AND connection_epoch=$2 AND certificate_sha256=$3)").bind(gateway).bind(epoch).bind(fingerprint).fetch_one(pool).await.map_err(internal)?;
                if !current{return Err(Status::permission_denied("FORBIDDEN"));}
                // Only reads may be leased again. Uncertain writes use receipt queries.
                let cipher = ciphers.read().await;
                let mut transaction=pool.begin().await.map_err(internal)?;
                cipher.lock_storage(&mut transaction).await.map_err(internal)?;
                // Serialize task admission against reconnect/disablement before
                // locking tasks, using the same gateway -> task lock order as receipts.
                let owned=sqlx::query("SELECT id FROM mgmt_gateways WHERE id=$1 AND enabled AND connection_epoch=$2 AND binding_generation=$3 FOR SHARE")
                    .bind(gateway).bind(epoch).bind(binding_generation).fetch_optional(&mut *transaction).await.map_err(internal)?;
                if owned.is_none(){return Err(Status::permission_denied("FORBIDDEN"));}
                let row=sqlx::query("UPDATE mgmt_tasks SET state='RUNNING',lease_until=now()+interval '10 seconds',dispatch_epoch=$2 WHERE id=(SELECT id FROM mgmt_tasks WHERE gateway_id=$1 AND binding_generation=$3 AND EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.enabled AND g.connection_epoch=$2 AND g.binding_generation=$3) AND (operation<>12 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'kura.configuration.update')) AND (operation NOT IN (13,14) OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'kura.snapshot.write')) AND (operation NOT BETWEEN 15 AND 19 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'ordinary.boot.v1')) AND (operation NOT BETWEEN 20 AND 22 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'recovery.ordinary.v1')) AND (operation<>29 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'recovery.ordinary.cancel.v1' AND g.capabilities ? 'recovery.ordinary.v1')) AND (operation NOT BETWEEN 23 AND 28 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'system.control.v1')) AND (operation NOT IN (24,25) OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'service.restart.v1')) AND (operation NOT IN (26,27) OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'system.reboot.v1')) AND ((operation NOT BETWEEN 24 AND 29 AND operation NOT BETWEEN 31 AND 33 AND operation NOT BETWEEN 34 AND 37) OR dispatch_epoch=$2) AND (operation NOT BETWEEN 30 AND 33 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'firewall.remote.v1')) AND (operation NOT BETWEEN 34 AND 37 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'nm.remote.v1')) AND (operation NOT BETWEEN 38 AND 48 OR EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'firewall.imported.original.v1')) AND (operation NOT IN (40,41,42,43,47) OR (dispatch_epoch=$2 AND EXISTS(SELECT 1 FROM mgmt_gateways g WHERE g.id=$1 AND g.capabilities ? 'firewall.imported.write.v1'))) AND (state='QUEUED' OR (state='RUNNING' AND operation IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48))) AND expires_at>now() AND (SELECT count(*) FROM mgmt_tasks current_tasks WHERE current_tasks.gateway_id=$1 AND current_tasks.state='RUNNING' AND current_tasks.lease_until>now() AND current_tasks.dispatch_epoch=$2)<2 AND (lease_until IS NULL OR lease_until<now()) ORDER BY created_at LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id,tenant_id,gateway_id,operation,expires_at,idempotency_key,input").bind(gateway).bind(epoch).bind(binding_generation).fetch_optional(&mut *transaction).await.map_err(internal)?;
                let payload = row.as_ref().map(|r| cipher.open_row(r,"input")).transpose().map_err(internal)?;
                if let Some(ref r)=row {
                    db::event(&mut *transaction,tenant,"task",&r.get::<Uuid,_>("id").to_string(),0,json!({"state":"RUNNING"})).await.map_err(internal)?;
                }
                transaction.commit().await.map_err(internal)?;
                if let (Some(r),Some(payload))=(row,payload) {
                    let command=ReadCommand{imported_firewall:payload.get("importedFirewall").filter(|v|!v.is_null()).map(|v|serde_json::from_value::<yofc_management_protocol::imported::Envelope>(v.clone()).and_then(|v|v.to_wire().map_err(serde::de::Error::custom))).transpose().map_err(internal)?,firewall_change:payload.get("firewallChange").filter(|v|!v.is_null()).map(crate::firewall_tasks::wire_change).transpose().map_err(internal)?,firewall_action:payload.get("firewallAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,nm_change:payload.get("nmChange").filter(|v|!v.is_null()).map(crate::nm_tasks::wire_change).transpose().map_err(internal)?,nm_action:payload.get("nmAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,system_control_action:payload.get("systemControlAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,ordinary_handoff_action:payload.get("ordinaryHandoffAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,ordinary_boot_action:payload.get("ordinaryBootAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_snapshot_rollback:payload.get("kuraSnapshotRollback").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_snapshot_save:payload.get("kuraSnapshotSave").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_configuration_update:payload.get("kuraConfigurationUpdate").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,kura_configuration:payload.get("kuraConfiguration").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,network_takeover:payload.get("networkTakeover").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,network_change:payload.get("networkChange").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,network_action:payload.get("networkAction").filter(|v|!v.is_null()).map(|v|serde_json::from_value(v.clone())).transpose().map_err(internal)?,command_id:r.get::<Uuid,_>("id").to_string(),operation:r.get("operation"),expires_at_unix_ms:r.get::<DateTime<Utc>,_>("expires_at").timestamp_millis(),idempotency_key:r.get("idempotency_key")};
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
pub async fn maintenance(pool: PgPool, gate: Arc<crate::maintenance::Gate>) {
    let mut tick = tokio::time::interval(Duration::from_secs(5));
    loop {
        tick.tick().await;
        let Ok(_admission)=gate.enter() else{continue};
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
