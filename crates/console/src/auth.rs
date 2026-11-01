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
    let row=sqlx::query("SELECT u.id,u.tenant_id,u.username,u.role,s.csrf FROM mgmt_sessions s JOIN mgmt_users u ON u.id=s.user_id WHERE s.token_hash=$1 AND s.expires_at>now() AND u.enabled")
        .bind(hash(token.as_bytes())).fetch_optional(pool).await?.ok_or(ApiError::unauthorized())?;
    Ok(Principal {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        username: row.get("username"),
        role: row.get("role"),
        csrf: row.get("csrf"),
    })
}
pub fn origin(headers: &HeaderMap, expected: &str) -> Result<(), ApiError> {
    if headers.get("origin").and_then(|v| v.to_str().ok()) != Some(expected) {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "FORBIDDEN"));
    }
    Ok(())
}
pub fn csrf(headers: &HeaderMap, p: &Principal, expected: &str) -> Result<(), ApiError> {
    origin(headers, expected)?;
    if headers.get("x-csrf-token").and_then(|v| v.to_str().ok()) != Some(p.csrf.as_str()) {
        return Err(ApiError::new(StatusCode::FORBIDDEN, "FORBIDDEN"));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
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
        };
        assert!(p.can_query());
        assert!(!p.can_command());
        let mut headers = HeaderMap::new();
        headers.insert("origin", "https://evil.invalid".parse().unwrap());
        assert!(csrf(&headers, &p, "https://localhost").is_err());
    }
}
