//! Center-side enrollment only. Never sends identity or ownership changes to an agent.
use anyhow::{Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use sqlx::PgPool;

pub fn certificate_sha256(value: &str) -> Result<String> {
    ensure!(value.len() <= 32768, "certificate too large");
    let pem = value.trim();
    let encoded = pem
        .strip_prefix("-----BEGIN CERTIFICATE-----")
        .and_then(|v| v.strip_suffix("-----END CERTIFICATE-----"))
        .ok_or_else(|| anyhow::anyhow!("one public certificate required"))?;
    let encoded: String = encoded
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let der = STANDARD.decode(&encoded)?;
    ensure!(STANDARD.encode(&der) == encoded, "noncanonical certificate");
    let cert = rustls::pki_types::CertificateDer::from(der.as_slice());
    rustls::server::ParsedCertificate::try_from(&cert)?;
    crate::tls_store::validate_leaf_certificate(value)
}

pub fn validate(id: &str, tenant: &str, name: &str) -> Result<()> {
    ensure!(crate::transfer::valid_identifier(id), "invalid gateway id");
    ensure!(
        !tenant.is_empty() && tenant.len() <= 128 && !tenant.chars().any(char::is_control),
        "invalid tenant"
    );
    ensure!(
        !name.trim().is_empty() && name.len() <= 256 && !name.chars().any(char::is_control),
        "invalid name"
    );
    Ok(())
}

pub async fn register(
    pool: &PgPool,
    id: &str,
    tenant: &str,
    name: &str,
    fingerprint: &str,
    actor: &str,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    register_in_transaction(&mut tx, id, tenant, name, fingerprint, actor).await?;
    tx.commit().await
}

pub async fn register_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: &str,
    tenant: &str,
    name: &str,
    fingerprint: &str,
    actor: &str,
) -> Result<(), sqlx::Error> {
    // Plain INSERT: neither ordinary registration nor duplicate requests can replace a binding.
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation) VALUES($1,$2,$3,$4,1)")
        .bind(id).bind(tenant).bind(name).bind(fingerprint).execute(&mut **tx).await?;
    crate::db::audit(
        &mut **tx,
        tenant,
        actor,
        "gateway.register",
        id,
        "SUCCEEDED",
    )
    .await
    .map_err(|_| sqlx::Error::Protocol("registration audit failed".into()))?;
    crate::db::event(
        &mut **tx,
        tenant,
        "gateway",
        id,
        0,
        serde_json::json!({"id":id,"registered":true}),
    )
    .await
    .map_err(|_| sqlx::Error::Protocol("registration event failed".into()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_public_certificate_rejects_extra_material_and_invalid_der() {
        let pem = include_str!("../tests/fixtures/registration-client.pem");
        assert!(certificate_sha256(pem).is_ok());
        for invalid in [
            format!("{pem}{pem}"),
            format!("garbage{pem}"),
            format!("{pem}-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----"),
            "-----BEGIN CERTIFICATE-----\nAAAA\n-----END CERTIFICATE-----".into(),
        ] {
            assert!(certificate_sha256(&invalid).is_err());
        }
        assert!(validate("gateway-1", "tenant", "Gateway 网关").is_ok());
        for id in ["", "../gateway", "gateway/1", "gateway 1"] {
            assert!(validate(id, "tenant", "Gateway").is_err());
        }
        assert!(validate("gateway", "tenant", " ").is_err());
        assert!(validate("gateway", "tenant", "name\nlog").is_err());
    }
}
