//! Encrypted imported task admission and strict receipt ownership validation.
#[cfg(test)]
mod pg_tests;
use anyhow::{Context, Result, ensure};
use prost::Message;
use ring::rand::{SecureRandom, SystemRandom};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, Row};
use uuid::Uuid;
use yofc_management_protocol::{
    imported::{DecisionEvidence, Envelope, OriginalResolution, Request},
    v1::*,
};

pub(crate) fn materialize(payload: &mut Value, previous: Option<&Value>) -> Result<()> {
    let Some(value) = payload.get("importedFirewall").filter(|v| !v.is_null()) else {
        return Ok(());
    };
    let request: Request = serde_json::from_value(value["request"].clone())?;
    ensure!(request.valid(), "INVALID_ARGUMENT");
    let owner_key = if request.needs_owner_key() {
        if let Some(previous) = previous {
            let previous: Envelope = serde_json::from_value(previous["importedFirewall"].clone())?;
            ensure!(previous.valid(previous.request.operation()), "CONFLICT");
            Some(previous.owner_key.context("CONFLICT")?)
        } else {
            let mut secret = [0u8; 32];
            SystemRandom::new()
                .fill(&mut secret)
                .map_err(|_| anyhow::anyhow!("INTERNAL: owner key generation"))?;
            Some(secret.iter().map(|byte| format!("{byte:02x}")).collect())
        }
    } else {
        None
    };
    payload["importedFirewall"] = serde_json::to_value(Envelope { request, owner_key })?;
    Ok(())
}
pub(crate) fn envelope(payload: &Value, operation: i32) -> Result<Envelope> {
    let envelope: Envelope = serde_json::from_value(payload["importedFirewall"].clone())?;
    ensure!(envelope.valid(operation), "INVALID_ARGUMENT");
    Ok(envelope)
}
pub(crate) fn command(
    id: String,
    operation: i32,
    key: String,
    expires: i64,
    payload: &Value,
) -> Result<ReadCommand> {
    ensure!(
        payload.as_object().is_some_and(|fields| fields
            .iter()
            .all(|(key, value)| value.is_null() || key == "importedFirewall")),
        "INVALID_ARGUMENT"
    );
    let command = ReadCommand {
        command_id: id,
        operation,
        idempotency_key: key,
        expires_at_unix_ms: expires,
        imported_firewall: Some(envelope(payload, operation)?.to_wire()?),
        ..Default::default()
    };
    ensure!(command.valid_payload(), "INVALID_ARGUMENT");
    Ok(command)
}
fn digest(v: &Value) -> bool {
    v.as_str().is_some_and(|v| {
        v.len() == 64
            && v.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn secret(v: &Value) -> bool {
    match v {
        Value::Object(fields) => fields.iter().any(|(key, value)| {
            matches!(key.as_str(), "ownerKey" | "confirmationToken") || secret(value)
        }),
        Value::Array(values) => values.iter().any(secret),
        _ => false,
    }
}
pub(crate) fn valid_receipt(
    data: &Value,
    command: &ReadCommand,
    state: TaskState,
    error: ErrorCode,
) -> bool {
    if !matches!(command.operation, 38..=48) || !command.valid_payload() {
        return false;
    }
    if state != TaskState::Succeeded {
        return data == &serde_json::json!({})
            && match state {
                TaskState::Unknown => error == ErrorCode::OutcomeUnknown,
                TaskState::Failed | TaskState::Expired => {
                    !matches!(error, ErrorCode::Unspecified | ErrorCode::OutcomeUnknown)
                }
                TaskState::Running => error == ErrorCode::Unspecified,
                _ => false,
            };
    }
    let Ok(envelope) = Envelope::from_wire(
        command.imported_firewall.as_ref().unwrap(),
        command.operation,
    ) else {
        return false;
    };
    let expected = envelope
        .request
        .original_task()
        .map(|id| id.to_string())
        .or_else(|| {
            envelope
                .request
                .needs_owner_key()
                .then(|| command.command_id.clone())
        });
    if error != ErrorCode::Unspecified
        || secret(data)
        || data["schemaVersion"] != 1
        || data["operation"] != command.operation
        || !data.as_object().is_some_and(|fields| {
            fields.len() == 9
                && fields.keys().all(|key| {
                    matches!(
                        key.as_str(),
                        "schemaVersion"
                            | "operation"
                            | "originalTaskId"
                            | "originalCommandSha256"
                            | "writeEnabled"
                            | "confirmationReady"
                            | "newManagementConnectionVerified"
                            | "receipt"
                            | "decisionEvidence"
                    )
                })
        })
        || !data["writeEnabled"].is_boolean()
        || !data["confirmationReady"].is_boolean()
        || !data["newManagementConnectionVerified"].is_boolean()
        || data["originalTaskId"] != serde_json::json!(expected)
        || if expected.is_some() {
            !digest(&data["originalCommandSha256"])
        } else {
            !data["originalCommandSha256"].is_null()
        }
        || !data["receipt"].is_object()
    {
        return false;
    }
    if matches!(command.operation, 45 | 48) {
        if !serde_json::from_value::<DecisionEvidence>(data["decisionEvidence"].clone())
            .is_ok_and(|v| v.valid() && v.commands.is_some())
        {
            return false;
        }
    } else if !data["decisionEvidence"].is_null() {
        return false;
    }
    let receipt = &data["receipt"];
    if data["confirmationReady"] == true
        && (data["newManagementConnectionVerified"] != true
            || data["writeEnabled"] != true
            || receipt["state"] != "WAIT_CONFIRM"
            || receipt["deadlineExpired"] == true
            || receipt["deadlineUnixMs"].as_u64().is_none())
    {
        return false;
    }
    if command.operation == 42 && data["newManagementConnectionVerified"] != true {
        return false;
    }
    if matches!(command.operation, 46..=48) && data["newManagementConnectionVerified"] != false {
        return false;
    }
    match &envelope.request {
        Request::View => {
            receipt["sourceKind"] == "nft.imported.confirmed.v1"
                && digest(&receipt["sourceDigest"])
                && receipt["revision"].is_u64()
                && receipt["initialized"] == true
        }
        Request::Review { review } => {
            receipt["sourceDigest"] == review.source_digest
                && receipt["revision"] == review.expected_revision
                && digest(&receipt["cohortDigest"])
                && receipt["nativePlan"].is_object()
                && receipt["nativePlan"]["operationId"] == serde_json::json!(review.operation_id)
                && receipt["ownerGrant"] == false
        }
        Request::SourceReview { review } => {
            receipt["sourceDigest"] == review.source_digest
                && receipt["revision"] == review.expected_revision
                && digest(&receipt["cohortDigest"])
                && receipt["kind"] == "SOURCE_OBSERVATION"
                && receipt["kernelEffect"] == false
                && receipt["nativePlan"]["operationId"] == serde_json::json!(review.operation_id)
        }
        _ => {
            receipt["id"]
                .as_str()
                .is_some_and(|id| Uuid::parse_str(id).is_ok_and(|id| !id.is_nil()))
                && receipt["state"].as_str().is_some_and(|state| {
                    matches!(
                        state,
                        "PREPARED"
                            | "APPLYING"
                            | "WAIT_CONFIRM"
                            | "CONFIRMING"
                            | "COMMITTED"
                            | "ROLLING_BACK"
                            | "ROLLED_BACK"
                            | "REJECTING"
                            | "REJECTED"
                            | "RECOVERY_REQUIRED"
                            | "OBSERVING"
                            | "OBSERVED"
                    )
                })
                && receipt["ownerGrant"] == false
        }
    }
}
/// Rebuild the exact encrypted original command. A gateway's valid mTLS identity
/// alone cannot relabel another actor/generation/task's receipt as this result.
pub(crate) async fn check_original(
    connection: &mut PgConnection,
    cipher: &crate::task_crypto::TaskCipher,
    current: &sqlx::postgres::PgRow,
    data: &Value,
) -> Result<()> {
    let Some(id) = data["originalTaskId"].as_str() else {
        return Ok(());
    };
    let row = sqlx::query("SELECT id,tenant_id,gateway_id,operation,input,idempotency_key,expires_at,actor_id,binding_generation,state FROM mgmt_tasks WHERE id=$1 AND gateway_id=$2 AND tenant_id=$3 FOR UPDATE")
        .bind(Uuid::parse_str(id)?).bind(current.get::<String,_>("gateway_id")).bind(current.get::<String,_>("tenant_id"))
        .fetch_optional(&mut *connection).await?.context("CONFLICT: original task unavailable")?;
    ensure!(
        row.get::<Uuid, _>("actor_id") == current.get::<Uuid, _>("actor_id")
            && row.get::<i64, _>("binding_generation")
                == current.get::<i64, _>("binding_generation"),
        "FORBIDDEN: original task owner/generation"
    );
    let payload = cipher.open_row(&row, "input")?;
    let original = command(
        id.into(),
        row.get("operation"),
        row.get("idempotency_key"),
        row.get::<chrono::DateTime<chrono::Utc>, _>("expires_at")
            .timestamp_millis(),
        &payload,
    )?;
    let original_hash = format!("{:x}", Sha256::digest(original.encode_to_vec()));
    ensure!(
        data["originalCommandSha256"] == original_hash,
        "CONFLICT: original command hash"
    );
    let input = envelope(&payload, original.operation)?;
    let mut prepared = input.request.original_operation();
    let mut parent_hash = None;
    if let Some(parent) = input.request.original_task() {
        ensure!(
            matches!(original.operation, 41..=44),
            "CONFLICT: original action family"
        );
        let parent=sqlx::query("SELECT id,tenant_id,gateway_id,operation,input,idempotency_key,expires_at FROM mgmt_tasks WHERE id=$1 AND gateway_id=$2 AND tenant_id=$3 AND actor_id=$4 AND binding_generation=$5 AND operation=40")
            .bind(parent).bind(row.get::<String,_>("gateway_id")).bind(row.get::<String,_>("tenant_id"))
            .bind(row.get::<Uuid,_>("actor_id")).bind(row.get::<i64,_>("binding_generation"))
            .fetch_optional(&mut *connection).await?.context("CONFLICT: original preparation owner")?;
        let payload = cipher.open_row(&parent, "input")?;
        prepared = envelope(&payload, 40)?.request.original_operation();
        parent_hash = Some(format!(
            "{:x}",
            Sha256::digest(
                command(
                    parent.get::<Uuid, _>("id").to_string(),
                    40,
                    parent.get("idempotency_key"),
                    parent
                        .get::<chrono::DateTime<chrono::Utc>, _>("expires_at")
                        .timestamp_millis(),
                    &payload
                )?
                .encode_to_vec()
            )
        ));
    }
    ensure!(
        prepared.is_some() && data["receipt"]["id"] == serde_json::json!(prepared),
        "CONFLICT: original proposal receipt identity"
    );
    if matches!(current.get::<i32, _>("operation"), 45 | 48)
        && matches!(
            row.get::<String, _>("state").as_str(),
            "RUNNING" | "UNKNOWN"
        )
    {
        let evidence: DecisionEvidence = serde_json::from_value(data["decisionEvidence"].clone())?;
        ensure!(
            evidence.commands.as_ref().is_some_and(
                |c| c.original_sha256 == parent_hash.as_deref().unwrap_or(&original_hash)
            ),
            "CONFLICT: original record command binding"
        );
        if let Some(resolution) = evidence.resolution(
            original.operation,
            data["receipt"]["state"].as_str().unwrap_or(""),
            &original_hash,
            data["newManagementConnectionVerified"] == true,
        ) {
            let (state, error) = match resolution {
                OriginalResolution::Succeeded => ("SUCCEEDED", None),
                OriginalResolution::Failed => ("FAILED", Some("CONFLICT")),
            };
            let mut resolved = data.clone();
            resolved["operation"] = serde_json::json!(original.operation);
            resolved["originalTaskId"] = serde_json::json!(
                input
                    .request
                    .original_task()
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| original.command_id.clone())
            );
            if let Some(hash) = parent_hash {
                resolved["originalCommandSha256"] = serde_json::json!(hash);
            }
            resolved["reconciliation"] = serde_json::json!({"source":"firewall.imported.original.v1",
                "queryTaskId":current.get::<Uuid,_>("id"),"originalCommandSha256":data["originalCommandSha256"],
                "originalPhaseEntered":resolution==OriginalResolution::Succeeded});
            let encrypted = cipher.seal(
                &crate::task_crypto::TaskContext::from_row(&row)?,
                "result",
                &resolved,
            )?;
            let changed=sqlx::query("UPDATE mgmt_tasks SET state=$3,error=$4,result=$2,lease_until=NULL WHERE id=$1 AND state IN ('RUNNING','UNKNOWN')")
                .bind(row.get::<Uuid,_>("id")).bind(encrypted).bind(state).bind(error).execute(&mut *connection).await?.rows_affected();
            if changed == 1 {
                let tenant = row.get::<String, _>("tenant_id");
                let id = row.get::<Uuid, _>("id").to_string();
                crate::db::event(
                    &mut *connection,
                    &tenant,
                    "task",
                    &id,
                    0,
                    serde_json::json!({"state":state}),
                )
                .await?;
                crate::db::audit(
                    &mut *connection,
                    &tenant,
                    &row.get::<String, _>("gateway_id"),
                    "task.reconcile.imported",
                    &id,
                    state,
                )
                .await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn admission_generates_one_private_key_and_idempotency_reuses_exact_encrypted_input() {
        let request = serde_json::json!({"kind":"prepare","proposal":{"operationId":Uuid::new_v4(),"expectedRevision":0,
            "sourceDigest":"a".repeat(64),"cohortDigest":"b".repeat(64),"confirmSeconds":90,"candidate":{},"destinationPrefixes":{}}});
        let public = serde_json::json!({"importedFirewall":{"request":request}});
        let mut first = public.clone();
        materialize(&mut first, None).unwrap();
        assert_eq!(
            first["importedFirewall"]["ownerKey"]
                .as_str()
                .unwrap()
                .len(),
            64
        );
        let mut replay = public.clone();
        materialize(&mut replay, Some(&first)).unwrap();
        assert_eq!(replay, first);
        let mut other = public;
        materialize(&mut other, None).unwrap();
        assert_ne!(first, other);
    }
    #[test]
    fn original_query_receipt_rejects_cross_task_secret_leaks_and_actionable_terminal_state() {
        let id = Uuid::new_v4();
        let payload = serde_json::json!({"importedFirewall":{"request":{"kind":"outcome","originalTaskId":id}}});
        let command = command(
            Uuid::new_v4().to_string(),
            45,
            "query".into(),
            i64::MAX,
            &payload,
        )
        .unwrap();
        let data = serde_json::json!({"schemaVersion":1,"operation":45,"originalTaskId":id,"originalCommandSha256":"a".repeat(64),
            "writeEnabled":false,"confirmationReady":false,"newManagementConnectionVerified":false,"receipt":{"id":Uuid::new_v4(),"state":"COMMITTED","ownerGrant":false},
            "decisionEvidence":{"recordSha256":"b".repeat(64),"historical":true,"sourceObservation":false,"forwardIntent":true,
                "forwardAcknowledged":true,"inverseIntent":false,"inverseAcknowledged":false,"negativeAcknowledged":false,"confirmedDecision":true,"commands":{"originalSha256":"a".repeat(64),"forwardSha256":"c".repeat(64),
                    "inverseSha256":null,"confirmationSha256":"d".repeat(64),"cancellationSha256":null,"expiryDecision":false}}});
        assert!(valid_receipt(
            &data,
            &command,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        let mut wrong = data.clone();
        wrong["originalTaskId"] = serde_json::json!(Uuid::new_v4());
        assert!(!valid_receipt(
            &wrong,
            &command,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        let mut wrong = data.clone();
        wrong["receipt"]["confirmationToken"] = serde_json::json!("private");
        assert!(!valid_receipt(
            &wrong,
            &command,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        let mut wrong = data;
        wrong["confirmationReady"] = serde_json::json!(true);
        wrong["writeEnabled"] = serde_json::json!(true);
        assert!(!valid_receipt(
            &wrong,
            &command,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
    }
}
