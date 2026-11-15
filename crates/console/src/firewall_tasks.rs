//! Center admission/receipt boundary for remote firewall operations. The agent
//! compiler owns system semantics; no receipt or query authorizes a replay.
use anyhow::{Result, ensure};
use prost::Message;
use serde_json::Value;
use sha2::{Digest, Sha256};
use yofc_management_protocol::{
    firewall::{Candidate, Change},
    v1::*,
};

pub(crate) fn wire_change(value: &Value) -> Result<FirewallChange> {
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
    ensure!(matches!(operation, 30..=33), "INVALID_ARGUMENT");
    let fields = payload
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("INVALID_ARGUMENT"))?;
    ensure!(
        fields
            .iter()
            .all(|(k, v)| v.is_null() || matches!(k.as_str(), "firewallChange" | "firewallAction")),
        "INVALID_ARGUMENT"
    );
    let command = ReadCommand {
        command_id: id,
        operation,
        idempotency_key: key,
        expires_at_unix_ms: expires,
        firewall_change: payload
            .get("firewallChange")
            .filter(|v| !v.is_null())
            .map(wire_change)
            .transpose()?,
        firewall_action: payload
            .get("firewallAction")
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
    v.as_str().is_some_and(|s| {
        s.strip_prefix("fw-")
            .is_some_and(|s| uuid::Uuid::parse_str(s).is_ok_and(|u| u.to_string() == s))
    })
}
fn keys(v: &Value, allowed: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.keys().all(|k| allowed.contains(&k.as_str())))
}
fn revision(v: &Value) -> bool {
    v.as_u64().is_some_and(|v| v <= i64::MAX as u64)
}
fn state_name(v: &Value) -> bool {
    v.as_str().is_some_and(|v| {
        matches!(
            v,
            "PREPARED"
                | "APPLYING"
                | "WAIT_CONFIRM"
                | "CONFIRMING"
                | "COMMITTED"
                | "ROLLING_BACK"
                | "ROLLED_BACK"
                | "RECOVERY_REQUIRED"
        )
    })
}

pub(crate) fn valid_receipt(
    data: &Value,
    command: &ReadCommand,
    state: TaskState,
    error: ErrorCode,
) -> bool {
    if !matches!(command.operation, 30..=33) || !command.valid_payload() {
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
    if command.operation == 30 {
        if proof.is_some()
            || state != TaskState::Succeeded
            || error != ErrorCode::Unspecified
            || !keys(
                data,
                &[
                    "schemaVersion",
                    "revision",
                    "sourceDigest",
                    "initialized",
                    "projectable",
                    "candidate",
                    "writeEnabled",
                    "confirmationReady",
                    "transaction",
                ],
            )
            || data["schemaVersion"] != 1
            || !revision(&data["revision"])
            || !digest(&data["sourceDigest"])
            || !data["initialized"].is_boolean()
            || !data["projectable"].is_boolean()
            || !data["confirmationReady"].is_boolean()
            || !data["writeEnabled"].is_boolean()
        {
            return false;
        }
        if data["projectable"] == true {
            if !serde_json::from_value::<Candidate>(data["candidate"].clone())
                .is_ok_and(|v| v.valid())
            {
                return false;
            }
        } else if !data["candidate"].is_null() {
            return false;
        }
        let t = &data["transaction"];
        return t.is_null()
            || (keys(
                t,
                &[
                    "id",
                    "state",
                    "deadlineUnixMs",
                    "revision",
                    "currentRevision",
                    "requestId",
                ],
            ) && identity(&t["id"])
                && state_name(&t["state"])
                && revision(&t["revision"])
                && revision(&t["currentRevision"])
                && t["deadlineUnixMs"].as_u64().is_some_and(|v| v > 0)
                && t["requestId"]
                    .as_str()
                    .is_some_and(|v| !v.is_empty() && v.len() <= 64));
    }
    if !keys(
        data,
        &[
            "id",
            "state",
            "deadlineUnixMs",
            "revision",
            "requestId",
            "newManagementConnectionRequired",
            "newManagementConnectionVerified",
            "confirmationToken",
            "candidate",
            "reconciliation",
        ],
    ) || !identity(&data["id"])
        || !revision(&data["revision"])
        || data["newManagementConnectionRequired"] != true
        || !data["newManagementConnectionVerified"].is_boolean()
        || data["deadlineUnixMs"].as_u64().is_none_or(|v| v == 0)
    {
        return false;
    }
    let prepare = command.operation == 31;
    if prepare {
        if data["requestId"] != command.command_id {
            return false;
        }
        if state == TaskState::Succeeded {
            let Some(original) = command.firewall_change.as_ref().and_then(|v| v.candidate())
            else {
                return false;
            };
            let Ok(candidate) = serde_json::from_value::<Candidate>(data["candidate"].clone())
            else {
                return false;
            };
            if !candidate.valid()
                || serde_json::to_value(candidate).ok() != serde_json::to_value(original).ok()
            {
                return false;
            }
            if !data["confirmationToken"].as_str().is_some_and(|s| {
                s.len() == 32
                    && s.bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            }) {
                return false;
            }
        } else if data.get("confirmationToken").is_some() || data.get("candidate").is_some() {
            return false;
        }
    } else if command
        .firewall_action
        .as_ref()
        .is_none_or(|a| data["id"] != a.transaction_id)
        || data.get("confirmationToken").is_some()
        || data.get("candidate").is_some()
    {
        return false;
    }
    let settled = match (command.operation, state) {
        (31, TaskState::Succeeded) => {
            data["state"] == "WAIT_CONFIRM" || data["state"] == "COMMITTED"
        }
        (32, TaskState::Succeeded) => {
            data["state"] == "COMMITTED" && data["newManagementConnectionVerified"] == true
        }
        (33, TaskState::Succeeded) => data["state"] == "ROLLED_BACK",
        (31 | 32, TaskState::Failed) => {
            proof.is_some() && error == ErrorCode::Conflict && data["state"] == "ROLLED_BACK"
        }
        _ => false,
    };
    if !settled {
        return false;
    }
    if let Some(p) = proof {
        p["source"] == "firewall.remote.journal.v1"
            && p["commandId"] == command.command_id
            && p["operation"] == command.operation
            && p["transactionId"] == data["id"]
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
    fn action() -> ReadCommand {
        ReadCommand {
            command_id: uuid::Uuid::new_v4().to_string(),
            operation: 32,
            idempotency_key: "once".into(),
            expires_at_unix_ms: 123,
            firewall_action: Some(FirewallAction {
                transaction_id: format!("fw-{}", uuid::Uuid::new_v4()),
                confirmation_token: "a".repeat(32),
            }),
            ..Default::default()
        }
    }
    fn receipt(c: &ReadCommand) -> Value {
        json!({"id":c.firewall_action.as_ref().unwrap().transaction_id,"state":"COMMITTED","deadlineUnixMs":100,
            "revision":1,"requestId":uuid::Uuid::new_v4().to_string(),"newManagementConnectionRequired":true,"newManagementConnectionVerified":true})
    }
    #[test]
    fn original_prepare_candidate_is_bound_and_connection_readiness_is_typed() {
        let candidate = json!({"schemaVersion":3,"ipv4":{"inputPolicy":"accept","forwardPolicy":"drop","allowIcmp":false},"ipv6":{"inputPolicy":"accept","forwardPolicy":"drop","allowIcmp":false},"input":[],"portForward":[],"nat":[]});
        let draft = json!({"expectedRevision":0,"sourceDigest":"a".repeat(64),"confirmSeconds":180,"candidate":candidate});
        let c = command(
            uuid::Uuid::new_v4().to_string(),
            31,
            "original".into(),
            123,
            &json!({"firewallChange":draft}),
        )
        .unwrap();
        let prepared = json!({"id":format!("fw-{}",uuid::Uuid::new_v4()),"state":"WAIT_CONFIRM","deadlineUnixMs":150,"revision":0,"requestId":c.command_id,"newManagementConnectionRequired":true,"newManagementConnectionVerified":false,"confirmationToken":"b".repeat(32),"candidate":candidate});
        assert!(valid_receipt(
            &prepared,
            &c,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        let mut changed = prepared.clone();
        changed["candidate"]["ipv4"]["inputPolicy"] = json!("drop");
        assert!(!valid_receipt(
            &changed,
            &c,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        let mut view = json!({"schemaVersion":1,"revision":0,"sourceDigest":"c".repeat(64),"initialized":true,"projectable":true,"candidate":candidate,"writeEnabled":true,"confirmationReady":false,"transaction":null});
        let read = ReadCommand {
            operation: 30,
            ..Default::default()
        };
        assert!(valid_receipt(
            &view,
            &read,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        view["confirmationReady"] = json!("true");
        assert!(!valid_receipt(
            &view,
            &read,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
    }
    #[test]
    fn confirmation_requires_exact_action_and_new_connection_and_rejects_token_leaks() {
        let c = action();
        let good = receipt(&c);
        assert!(valid_receipt(
            &good,
            &c,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        for (key, v) in [
            ("id", json!(format!("fw-{}", uuid::Uuid::new_v4()))),
            ("newManagementConnectionVerified", json!(false)),
            ("state", json!("WAIT_CONFIRM")),
            ("confirmationToken", json!("unexpected")),
            ("password", json!("unexpected")),
        ] {
            let mut bad = good.clone();
            bad[key] = v;
            assert!(!valid_receipt(
                &bad,
                &c,
                TaskState::Succeeded,
                ErrorCode::Unspecified
            ));
        }
    }
    #[test]
    fn reconciliation_binds_full_original_request_and_cannot_claim_success_from_rollback() {
        let c = action();
        let mut good = receipt(&c);
        good["reconciliation"] = json!({"source":"firewall.remote.journal.v1","commandId":c.command_id,
            "operation":c.operation,"transactionId":good["id"],"requestSha256":format!("{:x}",Sha256::digest(c.encode_to_vec())),"journalSha256":"a".repeat(64),"observedAtUnixMs":5});
        assert!(valid_receipt(
            &good,
            &c,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        for field in ["commandId", "requestSha256", "transactionId", "source"] {
            let mut bad = good.clone();
            bad["reconciliation"][field] = json!("wrong");
            assert!(!valid_receipt(
                &bad,
                &c,
                TaskState::Succeeded,
                ErrorCode::Unspecified
            ));
        }
        good["state"] = json!("ROLLED_BACK");
        good["newManagementConnectionVerified"] = json!(false);
        assert!(valid_receipt(
            &good,
            &c,
            TaskState::Failed,
            ErrorCode::Conflict
        ));
        assert!(!valid_receipt(
            &good,
            &c,
            TaskState::Succeeded,
            ErrorCode::Unspecified
        ));
        let mut changed = c.clone();
        changed.expires_at_unix_ms += 1;
        assert!(!valid_receipt(
            &good,
            &changed,
            TaskState::Failed,
            ErrorCode::Conflict
        ));
    }
    #[test]
    fn encrypted_payload_reconstruction_accepts_only_its_operation_shape() {
        let c = action();
        let payload = json!({"firewallAction":c.firewall_action,"networkChange":null});
        let actual = command(
            c.command_id.clone(),
            32,
            c.idempotency_key.clone(),
            c.expires_at_unix_ms,
            &payload,
        )
        .unwrap();
        assert_eq!(actual.encode_to_vec(), c.encode_to_vec());
        assert!(
            command(
                c.command_id.clone(),
                31,
                c.idempotency_key.clone(),
                123,
                &payload
            )
            .is_err()
        );
        let mut bad = payload.clone();
        bad["networkChange"] = json!({"interface":"eth0"});
        assert!(command(c.command_id, 32, c.idempotency_key, 123, &bad).is_err());
    }
}
