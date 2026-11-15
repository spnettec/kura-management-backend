//! Imported confirmed policy operations. Public requests contain no owner key;
//! only the encrypted center/agent task envelope carries that capability.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

pub const MAX_PAYLOAD: usize = 131_072;
pub const CAPABILITY: &str = "firewall.imported.original.v1";
pub const WRITE_CAPABILITY: &str = "firewall.imported.write.v1";

/// Audit of the already authenticated SAME original record. This can resolve
/// task metadata; it is never a fresh write, runtime or owner capability.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DecisionEvidence {
    pub record_sha256: String,
    pub historical: bool,
    pub source_observation: bool,
    pub forward_intent: bool,
    pub forward_acknowledged: bool,
    pub inverse_intent: bool,
    pub inverse_acknowledged: bool,
    pub negative_acknowledged: bool,
    pub confirmed_decision: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<OriginalCommands>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OriginalCommands {
    pub original_sha256: String,
    pub forward_sha256: Option<String>,
    pub inverse_sha256: Option<String>,
    pub confirmation_sha256: Option<String>,
    pub cancellation_sha256: Option<String>,
    pub expiry_decision: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginalResolution {
    Succeeded,
    Failed,
}
impl DecisionEvidence {
    pub fn valid(&self) -> bool {
        hex(&self.record_sha256)
            && self.commands.as_ref().is_none_or(|c| {
                hex(&c.original_sha256)
                    && [
                        &c.forward_sha256,
                        &c.inverse_sha256,
                        &c.confirmation_sha256,
                        &c.cancellation_sha256,
                    ]
                    .into_iter()
                    .flatten()
                    .all(|v| hex(v))
                    && self.forward_intent == c.forward_sha256.is_some()
                    && self.confirmed_decision == c.confirmation_sha256.is_some()
                    && (self.inverse_intent == c.inverse_sha256.is_some()
                        || c.expiry_decision && self.inverse_intent && c.inverse_sha256.is_none())
                    && (c.cancellation_sha256.is_none()
                        || !self.forward_intent && !self.inverse_intent && !self.confirmed_decision)
                    && (!c.expiry_decision
                        || c.inverse_sha256.is_none() && c.cancellation_sha256.is_none())
                    && (!self.source_observation
                        || !c.expiry_decision
                            && [
                                &c.forward_sha256,
                                &c.inverse_sha256,
                                &c.confirmation_sha256,
                                &c.cancellation_sha256,
                            ]
                            .into_iter()
                            .all(Option::is_none))
            })
            && !(self.forward_acknowledged && !self.forward_intent
                || self.inverse_acknowledged && !self.inverse_intent
                || self.inverse_intent && !self.forward_acknowledged
                || self.confirmed_decision && (!self.forward_acknowledged || self.inverse_intent)
                || self.source_observation
                    && (self.forward_intent
                        || self.inverse_intent
                        || self.negative_acknowledged
                        || self.confirmed_decision))
    }
    /// Exact original phase markers, never the current policy alone. A different
    /// immutable phase marker proves this command did not dispatch that phase.
    pub fn resolution(
        &self,
        operation: i32,
        state: &str,
        command_sha256: &str,
        connection_verified: bool,
    ) -> Option<OriginalResolution> {
        if !self.valid() || !hex(command_sha256) {
            return None;
        }
        let c = self.commands.as_ref()?;
        if self.source_observation {
            return (operation == 47
                && command_sha256 == c.original_sha256
                && state == "OBSERVED"
                && self.historical
                && !self.forward_intent
                && !self.inverse_intent
                && !self.negative_acknowledged
                && !self.confirmed_decision)
                .then_some(OriginalResolution::Succeeded);
        }
        if operation == 40 {
            return (command_sha256 == c.original_sha256
                && !matches!(state, "OBSERVING" | "OBSERVED"))
            .then_some(OriginalResolution::Succeeded);
        }
        let phase = match operation {
            41 => &c.forward_sha256,
            42 => &c.confirmation_sha256,
            43 => &c.inverse_sha256,
            44 => &c.cancellation_sha256,
            _ => return None,
        };
        if phase.as_deref() == Some(command_sha256) {
            let succeeded = match operation {
                41 => {
                    self.forward_acknowledged
                        || self.historical && state == "REJECTED" && self.negative_acknowledged
                }
                42 => {
                    self.historical
                        && state == "COMMITTED"
                        && self.confirmed_decision
                        && connection_verified
                }
                43 => {
                    self.historical && state == "ROLLED_BACK" && self.inverse_acknowledged
                        || state == "RECOVERY_REQUIRED"
                            && self.inverse_intent
                            && self.negative_acknowledged
                }
                44 => self.historical && state == "ROLLED_BACK" && !self.forward_intent,
                _ => false,
            };
            return succeeded.then_some(OriginalResolution::Succeeded);
        }
        let terminal = self.historical && matches!(state, "COMMITTED" | "ROLLED_BACK" | "REJECTED");
        let another_decision = match operation {
            41 => self.forward_intent || terminal,
            42 => self.confirmed_decision || self.inverse_intent || terminal,
            43 => self.inverse_intent || self.confirmed_decision || terminal,
            44 => self.forward_intent || terminal,
            _ => false,
        };
        another_decision.then_some(OriginalResolution::Failed)
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Review {
    pub operation_id: Uuid,
    pub expected_revision: u64,
    pub source_digest: String,
    pub candidate: Value,
    pub destination_prefixes: BTreeMap<String, Vec<u8>>,
}
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Proposal {
    pub operation_id: Uuid,
    pub expected_revision: u64,
    pub source_digest: String,
    pub cohort_digest: String,
    pub confirm_seconds: u64,
    pub candidate: Value,
    pub destination_prefixes: BTreeMap<String, Vec<u8>>,
}
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SourceReview {
    pub operation_id: Uuid,
    pub expected_revision: u64,
    pub source_digest: String,
}
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Observation {
    pub operation_id: Uuid,
    pub expected_revision: u64,
    pub source_digest: String,
    pub cohort_digest: String,
    pub observed_native_sha256: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum Request {
    View,
    Review {
        review: Review,
    },
    Prepare {
        proposal: Proposal,
    },
    Apply {
        #[serde(rename = "originalTaskId")]
        original_task_id: Uuid,
    },
    Confirm {
        #[serde(rename = "originalTaskId")]
        original_task_id: Uuid,
    },
    Rollback {
        #[serde(rename = "originalTaskId")]
        original_task_id: Uuid,
    },
    Cancel {
        #[serde(rename = "originalTaskId")]
        original_task_id: Uuid,
    },
    Outcome {
        #[serde(rename = "originalTaskId")]
        original_task_id: Uuid,
    },
    SourceReview {
        review: SourceReview,
    },
    SourceObserve {
        observation: Observation,
    },
    SourceOutcome {
        #[serde(rename = "originalTaskId")]
        original_task_id: Uuid,
    },
}
fn hex(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn prefixes(v: &BTreeMap<String, Vec<u8>>) -> bool {
    v.len() <= 2
        && v.iter().all(|(family, values)| {
            let max = match family.as_str() {
                "ipv4" => 32,
                "ipv6" => 128,
                _ => return false,
            };
            values.len() <= 256 && values.iter().all(|v| *v <= max)
        })
}
impl Request {
    pub fn operation(&self) -> i32 {
        match self {
            Self::View => 38,
            Self::Review { .. } => 39,
            Self::Prepare { .. } => 40,
            Self::Apply { .. } => 41,
            Self::Confirm { .. } => 42,
            Self::Rollback { .. } => 43,
            Self::Cancel { .. } => 44,
            Self::Outcome { .. } => 45,
            Self::SourceReview { .. } => 46,
            Self::SourceObserve { .. } => 47,
            Self::SourceOutcome { .. } => 48,
        }
    }
    pub fn original_task(&self) -> Option<Uuid> {
        match self {
            Self::Apply { original_task_id }
            | Self::Confirm { original_task_id }
            | Self::Rollback { original_task_id }
            | Self::Cancel { original_task_id }
            | Self::Outcome { original_task_id }
            | Self::SourceOutcome { original_task_id } => Some(*original_task_id),
            _ => None,
        }
    }
    pub fn original_operation(&self) -> Option<Uuid> {
        match self {
            Self::Prepare { proposal } => Some(proposal.operation_id),
            Self::SourceObserve { observation } => Some(observation.operation_id),
            _ => None,
        }
    }
    pub fn needs_owner_key(&self) -> bool {
        self.original_operation().is_some()
    }
    pub fn valid(&self) -> bool {
        let shape = match self {
            Self::View => true,
            Self::Review { review: r } => {
                !r.operation_id.is_nil()
                    && hex(&r.source_digest)
                    && r.candidate.is_object()
                    && prefixes(&r.destination_prefixes)
            }
            Self::Prepare { proposal: p } => {
                !p.operation_id.is_nil()
                    && hex(&p.source_digest)
                    && hex(&p.cohort_digest)
                    && (30..=300).contains(&p.confirm_seconds)
                    && p.candidate.is_object()
                    && prefixes(&p.destination_prefixes)
            }
            Self::SourceReview { review: r } => !r.operation_id.is_nil() && hex(&r.source_digest),
            Self::SourceObserve { observation: o } => {
                !o.operation_id.is_nil()
                    && hex(&o.source_digest)
                    && hex(&o.cohort_digest)
                    && hex(&o.observed_native_sha256)
            }
            _ => self.original_task().is_some_and(|id| !id.is_nil()),
        };
        shape && serde_json::to_vec(self).is_ok_and(|v| v.len() <= MAX_PAYLOAD - 256)
    }
    pub fn method(&self) -> &'static str {
        match self {
            Self::View => "firewall.kura.imported.view",
            Self::Review { .. } => "firewall.kura.imported.review",
            Self::Prepare { .. } => "firewall.kura.imported.prepare",
            Self::Apply { .. } => "firewall.kura.imported.apply",
            Self::Confirm { .. } => "firewall.kura.imported.confirm",
            Self::Rollback { .. } => "firewall.kura.imported.rollback",
            Self::Cancel { .. } => "firewall.kura.imported.cancel",
            Self::Outcome { .. } => "firewall.kura.imported.outcome",
            Self::SourceReview { .. } => "firewall.kura.imported.source.review",
            Self::SourceObserve { .. } => "firewall.kura.imported.source.observe",
            Self::SourceOutcome { .. } => "firewall.kura.imported.source.outcome",
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Envelope {
    pub request: Request,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_key: Option<String>,
}
impl Envelope {
    pub fn valid(&self, operation: i32) -> bool {
        self.request.operation() == operation
            && self.request.valid()
            && self.request.needs_owner_key() == self.owner_key.is_some()
            && self.owner_key.as_ref().is_none_or(|key| hex(key))
    }
    pub fn to_wire(&self) -> Result<crate::v1::ImportedFirewallPayload> {
        ensure!(
            self.valid(self.request.operation()),
            "INVALID_ARGUMENT: imported envelope"
        );
        let json = serde_json::to_vec(self)?;
        ensure!(
            json.len() <= MAX_PAYLOAD,
            "RESOURCE_EXHAUSTED: imported envelope"
        );
        Ok(crate::v1::ImportedFirewallPayload { json })
    }
    pub fn from_wire(value: &crate::v1::ImportedFirewallPayload, operation: i32) -> Result<Self> {
        ensure!(
            value.json.len() <= MAX_PAYLOAD,
            "RESOURCE_EXHAUSTED: imported envelope"
        );
        let result: Self = serde_json::from_slice(&value.json)?;
        ensure!(
            result.valid(operation),
            "INVALID_ARGUMENT: imported operation/payload"
        );
        Ok(result)
    }
    /// Fixed backend DTO only; action capabilities are resolved from the agent's
    /// encrypted original admission, never supplied by a browser or caller UID.
    pub fn original_input(&self) -> Result<Value> {
        ensure!(self.valid(self.request.operation()), "INVALID_ARGUMENT");
        let mut value = match &self.request {
            Request::View => serde_json::json!({}),
            Request::Review { review } => serde_json::to_value(review)?,
            Request::Prepare { proposal } => serde_json::to_value(proposal)?,
            Request::SourceReview { review } => serde_json::to_value(review)?,
            Request::SourceObserve { observation } => serde_json::to_value(observation)?,
            _ => anyhow::bail!("INVALID_ARGUMENT: original action requires saved admission"),
        };
        if let Some(key) = &self.owner_key {
            value["ownerKey"] = Value::String(key.clone());
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v1::ReadCommand;
    fn prepare() -> Request {
        Request::Prepare {
            proposal: Proposal {
                operation_id: Uuid::new_v4(),
                expected_revision: 0,
                source_digest: "a".repeat(64),
                cohort_digest: "b".repeat(64),
                confirm_seconds: 90,
                candidate: serde_json::json!({"schemaVersion":4}),
                destination_prefixes: BTreeMap::new(),
            },
        }
    }
    #[test]
    fn public_request_rejects_secret_identity_and_native_escape_fields() {
        for field in [
            "ownerKey",
            "uid",
            "unit",
            "path",
            "command",
            "confirmationToken",
        ] {
            let mut request = serde_json::to_value(prepare()).unwrap();
            request[field] = Value::String("untrusted".into());
            assert!(
                serde_json::from_value::<Request>(request).is_err(),
                "{field}"
            );
        }
    }
    #[test]
    fn encrypted_envelope_requires_exact_operation_and_exclusive_payload_family() {
        let envelope = Envelope {
            request: prepare(),
            owner_key: Some("c".repeat(64)),
        };
        let wire = envelope.to_wire().unwrap();
        assert!(Envelope::from_wire(&wire, 40).is_ok());
        for operation in [31, 35, 39, 41, 47, 48] {
            assert!(Envelope::from_wire(&wire, operation).is_err());
        }
        let mut command = ReadCommand {
            operation: 40,
            imported_firewall: Some(wire),
            ..Default::default()
        };
        assert!(command.valid_payload());
        command.firewall_action = Some(Default::default());
        assert!(!command.valid_payload());
        assert!(
            !Envelope {
                request: prepare(),
                owner_key: None
            }
            .valid(40)
        );
        assert!(
            !Envelope {
                request: Request::View,
                owner_key: Some("c".repeat(64))
            }
            .valid(38)
        );
    }
    #[test]
    fn original_query_cannot_become_a_write_and_payload_work_is_bounded() {
        let query = Request::Outcome {
            original_task_id: Uuid::new_v4(),
        };
        assert_eq!(query.operation(), 45);
        assert!(!crate::has_side_effect(45));
        assert!(crate::has_side_effect(41));
        assert!(crate::has_side_effect(44));
        let mut input = prepare();
        if let Request::Prepare { proposal } = &mut input {
            proposal
                .destination_prefixes
                .insert("ipv4".into(), vec![33]);
        }
        assert!(!input.valid());
        let mut input = prepare();
        if let Request::Prepare { proposal } = &mut input {
            proposal.candidate["oversize"] = Value::String("x".repeat(MAX_PAYLOAD));
        }
        assert!(!input.valid());
        assert!(
            Envelope::from_wire(
                &crate::v1::ImportedFirewallPayload {
                    json: vec![0; MAX_PAYLOAD + 1]
                },
                38
            )
            .is_err()
        );
    }
    #[test]
    fn task_resolution_requires_exact_original_command_phase_ack_and_connection_evidence() {
        use OriginalResolution::{Failed, Succeeded};
        let hash = "b".repeat(64);
        let other = "c".repeat(64);
        let parent = "d".repeat(64);
        let mut evidence = DecisionEvidence {
            record_sha256: "a".repeat(64),
            historical: false,
            source_observation: false,
            forward_intent: true,
            forward_acknowledged: false,
            inverse_intent: false,
            inverse_acknowledged: false,
            negative_acknowledged: false,
            confirmed_decision: false,
            commands: Some(OriginalCommands {
                original_sha256: parent.clone(),
                forward_sha256: Some(hash.clone()),
                inverse_sha256: None,
                confirmation_sha256: None,
                cancellation_sha256: None,
                expiry_decision: false,
            }),
        };
        assert_eq!(
            evidence.resolution(40, "PREPARED", &parent, false),
            Some(Succeeded)
        );
        assert_eq!(evidence.resolution(41, "WAIT_CONFIRM", &hash, false), None);
        assert_eq!(
            evidence.resolution(41, "WAIT_CONFIRM", &other, false),
            Some(Failed)
        );
        evidence.forward_acknowledged = true;
        assert_eq!(
            evidence.resolution(41, "WAIT_CONFIRM", &hash, false),
            Some(Succeeded)
        );
        assert_eq!(evidence.resolution(42, "WAIT_CONFIRM", &other, true), None);
        evidence.historical = true;
        evidence.confirmed_decision = true;
        evidence.commands.as_mut().unwrap().confirmation_sha256 = Some(other.clone());
        assert_eq!(evidence.resolution(42, "COMMITTED", &other, false), None);
        assert_eq!(
            evidence.resolution(42, "COMMITTED", &other, true),
            Some(Succeeded)
        );
        assert_eq!(
            evidence.resolution(42, "COMMITTED", &hash, true),
            Some(Failed)
        );
        assert_eq!(
            evidence.resolution(43, "COMMITTED", &hash, true),
            Some(Failed)
        );
        evidence.confirmed_decision = false;
        evidence.inverse_intent = true;
        evidence.inverse_acknowledged = true;
        let commands = evidence.commands.as_mut().unwrap();
        commands.confirmation_sha256 = None;
        commands.inverse_sha256 = Some(other.clone());
        assert_eq!(
            evidence.resolution(43, "ROLLED_BACK", &other, false),
            Some(Succeeded)
        );
        assert_eq!(
            evidence.resolution(43, "ROLLED_BACK", &hash, false),
            Some(Failed)
        );
        assert_eq!(
            evidence.resolution(44, "ROLLED_BACK", &other, false),
            Some(Failed)
        );
        evidence.source_observation = true;
        assert!(!evidence.valid());
        assert_eq!(evidence.resolution(47, "OBSERVED", &parent, false), None);
        evidence.source_observation = false;
        evidence.commands = None;
        assert!(evidence.valid());
        assert_eq!(evidence.resolution(41, "WAIT_CONFIRM", &hash, false), None);
    }
    #[test]
    fn source_observation_and_expiry_never_inherit_a_remote_action_marker() {
        use OriginalResolution::{Failed, Succeeded};
        let hash = "a".repeat(64);
        let mut evidence = DecisionEvidence {
            record_sha256: hash.clone(),
            historical: true,
            source_observation: true,
            forward_intent: false,
            forward_acknowledged: false,
            inverse_intent: false,
            inverse_acknowledged: false,
            negative_acknowledged: false,
            confirmed_decision: false,
            commands: Some(OriginalCommands {
                original_sha256: hash.clone(),
                forward_sha256: None,
                inverse_sha256: None,
                confirmation_sha256: None,
                cancellation_sha256: None,
                expiry_decision: false,
            }),
        };
        assert_eq!(
            evidence.resolution(47, "OBSERVED", &hash, false),
            Some(Succeeded)
        );
        evidence.commands.as_mut().unwrap().expiry_decision = true;
        assert!(!evidence.valid());
        evidence.source_observation = false;
        assert_eq!(
            evidence.resolution(44, "ROLLED_BACK", &hash, false),
            Some(Failed)
        );
        let c = evidence.commands.as_mut().unwrap();
        c.forward_sha256 = Some(hash.clone());
        evidence.forward_intent = true;
        evidence.forward_acknowledged = true;
        evidence.inverse_intent = true;
        evidence.inverse_acknowledged = true;
        assert!(evidence.valid());
        assert_eq!(
            evidence.resolution(43, "ROLLED_BACK", &hash, false),
            Some(Failed)
        );
    }
}
