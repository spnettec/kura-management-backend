//! Center admission/receipt boundary for remote joint NM/firewall operations.
//! The agent coordinator owns system semantics; no receipt or query authorizes
//! a replay. Mirrors `firewall_tasks.rs` with the NM projection shapes.
use anyhow::{Result, ensure};
use prost::Message;
use serde_json::Value;
use sha2::{Digest, Sha256};
use yofc_management_protocol::{nm::Change, v1::*};

pub(crate) fn wire_change(value: &Value) -> Result<NmChange> {
    let change: Change = serde_json::from_value(value.clone())?;
    change
        .to_wire()
        .ok_or_else(|| anyhow::anyhow!("INVALID_ARGUMENT"))
}
pub(crate) fn command(
    id: String,
    operation: i32,
    key: String,
    expires: i64,
    payload: &Value,
) -> Result<ReadCommand> {
    ensure!(matches!(operation, 34..=37), "INVALID_ARGUMENT");
    let fields = payload
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("INVALID_ARGUMENT"))?;
    ensure!(
        fields
            .iter()
            .all(|(k, v)| v.is_null() || matches!(k.as_str(), "nmChange" | "nmAction")),
        "INVALID_ARGUMENT"
    );
    let command = ReadCommand {
        command_id: id,
        operation,
        idempotency_key: key,
        expires_at_unix_ms: expires,
        nm_change: payload
            .get("nmChange")
            .filter(|v| !v.is_null())
            .map(wire_change)
            .transpose()?,
        nm_action: payload
            .get("nmAction")
            .filter(|v| !v.is_null())
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()?,
        ..Default::default()
    };
    ensure!(command.valid_payload(), "INVALID_ARGUMENT");
    Ok(command)
}
fn digest(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 64
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn identity(v: &Value) -> bool {
    v.as_str()
        .is_some_and(|s| uuid::Uuid::parse_str(s).is_ok_and(|u| u.to_string() == s))
}
fn token(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 32
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn keys(v: &Value, allowed: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.keys().all(|k| allowed.contains(&k.as_str())))
}
fn state_name(v: &Value) -> bool {
    v.as_str().is_some_and(|v| {
        matches!(
            v,
            "PREPARED"
                | "CHECKPOINT_INTENT"
                | "CHECKPOINT_CREATED"
                | "PROFILE_INTENT"
                | "PROFILE_APPLIED"
                | "FIREWALL_INTENT"
                | "WAIT_CONFIRM"
                | "CONFIRM_DECIDED"
                | "ROLLBACK_DECIDED"
                | "ROLLBACK_RUNNING"
                | "RECOVERY_REQUIRED"
                | "COMMITTED"
                | "ROLLED_BACK"
                | "CANCELLED"
        )
    })
}
pub(crate) fn valid_receipt(
    data: &Value,
    command: &ReadCommand,
    state: TaskState,
    error: ErrorCode,
) -> bool {
    if !matches!(command.operation, 34..=37) || !command.valid_payload() {
        return false;
    }
    if !match state {
        TaskState::Succeeded | TaskState::Running => error == ErrorCode::Unspecified,
        TaskState::Unknown => error == ErrorCode::OutcomeUnknown,
        TaskState::Failed => !matches!(error, ErrorCode::Unspecified | ErrorCode::OutcomeUnknown),
        _ => false,
    } {
        return false;
    }
    let proof = data.get("reconciliation");
    if state != TaskState::Succeeded && proof.is_none() {
        // Definite preflight rejection, in-flight identity or uncertainty may
        // carry only bounded diagnostics; none can smuggle a token/result.
        return matches!(
            state,
            TaskState::Failed | TaskState::Unknown | TaskState::Running
        ) && keys(data, &["code", "reason"])
            && data
                .as_object()
                .unwrap()
                .values()
                .all(|v| v.as_str().is_some_and(|s| s.len() <= 256))
            && data.get("code").is_none_or(|v| {
                v == error.as_str_name() || (state == TaskState::Unknown && v == "INTERNAL")
            });
    }
    if command.operation == 34 {
        if proof.is_some()
            || state != TaskState::Succeeded
            || error != ErrorCode::Unspecified
            || !keys(
                data,
                &[
                    "schemaVersion",
                    "backend",
                    "writeEnabled",
                    "sourceDigest",
                    "candidateDigest",
                    "confirmationReady",
                    "transaction",
                ],
            )
            || data["schemaVersion"] != 1
            || data["backend"] != "NetworkManager"
            || !data["writeEnabled"].is_boolean()
            || !digest(&data["sourceDigest"])
            || !data["candidateDigest"].is_null() && !digest(&data["candidateDigest"])
            || !data["confirmationReady"].is_boolean()
        {
            return false;
        }
        let t = &data["transaction"];
        if data["confirmationReady"] == true && (data["writeEnabled"] != true || t["state"] != "WAIT_CONFIRM") {
            return false;
        }
        return t.is_null()
            || (keys(t, &["id", "state", "deadlineUnixMs"])
                && identity(&t["id"])
                && state_name(&t["state"])
                && (t["deadlineUnixMs"].as_u64().is_some_and(|v| v > 0)
                    || (t["deadlineUnixMs"].is_null() && t["state"] != "WAIT_CONFIRM")));
    }
    if !keys(
        data,
        &[
            "id",
            "state",
            "deadlineUnixMs",
            "requestId",
            "newManagementConnectionRequired",
            "newManagementConnectionVerified",
            "confirmationToken",
            "candidateDigest",
            "operationId",
            "reconciliation",
        ],
    ) || !identity(&data["id"])
        || data["id"] != data["operationId"]
        || data["newManagementConnectionRequired"] != true
        || !data["newManagementConnectionVerified"].is_boolean()
        || data["deadlineUnixMs"].as_u64().is_none_or(|v| v == 0)
    {
        return false;
    }
    let prepare = command.operation == 35;
    if prepare {
        if data["requestId"] != command.command_id {
            return false;
        }
        if state == TaskState::Succeeded {
            if command.nm_change.is_none() {
                return false;
            }
            // The candidateDigest binds what the agent compiled under its
            // shared guard from the proposal this center submitted; the center
            // cannot recompile NM proposals, so only the shape is checked.
            if !token(&data["confirmationToken"]) || !digest(&data["candidateDigest"]) {
                return false;
            }
        } else if data.get("confirmationToken").is_some() {
            return false;
        }
    } else if command
        .nm_action
        .as_ref()
        .is_none_or(|a| data["id"] != a.operation_id)
        || data.get("confirmationToken").is_some()
    {
        return false;
    }
    let settled = match (command.operation, state) {
        (35, TaskState::Succeeded) => {
            data["state"] == "WAIT_CONFIRM" || data["state"] == "COMMITTED"
        }
        (36, TaskState::Succeeded) => {
            data["state"] == "COMMITTED" && data["newManagementConnectionVerified"] == true
        }
        (37, TaskState::Succeeded) => data["state"] == "ROLLED_BACK",
        (35 | 36, TaskState::Failed) => {
            proof.is_some()
                && error == ErrorCode::Conflict
                && (data["state"] == "ROLLED_BACK"
                    // A cancelled prepare never reached the backend: the
                    // original command resolves to a definite, retryable
                    // failure instead of staying UNKNOWN forever.
                    || (data["state"] == "CANCELLED" && command.operation == 35))
        }
        _ => false,
    };
    if !settled {
        return false;
    }
    if let Some(p) = proof {
        p["source"] == "nm.coordinator.journal.v1"
            && p["commandId"] == command.command_id
            && p["operation"] == command.operation
            && p["operationId"] == data["id"]
            && p["requestSha256"] == format!("{:x}", Sha256::digest(command.encode_to_vec()))
            && digest(&p["journalSha256"])
            && p["observedAtUnixMs"].as_u64().is_some_and(|v| v > 0)
    } else {
        state == TaskState::Succeeded && error == ErrorCode::Unspecified
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn prepare_command() -> ReadCommand {
        let change = Change {
            source_digest: "a".repeat(64),
            confirm_seconds: 60,
            changes: json!([{"interface":"eth0"}]),
            nat: None,
        };
        ReadCommand {
            command_id: uuid::Uuid::new_v4().to_string(),
            operation: 35,
            idempotency_key: "once".into(),
            expires_at_unix_ms: 123,
            nm_change: change.to_wire(),
            ..Default::default()
        }
    }
    fn action_command(operation: i32) -> ReadCommand {
        ReadCommand {
            command_id: uuid::Uuid::new_v4().to_string(),
            operation,
            idempotency_key: "once".into(),
            expires_at_unix_ms: 123,
            nm_action: Some(NmAction {
                operation_id: uuid::Uuid::new_v4().to_string(),
                confirmation_token: "a".repeat(32),
            }),
            ..Default::default()
        }
    }
    fn projection(c: &ReadCommand, state: &str, verified: bool) -> Value {
        json!({"id":c.nm_action.as_ref().map(|a|a.operation_id.clone()).unwrap_or_default(),
            "operationId":c.nm_action.as_ref().map(|a|a.operation_id.clone()).unwrap_or_default(),
            "state":state,"deadlineUnixMs":100,"requestId":uuid::Uuid::new_v4().to_string(),
            "newManagementConnectionRequired":true,"newManagementConnectionVerified":verified})
    }
    #[test]
    fn command_accepts_only_the_nm_family_with_matching_payloads() {
        let change = Change {
            source_digest: "a".repeat(64),
            confirm_seconds: 60,
            changes: json!([{"interface":"eth0"}]),
            nat: None,
        };
        let payload = json!({"nmChange":change});
        let c = command(uuid::Uuid::new_v4().to_string(), 35, "k".into(), 1, &payload).unwrap();
        assert_eq!(c.operation, 35);
        assert!(c.nm_change.is_some());
        assert!(command(uuid::Uuid::new_v4().to_string(), 30, "k".into(), 1, &payload).is_err());
        let mut foreign = payload.clone();
        // Null-valued foreign keys are tolerated (same grammar as firewall);
        // a non-null foreign payload never crosses.
        foreign["firewallChange"] = json!({"policy":"accept"});
        assert!(command(uuid::Uuid::new_v4().to_string(), 35, "k".into(), 1, &foreign).is_err());
        // Actions require the action payload and nothing else.
        let action_payload = json!({"nmAction":NmAction{operation_id:uuid::Uuid::new_v4().to_string(),confirmation_token:"a".repeat(32)}});
        assert!(command(uuid::Uuid::new_v4().to_string(), 36, "k".into(), 1, &action_payload).is_ok());
        assert!(command(uuid::Uuid::new_v4().to_string(), 37, "k".into(), 1, &action_payload).is_ok());
        assert!(command(uuid::Uuid::new_v4().to_string(), 35, "k".into(), 1, &action_payload).is_err());
    }
    #[test]
    fn prepare_receipt_binds_request_and_rejects_token_leaks() {
        let c = prepare_command();
        let good = json!({"id":uuid::Uuid::new_v4().to_string(),"operationId":json!(null),
            "state":"WAIT_CONFIRM","deadlineUnixMs":150,"requestId":c.command_id,
            "newManagementConnectionRequired":true,"newManagementConnectionVerified":false,
            "confirmationToken":"b".repeat(32),"candidateDigest":"c".repeat(64)});
        let mut good = good;
        good["operationId"] = good["id"].clone();
        assert!(valid_receipt(&good, &c, TaskState::Succeeded, ErrorCode::Unspecified));
        assert!(!valid_receipt(&good, &c, TaskState::Failed, ErrorCode::InvalidArgument));
        for (key, v) in [
            ("requestId", json!("wrong")),
            ("confirmationToken", json!(uuid::Uuid::new_v4().to_string())),
            ("candidateDigest", json!("zz")),
            ("newManagementConnectionRequired", json!(false)),
        ] {
            let mut bad = good.clone();
            bad[key] = v;
            assert!(!valid_receipt(&bad, &c, TaskState::Succeeded, ErrorCode::Unspecified), "{key}");
        }
        // Non-success receipts may not carry the token.
        let mut leaked = good.clone();
        leaked["state"] = json!("ROLLED_BACK");
        assert!(!valid_receipt(&leaked, &c, TaskState::Failed, ErrorCode::Conflict));
    }
    #[test]
    fn confirm_receipt_requires_verified_new_connection_and_rollback_rolls_back() {
        let c = action_command(36);
        let good = projection(&c, "COMMITTED", true);
        assert!(valid_receipt(&good, &c, TaskState::Succeeded, ErrorCode::Unspecified));
        let mut unverified = good.clone();
        unverified["newManagementConnectionVerified"] = json!(false);
        assert!(!valid_receipt(&unverified, &c, TaskState::Succeeded, ErrorCode::Unspecified));
        let rollback = action_command(37);
        let mut rolled = projection(&rollback, "ROLLED_BACK", false);
        assert!(valid_receipt(&rolled, &rollback, TaskState::Succeeded, ErrorCode::Unspecified));
        // A confirm-shaped receipt can never validate a rollback command.
        rolled["state"] = json!("COMMITTED");
        rolled["newManagementConnectionVerified"] = json!(true);
        assert!(!valid_receipt(&rolled, &rollback, TaskState::Succeeded, ErrorCode::Unspecified));
    }
    #[test]
    fn reconciliation_binds_the_original_request_and_journal() {
        let c = action_command(36);
        let mut good = projection(&c, "COMMITTED", true);
        good["reconciliation"] = json!({"source":"nm.coordinator.journal.v1","commandId":c.command_id,
            "operation":c.operation,"operationId":good["id"],"requestSha256":format!("{:x}",Sha256::digest(c.encode_to_vec())),
            "journalSha256":"a".repeat(64),"observedAtUnixMs":5});
        assert!(valid_receipt(&good, &c, TaskState::Succeeded, ErrorCode::Unspecified));
        for field in ["commandId", "requestSha256", "operationId", "source"] {
            let mut bad = good.clone();
            bad["reconciliation"][field] = json!("wrong");
            assert!(!valid_receipt(&bad, &c, TaskState::Succeeded, ErrorCode::Unspecified), "{field}");
        }
    }
    #[test]
    fn cancelled_prepare_receipt_resolves_the_original_command_as_failed() {
        let c = prepare_command();
        let mut receipt = json!({"id":uuid::Uuid::new_v4().to_string(),"operationId":json!(null),
            "state":"CANCELLED","deadlineUnixMs":100,"requestId":c.command_id,
            "newManagementConnectionRequired":true,"newManagementConnectionVerified":false});
        receipt["operationId"] = receipt["id"].clone();
        receipt["reconciliation"] = json!({"source":"nm.coordinator.journal.v1","commandId":c.command_id,
            "operation":35,"operationId":receipt["id"],"requestSha256":format!("{:x}",Sha256::digest(c.encode_to_vec())),
            "journalSha256":"a".repeat(64),"observedAtUnixMs":5});
        assert!(valid_receipt(&receipt, &c, TaskState::Failed, ErrorCode::Conflict));
        // A successful CANCELLED claim is always contradictory.
        assert!(!valid_receipt(&receipt, &c, TaskState::Succeeded, ErrorCode::Unspecified));
    }

    #[test]
    fn view_receipt_is_read_only_and_never_carries_tokens() {
        let read = ReadCommand {
            operation: 34,
            ..Default::default()
        };
        let view = json!({"schemaVersion":1,"backend":"NetworkManager","writeEnabled":true,
            "sourceDigest":"a".repeat(64),"candidateDigest":serde_json::Value::Null,
            "confirmationReady":false,"transaction":null});
        assert!(valid_receipt(&view, &read, TaskState::Succeeded, ErrorCode::Unspecified));
        let mut bad = view.clone();
        bad["confirmationToken"] = json!("leak");
        assert!(!valid_receipt(&bad, &read, TaskState::Succeeded, ErrorCode::Unspecified));
        bad = view.clone();
        bad["writeEnabled"] = json!(false);
        assert!(valid_receipt(&bad, &read, TaskState::Succeeded, ErrorCode::Unspecified));
    }
    #[test]
    fn readonly_remote_contract_accepts_boolean_false_without_confirmation_authority() {
        let read=ReadCommand{operation:34,..Default::default()};
        let mut view=json!({"schemaVersion":1,"backend":"NetworkManager","writeEnabled":false,
            "sourceDigest":"a".repeat(64),"candidateDigest":null,"confirmationReady":false,"transaction":null});
        assert!(valid_receipt(&view,&read,TaskState::Succeeded,ErrorCode::Unspecified));
        for wrong in [json!(null),json!(0),json!("false"),json!([])] {
            view["writeEnabled"]=wrong;
            assert!(!valid_receipt(&view,&read,TaskState::Succeeded,ErrorCode::Unspecified));
        }
        view["writeEnabled"]=json!(false);view["confirmationReady"]=json!(true);
        assert!(!valid_receipt(&view,&read,TaskState::Succeeded,ErrorCode::Unspecified));
    }

    #[test]
    fn readonly_remote_contract_retained_states_have_no_fabricated_deadline_or_tokens() {
        let command=ReadCommand{operation:34,..Default::default()};
        let mut view=json!({"schemaVersion":1,"backend":"NetworkManager","writeEnabled":false,
            "sourceDigest":"a".repeat(64),"candidateDigest":"b".repeat(64),"confirmationReady":false,
            "transaction":{"id":uuid::Uuid::new_v4().to_string(),"state":"PREPARED","deadlineUnixMs":null}});
        assert!(valid_receipt(&view,&command,TaskState::Succeeded,ErrorCode::Unspecified));
        view["transaction"]["state"]=json!("WAIT_CONFIRM");
        assert!(!valid_receipt(&view,&command,TaskState::Succeeded,ErrorCode::Unspecified));
        view["transaction"]["deadlineUnixMs"]=json!(10);
        assert!(valid_receipt(&view,&command,TaskState::Succeeded,ErrorCode::Unspecified));
        view["transaction"]["confirmationToken"]=json!("a".repeat(32));
        assert!(!valid_receipt(&view,&command,TaskState::Succeeded,ErrorCode::Unspecified));
    }

}
