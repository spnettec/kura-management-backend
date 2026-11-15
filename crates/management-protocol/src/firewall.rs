//! Remote firewall wire admission. The agent's existing V3 compiler remains the
//! authority for address/interface/NAT semantics before any system mutation.
//! No raw command, expression, chain or table name crosses this grammar.
use crate::v1;
use serde::{Deserialize, Serialize};

/// HTTP/storage representation. Candidate bytes are an internal wire encoding;
/// operators edit the structured object and never supply a serialized program.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Change {
    pub expected_revision: u64,
    pub source_digest: String,
    pub confirm_seconds: u32,
    pub candidate: Candidate,
}
impl Change {
    pub fn to_wire(&self) -> Option<v1::FirewallChange> {
        let value = v1::FirewallChange {
            expected_revision: self.expected_revision,
            source_digest: self.source_digest.clone(),
            confirm_seconds: self.confirm_seconds,
            candidate_json: serde_json::to_vec(&self.candidate).ok()?,
        };
        value.valid().then_some(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Candidate {
    pub schema_version: u8,
    pub ipv4: FamilyPolicy,
    pub ipv6: FamilyPolicy,
    pub input: Vec<Input>,
    pub port_forward: Vec<PortForward>,
    pub nat: Vec<Nat>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FamilyPolicy {
    pub input_policy: Policy,
    pub forward_policy: Policy,
    pub allow_icmp: bool,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Policy {
    Accept,
    Drop,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Family {
    Ipv4,
    Ipv6,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ports {
    pub first: u16,
    pub last: u16,
}
impl Ports {
    fn valid(&self) -> bool {
        self.first > 0 && self.first <= self.last
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Input {
    pub family: Family,
    pub protocol: Protocol,
    pub ports: Ports,
    pub source: Option<String>,
    pub input_interface: Option<String>,
    pub excluded_interface: Option<String>,
    pub source_mac: Option<String>,
    pub source_ports: Option<Ports>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortForward {
    pub family: Family,
    pub protocol: Protocol,
    pub input_interface: String,
    pub output_interface: String,
    pub address: String,
    pub in_port: u16,
    pub out_port: u16,
    pub masquerade: bool,
    pub source: Option<String>,
    pub source_mac: Option<String>,
    pub source_ports: Option<Ports>,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NatProtocol {
    All,
    Tcp,
    Udp,
    Icmp,
    Icmpv6,
    Gre,
    Esp,
    Ah,
    Sctp,
    #[serde(untagged)]
    Number(u8),
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleType {
    Generic,
    IpForwarding,
    PortForwarding,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Nat {
    pub family: Family,
    pub input_interface: String,
    pub output_interface: String,
    pub protocol: Option<NatProtocol>,
    pub masquerade: bool,
    pub automatic: bool,
    pub rule_type: Option<RuleType>,
    pub source: Option<String>,
    pub destination: Option<String>,
}
impl Candidate {
    pub fn valid(&self) -> bool {
        self.schema_version == 3
            && self.input.len() + self.port_forward.len() + self.nat.len() <= 256
            && self
                .input
                .iter()
                .all(|r| r.ports.valid() && r.source_ports.as_ref().is_none_or(Ports::valid))
            && self.port_forward.iter().all(|r| {
                r.in_port > 0 && r.out_port > 0 && r.source_ports.as_ref().is_none_or(Ports::valid)
            })
    }
}
impl v1::FirewallChange {
    pub fn candidate(&self) -> Option<Candidate> {
        if self.candidate_json.len() > 16384 {
            return None;
        }
        let candidate: Candidate = serde_json::from_slice(&self.candidate_json).ok()?;
        candidate.valid().then_some(candidate)
    }
    pub fn valid(&self) -> bool {
        self.expected_revision <= i64::MAX as u64
            && (30..=300).contains(&self.confirm_seconds)
            && self.source_digest.len() == 64
            && self
                .source_digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            && self.candidate().is_some()
    }
}
impl v1::FirewallAction {
    pub fn valid(&self) -> bool {
        let uuid = |s: &str| uuid::Uuid::parse_str(s).is_ok_and(|v| v.to_string() == s);
        self.transaction_id.strip_prefix("fw-").is_some_and(uuid)
            && self.confirmation_token.len() == 32
            && self
                .confirmation_token
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    }
}

#[cfg(test)]
mod tests {
    use crate::{has_side_effect, v1::*};
    fn change() -> FirewallChange {
        FirewallChange {
            expected_revision: 0,
            source_digest: "a".repeat(64),
            confirm_seconds: 180,
            candidate_json: serde_json::to_vec(&serde_json::json!({"schemaVersion":3,
              "ipv4":{"inputPolicy":"drop","forwardPolicy":"drop","allowIcmp":true},
              "ipv6":{"inputPolicy":"accept","forwardPolicy":"drop","allowIcmp":false},
              "input":[],"portForward":[],"nat":[]}))
            .unwrap(),
        }
    }
    #[test]
    fn additive_wire_fields_preserve_all_earlier_payload_exclusion() {
        use prost::Message;
        assert_eq!(Operation::FirewallView as i32, 30);
        assert_eq!(Operation::FirewallRollback as i32, 33);
        assert_eq!(
            ReadCommand {
                operation: 30,
                ..Default::default()
            }
            .encode_to_vec(),
            [16, 30]
        );
        assert!(!has_side_effect(30));
        let action = FirewallAction {
            transaction_id: format!("fw-{}", uuid::Uuid::new_v4()),
            confirmation_token: "a".repeat(32),
        };
        for op in 1..=34 {
            let c = ReadCommand {
                operation: op,
                firewall_change: Some(change()),
                ..Default::default()
            };
            assert_eq!(c.valid_payload(), op == 31);
            let c = ReadCommand {
                operation: op,
                firewall_action: Some(action.clone()),
                ..Default::default()
            };
            assert_eq!(c.valid_payload(), matches!(op, 32 | 33));
        }
        for op in 31..=33 {
            assert!(has_side_effect(op));
        }
        let bad = FirewallAction {
            transaction_id: "firewall-../../state".into(),
            ..action
        };
        assert!(!bad.valid());
        let encoded = ReadCommand {
            operation: 31,
            firewall_change: Some(change()),
            ..Default::default()
        }
        .encode_to_vec();
        assert!(
            ReadCommand::decode(encoded.as_slice())
                .unwrap()
                .valid_payload()
        );
    }
    #[test]
    fn candidate_is_structured_bounded_and_has_no_expression_escape() {
        let good = change();
        assert!(good.valid());
        let base: serde_json::Value = serde_json::from_slice(&good.candidate_json).unwrap();
        for (key, value) in [
            ("command", serde_json::json!("nft flush ruleset")),
            ("schemaVersion", serde_json::json!(2)),
            (
                "input",
                serde_json::json!([{"family":"ipv4","protocol":"tcp","ports":{"first":0,"last":65535}}]),
            ),
        ] {
            let mut raw = base.clone();
            raw[key] = value;
            let bad = FirewallChange {
                candidate_json: serde_json::to_vec(&raw).unwrap(),
                ..good.clone()
            };
            assert!(!bad.valid());
        }
        for seconds in [0, 29, 301, u32::MAX] {
            assert!(
                !FirewallChange {
                    confirm_seconds: seconds,
                    ..good.clone()
                }
                .valid()
            );
        }
        assert!(
            !FirewallChange {
                candidate_json: vec![b' '; 16385],
                ..good.clone()
            }
            .valid()
        );
        assert!(
            !FirewallChange {
                expected_revision: u64::MAX,
                ..good.clone()
            }
            .valid()
        );
        let mut raw = base.clone();
        raw["ipv4"]["expression"] = serde_json::json!("accept");
        assert!(
            !FirewallChange {
                candidate_json: serde_json::to_vec(&raw).unwrap(),
                ..good
            }
            .valid()
        );
    }
}
