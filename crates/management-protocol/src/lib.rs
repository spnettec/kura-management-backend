pub mod v1 {
    tonic::include_proto!("yofc.management.v1");
}
pub mod firewall;
pub mod nm;
pub mod imported;

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const PROTOCOL_MAJOR: u32 = 1;
pub const MAX_FRAME: usize = 1_048_576;
// Unrecognized future operations are conservative until explicitly classified.
pub fn has_side_effect(operation: i32) -> bool {
    !matches!(operation, 1 | 2 | 3 | 4 | 8 | 9 | 11 | 15 | 20 | 23 | 30 | 34 | 38 | 39 | 45 | 46 | 48)
}

fn decimal_snapshot_id(value: &str) -> Option<i64> {
    let parsed = value.parse::<i64>().ok()?;
    (parsed >= 0 && value == parsed.to_string() && value.len() <= 18).then_some(parsed)
}

fn snapshot_id(value: &str, zero_allowed: bool) -> bool {
    decimal_snapshot_id(value).is_some_and(|parsed| zero_allowed || parsed > 0)
}
pub const READ_METHODS: &[&str] = &[
    "agent.status",
    "kura.configuration.list",
    "kura.snapshot.list",
    "kura.configuration.get",
];

impl v1::KuraConfigurationTarget {
    pub fn valid(&self) -> bool {
        !self.pid.trim().is_empty()
            && self.pid.len() <= 512
            && !self.pid.chars().any(char::is_control)
            && matches!(self.locale.as_str(), "en" | "zh")
    }
}

impl v1::KuraSnapshotSave {
    pub fn valid(&self) -> bool {
        snapshot_id(&self.expected_latest_id, true)
    }
    pub fn parameters(&self, expires_at_unix_ms: i64) -> Option<serde_json::Value> {
        if !self.valid() || expires_at_unix_ms <= 0 {
            return None;
        }
        Some(
            serde_json::json!({"expectedLatestId": self.expected_latest_id, "expiresAtUnixMs": expires_at_unix_ms}),
        )
    }
}
impl v1::KuraSnapshotRollback {
    pub fn valid(&self) -> bool {
        match (
            decimal_snapshot_id(&self.snapshot_id),
            decimal_snapshot_id(&self.expected_latest_id),
        ) {
            (Some(target), Some(latest)) => target >= 0 && latest > 0 && target < latest,
            _ => false,
        }
    }
    pub fn parameters(&self, expires_at_unix_ms: i64) -> Option<serde_json::Value> {
        if !self.valid() || expires_at_unix_ms <= 0 {
            return None;
        }
        Some(
            serde_json::json!({"snapshotId": self.snapshot_id, "expectedLatestId": self.expected_latest_id, "expiresAtUnixMs": expires_at_unix_ms}),
        )
    }
}
impl v1::KuraConfigurationUpdate {
    /// The OS bridge performs the final metatype/revision check. This boundary
    /// rejects ambiguous inputs and bounds work before durable admission.
    pub fn parameters(&self, expires_at_unix_ms: i64) -> Option<Value> {
        use v1::kura_configuration_property::Value as PropertyValue;
        if self.pid.trim().is_empty()
            || self.pid.len() > 512
            || self.pid.chars().any(char::is_control)
            || self.expected_revision.len() != 64
            || !self
                .expected_revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || self.properties.is_empty()
            || self.properties.len() > 64
            || expires_at_unix_ms <= 0
        {
            return None;
        }
        let mut properties = serde_json::Map::new();
        for property in &self.properties {
            if property.id.is_empty()
                || property.id.len() > 512
                || property.id.chars().any(char::is_control)
            {
                return None;
            }
            let value = match property.value.as_ref()? {
                PropertyValue::Text(text) if text.len() <= 16384 => Value::String(text.clone()),
                PropertyValue::Boolean(value) => Value::Bool(*value),
                PropertyValue::TextArray(array)
                    if array.values.len() <= 256
                        && array.values.iter().all(|v| v.len() <= 16384) =>
                {
                    Value::Array(array.values.iter().cloned().map(Value::String).collect())
                }
                PropertyValue::BooleanArray(array) if array.values.len() <= 256 => {
                    Value::Array(array.values.iter().copied().map(Value::Bool).collect())
                }
                _ => return None,
            };
            if properties.insert(property.id.clone(), value).is_some() {
                return None;
            }
        }
        let params = serde_json::json!({"pid": self.pid, "expectedRevision": self.expected_revision,
            "expiresAtUnixMs": expires_at_unix_ms, "properties": properties});
        (serde_json::to_vec(&params).ok()?.len() <= 16384).then_some(params)
    }
}

impl v1::OrdinaryBootAction {
    pub fn valid(&self) -> bool {
        fn uuid(s: &str) -> bool {
            uuid::Uuid::parse_str(s).is_ok_and(|v| v.to_string() == s)
        }
        self.transaction_id.strip_prefix("boot-").is_some_and(uuid)
            && uuid(&self.confirmation_token)
    }
}

impl v1::ReadCommand {
    pub fn valid_payload(&self) -> bool {
        (self.operation == 5) == self.network_change.is_some()
            && matches!(self.operation, 38..=48) == self.imported_firewall.is_some()
            && self.imported_firewall.as_ref().is_none_or(|v| imported::Envelope::from_wire(v, self.operation).is_ok())
            && (self.operation == 31) == self.firewall_change.is_some()
            && matches!(self.operation, 32 | 33) == self.firewall_action.is_some()
            && self.firewall_change.as_ref().is_none_or(|v| v.valid())
            && self.firewall_action.as_ref().is_none_or(|v| v.valid())
            && (self.operation == 35) == self.nm_change.is_some()
            && matches!(self.operation, 36 | 37) == self.nm_action.is_some()
            && self.nm_change.as_ref().is_none_or(|v| v.valid())
            && self.nm_action.as_ref().is_none_or(|v| v.valid())
            && matches!(self.operation, 6..=8) == self.network_action.is_some()
            && matches!(self.operation, 9 | 10) == self.network_takeover.is_some()
            && (self.operation == 11) == self.kura_configuration.is_some()
            && (self.operation == 12) == self.kura_configuration_update.is_some()
            && (self.operation == 13) == self.kura_snapshot_save.is_some()
            && (self.operation == 14) == self.kura_snapshot_rollback.is_some()
            && matches!(self.operation, 17 | 18) == self.ordinary_boot_action.is_some()
            && self.ordinary_boot_action.as_ref().is_none_or(|v| v.valid())
            && matches!(self.operation, 22 | 29) == self.ordinary_handoff_action.is_some()
            && self
                .ordinary_handoff_action
                .as_ref()
                .is_none_or(|v| v.valid())
            && matches!(self.operation, 25 | 27 | 28) == self.system_control_action.is_some()
            && self
                .system_control_action
                .as_ref()
                .is_none_or(|v| v.valid())
            && self.kura_configuration.as_ref().is_none_or(|v| v.valid())
            && self
                .kura_configuration_update
                .as_ref()
                .is_none_or(|v| v.parameters(self.expires_at_unix_ms).is_some())
            && self
                .kura_snapshot_save
                .as_ref()
                .is_none_or(|v| v.parameters(self.expires_at_unix_ms).is_some())
            && self
                .kura_snapshot_rollback
                .as_ref()
                .is_none_or(|v| v.parameters(self.expires_at_unix_ms).is_some())
    }
}

impl v1::OrdinaryHandoffAction {
    pub fn valid(&self) -> bool {
        fn uuid(s: &str) -> bool {
            uuid::Uuid::parse_str(s).is_ok_and(|v| v.to_string() == s)
        }
        self.transaction_id
            .strip_prefix("handoff-")
            .is_some_and(uuid)
            && uuid(&self.confirmation_token)
    }
}

impl v1::SystemControlAction {
    pub fn valid(&self) -> bool {
        let canonical = |s: &str| uuid::Uuid::parse_str(s).is_ok_and(|v| v.to_string() == s);
        self.transaction_id
            .strip_prefix("system-")
            .is_some_and(canonical)
            && canonical(&self.confirmation_token)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalFrame {
    pub version: u32,
    pub kind: String,
    pub request_id: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl LocalFrame {
    pub fn validate(&self) -> Result<()> {
        ensure!(self.version == PROTOCOL_MAJOR, "PROTOCOL_MISMATCH");
        ensure!(
            !self.request_id.is_empty() && self.request_id.len() <= 64,
            "INVALID_ARGUMENT"
        );
        ensure!(
            ["request", "response", "register"].contains(&self.kind.as_str()),
            "INVALID_ARGUMENT"
        );
        ensure!(self.method.len() <= 128, "INVALID_ARGUMENT");
        Ok(())
    }
    pub fn response(&self, result: Option<Value>, error: Option<&str>) -> Self {
        Self {
            kind: "response".into(),
            params: None,
            result,
            error: error.map(str::to_owned),
            ..self.clone()
        }
    }
}

pub async fn read_frame<R: AsyncRead + Unpin>(r: &mut R) -> Result<LocalFrame> {
    let n = r.read_u32().await? as usize;
    ensure!(n > 0 && n <= MAX_FRAME, "RESOURCE_EXHAUSTED");
    let mut bytes = vec![0; n];
    r.read_exact(&mut bytes).await?;
    let frame: LocalFrame = serde_json::from_slice(&bytes)?;
    frame.validate()?;
    Ok(frame)
}
pub async fn write_frame<W: AsyncWrite + Unpin>(w: &mut W, frame: &LocalFrame) -> Result<()> {
    frame.validate()?;
    let bytes = serde_json::to_vec(frame)?;
    ensure!(bytes.len() <= MAX_FRAME, "RESOURCE_EXHAUSTED");
    w.write_u32(bytes.len() as u32).await?;
    w.write_all(&bytes).await?;
    w.flush().await?;
    Ok(())
}

pub fn terminal(state: v1::TaskState) -> bool {
    use v1::TaskState::*;
    matches!(
        state,
        Succeeded | Failed | Expired | Unknown | RolledBack | RecoveryRequired | Cancelled
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn configuration_update_has_distinct_wire_identity_and_strict_typed_sets() {
        use prost::Message;
        let wire = v1::ReadCommand {
            operation: 12,
            kura_configuration_update: Some(v1::KuraConfigurationUpdate {
                pid: "p".into(),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(wire.encode_to_vec(), vec![16, 12, 74, 3, 10, 1, b'p']);
        let body = serde_json::json!({"pid":"p","expectedRevision":"a".repeat(64),"properties":[
            {"id":"count","value":{"text":"9223372036854775807"}},
            {"id":"enabled","value":{"boolean":false}},
            {"id":"password","value":{"text":"explicit-test-secret"}},
            {"id":"array","value":{"textArray":{"values":["1","2"]}}}
        ]});
        let update: v1::KuraConfigurationUpdate = serde_json::from_value(body.clone()).unwrap();
        let params = update.parameters(12345).unwrap();
        assert_eq!(params["properties"]["count"], "9223372036854775807");
        assert_eq!(params["properties"]["enabled"], false);
        assert_eq!(params["properties"]["array"], serde_json::json!(["1", "2"]));
        assert_eq!(serde_json::to_value(&update).unwrap(), body);
        assert!(
            v1::ReadCommand {
                operation: 12,
                expires_at_unix_ms: 12345,
                kura_configuration_update: Some(update.clone()),
                ..Default::default()
            }
            .valid_payload()
        );
        assert!(
            !v1::ReadCommand {
                operation: 11,
                expires_at_unix_ms: 12345,
                kura_configuration_update: Some(update.clone()),
                ..Default::default()
            }
            .valid_payload()
        );
        let mut duplicate = update.clone();
        duplicate.properties.push(duplicate.properties[0].clone());
        assert!(duplicate.parameters(12345).is_none());
        let mut no_value = update.clone();
        no_value.properties[0].value = None;
        assert!(no_value.parameters(12345).is_none());
        let mut excess = update.clone();
        excess.properties[0].value = Some(v1::kura_configuration_property::Value::Text(
            "x".repeat(16385),
        ));
        assert!(excess.parameters(12345).is_none());
        for value in [
            serde_json::json!({"text":42}),
            serde_json::json!({"text":"x","boolean":true}),
            serde_json::json!({"delete":true}),
        ] {
            let mut invalid = body.clone();
            invalid["properties"][0]["value"] = value;
            assert!(serde_json::from_value::<v1::KuraConfigurationUpdate>(invalid).is_err());
        }
        assert!(update.parameters(0).is_none());
    }
    #[test]
    fn snapshot_actions_require_decimal_ids_and_do_not_target_the_current_snapshot() {
        let save = v1::KuraSnapshotSave {
            expected_latest_id: "0".into(),
        };
        assert!(save.parameters(10).unwrap()["expectedLatestId"] == "0");
        assert!(save.parameters(0).is_none());
        assert!(
            !v1::KuraSnapshotSave {
                expected_latest_id: "01".into()
            }
            .valid()
        );
        assert!(
            !v1::KuraSnapshotSave {
                expected_latest_id: "9223372036854775807".into()
            }
            .valid()
        );
        let rollback = v1::KuraSnapshotRollback {
            snapshot_id: "2".into(),
            expected_latest_id: "4".into(),
        };
        assert!(rollback.valid());
        assert!(
            !v1::KuraSnapshotRollback {
                snapshot_id: "4".into(),
                expected_latest_id: "4".into()
            }
            .valid()
        );
        assert!(
            v1::KuraSnapshotRollback {
                snapshot_id: "0".into(),
                expected_latest_id: "4".into()
            }
            .valid()
        );
        assert!(
            !v1::KuraSnapshotRollback {
                snapshot_id: "5".into(),
                expected_latest_id: "4".into()
            }
            .valid()
        );
        assert!(
            !v1::KuraSnapshotRollback {
                snapshot_id: "0".into(),
                expected_latest_id: "0".into()
            }
            .valid()
        );
        assert!(
            v1::ReadCommand {
                operation: 13,
                expires_at_unix_ms: 10,
                kura_snapshot_save: Some(save),
                ..Default::default()
            }
            .valid_payload()
        );
        assert!(
            !v1::ReadCommand {
                operation: 14,
                expires_at_unix_ms: 10,
                kura_snapshot_save: Some(v1::KuraSnapshotSave {
                    expected_latest_id: "0".into()
                }),
                ..Default::default()
            }
            .valid_payload()
        );
        assert!(
            v1::ReadCommand {
                operation: 14,
                expires_at_unix_ms: 10,
                kura_snapshot_rollback: Some(rollback),
                ..Default::default()
            }
            .valid_payload()
        );
    }
    #[test]
    fn side_effects_require_explicit_read_classification() {
        for operation in [5, 6, 7, 10, 12, 13, 14, 99] {
            assert!(has_side_effect(operation));
        }
        for operation in [1, 2, 3, 4, 8, 9, 11, 15, 20, 23] {
            assert!(!has_side_effect(operation));
        }
    }
    #[test]
    fn reconciliation_has_an_additive_wire_field() {
        use prost::Message;
        let id = "00000000-0000-4000-8000-000000000001";
        let query = v1::ConsoleFrame {
            payload: Some(v1::console_frame::Payload::TaskQuery(v1::TaskQuery {
                command_ids: vec![id.into()],
            })),
        };
        let mut golden = vec![0x1a, 38, 0x0a, 36];
        golden.extend_from_slice(id.as_bytes());
        assert_eq!(query.encode_to_vec(), golden);
    }
    #[test]
    fn configuration_target_rejects_ambiguous_or_unbounded_input() {
        for (pid, locale, valid) in [
            ("test.pid", "en", true),
            ("测试.pid", "zh", true),
            (" ", "en", false),
            ("test\n.pid", "en", false),
            ("test.pid", "zh-CN", false),
        ] {
            assert_eq!(
                v1::KuraConfigurationTarget {
                    pid: pid.into(),
                    locale: locale.into()
                }
                .valid(),
                valid
            );
        }
        assert!(
            !v1::KuraConfigurationTarget {
                pid: "x".repeat(513),
                locale: "en".into()
            }
            .valid()
        );
        assert!(
            serde_json::from_value::<v1::KuraConfigurationTarget>(
                serde_json::json!({"pid":"test","locale":"en","execute":"bad"})
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<v1::KuraConfigurationTarget>(
                serde_json::json!({"pid":"test"})
            )
            .is_err()
        );
    }
    #[tokio::test]
    async fn framing_roundtrip_and_oversize_rejection() {
        let (mut a, mut b) = tokio::io::duplex(4096);
        let f = LocalFrame {
            version: 1,
            kind: "request".into(),
            request_id: "golden-1".into(),
            method: "agent.status".into(),
            params: None,
            result: None,
            error: None,
        };
        write_frame(&mut a, &f).await.unwrap();
        assert_eq!(read_frame(&mut b).await.unwrap().request_id, "golden-1");
        a.write_u32(MAX_FRAME as u32 + 1).await.unwrap();
        assert!(read_frame(&mut b).await.is_err());
    }
    #[test]
    fn unknown_version_fails_before_dispatch() {
        let f: LocalFrame = serde_json::from_str(
            r#"{"version":2,"kind":"request","requestId":"x","method":"system.reboot"}"#,
        )
        .unwrap();
        assert!(f.validate().is_err());
        assert!(!READ_METHODS.contains(&f.method.as_str()));
    }
    #[test]
    fn protobuf_golden_wire() {
        use prost::Message;
        let h = v1::Heartbeat {
            sequence: 1,
            kura_available: true,
            uptime_seconds: 10,
        };
        assert_eq!(h.encode_to_vec(), vec![8, 1, 16, 1, 24, 10]);
        assert!(terminal(v1::TaskState::Unknown));
        assert!(!terminal(v1::TaskState::Running));
    }
}

#[cfg(test)]
mod system_control_contract_tests {
    use super::*;
    #[test]
    fn fixed_system_control_is_additive_bounded_and_has_no_arbitrary_target_payload() {
        use v1::*;
        assert_eq!(Operation::ServiceStatus as i32, 23);
        assert_eq!(Operation::ServiceRestartPrepare as i32, 24);
        assert_eq!(Operation::ServiceRestart as i32, 25);
        assert_eq!(Operation::SystemRebootPrepare as i32, 26);
        assert_eq!(Operation::SystemReboot as i32, 27);
        assert_eq!(Operation::SystemControlCancel as i32, 28);
        let action = SystemControlAction {
            transaction_id: format!("system-{}", uuid::Uuid::new_v4()),
            confirmation_token: uuid::Uuid::new_v4().to_string(),
        };
        assert!(action.valid());
        for op in 1..=29 {
            let c = ReadCommand {
                operation: op,
                system_control_action: Some(action.clone()),
                ..Default::default()
            };
            assert_eq!(c.valid_payload(), matches!(op, 25 | 27 | 28));
        }
        for op in [23, 24, 26] {
            assert!(
                ReadCommand {
                    operation: op,
                    ..Default::default()
                }
                .valid_payload()
            );
        }
        for op in [24, 25, 26, 27, 28, 29] {
            assert!(has_side_effect(op));
        }
        assert!(!has_side_effect(23));
        let mut bad = action.clone();
        bad.transaction_id = "system-../../target".into();
        assert!(!bad.valid());
        bad = action.clone();
        bad.confirmation_token = "not-a-token".into();
        assert!(!bad.valid());
        let mut raw = serde_json::to_value(action).unwrap();
        raw["unit"] = serde_json::json!("other.service");
        assert!(serde_json::from_value::<SystemControlAction>(raw).is_err());
    }
}

#[cfg(test)]
mod boot_contract_tests {
    use super::*;
    #[test]
    fn ordinary_handoff_is_a_distinct_fixed_additive_contract() {
        use prost::Message;
        use v1::*;
        let action = OrdinaryHandoffAction {
            transaction_id: "handoff-11111111-1111-4111-8111-111111111111".into(),
            confirmation_token: "22222222-2222-4222-8222-222222222222".into(),
        };
        assert!(action.valid());
        for op in [20, 21, 22, 29] {
            let c = ReadCommand {
                operation: op,
                ordinary_handoff_action: matches!(op, 22 | 29).then_some(action.clone()),
                ..Default::default()
            };
            assert!(c.valid_payload());
            assert_eq!(has_side_effect(op), op != 20);
            let bad = ReadCommand {
                ordinary_handoff_action: (!matches!(op, 22 | 29)).then_some(action.clone()),
                ..c
            };
            assert!(!bad.valid_payload());
        }
        assert_eq!(
            ReadCommand {
                operation: 20,
                ..Default::default()
            }
            .encode_to_vec(),
            [16, 20]
        );
        assert_eq!(Operation::OrdinaryHandoffExecute as i32, 22);
        assert_eq!(Operation::OrdinaryHandoffCancel as i32, 29);
        assert!(
            !OrdinaryHandoffAction {
                transaction_id: "boot-11111111-1111-4111-8111-111111111111".into(),
                ..action.clone()
            }
            .valid()
        );
        assert!(serde_json::from_value::<OrdinaryHandoffAction>(serde_json::json!({"transactionId":action.transaction_id,"confirmationToken":action.confirmation_token,"unit":"other.service"})).is_err());
    }
    #[test]
    fn boot_operations_are_additive_fixed_and_conservative() {
        use v1::*;
        let action = OrdinaryBootAction {
            transaction_id: "boot-11111111-1111-4111-8111-111111111111".into(),
            confirmation_token: "22222222-2222-4222-8222-222222222222".into(),
        };
        assert!(action.valid());
        for op in 15..=19 {
            assert_eq!(has_side_effect(op), op != 15);
            let c = ReadCommand {
                operation: op,
                ordinary_boot_action: matches!(op, 17 | 18).then_some(action.clone()),
                ..Default::default()
            };
            assert!(c.valid_payload());
            let extra = ReadCommand {
                ordinary_boot_action: if matches!(op, 17 | 18) {
                    None
                } else {
                    Some(action.clone())
                },
                ..c
            };
            assert!(!extra.valid_payload());
        }
        assert!(
            !OrdinaryBootAction {
                transaction_id: "boot-../unit".into(),
                ..action.clone()
            }
            .valid()
        );
        assert!(serde_json::from_value::<OrdinaryBootAction>(serde_json::json!({"transactionId":action.transaction_id,"confirmationToken":action.confirmation_token,"unit":"x.service"})).is_err());
        assert_eq!(Operation::OrdinaryBootStatus as i32, 15);
        assert_eq!(Operation::OrdinaryBootReconcile as i32, 19);
    }
}
