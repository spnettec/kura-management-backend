//! Remote joint NM/firewall wire admission. The agent's coordinator remains the
//! authority for interface/version/NAT semantics before any system mutation.
//! No raw NM settings, D-Bus arguments, checkpoint paths or profile text cross
//! this grammar.
use crate::v1;
use serde::{Deserialize, Serialize};

/// HTTP/storage representation. `changes` is the typed preview input grammar
/// and `nat` the optional automatic NAT set; both travel as JSON bytes.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Change {
    pub source_digest: String,
    pub confirm_seconds: u32,
    pub changes: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nat: Option<serde_json::Value>,
}
// Omission and legacy null preserve policy. A versioned structured action
// requests removal without reinterpreting stored queued/UNKNOWN task inputs.
impl Change {
    pub fn coordinator_input(&self) -> serde_json::Value {
        let mut input = serde_json::json!({"sourceDigest":self.source_digest,
            "confirmSeconds":self.confirm_seconds,"changes":self.changes});
        if let Some(nat) = self.nat.as_ref().filter(|value| !value.is_null()) {
            let disable = nat.as_object().is_some_and(|value| {
                value.len() == 2
                    && value
                        .get("schemaVersion")
                        .is_some_and(|version| version == 1)
                    && value
                        .get("action")
                        .is_some_and(|action| action == "disable")
            });
            input["nat"] = if disable {
                serde_json::Value::Null
            } else {
                nat.clone()
            };
        }
        input
    }
    pub fn to_wire(&self) -> Option<v1::NmChange> {
        let value = v1::NmChange {
            source_digest: self.source_digest.clone(),
            confirm_seconds: self.confirm_seconds,
            changes_json: serde_json::to_vec(&self.changes).ok()?,
            nat_json: match &self.nat {
                Some(nat) => serde_json::to_vec(nat).ok()?,
                None => Vec::new(),
            },
        };
        value.valid().then_some(value)
    }
    pub fn from_wire(value: &v1::NmChange) -> Option<Self> {
        if !value.valid() {
            return None;
        }
        Some(Self {
            source_digest: value.source_digest.clone(),
            confirm_seconds: value.confirm_seconds,
            changes: serde_json::from_slice(&value.changes_json).ok()?,
            nat: if value.nat_json.is_empty() {
                None
            } else {
                serde_json::from_slice(&value.nat_json).ok()?
            },
        })
    }
}
impl v1::NmChange {
    pub fn valid(&self) -> bool {
        self.source_digest.len() == 64
            && self
                .source_digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            && (30..=300).contains(&self.confirm_seconds)
            && !self.changes_json.is_empty()
            && self.changes_json.len() <= 262_144
            && self.nat_json.len() <= 65_536
    }
}
impl v1::NmAction {
    pub fn valid(&self) -> bool {
        let uuid = |s: &str| uuid::Uuid::parse_str(s).is_ok_and(|v| v.to_string() == s);
        uuid(&self.operation_id)
            && self.confirmation_token.len() == 32
            && self
                .confirmation_token
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operation_numbers_are_adjacent_to_the_firewall_family() {
        assert_eq!(v1::Operation::NmView as i32, 34);
        assert_eq!(v1::Operation::NmPrepare as i32, 35);
        assert_eq!(v1::Operation::NmConfirm as i32, 36);
        assert_eq!(v1::Operation::NmRollback as i32, 37);
    }
    #[test]
    fn change_roundtrip_keeps_the_typed_grammar_and_rejects_bad_inputs() {
        let change = Change {
            source_digest: "a".repeat(64),
            confirm_seconds: 60,
            changes: serde_json::json!([{"interface":"eth0"}]),
            nat: Some(serde_json::json!({"wan":"eth0","lan":["eth1"]})),
        };
        let wire = change.to_wire().unwrap();
        assert!(wire.valid());
        let back = Change::from_wire(&wire).unwrap();
        assert_eq!(back.changes, change.changes);
        assert_eq!(back.nat, change.nat);
        // Structurally invalid changes never produce a wire form at all:
        // nothing is silently coerced into a writable proposal. The typed
        // grammar inside `changes` is the agent's preview validation.
        for bad in [
            Change {
                source_digest: "zz".into(),
                confirm_seconds: 60,
                changes: serde_json::json!([{"interface":"eth0"}]),
                nat: None,
            },
            Change {
                source_digest: "a".repeat(64),
                confirm_seconds: 10,
                changes: serde_json::json!([{"interface":"eth0"}]),
                nat: None,
            },
        ] {
            assert!(bad.to_wire().is_none());
        }
        // Absent NAT roundtrips to None.
        let plain = Change {
            source_digest: "a".repeat(64),
            confirm_seconds: 60,
            changes: serde_json::json!([{"interface":"eth0"}]),
            nat: None,
        };
        let wire = plain.to_wire().unwrap();
        assert!(wire.nat_json.is_empty());
        assert_eq!(Change::from_wire(&wire).unwrap().nat, None);
    }
    #[test]
    fn nat_omission_and_explicit_disable_survive_http_storage_and_wire() {
        let mut request = serde_json::json!({"sourceDigest":"a".repeat(64),"confirmSeconds":60,
            "changes":[{"interface":"eth0","ethernet":{"mtu":1500}}]});
        let keep: Change = serde_json::from_value(request.clone()).unwrap();
        assert!(keep.to_wire().unwrap().nat_json.is_empty());
        assert!(serde_json::to_value(&keep).unwrap().get("nat").is_none());
        assert!(keep.coordinator_input().get("nat").is_none());
        request["nat"] = serde_json::Value::Null;
        let legacy: Change = serde_json::from_value(request.clone()).unwrap();
        assert!(legacy.to_wire().unwrap().nat_json.is_empty());
        assert!(legacy.coordinator_input().get("nat").is_none());
        let intent = serde_json::json!({"schemaVersion":1,"action":"disable"});
        request["nat"] = intent.clone();
        let disable: Change = serde_json::from_value(request).unwrap();
        let wire = disable.to_wire().unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&wire.nat_json).unwrap(),
            intent
        );
        let decoded = Change::from_wire(&wire).unwrap();
        assert_eq!(decoded.nat, Some(intent));
        assert_eq!(decoded.coordinator_input()["nat"], serde_json::Value::Null);
    }
    #[test]
    fn legacy_stored_null_keeps_the_original_wire_identity() {
        let legacy = serde_json::json!({"sourceDigest":"a".repeat(64),"confirmSeconds":60,
            "changes":[{"interface":"eth0"}],"nat":null});
        let rebuilt: Change = serde_json::from_value(legacy).unwrap();
        let original = v1::NmChange {
            source_digest: "a".repeat(64),
            confirm_seconds: 60,
            changes_json: br#"[{"interface":"eth0"}]"#.to_vec(),
            nat_json: Vec::new(),
        };
        use prost::Message;
        assert_eq!(
            rebuilt.to_wire().unwrap().encode_to_vec(),
            original.encode_to_vec(),
            "old queued or uncertain tasks may not acquire a different NAT intention or request hash"
        );
    }
    #[test]
    fn action_requires_uuid_operation_and_bounded_hex_token() {
        let token: String = "aa3f".repeat(8);
        let good = v1::NmAction {
            operation_id: uuid::Uuid::new_v4().to_string(),
            confirmation_token: token,
        };
        assert!(good.valid());
        let bad = v1::NmAction {
            operation_id: "not-a-uuid".into(),
            confirmation_token: good.confirmation_token.clone(),
        };
        assert!(!bad.valid());
        let bad = v1::NmAction {
            operation_id: good.operation_id.clone(),
            confirmation_token: uuid::Uuid::new_v4().to_string(),
        };
        assert!(!bad.valid());
    }
}
