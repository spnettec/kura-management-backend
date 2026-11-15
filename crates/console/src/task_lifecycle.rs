//! Task state transitions and receipt reconciliation. Transport adapters do not
//! decide whether an ambiguous write can be replayed.
use crate::{
    db,
    task_crypto::{TaskCipher, TaskContext},
};
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use tonic::Status;
use uuid::Uuid;
use yofc_management_protocol::{MAX_FRAME, v1::*};
fn internal(e: impl std::fmt::Display) -> Status {
    tracing::error!(error=%e,"task lifecycle operation failed");
    Status::unavailable("INTERNAL")
}

/// Called after locking/replacing the gateway connection epoch, in the same
/// transaction. A reconnect never proves that a delivered write had no effect.
pub(crate) async fn on_connect(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    gateway: &str,
    generation: i64,
) -> anyhow::Result<()> {
    let changed = sqlx::query("UPDATE mgmt_tasks SET state=CASE WHEN state='RUNNING' AND operation NOT IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48) THEN 'UNKNOWN' ELSE 'EXPIRED' END,error=CASE WHEN state='RUNNING' AND operation NOT IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48) THEN 'OUTCOME_UNKNOWN' ELSE 'BINDING_GENERATION_CHANGED' END,lease_until=NULL WHERE gateway_id=$1 AND ((binding_generation<>$2 AND state IN ('QUEUED','RUNNING')) OR (binding_generation=$2 AND state='RUNNING' AND operation NOT IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48))) RETURNING id,tenant_id,state")
        .bind(gateway).bind(generation).fetch_all(&mut **tx).await?;
    for row in changed {
        let id = row.get::<Uuid, _>("id").to_string();
        let tenant: String = row.get("tenant_id");
        let state: String = row.get("state");
        db::event(&mut **tx, &tenant, "task", &id, 0, json!({"state":state})).await?;
        db::audit(
            &mut **tx,
            &tenant,
            gateway,
            if state == "UNKNOWN" {
                "task.uncertain"
            } else {
                "task.binding.expire"
            },
            &id,
            &state,
        )
        .await?;
    }
    // Reads can be leased again; queued writes retain their original deadline.
    // A system action is approved for the currently live connection, not an
    // offline queue. Unleased work from another epoch is definitively expired.
    let stale=sqlx::query("UPDATE mgmt_tasks SET state='EXPIRED',error='TIMEOUT',lease_until=NULL WHERE gateway_id=$1 AND binding_generation=$2 AND state='QUEUED' AND (operation BETWEEN 24 AND 29 OR operation BETWEEN 31 AND 33 OR operation BETWEEN 35 AND 37 OR operation IN (40,41,42,43,47)) AND dispatch_epoch IS DISTINCT FROM (SELECT connection_epoch FROM mgmt_gateways WHERE id=$1) RETURNING id,tenant_id")
        .bind(gateway).bind(generation).fetch_all(&mut **tx).await?;
    for row in stale {
        let id = row.get::<Uuid, _>("id").to_string();
        let tenant: String = row.get("tenant_id");
        db::event(
            &mut **tx,
            &tenant,
            "task",
            &id,
            0,
            json!({"state":"EXPIRED"}),
        )
        .await?;
        db::audit(
            &mut **tx,
            &tenant,
            gateway,
            "task.system.connection.expire",
            &id,
            "EXPIRED",
        )
        .await?;
    }
    sqlx::query("UPDATE mgmt_tasks SET lease_until=NULL WHERE gateway_id=$1 AND binding_generation=$2 AND state='RUNNING' AND operation IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48)")
        .bind(gateway).bind(generation).execute(&mut **tx).await?;
    Ok(())
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
        && proof["pid"]
            .as_str()
            .is_some_and(|v| !v.is_empty() && v.len() <= 512)
        && proof["previousRevision"].as_str().is_some_and(|v| {
            v.len() == 64
                && v.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
        && ["paramsSha256", "journalSha256"].iter().all(|key| {
            proof[key]
                .as_str()
                .is_some_and(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit()))
        })
        && data["submitted"] == true
        && data.get("properties").is_none()
}

fn handoff_result(data: &Value, operation: i32) -> bool {
    if !matches!(operation, 20..=22 | 29) {
        return true;
    }
    if data["scope"] != "administrator-selected-checks"
        || data["kuraRecoveryVerified"] != false
        || data["normalBootVerified"] != false
    {
        return false;
    }
    let uuid_id = |value: &Value, prefix: &str| {
        value
            .as_str()
            .and_then(|s| s.strip_prefix(prefix))
            .is_some_and(|s| Uuid::parse_str(s).is_ok_and(|id| id.to_string() == s))
    };
    match operation {
        20 => data["canPrepare"].is_boolean(),
        21 => {
            let action = OrdinaryHandoffAction {
                transaction_id: data["transactionId"].as_str().unwrap_or_default().into(),
                confirmation_token: data["confirmToken"].as_str().unwrap_or_default().into(),
            };
            data["state"] == "ORDINARY_PREPARED"
                && action.valid()
                && uuid_id(&data["profileId"], "profile-")
                && [
                    "profileDigest",
                    "grantSha256",
                    "sourceJournalSha256",
                    "snapshotSha256",
                ]
                .iter()
                .all(|k| {
                    data[k].as_str().is_some_and(|v| {
                        v.len() == 64
                            && v.bytes()
                                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    })
                })
                && data["expiresAtUnixMs"].as_u64().is_some_and(|v| v > 0)
                && ["componentPids", "checkIds"].iter().all(|k| {
                    data[k].as_array().is_some_and(|v| {
                        !v.is_empty()
                            && v.len() <= 32
                            && v.iter().all(|s| s.as_str().is_some_and(|s| !s.is_empty()))
                    })
                })
        }
        29 => {
            data["state"] == "ORDINARY_CANCELLED"
                && data["cancelled"] == true
                && data["issued"] == false
                && data["ordinaryUnitHandedOff"] == false
                && uuid_id(&data["id"], "handoff-")
                && uuid_id(&data["profileId"], "profile-")
        }
        _ => {
            data["state"] == "ORDINARY_ACTIVE_APPROVED_CHECKS"
                && data["ordinaryUnitHandedOff"] == true
                && uuid_id(&data["id"], "handoff-")
                && uuid_id(&data["profileId"], "profile-")
                && data["report"]["schema"] == 2
                && data["report"]["ready"] == true
                && data["report"]["ordinaryLoad"] == true
                && data["report"]["processId"].as_u64().is_some_and(|v| v > 0)
        }
    }
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

fn boot_proof(data: &Value, command: &ReadCommand) -> bool {
    use prost::Message;
    use sha2::{Digest, Sha256};
    let proof = &data["reconciliation"];
    let value = if command.operation == 16 {
        &data["plan"]
    } else {
        data
    };
    let canonical = |s: &str| Uuid::parse_str(s).is_ok_and(|u| u.to_string() == s);
    let digest = |v: &Value| {
        v.as_str().is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
    };
    matches!(command.operation, 16..=19)
        && command.valid_payload()
        && proof["source"] == "ordinary.boot.journal.v1"
        && proof["commandId"] == command.command_id
        && proof["operation"] == command.operation
        && proof["requestSha256"] == format!("{:x}", Sha256::digest(command.encode_to_vec()))
        && digest(&proof["journalSha256"])
        && digest(&proof["decisionSha256"])
        && proof["transactionId"] == value["id"]
        && value["id"]
            .as_str()
            .is_some_and(|s| s.strip_prefix("boot-").is_some_and(canonical))
        && command
            .ordinary_boot_action
            .as_ref()
            .is_none_or(|a| value["id"] == a.transaction_id)
        && value["state"]
            == match command.operation {
                16 => "BOOT_POLICY_PREPARED",
                18 => "CANCELLED",
                _ => "BOOT_POLICY_COMMITTED",
            }
        && value["scope"] == "administrator-reviewed-native-boot-entries"
        && value["bootVerified"] == false
        && value["kuraRecoveryVerified"] == false
        && (command.operation != 16 || data["confirmToken"].as_str().is_some_and(canonical))
}

fn handoff_proof(data: &Value, command: &ReadCommand) -> bool {
    use prost::Message;
    use sha2::{Digest, Sha256};
    let proof = &data["reconciliation"];
    let id = &data[if command.operation == 21 {
        "transactionId"
    } else {
        "id"
    }];
    let digest = |value: &Value| {
        value.as_str().is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
    };
    matches!(command.operation, 21 | 22 | 29)
        && command.valid_payload()
        && handoff_result(data, command.operation)
        && proof["source"] == "ordinary.handoff.journal.v1"
        && proof["commandId"] == command.command_id
        && proof["operation"] == command.operation
        && proof["requestSha256"] == format!("{:x}", Sha256::digest(command.encode_to_vec()))
        && digest(&proof["journalSha256"])
        && digest(&proof["decisionSha256"])
        && proof["transactionId"] == *id
        && command
            .ordinary_handoff_action
            .as_ref()
            .is_none_or(|a| *id == a.transaction_id)
}

fn system_result(data: &Value, operation: i32, state: TaskState) -> bool {
    if !matches!(operation, 23..=28) {
        return true;
    }
    if data.get("reconciliation").is_some()
        && data["reconciliation"]["source"] != "system.control.journal.v1"
    {
        return false;
    }
    if data["state"].is_null() && state != TaskState::Succeeded {
        return matches!(
            state,
            TaskState::Failed | TaskState::Unknown | TaskState::Expired
        ) && data
            .as_object()
            .is_some_and(|m| m.keys().all(|k| matches!(k.as_str(), "code" | "reason")));
    }
    if data["kuraRecoveryVerified"] != false || data["normalBootVerified"] != false {
        return false;
    }
    if operation == 23 {
        return state == TaskState::Succeeded
            && data["configured"] == true
            && matches!(
                data["configurationState"].as_str(),
                Some("READY" | "RECOVERY_REQUIRED")
            );
    }
    let uuid = |v: &Value| {
        v.as_str()
            .is_some_and(|s| Uuid::parse_str(s).is_ok_and(|u| u.to_string() == s))
    };
    let digest = |v: &Value| {
        v.as_str().is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
    };
    let identity = data["id"].as_str().is_some_and(|s| {
        s.strip_prefix("system-")
            .is_some_and(|s| Uuid::parse_str(s).is_ok_and(|u| u.to_string() == s))
    }) && data["unit"].as_str().is_some_and(|s| {
        s.ends_with(".service")
            && s.len() <= 128
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
    }) && uuid(&data["beforeBootId"])
        && data["applicationVerificationConfigured"].is_boolean()
        && data["scope"]
            == if data["applicationVerificationConfigured"] == true {
                "administrator-selected-checks"
            } else {
                "system-process-only"
            };
    if !identity {
        return false;
    }
    match operation {
        24 | 26 => {
            state == TaskState::Succeeded
                && data["state"] == "PREPARED"
                && data["issued"] == false
                && data["requestAccepted"] == false
                && data["action"]
                    == if operation == 24 {
                        "service.restart"
                    } else {
                        "system.reboot"
                    }
                && uuid(&data["confirmToken"])
                && digest(&data["grantSha256"])
                && digest(&data["unitDefinitionSha256"])
                && data["expiresAtUnixMs"].as_u64().is_some_and(|v| v > 0)
        }
        28 => {
            state == TaskState::Succeeded
                && data["state"] == "CANCELLED"
                && data["issued"] == false
                && data["requestAccepted"] == false
        }
        25 | 27 => {
            if data["action"]
                != if operation == 25 {
                    "service.restart"
                } else {
                    "system.reboot"
                }
                || data["issued"] != true
                || data["requestAccepted"] != true
            {
                return false;
            }
            if state == TaskState::Unknown {
                return operation == 27 && data["state"] == "AWAITING_BOOT";
            }
            let reboot = operation == 27
                && uuid(&data["afterBootId"])
                && data["afterBootId"] != data["beforeBootId"];
            let process = data["after"]["pid"].as_u64().is_some_and(|n| n > 0)
                && data["after"]["activeState"] == "active"
                && data["after"]["unit"] == data["unit"]
                && data["after"]["invocationId"]
                    .as_str()
                    .is_some_and(|s| s.len() == 32 && s.bytes().all(|b| b.is_ascii_hexdigit()))
                && data["after"]["invocationId"] != data["before"]["invocationId"]
                && data["after"]["controlGroup"]
                    == format!("/system.slice/{}", data["unit"].as_str().unwrap())
                && if operation == 25 {
                    data["afterBootId"] == data["beforeBootId"]
                } else {
                    reboot
                };
            let verified = data["applicationVerified"] == true;
            if data["state"] != "COMPLETED"
                || data["stage"]
                    != if operation == 25 {
                        "SERVICE_RESTART_OBSERVED"
                    } else {
                        "HOST_REBOOT_OBSERVED"
                    }
                || data["hostRebootVerified"] != reboot
                || data["processRestartVerified"] != process
                || !data["applicationVerified"].is_boolean()
                || data["application"]["configured"] != data["applicationVerificationConfigured"]
                || data["application"]["verified"] != data["applicationVerified"]
            {
                return false;
            }
            if verified {
                let r = &data["application"]["report"];
                let names = |v: &Value, key: &str| -> Option<Vec<String>> {
                    let v = v.as_array()?;
                    if v.is_empty() || v.len() > 32 {
                        return None;
                    }
                    let ids = v
                        .iter()
                        .map(|v| {
                            if key.is_empty() {
                                v.as_str()
                            } else {
                                v[key].as_str()
                            }
                        })
                        .map(|v| {
                            v.filter(|s| !s.is_empty() && s.len() <= 512)
                                .map(str::to_owned)
                        })
                        .collect::<Option<Vec<_>>>()?;
                    if ids.iter().collect::<std::collections::BTreeSet<_>>().len() != ids.len() {
                        return None;
                    }
                    Some(ids)
                };
                let checks = names(&data["checkIds"], "");
                let components = names(&data["componentPids"], "");
                if data["applicationVerificationConfigured"] != true
                    || !process
                    || data["profileId"].as_str().is_none_or(|s| {
                        s.strip_prefix("profile-")
                            .is_none_or(|s| Uuid::parse_str(s).is_err())
                    })
                    || !digest(&data["profileDigest"])
                    || !digest(&data["approvedSnapshotSha256"])
                    || r["schema"] != 2
                    || r["ready"] != true
                    || r["ordinaryLoad"] != true
                    || !r["frameworkStartLevel"]
                        .as_u64()
                        .is_some_and(|level| (5..=i32::MAX as u64).contains(&level))
                    || r["profileId"] != data["profileId"]
                    || r["approvedSnapshotSha256"] != data["approvedSnapshotSha256"]
                    || !digest(&r["nonce"])
                    || !uuid(&r["observerInstanceId"])
                    || checks.is_none()
                    || components.is_none()
                    || names(&r["checks"], "id") != checks
                    || names(&r["providers"], "id") != checks
                    || names(&r["components"], "pid") != components
                    || !r["components"]
                        .as_array()
                        .is_some_and(|v| v.iter().all(|v| v["registered"] == true))
                    || r["processId"] != data["after"]["pid"]
                    || !r["checks"].as_array().is_some_and(|v| {
                        !v.is_empty() && v.len() <= 32 && v.iter().all(|v| v["passed"] == true)
                    })
                    || !r["providers"].as_array().is_some_and(|v| {
                        !v.is_empty()
                            && v.len() <= 32
                            && v.iter()
                                .all(|v| v["available"] == true && v["unique"] == true)
                    })
                {
                    return false;
                }
            }
            let passed =
                process && (data["applicationVerificationConfigured"] == false || verified);
            (if operation == 25 { process } else { reboot })
                && if passed {
                    state == TaskState::Succeeded
                } else {
                    state == TaskState::Failed
                }
        }
        _ => false,
    }
}
fn system_proof(data: &Value, command: &ReadCommand, state: TaskState) -> bool {
    use prost::Message;
    use sha2::{Digest, Sha256};
    let p = &data["reconciliation"];
    let digest = |v: &Value| {
        v.as_str().is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
    };
    matches!(command.operation, 24..=28)
        && command.valid_payload()
        && matches!(state, TaskState::Succeeded | TaskState::Failed)
        && data["id"].as_str().is_some_and(|s| {
            s.strip_prefix("system-")
                .is_some_and(|s| Uuid::parse_str(s).is_ok_and(|u| u.to_string() == s))
        })
        && system_result(data, command.operation, state)
        && p["source"] == "system.control.journal.v1"
        && p["commandId"] == command.command_id
        && p["operation"] == command.operation
        && p["requestSha256"] == format!("{:x}", Sha256::digest(command.encode_to_vec()))
        && p["transactionId"] == data["id"]
        && digest(&p["journalSha256"])
        && digest(&p["decisionSha256"])
        && command
            .system_control_action
            .as_ref()
            .is_none_or(|a| data["id"] == a.transaction_id)
}

pub(crate) async fn apply_result(
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
    cipher.lock_storage(&mut tx).await.map_err(internal)?;
    // Hold ownership stable while recording a receipt, including late replies.
    let owned=sqlx::query("SELECT id FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 AND enabled AND connection_epoch=$3 FOR SHARE")
        .bind(gateway).bind(tenant).bind(epoch).fetch_optional(&mut *tx).await.map_err(internal)?;
    if owned.is_none() {
        return Err(Status::permission_denied("FORBIDDEN"));
    }
    let previous=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,input,idempotency_key,expires_at,actor_id,binding_generation FROM mgmt_tasks WHERE id=$1 AND gateway_id=$2 AND tenant_id=$3 AND dispatch_epoch=$4 AND state IN ('RUNNING','UNKNOWN') FOR UPDATE")
        .bind(id).bind(gateway).bind(tenant).bind(epoch).fetch_optional(&mut *tx).await.map_err(internal)?;
    if let Some(previous) = previous {
        let operation: i32 = previous.get("operation");
        let firewall_reconciliation =
            data["reconciliation"]["source"] == "firewall.remote.journal.v1";
        if firewall_reconciliation && !matches!(operation, 31..=33) {
            return Err(Status::invalid_argument("INVALID_ARGUMENT"));
        }
        let nm_reconciliation = data["reconciliation"]["source"] == "nm.coordinator.journal.v1";
        if nm_reconciliation && !matches!(operation, 34..=37) {
            return Err(Status::invalid_argument("INVALID_ARGUMENT"));
        }
        if matches!(operation,38..=48) {
            let payload=cipher.open_row(&previous,"input").map_err(internal)?;
            let command=crate::imported_tasks::command(id.to_string(),operation,previous.get("idempotency_key"),
                previous.get::<chrono::DateTime<chrono::Utc>,_>("expires_at").timestamp_millis(),&payload).map_err(internal)?;
            if !crate::imported_tasks::valid_receipt(&data,&command,state,error) {return Err(Status::invalid_argument("INVALID_ARGUMENT"));}
            if state==TaskState::Succeeded {crate::imported_tasks::check_original(&mut tx,cipher,&previous,&data).await.map_err(|_|Status::invalid_argument("INVALID_ARGUMENT"))?;}
        }
        if matches!(operation, 34..=37) {
            let payload = cipher.open_row(&previous, "input").map_err(internal)?;
            let command = crate::nm_tasks::command(
                id.to_string(),
                operation,
                previous.get("idempotency_key"),
                previous
                    .get::<chrono::DateTime<chrono::Utc>, _>("expires_at")
                    .timestamp_millis(),
                &payload,
            )
            .map_err(internal)?;
            if !crate::nm_tasks::valid_receipt(&data, &command, state, error) {
                return Err(Status::invalid_argument("INVALID_ARGUMENT"));
            }
        }
        if matches!(operation, 30..=33) {
            let payload = cipher.open_row(&previous, "input").map_err(internal)?;
            let command = crate::firewall_tasks::command(
                id.to_string(),
                operation,
                previous.get("idempotency_key"),
                previous
                    .get::<chrono::DateTime<chrono::Utc>, _>("expires_at")
                    .timestamp_millis(),
                &payload,
            )
            .map_err(internal)?;
            if !crate::firewall_tasks::valid_receipt(&data, &command, state, error) {
                return Err(Status::invalid_argument("INVALID_ARGUMENT"));
            }
        }
        if !system_result(&data, previous.get("operation"), state) {
            return Err(Status::invalid_argument("INVALID_ARGUMENT"));
        }
        if matches!(previous.get::<i32, _>("operation"), 25 | 27 | 28) && data.get("id").is_some() {
            let input = cipher.open_row(&previous, "input").map_err(internal)?;
            if input["systemControlAction"]["transactionId"] != data["id"] {
                return Err(Status::invalid_argument("INVALID_ARGUMENT"));
            }
        }
        if state == TaskState::Succeeded && !handoff_result(&data, previous.get("operation")) {
            return Err(Status::invalid_argument("INVALID_ARGUMENT"));
        }
        if state == TaskState::Succeeded && matches!(previous.get::<i32, _>("operation"), 22 | 29) {
            let input = cipher.open_row(&previous, "input").map_err(internal)?;
            if input["ordinaryHandoffAction"]["transactionId"] != data["id"] {
                return Err(Status::invalid_argument("INVALID_ARGUMENT"));
            }
        }
        let boot_reconciliation = data["reconciliation"]["source"] == "ordinary.boot.journal.v1";
        let handoff_reconciliation =
            data["reconciliation"]["source"] == "ordinary.handoff.journal.v1";
        let system_reconciliation = data["reconciliation"]["source"] == "system.control.journal.v1";
        if boot_reconciliation || handoff_reconciliation || system_reconciliation {
            let payload = cipher.open_row(&previous, "input").map_err(internal)?;
            let command = ReadCommand {
                command_id: id.to_string(),
                operation: previous.get("operation"),
                idempotency_key: previous.get("idempotency_key"),
                expires_at_unix_ms: previous
                    .get::<chrono::DateTime<chrono::Utc>, _>("expires_at")
                    .timestamp_millis(),
                ordinary_boot_action: payload
                    .get("ordinaryBootAction")
                    .filter(|v| !v.is_null())
                    .map(|v| serde_json::from_value(v.clone()))
                    .transpose()
                    .map_err(internal)?,
                ordinary_handoff_action: payload
                    .get("ordinaryHandoffAction")
                    .filter(|v| !v.is_null())
                    .map(|v| serde_json::from_value(v.clone()))
                    .transpose()
                    .map_err(internal)?,
                system_control_action: payload
                    .get("systemControlAction")
                    .filter(|v| !v.is_null())
                    .map(|v| serde_json::from_value(v.clone()))
                    .transpose()
                    .map_err(internal)?,
                ..Default::default()
            };
            if !(if system_reconciliation {
                system_proof(&data, &command, state)
            } else if handoff_reconciliation {
                handoff_proof(&data, &command)
            } else {
                boot_proof(&data, &command)
            }) || (!system_reconciliation && state != TaskState::Succeeded)
            {
                return Err(Status::invalid_argument("INVALID_ARGUMENT"));
            }
        }
        let proved = firewall_reconciliation
            || system_reconciliation
            || boot_reconciliation
            || handoff_reconciliation
            || outcome_proof(&data, id, previous.get("operation"));
        let network_reconciliation = data["reconciliation"]["source"] == "network.journal.v1";
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
                        if firewall_reconciliation {
                            "task.reconcile.firewall"
                        } else if system_reconciliation {
                            "task.reconcile.system_control"
                        } else if boot_reconciliation {
                            "task.reconcile.ordinary_boot"
                        } else if handoff_reconciliation {
                            "task.reconcile.ordinary_handoff"
                        } else if network_reconciliation {
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

pub(crate) async fn reconciliation_batch(
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

pub(crate) async fn expire_tasks(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> anyhow::Result<()> {
    let uncertain=sqlx::query("UPDATE mgmt_tasks SET state='UNKNOWN',error='OUTCOME_UNKNOWN',lease_until=NULL WHERE state='RUNNING' AND operation NOT IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48) AND (expires_at<now() OR lease_until<now()) RETURNING id,tenant_id")
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

#[cfg(test)]
#[path = "system_control_tests.rs"]
mod system_control_tests;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handoff_proof_requires_original_wire_identity_and_selected_scope() {
        use prost::Message;
        use sha2::{Digest, Sha256};
        for operation in [21, 22, 29] {
            let handoff = format!("handoff-{}", Uuid::new_v4());
            let command = ReadCommand {
                command_id: Uuid::new_v4().to_string(),
                operation,
                idempotency_key: Uuid::new_v4().to_string(),
                expires_at_unix_ms: 1,
                ordinary_handoff_action: matches!(operation, 22 | 29).then(|| {
                    OrdinaryHandoffAction {
                        transaction_id: handoff.clone(),
                        confirmation_token: Uuid::new_v4().to_string(),
                    }
                }),
                ..Default::default()
            };
            let mut data = json!({"scope":"administrator-selected-checks","kuraRecoveryVerified":false,"normalBootVerified":false,"profileId":format!("profile-{}",Uuid::new_v4())});
            if operation == 21 {
                data["transactionId"] = json!(handoff);
                data["confirmToken"] = json!(Uuid::new_v4().to_string());
                data["state"] = json!("ORDINARY_PREPARED");
                for k in [
                    "profileDigest",
                    "grantSha256",
                    "sourceJournalSha256",
                    "snapshotSha256",
                ] {
                    data[k] = json!("a".repeat(64));
                }
                data["expiresAtUnixMs"] = json!(1);
                data["componentPids"] = json!(["pid"]);
                data["checkIds"] = json!(["business.check"]);
            } else if operation == 29 {
                data["id"] = json!(handoff);
                data["state"] = json!("ORDINARY_CANCELLED");
                data["cancelled"] = json!(true);
                data["issued"] = json!(false);
                data["ordinaryUnitHandedOff"] = json!(false);
            } else {
                data["id"] = json!(handoff);
                data["state"] = json!("ORDINARY_ACTIVE_APPROVED_CHECKS");
                data["ordinaryUnitHandedOff"] = json!(true);
                data["report"] =
                    json!({"schema":2,"ready":true,"ordinaryLoad":true,"processId":123});
            }
            data["reconciliation"] = json!({"source":"ordinary.handoff.journal.v1","commandId":command.command_id,"operation":operation,"requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"transactionId":handoff,"journalSha256":"b".repeat(64),"decisionSha256":"c".repeat(64)});
            assert!(handoff_proof(&data, &command));
            for n in 0..5 {
                let mut changed = command.clone();
                match n {
                    0 => changed.command_id = Uuid::new_v4().to_string(),
                    1 => changed.idempotency_key.push('x'),
                    2 => changed.expires_at_unix_ms += 1,
                    3 => changed.operation = 20,
                    _ => {
                        if let Some(a) = changed.ordinary_handoff_action.as_mut() {
                            a.confirmation_token = Uuid::new_v4().to_string();
                        } else {
                            changed.operation = 22;
                        }
                    }
                }
                assert!(!handoff_proof(&data, &changed));
            }
            for key in ["normalBootVerified", "kuraRecoveryVerified"] {
                let mut changed = data.clone();
                changed[key] = json!(true);
                assert!(!handoff_proof(&changed, &command));
            }
            for key in [
                "journalSha256",
                "decisionSha256",
                "requestSha256",
                "transactionId",
                "source",
            ] {
                let mut changed = data.clone();
                changed["reconciliation"][key] = json!("wrong");
                assert!(!handoff_proof(&changed, &command));
            }
        }
    }
    #[test]
    fn boot_proof_is_bound_to_every_original_wire_field_and_never_certifies_boot_or_full_business()
    {
        use prost::Message;
        use sha2::{Digest, Sha256};
        for op in 16..=19 {
            let command = ReadCommand {
                command_id: Uuid::new_v4().to_string(),
                operation: op,
                idempotency_key: Uuid::new_v4().to_string(),
                expires_at_unix_ms: 123456,
                ordinary_boot_action: matches!(op, 17 | 18).then(|| OrdinaryBootAction {
                    transaction_id: format!("boot-{}", Uuid::new_v4()),
                    confirmation_token: Uuid::new_v4().to_string(),
                }),
                ..Default::default()
            };
            let txn = command
                .ordinary_boot_action
                .as_ref()
                .map(|a| a.transaction_id.clone())
                .unwrap_or_else(|| format!("boot-{}", Uuid::new_v4()));
            let value = json!({"id":txn,"state":match op{16=>"BOOT_POLICY_PREPARED",18=>"CANCELLED",_=>"BOOT_POLICY_COMMITTED"},"scope":"administrator-reviewed-native-boot-entries","bootVerified":false,"kuraRecoveryVerified":false});
            let mut data = if op == 16 {
                json!({"plan":value,"confirmToken":Uuid::new_v4().to_string()})
            } else {
                value
            };
            data["reconciliation"] = json!({"source":"ordinary.boot.journal.v1","commandId":command.command_id,"operation":op,"transactionId":txn,"requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"journalSha256":"a".repeat(64),"decisionSha256":"b".repeat(64)});
            assert!(boot_proof(&data, &command));
            let mut changed = command.clone();
            changed.expires_at_unix_ms += 1;
            assert!(!boot_proof(&data, &changed));
            changed = command.clone();
            changed.idempotency_key.push('x');
            assert!(!boot_proof(&data, &changed));
            changed = command.clone();
            changed.command_id = Uuid::new_v4().to_string();
            assert!(!boot_proof(&data, &changed));
            if let Some(a) = changed.ordinary_boot_action.as_mut() {
                a.confirmation_token = Uuid::new_v4().to_string();
                assert!(!boot_proof(&data, &changed));
            }
            for key in ["journalSha256", "decisionSha256", "requestSha256"] {
                let mut bad = data.clone();
                bad["reconciliation"][key] = json!("bad");
                assert!(!boot_proof(&bad, &command));
            }
            let mut bad = data.clone();
            if op == 16 {
                bad["plan"]["bootVerified"] = json!(true)
            } else {
                bad["kuraRecoveryVerified"] = json!(true)
            };
            assert!(!boot_proof(&bad, &command));
        }
    }
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
    #[tokio::test]
    #[ignore = "requires explicit isolated PostgreSQL URL; creates a disposable schema"]
    async fn postgres_reconnect_preserves_same_generation_and_never_expires_delivered_writes() {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("reconnect_{}", Uuid::new_v4().simple());
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
        let actor = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'test','test','unused','admin')").bind(actor).execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation) VALUES('test','test','test','unused',4)").execute(&db).await.unwrap();
        let mut records = Vec::new();
        for (generation, operation, state, expected) in [
            (4, 12, "QUEUED", "QUEUED"),
            (4, 1, "RUNNING", "RUNNING"),
            (4, 12, "RUNNING", "UNKNOWN"),
            (4, 12, "UNKNOWN", "UNKNOWN"),
            (3, 12, "QUEUED", "EXPIRED"),
            (3, 1, "RUNNING", "EXPIRED"),
            (3, 12, "RUNNING", "UNKNOWN"),
            (3, 12, "UNKNOWN", "UNKNOWN"),
            (4, 12, "SUCCEEDED", "SUCCEEDED"),
            (4, 15, "RUNNING", "RUNNING"),
            (3, 15, "RUNNING", "EXPIRED"),
            (4, 16, "RUNNING", "UNKNOWN"),
            (4, 17, "RUNNING", "UNKNOWN"),
            (4, 18, "RUNNING", "UNKNOWN"),
            (4, 19, "RUNNING", "UNKNOWN"),
        ] {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,lease_until) VALUES($1,'test','test',$2,$3,$4,$5,$6,now()+interval '1 hour',now()+interval '10 seconds')")
                .bind(id).bind(actor).bind(operation).bind(id.to_string()).bind(state).bind(generation).execute(&db).await.unwrap();
            records.push((id, generation, operation, state, expected));
        }
        for _ in 0..2 {
            let mut tx = db.begin().await.unwrap();
            on_connect(&mut tx, "test", 4).await.unwrap();
            tx.commit().await.unwrap();
        }
        for (id, generation, operation, previous, expected) in records {
            let row = sqlx::query("SELECT state,error,lease_until FROM mgmt_tasks WHERE id=$1")
                .bind(id)
                .fetch_one(&db)
                .await
                .unwrap();
            assert_eq!(row.get::<String, _>("state"), expected);
            if previous == "RUNNING" {
                assert!(
                    row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("lease_until")
                        .is_none()
                );
            }
            if previous == "RUNNING" && yofc_management_protocol::has_side_effect(operation) {
                assert_eq!(row.get::<String, _>("error"), "OUTCOME_UNKNOWN");
            }
            if generation == 3 && expected == "EXPIRED" {
                assert_eq!(row.get::<String, _>("error"), "BINDING_GENERATION_CHANGED");
            }
        }
        let events: i64 = sqlx::query_scalar("SELECT count(*) FROM mgmt_events WHERE kind='task'")
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(events, 9);
        db.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
    #[tokio::test]
    #[ignore = "requires explicit isolated PostgreSQL URL; creates a disposable schema"]
    async fn postgres_boot_outcome_checks_original_encrypted_admission_and_commits_one_audit() {
        use prost::Message;
        use sha2::{Digest, Sha256};
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("boot_outcome_{}", Uuid::new_v4().simple());
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
        let actor = Uuid::new_v4();
        let epoch = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'test','boot','unused','admin')").bind(actor).execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('boot','test','boot','unused',$1,1)").bind(epoch).execute(&db).await.unwrap();
        let keydir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(
            keydir.path(),
            std::os::unix::fs::PermissionsExt::from_mode(0o700),
        )
        .unwrap();
        let key = keydir.path().join("key");
        crate::task_crypto::create_key(&key).unwrap();
        let cipher = TaskCipher::load(&key).unwrap();
        let mut records = vec![];
        for op in 16..=19 {
            let id = Uuid::new_v4();
            let expires = chrono::Utc::now() - chrono::Duration::seconds(30);
            let action = matches!(op, 17 | 18).then(|| OrdinaryBootAction {
                transaction_id: format!("boot-{}", Uuid::new_v4()),
                confirmation_token: Uuid::new_v4().to_string(),
            });
            let command = ReadCommand {
                command_id: id.to_string(),
                operation: op,
                idempotency_key: id.to_string(),
                expires_at_unix_ms: expires.timestamp_millis(),
                ordinary_boot_action: action.clone(),
                ..Default::default()
            };
            sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,dispatch_epoch,input) VALUES($1,'boot','test',$2,$3,$4,'UNKNOWN',1,$5,$6,$7)").bind(id).bind(actor).bind(op).bind(id.to_string()).bind(expires).bind(epoch).bind(json!({"ordinaryBootAction":action})).execute(&db).await.unwrap();
            let txn = action
                .as_ref()
                .map(|a| a.transaction_id.clone())
                .unwrap_or_else(|| format!("boot-{}", Uuid::new_v4()));
            let value = json!({"id":txn,"state":match op{16=>"BOOT_POLICY_PREPARED",18=>"CANCELLED",_=>"BOOT_POLICY_COMMITTED"},"scope":"administrator-reviewed-native-boot-entries","bootVerified":false,"kuraRecoveryVerified":false});
            let mut data = if op == 16 {
                json!({"plan":value,"confirmToken":Uuid::new_v4().to_string()})
            } else {
                value
            };
            data["reconciliation"] = json!({"source":"ordinary.boot.journal.v1","commandId":id,"operation":op,"transactionId":txn,"requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"journalSha256":"a".repeat(64),"decisionSha256":"b".repeat(64)});
            records.push((id, data));
        }
        assert_eq!(cipher.prepare_storage(&db).await.unwrap(), 4);
        for (id, data) in records {
            let mut bad = data.clone();
            bad["reconciliation"]["requestSha256"] = json!("c".repeat(64));
            let receipt = |v: &Value| TaskResult {
                command_id: id.to_string(),
                state: TaskState::Succeeded as i32,
                error: 0,
                result_json: serde_json::to_vec(v).unwrap(),
            };
            assert!(
                apply_result(&db, &cipher, "boot", "test", epoch, receipt(&bad))
                    .await
                    .is_err()
            );
            assert_eq!(
                sqlx::query_scalar::<_, String>("SELECT state FROM mgmt_tasks WHERE id=$1")
                    .bind(id)
                    .fetch_one(&db)
                    .await
                    .unwrap(),
                "UNKNOWN"
            );
            for _ in 0..2 {
                apply_result(&db, &cipher, "boot", "test", epoch, receipt(&data))
                    .await
                    .unwrap();
            }
            let row = sqlx::query(
                "SELECT id,tenant_id,gateway_id,operation,result,state FROM mgmt_tasks WHERE id=$1",
            )
            .bind(id)
            .fetch_one(&db)
            .await
            .unwrap();
            assert_eq!(row.get::<String, _>("state"), "SUCCEEDED");
            assert_eq!(cipher.open_row(&row, "result").unwrap(), data);
            assert!(row.get::<Value, _>("result").get("ciphertext").is_some());
        }
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM mgmt_audit WHERE operation='task.reconcile.ordinary_boot'"
            )
            .fetch_one(&db)
            .await
            .unwrap(),
            4
        );
        db.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}

#[cfg(test)]
mod handoff_tests {
    use super::*;
    fn completed() -> Value {
        let id = format!("handoff-{}", Uuid::new_v4());
        let profile = format!("profile-{}", Uuid::new_v4());
        json!({"id":id,"profileId":profile,"state":"ORDINARY_ACTIVE_APPROVED_CHECKS","ordinaryUnitHandedOff":true,
            "scope":"administrator-selected-checks","normalBootVerified":false,"kuraRecoveryVerified":false,
            "report":{"schema":2,"ready":true,"ordinaryLoad":true,"processId":123}})
    }
    #[test]
    fn handoff_success_never_certifies_full_recovery_or_accepts_a_prepare_as_execution() {
        let good = completed();
        assert!(handoff_result(&good, 22));
        assert!(!handoff_result(&good, 21));
        for (key, value) in [
            ("kuraRecoveryVerified", json!(true)),
            ("normalBootVerified", json!(true)),
            ("ordinaryUnitHandedOff", json!(false)),
            ("state", json!("RECOVERY_REQUIRED")),
            ("scope", json!("full-recovery")),
        ] {
            let mut bad = good.clone();
            bad[key] = value;
            assert!(!handoff_result(&bad, 22));
        }
        for (key, value) in [
            ("schema", json!(1)),
            ("ready", json!(false)),
            ("ordinaryLoad", json!(false)),
            ("processId", json!(0)),
        ] {
            let mut bad = good.clone();
            bad["report"][key] = value;
            assert!(!handoff_result(&bad, 22));
        }
        assert!(!handoff_result(&json!({"canPrepare":true}), 20));
    }
    #[tokio::test]
    #[ignore = "requires an explicit isolated PostgreSQL database"]
    async fn postgres_handoff_late_receipt_is_encrypted_bound_and_never_requeued() {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("handoff_{}", Uuid::new_v4().simple());
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
        let actor = Uuid::new_v4();
        let epoch = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'test','handoff','unused','admin')").bind(actor).execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('handoff','test','handoff','unused',$1,1)").bind(epoch).execute(&db).await.unwrap();
        let keydir = tempfile::tempdir().unwrap().keep();
        std::fs::set_permissions(&keydir, std::os::unix::fs::PermissionsExt::from_mode(0o700))
            .unwrap();
        let key = keydir.join("key");
        crate::task_crypto::create_key(&key).unwrap();
        let cipher = TaskCipher::load(&key).unwrap();
        let id = Uuid::new_v4();
        let value = completed();
        let context = TaskContext {
            id,
            tenant: "test".into(),
            gateway: "handoff".into(),
            operation: 22,
        };
        let input=cipher.seal(&context,"input",&json!({"ordinaryHandoffAction":{"transactionId":value["id"],"confirmationToken":Uuid::new_v4().to_string()}})).unwrap();
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,lease_until,dispatch_epoch,input) VALUES($1,'handoff','test',$2,22,$3,'RUNNING',1,now()-interval '30 seconds',now()-interval '20 seconds',$4,$5)").bind(id).bind(actor).bind(id.to_string()).bind(epoch).bind(input).execute(&db).await.unwrap();
        assert_eq!(
            crate::http::find_task_intent(&db, "test", actor, "handoff", &id.to_string())
                .await
                .unwrap(),
            Some(id)
        );
        for (tenant, user, gateway, key) in [
            ("other", actor, "handoff", id.to_string()),
            ("test", Uuid::new_v4(), "handoff", id.to_string()),
            ("test", actor, "other", id.to_string()),
            ("test", actor, "handoff", Uuid::new_v4().to_string()),
        ] {
            assert_eq!(
                crate::http::find_task_intent(&db, tenant, user, gateway, &key)
                    .await
                    .unwrap(),
                None
            );
        }
        let mut tx = db.begin().await.unwrap();
        expire_tasks(&mut tx).await.unwrap();
        on_connect(&mut tx, "handoff", 1).await.unwrap();
        tx.commit().await.unwrap();
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(id)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "UNKNOWN");
        let make = |v: &Value| TaskResult {
            command_id: id.to_string(),
            state: TaskState::Succeeded as i32,
            error: 0,
            result_json: serde_json::to_vec(v).unwrap(),
        };
        let mut bad = value.clone();
        bad["id"] = json!(format!("handoff-{}", Uuid::new_v4()));
        assert!(
            apply_result(&db, &cipher, "handoff", "test", epoch, make(&bad))
                .await
                .is_err()
        );
        let mut bad = value.clone();
        bad["kuraRecoveryVerified"] = json!(true);
        assert!(
            apply_result(&db, &cipher, "handoff", "test", epoch, make(&bad))
                .await
                .is_err()
        );
        for _ in 0..2 {
            apply_result(&db, &cipher, "handoff", "test", epoch, make(&value))
                .await
                .unwrap();
        }
        let row = sqlx::query(
            "SELECT id,tenant_id,gateway_id,operation,state,result FROM mgmt_tasks WHERE id=$1",
        )
        .bind(id)
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(row.get::<String, _>("state"), "SUCCEEDED");
        assert_eq!(cipher.open_row(&row, "result").unwrap(), value);
        assert!(
            !row.get::<Value, _>("result")
                .to_string()
                .contains("ORDINARY_ACTIVE_APPROVED_CHECKS")
        );
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE target=$1 AND operation='task.reconcile.receipt'").bind(id.to_string()).fetch_one(&db).await.unwrap();
        assert_eq!(count, 1);
        // Retain the test schema and key location through the test evidence root.
        println!(
            "retained PostgreSQL schema {schema}; key {}; exact command {id}",
            key.display()
        );
        db.close().await;
        admin.close().await;
    }
    #[tokio::test]
    #[ignore = "requires an explicit isolated PostgreSQL database"]
    async fn postgres_handoff_proof_is_original_bound_encrypted_and_audited_once() {
        check_handoff_original_receipt(22).await;
    }
    #[tokio::test]
    #[ignore = "requires an explicit isolated PostgreSQL database"]
    async fn postgres_cancelled_handoff_proof_is_original_bound_encrypted_and_audited_once() {
        check_handoff_original_receipt(29).await;
    }
    async fn check_handoff_original_receipt(operation: i32) {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("handoff_{}", Uuid::new_v4().simple());
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
        let actor = Uuid::new_v4();
        let epoch = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'test','handoff','unused','admin')").bind(actor).execute(&db).await.unwrap();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,connection_epoch,binding_generation) VALUES('handoff','test','handoff','unused',$1,1)").bind(epoch).execute(&db).await.unwrap();
        let keydir = tempfile::tempdir().unwrap().keep();
        std::fs::set_permissions(&keydir, std::os::unix::fs::PermissionsExt::from_mode(0o700))
            .unwrap();
        let key = keydir.join("key");
        crate::task_crypto::create_key(&key).unwrap();
        let cipher = TaskCipher::load(&key).unwrap();
        let id = Uuid::new_v4();
        let mut value = completed();
        if operation == 29 {
            value = json!({"id":value["id"],"profileId":value["profileId"],"unit":"ordinary.service","state":"ORDINARY_CANCELLED","cancelled":true,"issued":false,"ordinaryUnitHandedOff":false,"scope":"administrator-selected-checks","kuraRecoveryVerified":false,"normalBootVerified":false});
        }
        let token = Uuid::new_v4().to_string();
        let context = TaskContext {
            id,
            tenant: "test".into(),
            gateway: "handoff".into(),
            operation,
        };
        let input=cipher.seal(&context,"input",&json!({"ordinaryHandoffAction":{"transactionId":value["id"],"confirmationToken":token}})).unwrap();
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,binding_generation,expires_at,lease_until,dispatch_epoch,input) VALUES($1,'handoff','test',$2,$6,$3,'RUNNING',1,now()-interval '30 seconds',now()-interval '20 seconds',$4,$5)").bind(id).bind(actor).bind(id.to_string()).bind(epoch).bind(input).bind(operation).execute(&db).await.unwrap();
        assert_eq!(
            crate::http::find_task_intent(&db, "test", actor, "handoff", &id.to_string())
                .await
                .unwrap(),
            Some(id)
        );
        for (tenant, user, gateway, key) in [
            ("other", actor, "handoff", id.to_string()),
            ("test", Uuid::new_v4(), "handoff", id.to_string()),
            ("test", actor, "other", id.to_string()),
            ("test", actor, "handoff", Uuid::new_v4().to_string()),
        ] {
            assert_eq!(
                crate::http::find_task_intent(&db, tenant, user, gateway, &key)
                    .await
                    .unwrap(),
                None
            );
        }
        let expires: chrono::DateTime<chrono::Utc> =
            sqlx::query_scalar("SELECT expires_at FROM mgmt_tasks WHERE id=$1")
                .bind(id)
                .fetch_one(&db)
                .await
                .unwrap();
        let command = ReadCommand {
            command_id: id.to_string(),
            operation,
            idempotency_key: id.to_string(),
            expires_at_unix_ms: expires.timestamp_millis(),
            ordinary_handoff_action: Some(OrdinaryHandoffAction {
                transaction_id: value["id"].as_str().unwrap().into(),
                confirmation_token: token,
            }),
            ..Default::default()
        };
        use prost::Message;
        use sha2::{Digest, Sha256};
        value["reconciliation"] = json!({"source":"ordinary.handoff.journal.v1","commandId":id.to_string(),"operation":operation,"requestSha256":format!("{:x}",Sha256::digest(command.encode_to_vec())),"transactionId":value["id"],"journalSha256":"a".repeat(64),"decisionSha256":"b".repeat(64)});
        let mut tx = db.begin().await.unwrap();
        expire_tasks(&mut tx).await.unwrap();
        on_connect(&mut tx, "handoff", 1).await.unwrap();
        tx.commit().await.unwrap();
        let state: String = sqlx::query_scalar("SELECT state FROM mgmt_tasks WHERE id=$1")
            .bind(id)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(state, "UNKNOWN");
        let make = |v: &Value| TaskResult {
            command_id: id.to_string(),
            state: TaskState::Succeeded as i32,
            error: 0,
            result_json: serde_json::to_vec(v).unwrap(),
        };
        let mut bad = value.clone();
        bad["id"] = json!(format!("handoff-{}", Uuid::new_v4()));
        assert!(
            apply_result(&db, &cipher, "handoff", "test", epoch, make(&bad))
                .await
                .is_err()
        );
        let mut bad = value.clone();
        bad["kuraRecoveryVerified"] = json!(true);
        assert!(
            apply_result(&db, &cipher, "handoff", "test", epoch, make(&bad))
                .await
                .is_err()
        );
        for field in [
            "requestSha256",
            "decisionSha256",
            "journalSha256",
            "operation",
            "commandId",
            "transactionId",
        ] {
            let mut bad = value.clone();
            bad["reconciliation"][field] = json!("wrong");
            assert!(
                apply_result(&db, &cipher, "handoff", "test", epoch, make(&bad))
                    .await
                    .is_err()
            );
        }
        if operation == 29 {
            for (field, flag) in [
                ("issued", true),
                ("cancelled", false),
                ("ordinaryUnitHandedOff", true),
            ] {
                let mut bad = value.clone();
                bad[field] = json!(flag);
                assert!(
                    apply_result(&db, &cipher, "handoff", "test", epoch, make(&bad))
                        .await
                        .is_err()
                );
            }
        }
        for _ in 0..2 {
            apply_result(&db, &cipher, "handoff", "test", epoch, make(&value))
                .await
                .unwrap();
        }
        let row = sqlx::query(
            "SELECT id,tenant_id,gateway_id,operation,state,result FROM mgmt_tasks WHERE id=$1",
        )
        .bind(id)
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(row.get::<String, _>("state"), "SUCCEEDED");
        assert_eq!(cipher.open_row(&row, "result").unwrap(), value);
        assert!(
            !row.get::<Value, _>("result")
                .to_string()
                .contains(if operation == 29 {
                    "ORDINARY_CANCELLED"
                } else {
                    "ORDINARY_ACTIVE_APPROVED_CHECKS"
                })
        );
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE target=$1 AND operation='task.reconcile.ordinary_handoff'").bind(id.to_string()).fetch_one(&db).await.unwrap();
        assert_eq!(count, 1);
        // Retain the test schema and key location through the test evidence root.
        println!(
            "retained PostgreSQL schema {schema}; key {}; exact command {id}",
            key.display()
        );
        db.close().await;
        admin.close().await;
    }
}
