use crate::http::ApiError;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::http::{HeaderMap, StatusCode};
use rand_core::{OsRng, RngCore};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub fn random_token() -> String {
    let mut b = [0u8; 32];
    OsRng.fill_bytes(&mut b);
    hex::encode(b)
}
pub fn hash(value: &[u8]) -> String {
    hex::encode(Sha256::digest(value))
}
/// Seeding-only hash: the well-known initial password is intentionally short and
/// every session that uses it is forced through a real password change.
pub fn password_hash_unchecked(value: &str) -> anyhow::Result<String> {
    Ok(Argon2::default()
        .hash_password(value.as_bytes(), &SaltString::generate(&mut OsRng))?
        .to_string())
}
pub fn password_hash(value: &str) -> anyhow::Result<String> {
    anyhow::ensure!(
        value.len() >= 12 && value.len() <= 256,
        "password length must be 12..256 bytes"
    );
    Ok(Argon2::default()
        .hash_password(value.as_bytes(), &SaltString::generate(&mut OsRng))?
        .to_string())
}
pub fn password_valid(value: &str, encoded: &str) -> bool {
    PasswordHash::new(encoded).is_ok_and(|p| {
        Argon2::default()
            .verify_password(value.as_bytes(), &p)
            .is_ok()
    })
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Principal {
    pub id: Uuid,
    pub tenant_id: String,
    pub username: String,
    pub role: String,
    pub csrf: String,
    pub must_change_password: bool,
}
impl Principal {
    pub fn can_query(&self) -> bool {
        ["viewer", "operator", "admin", "recovery"].contains(&self.role.as_str())
    }
    pub fn can_command(&self) -> bool {
        ["operator", "admin"].contains(&self.role.as_str())
    }
}
pub fn cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get("cookie")?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|p| p.trim().strip_prefix("kura_session=").map(str::to_owned))
}
pub async fn principal(pool: &PgPool, headers: &HeaderMap) -> Result<Principal, ApiError> {
    let token = cookie(headers).ok_or(ApiError::unauthorized())?;
    if token.len() != 64 {
        return Err(ApiError::unauthorized());
    }
    let row=sqlx::query("SELECT u.id,u.tenant_id,u.username,u.role,s.csrf,u.must_change_password FROM mgmt_sessions s JOIN mgmt_users u ON u.id=s.user_id WHERE s.token_hash=$1 AND s.expires_at>now() AND u.enabled")
        .bind(hash(token.as_bytes())).fetch_optional(pool).await?.ok_or(ApiError::unauthorized())?;
    Ok(Principal {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        username: row.get("username"),
        role: row.get("role"),
        csrf: row.get("csrf"),
        must_change_password: row.get("must_change_password"),
    })
}
pub fn origin(headers: &HeaderMap, expected: &str) -> Result<(), ApiError> {
    let mut origins = headers.get_all("origin").iter();
    let received = origins.next().and_then(|v| v.to_str().ok());
    if origins.next().is_some() || !received.is_some_and(|value| same_origin(value, expected)) {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "FORBIDDEN"));
    }
    Ok(())
}

fn same_origin(received: &str, expected: &str) -> bool {
    use axum::http::Uri;
    let (Ok(received), Ok(expected)) = (received.parse::<Uri>(), expected.parse::<Uri>()) else {
        return false;
    };
    let genuine_origin = |uri: &Uri| {
        matches!(uri.scheme_str(), Some("http" | "https"))
            && uri.authority().is_some_and(|authority| !authority.as_str().contains('@'))
            && uri.path() == "/"
            && uri.query().is_none()
    };
    genuine_origin(&received)
        && genuine_origin(&expected)
        && received.scheme() == expected.scheme()
        && received.port_u16() == expected.port_u16()
        && received.host().zip(expected.host()).is_some_and(|(a, b)| a.eq_ignore_ascii_case(b))
}

#[test]
fn browser_origin_normalizes_dns_case_without_changing_authority() {
    let expected = "https://Example-Gateway.local:19443";
    assert!(same_origin("https://example-gateway.local:19443", expected));
    for other in [
        "http://example-gateway.local:19443",
        "https://example-gateway.local:19444",
        "https://other.local:19443",
        "https://example-gateway.local:19443/settings",
        "null",
    ] {
        assert!(!same_origin(other, expected), "{other}");
    }
}
pub fn csrf(headers: &HeaderMap, p: &Principal, expected: &str) -> Result<(), ApiError> {
    origin(headers, expected)?;
    if headers.get("x-csrf-token").and_then(|v| v.to_str().ok()) != Some(p.csrf.as_str()) {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "FORBIDDEN"));
    }
    Ok(())
}
/// Seed the well-known bootstrap administrator exactly once: only when the
/// deployment has no enabled admin at all. admin/admin must be rotated at first
/// login (must_change_password) before the session is allowed to do anything.
pub async fn ensure_bootstrap_admin(pool: &PgPool) -> anyhow::Result<()> {
    let existing: i64 =
        sqlx::query_scalar("SELECT count(*) FROM mgmt_users WHERE enabled AND role='admin'")
            .fetch_one(pool)
            .await?;
    if existing > 0 {
        return Ok(());
    }
    let encoded = tokio::task::spawn_blocking(|| password_hash_unchecked("admin"))
        .await
        .map_err(|_| anyhow::anyhow!("bootstrap hash join"))??;
    let mut tx = pool.begin().await?;
    let inserted: Option<Uuid> = sqlx::query_scalar(
        "INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) \
         VALUES($1,'default','admin',$2,'admin',true) \
         ON CONFLICT (username) DO NOTHING RETURNING id",
    )
    .bind(Uuid::new_v4())
    .bind(&encoded)
    .fetch_optional(&mut *tx)
    .await?;
    if inserted.is_some() {
        sqlx::query("INSERT INTO mgmt_audit(tenant_id,actor,operation,target,outcome) VALUES('default','system','user.bootstrap','admin','SUCCEEDED')")
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bootstrap_seed_uses_unchecked_hash() {
        // "admin" is far below the normal strength floor by design and must
        // never pass the regular hashing path.
        assert!(password_hash("admin").is_err());
        let seeded = password_hash_unchecked("admin").unwrap();
        assert!(password_valid("admin", &seeded));
        assert!(!password_valid("Admin", &seeded));
    }

    #[test]
    fn passwords_and_roles() {
        assert!(password_hash("short").is_err());
        let h = password_hash("a-long-unique-password").unwrap();
        assert!(password_valid("a-long-unique-password", &h));
        assert!(!password_valid("wrong", &h));
        let p = Principal {
            id: Uuid::nil(),
            tenant_id: "a".into(),
            username: "v".into(),
            role: "viewer".into(),
            csrf: "x".into(),
            must_change_password: false,
        };
        assert!(p.can_query());
        assert!(!p.can_command());
        let mut headers = HeaderMap::new();
        headers.insert("origin", "https://evil.invalid".parse().unwrap());
        assert!(csrf(&headers, &p, "https://localhost").is_err());
    }
}
