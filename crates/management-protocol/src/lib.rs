pub mod v1 {
    tonic::include_proto!("yofc.management.v1");
}

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const PROTOCOL_MAJOR: u32 = 1;
pub const MAX_FRAME: usize = 1_048_576;
// Unrecognized future operations are conservative until explicitly classified.
pub fn has_side_effect(operation: i32) -> bool {
    !matches!(operation, 1 | 2 | 3 | 4 | 8 | 9 | 11)
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

impl v1::ReadCommand {
    pub fn valid_payload(&self) -> bool {
        (self.operation == 5) == self.network_change.is_some()
            && matches!(self.operation, 6..=8) == self.network_action.is_some()
            && matches!(self.operation, 9 | 10) == self.network_takeover.is_some()
            && (self.operation == 11) == self.kura_configuration.is_some()
            && (self.operation == 12) == self.kura_configuration_update.is_some()
            && self.kura_configuration.as_ref().is_none_or(|v| v.valid())
            && self
                .kura_configuration_update
                .as_ref()
                .is_none_or(|v| v.parameters(self.expires_at_unix_ms).is_some())
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
    fn side_effects_require_explicit_read_classification() {
        for operation in [5, 6, 7, 10, 12, 99] {
            assert!(has_side_effect(operation));
        }
        for operation in [1, 2, 3, 4, 8, 9, 11] {
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
