use anyhow::{Result, ensure};

pub const ED25519_PREFIX: [u8; 12] = [
    0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
];

pub fn ed25519_public_raw(key: &ring::signature::Ed25519KeyPair) -> Result<&[u8]> {
    use ring::signature::KeyPair;
    Ok(key.public_key().as_ref())
}

pub fn ed25519_public_from_pem_path(path: &str) -> Result<[u8; 32]> {
    let document = std::fs::read(path)?;
    if let Ok(text) = std::str::from_utf8(&document) {
        return ed25519_public_from_pem(text);
    }
    ensure!(
        document.len() == 44 && document[..12] == ED25519_PREFIX,
        "expected Ed25519 PEM or DER public key"
    );
    Ok(document[12..].try_into()?)
}

pub fn ed25519_public_from_pem(document: &str) -> Result<[u8; 32]> {
    let mut lines = document.lines().filter(|line| !line.trim().is_empty());
    ensure!(
        lines
            .next()
            .is_some_and(|line| line == "-----BEGIN PUBLIC KEY-----"),
        "expected PEM public key"
    );
    let mut encoded = String::new();
    loop {
        match lines.next() {
            Some(line) if line != "-----END PUBLIC KEY-----" => encoded.push_str(line),
            Some("-----END PUBLIC KEY-----") => break,
            _ => anyhow::bail!("unexpected PEM public key trailer"),
        }
    }
    ensure!(lines.next().is_none(), "unexpected PEM public key trailer");
    let der = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        encoded.as_bytes(),
    )
    .map_err(|_| anyhow::anyhow!("invalid PEM public key encoding"))?;
    ensure!(
        der.len() == 44 && der[..12] == ED25519_PREFIX,
        "expected Ed25519 PEM public key"
    );
    Ok(der[12..].try_into()?)
}

pub fn ed25519_public_der(raw: &[u8]) -> Result<Vec<u8>> {
    ensure!(raw.len() == 32, "invalid Ed25519 public key");
    let mut der = ED25519_PREFIX.to_vec();
    der.extend_from_slice(raw);
    Ok(der)
}

pub fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub fn valid_transfer_endpoint(value: &str) -> bool {
    if value.len() > 512 || !value.starts_with("https://") || value.contains(['?', '#']) {
        return false;
    }
    let Ok(uri) = value.parse::<axum::http::Uri>() else {
        return false;
    };
    let Some(authority) = uri.authority() else {
        return false;
    };
    let host = authority.host();
    let text = authority.as_str();
    let port = if text.starts_with('[') {
        text.split_once(']')
            .map(|(_, tail)| tail)
            .unwrap_or("invalid")
    } else {
        text.strip_prefix(host).unwrap_or("invalid")
    };
    let valid_port = port.is_empty()
        || port.strip_prefix(':').is_some_and(|number| {
            !number.is_empty()
                && number.bytes().all(|b| b.is_ascii_digit())
                && number.parse::<u16>().is_ok_and(|p| p > 0)
        });
    uri.scheme_str() == Some("https")
        && !host.is_empty()
        && authority
            .as_str()
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':' | '[' | ']'))
        && matches!(
            uri.path_and_query().map(|p| p.as_str()),
            None | Some("/") | Some("")
        )
        && valid_port
}

#[derive(Clone, Copy)]
pub enum ApprovalAction {
    Approve,
    Reject,
    Revoke,
}
impl ApprovalAction {
    pub fn parse(action: &str) -> Option<Self> {
        match action {
            "approve" => Some(Self::Approve),
            "reject" => Some(Self::Reject),
            "revoke" => Some(Self::Revoke),
            _ => None,
        }
    }
    pub fn status(self) -> &'static str {
        match self {
            Self::Approve => "APPROVED",
            Self::Reject => "REJECTED",
            Self::Revoke => "REVOKED",
        }
    }
    fn audit(self) -> &'static str {
        match self {
            Self::Approve => "transfer.approval.approve",
            Self::Reject => "transfer.approval.reject",
            Self::Revoke => "transfer.approval.revoke",
        }
    }
}

/// Shared by HTTP and the root CLI. Actor permissions, tenant membership,
/// separation of duties, transition and audit are committed together.
pub async fn decide_approval(
    pool: &sqlx::PgPool,
    tenant: Option<&str>,
    id: uuid::Uuid,
    actor: uuid::Uuid,
    action: ApprovalAction,
) -> Result<Option<String>> {
    use sqlx::Row;
    let mut tx = pool.begin().await?;
    let sql = match action {
        ApprovalAction::Approve => {
            "UPDATE mgmt_transfer_approvals a SET approved_by=$2,approved_at=now(),status='APPROVED' FROM mgmt_users u WHERE a.id=$1 AND a.status='PENDING' AND a.approved_by IS NULL AND u.id=$2 AND u.enabled AND u.tenant_id=a.tenant_id AND u.role='admin' AND u.id<>a.requested_by AND ($3::text IS NULL OR a.tenant_id=$3) RETURNING a.tenant_id,a.gateway_id"
        }
        ApprovalAction::Reject | ApprovalAction::Revoke => {
            "UPDATE mgmt_transfer_approvals a SET status=$4,closed_at=now() FROM mgmt_users u WHERE a.id=$1 AND a.status IN ('PENDING','APPROVED') AND u.id=$2 AND u.enabled AND u.tenant_id=a.tenant_id AND u.role='admin' AND ($3::text IS NULL OR a.tenant_id=$3) RETURNING a.tenant_id,a.gateway_id"
        }
    };
    let query = sqlx::query(sql).bind(id).bind(actor).bind(tenant);
    let query = if matches!(action, ApprovalAction::Approve) {
        query
    } else {
        query.bind(action.status())
    };
    let Some(row) = query.fetch_optional(&mut *tx).await? else {
        return Ok(None);
    };
    let gateway: String = row.get("gateway_id");
    let tenant: String = row.get("tenant_id");
    crate::db::audit(
        &mut *tx,
        &tenant,
        &actor.to_string(),
        action.audit(),
        &gateway,
        "SUCCEEDED",
    )
    .await?;
    tx.commit().await?;
    Ok(Some(gateway))
}

#[cfg(test)]
mod endpoint_tests {
    use super::*;
    #[test]
    fn transfer_destination_is_a_plain_https_authority() {
        for value in [
            "https://center.example:18444",
            "https://127.0.0.1:443/",
            "https://[fd00::1]:8443",
        ] {
            assert!(valid_transfer_endpoint(value), "{value}");
        }
        for value in [
            "https://",
            "http://center",
            "https://user@center",
            "https://center/path",
            "https://center?token=x",
            "https://center#x",
            "https://center:0",
            "https://center:99999",
            "https://center:abc",
            "https://center/\n",
        ] {
            assert!(!valid_transfer_endpoint(value), "{value}");
        }
    }
    #[tokio::test]
    #[ignore = "requires explicit isolated PostgreSQL URL; creates a disposable schema"]
    async fn postgres_transfer_decisions_share_authority_and_rollback_when_audit_fails() {
        use sqlx::{PgPool, Row};
        use uuid::Uuid;
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("transfer_decision_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let db = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(options)
            .await
            .unwrap();
        sqlx::migrate!("../../migrations").run(&db).await.unwrap();
        let actors = [
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        ];
        for (id, tenant, role) in [
            (actors[0], "a", "admin"),
            (actors[1], "a", "admin"),
            (actors[2], "b", "admin"),
            (actors[3], "a", "viewer"),
        ] {
            sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,$2,$3,'unused',$4)").bind(id).bind(tenant).bind(id.to_string()).bind(role).execute(&db).await.unwrap();
        }
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation) VALUES('gateway','a','gateway','unused',1)").execute(&db).await.unwrap();
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_transfer_approvals(id,tenant_id,gateway_id,expected_generation,to_generation,new_gateway_id,new_endpoint,new_client_ca_sha256,new_client_certificate_sha256,requested_by) VALUES($1,'a','gateway',1,2,'gateway','https://other.invalid','ca','cert',$2)").bind(id).bind(actors[0]).execute(&db).await.unwrap();
        for actor in [actors[0], actors[2], actors[3]] {
            assert!(
                decide_approval(&db, None, id, actor, ApprovalAction::Approve)
                    .await
                    .unwrap()
                    .is_none()
            );
        }
        assert!(
            decide_approval(&db, Some("b"), id, actors[1], ApprovalAction::Approve)
                .await
                .unwrap()
                .is_none()
        );
        sqlx::query("ALTER TABLE mgmt_audit ADD CONSTRAINT reject_approval CHECK(operation<>'transfer.approval.approve')").execute(&db).await.unwrap();
        assert!(
            decide_approval(&db, Some("a"), id, actors[1], ApprovalAction::Approve)
                .await
                .is_err()
        );
        let row = sqlx::query("SELECT status,approved_by FROM mgmt_transfer_approvals WHERE id=$1")
            .bind(id)
            .fetch_one(&db)
            .await
            .unwrap();
        assert_eq!(row.get::<String, _>("status"), "PENDING");
        assert!(row.get::<Option<Uuid>, _>("approved_by").is_none());
        sqlx::query("ALTER TABLE mgmt_audit DROP CONSTRAINT reject_approval")
            .execute(&db)
            .await
            .unwrap();
        assert_eq!(
            decide_approval(&db, Some("a"), id, actors[1], ApprovalAction::Approve)
                .await
                .unwrap(),
            Some("gateway".into())
        );
        assert!(
            decide_approval(&db, None, id, actors[1], ApprovalAction::Approve)
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            decide_approval(&db, None, id, actors[0], ApprovalAction::Revoke)
                .await
                .unwrap()
                .is_some()
        );
        assert!(
            decide_approval(&db, None, id, actors[0], ApprovalAction::Reject)
                .await
                .unwrap()
                .is_none()
        );
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM mgmt_audit WHERE operation LIKE 'transfer.approval.%'",
        )
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(count, 2);
        db.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}
