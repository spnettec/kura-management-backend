mod auth;
mod config;
mod db;
mod firewall_tasks;
mod nm_tasks;
mod gateway_registration;
mod grpc;
mod http;
mod provisioning;
mod settings;
mod settings_files;
mod client_ca_store;
mod tls_store;
mod task_crypto;
mod task_key_store;
mod task_lifecycle;
mod transfer;
use anyhow::{Context, Result, ensure};
use ring::signature::{Ed25519KeyPair, KeyPair};
use sqlx::Row;
use std::{
    env, fs,
    io::Write,
    path::Path,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Semaphore;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use transfer::{ed25519_public_der, ed25519_public_from_pem_path, ed25519_public_raw};
use uuid::Uuid;
use yofc_management_protocol::{MAX_FRAME, v1::control_server::ControlServer};

fn base64_url_no_pad(value: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    for chunk in value.chunks(3) {
        let byte = |index: usize| -> u32 { *chunk.get(index).unwrap_or(&0) as u32 };
        let number = (byte(0) << 16) | (byte(1) << 8) | byte(2);
        out.push(ALPHABET[(number >> 18) as usize & 63] as char);
        out.push(ALPHABET[(number >> 12) as usize & 63] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[(number >> 6) as usize & 63] as char);
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[number as usize & 63] as char);
        }
    }
    out
}

fn write_new(path: &Path, value: &[u8], mode: u32) -> Result<()> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    if path.exists() {
        anyhow::bail!("refusing to overwrite {}", path.display());
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .with_context(|| format!("create {}", path.display()))?;
    file.set_permissions(fs::Permissions::from_mode(mode))?;
    file.write_all(value)?;
    file.sync_all()?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}

fn certificate_hash(path: &str) -> Result<String> {
    let data = fs::read(path)?;
    let cert = rustls_pemfile::certs(&mut data.as_slice())
        .next()
        .context("no certificate")??;
    Ok(auth::hash(cert.as_ref()))
}

fn transfer_key_pair(path: &str) -> Result<Ed25519KeyPair> {
    let document = fs::read(path)?;
    Ed25519KeyPair::from_pkcs8_maybe_unchecked(&document)
        .or_else(|_| {
            ensure!(
                document.len() == 83 && document[..2] == [0x30, 0x51],
                "unsupported legacy Ed25519 private key format"
            );
            let key_pair = Ed25519KeyPair::from_seed_unchecked(&document[16..48])
                .map_err(|_| anyhow::anyhow!("invalid legacy Ed25519 seed"))?;
            Ok(key_pair)
        })
        .map_err(|_| anyhow::anyhow!("read Ed25519 transfer signer {}", path))
}

fn signer_sha256(path: &str) -> Result<String> {
    Ok(auth::hash(&ed25519_public_from_pem_path(path)?))
}

fn transfer_expires_at_unix_ms(now_ms: i64, expires_in_seconds: u64) -> Result<i64> {
    ensure!(
        (1..=900).contains(&expires_in_seconds),
        "transfer token validity must be 1..=900 seconds"
    );
    Ok(now_ms + (expires_in_seconds * 1000) as i64)
}
#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args: Vec<String> = env::args().collect();
    if args.get(1).map(String::as_str) == Some("generate-transfer-key") {
        ensure!(
            args.len() == 4,
            "usage: generate-transfer-key PRIVATE_KEY_DER PUBLIC_KEY_PEM"
        );
        let rng = ring::rand::SystemRandom::new();
        let document = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| anyhow::anyhow!("generate Ed25519 transfer signer"))?;
        write_new(Path::new(&args[2]), document.as_ref(), 0o600)?;
        let key = Ed25519KeyPair::from_pkcs8_maybe_unchecked(document.as_ref())
            .map_err(|_| anyhow::anyhow!("read generated Ed25519 transfer signer"))?;
        let mut public_der = vec![
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
        ];
        public_der.extend_from_slice(key.public_key().as_ref());
        let encoded =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_der);
        let mut public = String::from("-----BEGIN PUBLIC KEY-----\n");
        for chunk in encoded.as_bytes().chunks(64) {
            public.push_str(std::str::from_utf8(chunk)?);
            public.push('\n');
        }
        public.push_str("-----END PUBLIC KEY-----\n");
        write_new(Path::new(&args[3]), public.as_bytes(), 0o644)?;
        println!("Transfer signer generated; import the public key with agent import-identity");
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("issue-transfer-signer-rotation") {
        ensure!(
            args.len() == 10,
            "usage: issue-transfer-signer-rotation GATEWAY_ID EXPECTED_GENERATION NEW_PUBLIC_KEY_PEM OPERATOR EXPIRES_IN_SECONDS CURRENT_PRIVATE_KEY_DER CURRENT_PUBLIC_KEY_PEM TOKEN_FILE"
        );
        let gateway_id = &args[2];
        ensure!(transfer::valid_identifier(gateway_id), "invalid gateway id");
        let expected_generation = args[3]
            .parse::<i64>()
            .context("expected generation must be an integer")?;
        ensure!(
            expected_generation >= 1,
            "expected generation must be positive"
        );
        let operator = &args[5];
        ensure!(
            !operator.is_empty() && operator.len() <= 128 && !operator.contains(['\r', '\n', '\0']),
            "operator must be a bounded safe identity"
        );
        let expires_in = args[6]
            .parse::<u64>()
            .context("expires seconds must be an integer")?;
        let key_pair = transfer_key_pair(&args[7])?;
        let new_public = ed25519_public_from_pem_path(&args[4])?;
        let current_public = ed25519_public_from_pem_path(&args[8])?;
        ensure!(
            auth::hash(ed25519_public_raw(&key_pair)?) == auth::hash(&current_public),
            "current public key does not match the private rotation signer"
        );
        let current_spki = auth::hash(&ed25519_public_der(&current_public)?);
        let new_spki = auth::hash(&ed25519_public_der(&new_public)?);
        ensure!(
            current_spki != new_spki,
            "the proposed transfer signer is unchanged"
        );
        let now_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;
        let expires_ms = transfer_expires_at_unix_ms(now_ms, expires_in)?;
        let rotation_id = Uuid::new_v4();
        let payload = serde_json::json!({
            "schema":"yofc.agent.transfer-signer-rotation.v1",
            "rotationId":rotation_id,
            "gatewayId":gateway_id,
            "bindingGeneration":expected_generation,
            "fromSignerSpkiSha256":current_spki,
            "toSignerSpkiSha256":new_spki,
            "operator":operator,
            "expiresAtUnixMs":expires_ms
        });
        let payload_bytes = serde_json::to_vec(&payload)?;
        let signature = key_pair.sign(&payload_bytes);
        let token = format!(
            "YOFC-AGENT-SIGNER-ROTATION-v1\n{}\n{}\n",
            base64_url_no_pad(&payload_bytes),
            hex::encode(signature.as_ref())
        );
        write_new(Path::new(&args[9]), token.as_bytes(), 0o600)?;
        println!("One-time signer rotation token written; deliver out of logs and command history");
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("task-key-init") {
        ensure!(args.len() == 3, "usage: task-key-init ABSOLUTE_KEY_PATH");
        return task_crypto::create_key(std::path::Path::new(&args[2]));
    }
    if args.get(1).map(String::as_str) == Some("task-key-backup") {
        ensure!(
            args.len() == 5,
            "usage: task-key-backup KEY_FILE BACKUP_FILE PASSPHRASE_FILE"
        );
        return task_crypto::backup_key(
            std::path::Path::new(&args[2]),
            std::path::Path::new(&args[3]),
            std::path::Path::new(&args[4]),
        );
    }
    if args.get(1).map(String::as_str) == Some("task-key-restore") {
        ensure!(
            args.len() == 5,
            "usage: task-key-restore BACKUP_FILE KEY_FILE PASSPHRASE_FILE"
        );
        return task_crypto::restore_key(
            std::path::Path::new(&args[2]),
            std::path::Path::new(&args[3]),
            std::path::Path::new(&args[4]),
        );
    }
    if args.get(1).map(String::as_str) == Some("task-key-rotate") {
        ensure!(
            args.len() == 4,
            "usage: task-key-rotate CURRENT_KEY_FILE NEW_KEY_FILE"
        );
        let old_path = std::path::Path::new(&args[2]);
        let new_path = std::path::Path::new(&args[3]);
        let old_bytes = fs::read(old_path).context("current task data key unavailable")?;
        let new_bytes = fs::read(new_path).context("new task data key unavailable")?;
        ensure!(
            old_bytes != new_bytes,
            "new task data key must differ from the current key"
        );
        let old = task_crypto::TaskCipher::load(old_path)?;
        let new = task_crypto::TaskCipher::load(new_path)?;
        let pool =
            db::connect(&env::var("DATABASE_URL").context("DATABASE_URL is required")?).await?;
        let rotated = old.rotate_storage(&new, &pool).await?;
        db::audit(
            &pool,
            "dev",
            "local-admin",
            "task.key.rotate",
            "singleton",
            "SUCCEEDED",
        )
        .await?;
        tracing::info!(rotated_tasks = rotated, "task storage key rotated");
        return Ok(());
    }
    let key_path = env::var("CONSOLE_DATA_KEY_FILE").context("CONSOLE_DATA_KEY_FILE is required")?;
    let pool = db::connect(&env::var("DATABASE_URL").context("DATABASE_URL is required")?).await?;
    let state = env::var("CONSOLE_STATE_DIR").unwrap_or("/var/lib/kura-management-console".into());
    let cipher = task_key_store::recover(&pool, Path::new(&key_path), Path::new(&state)).await?;
    let migrated = cipher.prepare_storage(&pool).await?;
    tracing::info!(migrated_tasks = migrated, "task storage key verified");
    auth::ensure_bootstrap_admin(&pool).await?;
    match args.get(1).map(String::as_str).unwrap_or("serve") {
        "migrate" => return Ok(()),
        "create-user" => {
            ensure!(
                args.len() == 6,
                "usage: create-user USERNAME TENANT ROLE PASSWORD_FILE"
            );
            ensure!(
                ["admin", "operator", "viewer", "recovery"].contains(&args[4].as_str()),
                "invalid role"
            );
            let password = fs::read_to_string(&args[5])?;
            let hash = auth::password_hash(password.trim_end_matches(['\r', '\n']))?;
            ensure!(
                !args[2].is_empty() && args[2].len() <= 128 && !args[3].is_empty(),
                "invalid identity"
            );
            sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,$2,$3,$4,$5)").bind(Uuid::new_v4()).bind(&args[3]).bind(&args[2]).bind(hash).bind(&args[4]).execute(&pool).await?;
            db::audit(
                &pool,
                &args[3],
                "local-admin",
                "user.create",
                &args[2],
                "SUCCEEDED",
            )
            .await?;
            return Ok(());
        }
        "issue-gateway-transfer" => {
            ensure!(
                args.len() == 15,
                "usage: issue-gateway-transfer APPROVAL_ID OPERATOR GATEWAY_ID EXPECTED_GENERATION NEW_ENDPOINT NEW_GATEWAY_ID NEW_CA_PEM NEW_CERT_PEM EXPIRES_IN_SECONDS SIGNER_KEY_ID PRIVATE_KEY_PEM PUBLIC_KEY_PEM TOKEN_FILE"
            );
            let approval_id = Uuid::parse_str(&args[2]).context("approval id must be a UUID")?;
            let operator = Uuid::parse_str(&args[3]).context("operator must be a user UUID")?;
            let gateway_id = &args[4];
            let expected_generation = args[5]
                .parse::<i64>()
                .context("expected generation must be an integer")?;
            ensure!(
                expected_generation >= 1,
                "expected generation must be positive"
            );
            let new_endpoint = &args[6];
            let new_gateway = &args[7];
            let expires_in = args[10]
                .parse::<u64>()
                .context("expires seconds must be an integer")?;
            let signer_key_id = &args[11];
            let ca_hash = certificate_hash(&args[8])?;
            let cert_hash = certificate_hash(&args[9])?;
            let key_pair = transfer_key_pair(&args[12])?;
            let public_raw = ed25519_public_raw(&key_pair)?;
            let public_der = ed25519_public_der(public_raw)?;
            let public_der_sha = auth::hash(&public_der);
            ensure!(
                auth::hash(public_raw) == signer_sha256(&args[13])?,
                "public key does not match the private transfer signer"
            );
            let now_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;
            let expires_ms = transfer_expires_at_unix_ms(now_ms, expires_in)?;
            let transfer_id = Uuid::new_v4();
            let mut tx = pool.begin().await?;
            let approval = sqlx::query(
                "UPDATE mgmt_transfer_approvals a SET status='ISSUED',closed_at=now() \
                 FROM mgmt_gateways g,mgmt_transfer_signers s,mgmt_users u \
                 WHERE a.id=$1 AND a.status='APPROVED' AND a.gateway_id=g.id \
                   AND a.tenant_id=g.tenant_id AND g.id=$2 AND g.enabled \
                   AND g.binding_generation=$3 \
                   AND s.key_id=$4 AND s.tenant_id=a.tenant_id AND s.status='ACTIVE' \
                   AND s.public_key_sha256=$5 \
                   AND u.id=$6 AND u.enabled AND u.tenant_id=a.tenant_id AND u.role='admin' \
                   AND a.expected_generation=$3 AND a.expected_generation+1=a.to_generation \
                   AND a.new_gateway_id=$7 AND a.new_endpoint=$8 \
                   AND a.new_client_ca_sha256=$9 AND a.new_client_certificate_sha256=$10 \
                 RETURNING a.tenant_id",
            )
            .bind(approval_id)
            .bind(gateway_id)
            .bind(expected_generation)
            .bind(signer_key_id)
            .bind(public_der_sha)
            .bind(operator)
            .bind(new_gateway)
            .bind(new_endpoint)
            .bind(&ca_hash)
            .bind(&cert_hash)
            .fetch_optional(&mut *tx)
            .await?
            .context("approved transfer does not match this operator, signer, and exact request")?;
            let tenant = approval.get::<String, _>("tenant_id");
            sqlx::query(
                "INSERT INTO mgmt_transfer_issuances(transfer_id,approval_id,signer_key_id,issued_by) VALUES($1,$2,$3,$4)",
            )
            .bind(transfer_id)
            .bind(approval_id)
            .bind(signer_key_id)
            .bind(operator)
            .execute(&mut *tx)
            .await?;
            db::audit(
                &mut *tx,
                &tenant,
                &operator.to_string(),
                "transfer.issue",
                &transfer_id.to_string(),
                "SUCCEEDED",
            )
            .await?;
            tx.commit().await?;
            let payload = serde_json::json!({
                "schema":"yofc.agent.transfer.v1",
                "transferId":transfer_id,
                "gatewayId":gateway_id,
                "fromBindingGeneration":expected_generation,
                "toBindingGeneration":expected_generation+1,
                "newEndpoint":new_endpoint,
                "newGatewayId":new_gateway,
                "newClientCertificateSha256":cert_hash,
                "newClientCaSha256":ca_hash,
                "expiresAtUnixMs":expires_ms
            });
            let payload_bytes = serde_json::to_vec(&payload)?;
            let signature = key_pair.sign(&payload_bytes);
            let token = format!(
                "YOFC-AGENT-TRANSFER-v1\n{}\n{}\n",
                base64_url_no_pad(&payload_bytes),
                hex::encode(signature.as_ref())
            );
            write_new(Path::new(&args[14]), token.as_bytes(), 0o600)?;
            println!(
                "One-time approved transfer token written; deliver out of logs and command history"
            );
            return Ok(());
        }
        "request-gateway-transfer-approval" => {
            ensure!(
                args.len() == 9,
                "usage: request-gateway-transfer-approval GATEWAY_ID EXPECTED_GENERATION NEW_ENDPOINT NEW_GATEWAY_ID NEW_CA_PEM NEW_CERT_PEM REQUESTER"
            );
            let gateway_id = &args[2];
            let expected_generation = args[3]
                .parse::<i64>()
                .context("expected generation must be an integer")?;
            ensure!(
                expected_generation >= 1,
                "expected generation must be positive"
            );
            let new_endpoint = &args[4];
            let new_gateway = &args[5];
            ensure!(
                !new_gateway.is_empty()
                    && new_gateway.len() <= 128
                    && new_gateway
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
                "invalid new gateway id"
            );
            ensure!(
                new_endpoint.starts_with("https://") && new_endpoint.len() <= 512,
                "invalid transfer endpoint"
            );
            let ca_hash = certificate_hash(&args[6])?;
            let cert_hash = certificate_hash(&args[7])?;
            let requester = Uuid::parse_str(&args[8]).context("requester must be a user UUID")?;
            let mut tx = pool.begin().await?;
            let gateway = sqlx::query(
                "SELECT tenant_id,binding_generation FROM mgmt_gateways WHERE id=$1 AND enabled",
            )
            .bind(gateway_id)
            .fetch_optional(&mut *tx)
            .await?
            .context("gateway not found or disabled")?;
            ensure!(
                gateway.get::<i64, _>("binding_generation") == expected_generation,
                "expected generation does not match the gateway"
            );
            let tenant = gateway.get::<String, _>("tenant_id");
            let requester_row =
                sqlx::query("SELECT tenant_id,role FROM mgmt_users WHERE id=$1 AND enabled")
                    .bind(requester)
                    .fetch_optional(&mut *tx)
                    .await?
                    .context("requester not found or disabled")?;
            ensure!(
                requester_row.get::<String, _>("tenant_id") == tenant
                    && requester_row.get::<&str, _>("role") == "admin",
                "requester must be an enabled tenant admin"
            );
            let approval_id = Uuid::new_v4();
            sqlx::query("INSERT INTO mgmt_transfer_approvals(id,tenant_id,gateway_id,expected_generation,to_generation,new_gateway_id,new_endpoint,new_client_ca_sha256,new_client_certificate_sha256,requested_by) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)")
                .bind(approval_id).bind(&tenant).bind(gateway_id).bind(expected_generation).bind(expected_generation+1)
                .bind(new_gateway).bind(new_endpoint).bind(&ca_hash).bind(&cert_hash).bind(requester)
                .execute(&mut *tx).await?;
            db::audit(
                &mut *tx,
                &tenant,
                &requester.to_string(),
                "transfer.approval.request",
                gateway_id,
                "SUCCEEDED",
            )
            .await?;
            tx.commit().await?;
            println!("{approval_id}");
            return Ok(());
        }
        "approve-gateway-transfer-approval"
        | "reject-gateway-transfer-approval"
        | "revoke-gateway-transfer-approval" => {
            ensure!(
                args.len() == 4,
                "usage: ACTION-gateway-transfer-approval APPROVAL_ID ACTOR"
            );
            let id = Uuid::parse_str(&args[2]).context("approval id must be a UUID")?;
            let actor = Uuid::parse_str(&args[3]).context("actor must be a user UUID")?;
            let action =
                transfer::ApprovalAction::parse(args[1].split('-').next().unwrap()).unwrap();
            transfer::decide_approval(&pool, None, id, actor, action)
                .await?
                .context("open approval missing or actor is not a permitted tenant admin")?;
            println!("Transfer approval {id}: {}", action.status());
            return Ok(());
        }
        "register-transfer-signer" => {
            ensure!(
                args.len() == 6,
                "usage: register-transfer-signer KEY_ID PUBLIC_KEY_PEM TENANT ACTOR"
            );
            let key_id = &args[2];
            let tenant = &args[4];
            let actor = Uuid::parse_str(&args[5]).context("actor must be a user UUID")?;
            ensure!(transfer::valid_identifier(key_id), "invalid signer key id");
            let public_der = ed25519_public_der(&ed25519_public_from_pem_path(&args[3])?)?;
            let digest = auth::hash(&public_der);
            let mut tx = pool.begin().await?;
            let actor_row =
                sqlx::query("SELECT tenant_id,role FROM mgmt_users WHERE id=$1 AND enabled")
                    .bind(actor)
                    .fetch_optional(&mut *tx)
                    .await?
                    .context("actor not found or disabled")?;
            ensure!(
                actor_row.get::<String, _>("tenant_id") == *tenant
                    && actor_row.get::<&str, _>("role") == "admin",
                "signer registration requires an enabled tenant admin"
            );
            let inserted=sqlx::query("INSERT INTO mgmt_transfer_signers(key_id,tenant_id,public_key_sha256) VALUES($1,$2,$3) ON CONFLICT (key_id) DO NOTHING RETURNING key_id")
                .bind(key_id).bind(tenant).bind(&digest).fetch_optional(&mut *tx).await?;
            if inserted.is_none() {
                let existing=sqlx::query("SELECT tenant_id,public_key_sha256,status FROM mgmt_transfer_signers WHERE key_id=$1")
                    .bind(key_id).fetch_optional(&mut *tx).await?.context("signer key id already exists")?;
                ensure!(
                    existing.get::<String, _>("tenant_id") == *tenant
                        && existing.get::<String, _>("public_key_sha256") == digest
                        && existing.get::<&str, _>("status") == "ACTIVE",
                    "transfer signer key id is already registered with different tenant, key, or revoked status"
                );
            }
            db::audit(
                &mut *tx,
                tenant,
                &actor.to_string(),
                "transfer.signer.register",
                key_id,
                "SUCCEEDED",
            )
            .await?;
            tx.commit().await?;
            println!("Transfer signer {key_id} registered or idempotently active");
            return Ok(());
        }
        "revoke-transfer-signer" => {
            ensure!(
                args.len() == 4,
                "usage: revoke-transfer-signer KEY_ID ACTOR"
            );
            let key_id = &args[2];
            let actor = Uuid::parse_str(&args[3]).context("actor must be a user UUID")?;
            let mut tx = pool.begin().await?;
            let row=sqlx::query(
                "UPDATE mgmt_transfer_signers s SET status='REVOKED',revoked_at=now()                  FROM mgmt_users u WHERE s.key_id=$1 AND s.status='ACTIVE'                    AND u.id=$2 AND u.enabled AND u.tenant_id=s.tenant_id AND u.role='admin'                  RETURNING s.tenant_id"
            ).bind(key_id).bind(actor).fetch_optional(&mut *tx).await?
             .context("active signer is missing or actor is not its enabled tenant admin")?;
            let tenant = row.get::<String, _>("tenant_id");
            db::audit(
                &mut *tx,
                &tenant,
                &actor.to_string(),
                "transfer.signer.revoke",
                key_id,
                "SUCCEEDED",
            )
            .await?;
            tx.commit().await?;
            println!("Transfer signer {key_id} revoked");
            return Ok(());
        }
        "revoke-transfer-issuance" => {
            ensure!(
                args.len() == 4,
                "usage: revoke-transfer-issuance TRANSFER_ID ACTOR"
            );
            let transfer_id = Uuid::parse_str(&args[2]).context("transfer id must be a UUID")?;
            let actor = Uuid::parse_str(&args[3]).context("actor must be a user UUID")?;
            let mut tx = pool.begin().await?;
            let row = sqlx::query(
                "UPDATE mgmt_transfer_issuances i SET revoked_at=now(),revoked_by=$2 \
                 FROM mgmt_users u,mgmt_transfer_approvals a \
                 WHERE i.transfer_id=$1 AND i.revoked_at IS NULL AND a.id=i.approval_id \
                   AND u.id=$2 AND u.enabled AND u.tenant_id=a.tenant_id AND u.role='admin' \
                 RETURNING a.tenant_id,a.gateway_id",
            )
            .bind(transfer_id)
            .bind(actor)
            .fetch_optional(&mut *tx)
            .await?
            .context("unrevoked issuance is missing or actor is not its enabled tenant admin")?;
            use sqlx::Row;
            let tenant: String = row.get("tenant_id");
            let gateway: String = row.get("gateway_id");
            db::audit(
                &mut *tx,
                &tenant,
                &actor.to_string(),
                "transfer.revocation",
                &transfer_id.to_string(),
                "SUCCEEDED",
            )
            .await?;
            tx.commit().await?;
            println!("Transfer issuance {transfer_id} revoked for gateway {gateway}");
            return Ok(());
        }
        "register-gateway" => {
            ensure!(
                args.len() == 6,
                "usage: register-gateway GATEWAY_ID TENANT NAME CLIENT_CERT_PEM"
            );
            gateway_registration::validate(&args[2], &args[3], &args[4])?;
            let fingerprint =
                gateway_registration::certificate_sha256(&fs::read_to_string(&args[5])?)?;
            gateway_registration::register(
                &pool,
                &args[2],
                &args[3],
                &args[4],
                &fingerprint,
                "local-admin",
            )
            .await?;
            return Ok(());
        }
        "set-gateway-enabled" | "rotate-gateway-certificate" | "set-gateway-binding-generation" => {
            ensure!(
                args.len() == 4,
                "usage: set-gateway-enabled GATEWAY_ID true|false OR rotate-gateway-certificate GATEWAY_ID CERT_PEM OR set-gateway-binding-generation GATEWAY_ID GENERATION"
            );
            let mut tx = pool.begin().await?;
            let row=if args[1]=="set-gateway-enabled" {
                  let enabled=args[3].parse::<bool>().context("expected true or false")?;
                  sqlx::query("UPDATE mgmt_gateways SET enabled=$2,connection_epoch=NULL,last_seen=NULL,online_reported=false,kura_available=false,revision=revision+1 WHERE id=$1 RETURNING tenant_id,revision")
                      .bind(&args[2]).bind(enabled).fetch_optional(&mut *tx).await?
              } else if args[1]=="rotate-gateway-certificate" {
                  let data=fs::read(&args[3])?;
                  let cert=rustls_pemfile::certs(&mut data.as_slice()).next().context("no certificate")??;
                  sqlx::query("UPDATE mgmt_gateways SET certificate_sha256=$2,connection_epoch=NULL,last_seen=NULL,online_reported=false,kura_available=false,revision=revision+1 WHERE id=$1 RETURNING tenant_id,revision")
                      .bind(&args[2]).bind(auth::hash(cert.as_ref())).fetch_optional(&mut *tx).await?
              } else {
                  let generation=args[3].parse::<i64>().context("expected positive binding generation")?;
                  ensure!(generation >= 1, "binding generation must be positive");
                  sqlx::query("UPDATE mgmt_gateways SET binding_generation=$2,connection_epoch=NULL,last_seen=NULL,online_reported=false,kura_available=false,revision=revision+1 WHERE id=$1 RETURNING tenant_id,revision")
                      .bind(&args[2]).bind(generation).fetch_optional(&mut *tx).await?
              }.context("gateway not found")?;
            use sqlx::Row;
            let tenant = row.get::<String, _>("tenant_id");
            db::audit(
                &mut *tx,
                &tenant,
                "local-admin",
                &args[1],
                &args[2],
                "SUCCEEDED",
            )
            .await?;
            db::event(
                &mut *tx,
                &tenant,
                "gateway",
                &args[2],
                row.get("revision"),
                serde_json::json!({"online":false,"identityChanged":true}),
            )
            .await?;
            tx.commit().await?;
            return Ok(());
        }
        "serve" => {}
        _ => anyhow::bail!("unknown subcommand"),
    }
    let mut cfg = config::Config::load()?;
    tls_store::activate(&mut cfg)?;
    client_ca_store::activate(&mut cfg)?;
    ensure!(
        std::path::Path::new(&cfg.web_dir)
            .join("index.html")
            .is_file(),
        "React build missing; run frontend/react npm ci and npm run build"
    );
    let tls = ServerTlsConfig::new()
        .identity(Identity::from_pem(
            fs::read(&cfg.tls_cert)?,
            fs::read(&cfg.tls_key)?,
        ))
        .client_ca_root(Certificate::from_pem(fs::read(&cfg.client_ca)?));
    let http_tls =
        axum_server::tls_rustls::RustlsConfig::from_pem_file(&cfg.tls_cert, &cfg.tls_key).await?;
    let app = http::App::new(pool.clone(), cfg.clone(), cipher);
    let control = grpc::ControlService {
        pool: pool.clone(),
        cipher: app.cipher.clone(),
        slots: Arc::new(Semaphore::new(128)),
    };
    let grpc = Server::builder()
        .tls_config(tls)?
        .concurrency_limit_per_connection(2)
        .add_service(
            ControlServer::new(control)
                .max_decoding_message_size(MAX_FRAME)
                .max_encoding_message_size(MAX_FRAME),
        )
        .serve(cfg.grpc_addr);
    let web = axum_server::bind_rustls(cfg.http_addr, http_tls).serve(
        http::router(app)
            .into_make_service_with_connect_info::<std::net::SocketAddr>(),
    );
    tokio::spawn(grpc::maintenance(pool));
    tracing::info!(https=%cfg.http_addr,grpc=%cfg.grpc_addr,"console ready");
    tokio::select! {result=grpc=>result?,result=web=>result?,_=tokio::signal::ctrl_c()=>{}}
    Ok(())
}

#[cfg(test)]
mod transfer_authority_tests {
    use super::*;
    use ring::signature::KeyPair;

    fn generated_signer(_dir: &Path) -> ([u8; 32], Vec<u8>) {
        let rng = ring::rand::SystemRandom::new();
        let document = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key = Ed25519KeyPair::from_pkcs8_maybe_unchecked(document.as_ref()).unwrap();
        let raw: [u8; 32] = key.public_key().as_ref().try_into().unwrap();
        let prefix = [
            0x30u8, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
        ];
        let mut pem = prefix.to_vec();
        pem.extend_from_slice(&raw);
        (raw, pem)
    }

    fn write_public(dir: &Path, name: &str, der: &[u8]) -> String {
        use std::fmt::Write as _;
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, der);
        let mut text = String::from("-----BEGIN PUBLIC KEY-----\n");
        for chunk in encoded.as_bytes().chunks(64) {
            unsafe {
                write!(text, "{}", std::str::from_utf8_unchecked(chunk)).unwrap();
            }
            text.push('\n');
        }
        text.push_str("-----END PUBLIC KEY-----\n");
        let path = dir.join(name);
        fs::write(&path, text).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn transfer_public_pem_is_bounded_ed25519_and_matches_signer() {
        let dir = tempfile::tempdir().unwrap();
        let (raw, der) = generated_signer(dir.path());
        let path = write_public(dir.path(), "signer.pem", &der);
        assert_eq!(transfer::ed25519_public_from_pem_path(&path).unwrap(), raw);
        assert_eq!(signer_sha256(&path).unwrap(), auth::hash(&raw));
        let other_dir = tempfile::tempdir().unwrap();
        let (_, other) = generated_signer(other_dir.path());
        let other_path = write_public(other_dir.path(), "other.pem", &other);
        assert_ne!(signer_sha256(&other_path).unwrap(), auth::hash(&raw));
    }

    #[test]
    fn transfer_expiry_is_milliseconds_and_bounded() {
        assert_eq!(transfer_expires_at_unix_ms(1_000, 600).unwrap(), 601_000);
        assert_eq!(transfer_expires_at_unix_ms(0, 900).unwrap(), 900_000);
        assert!(transfer_expires_at_unix_ms(0, 0).is_err());
        assert!(transfer_expires_at_unix_ms(0, 901).is_err());
    }

    #[test]
    fn transfer_public_pem_rejects_truncated_spki() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_public(dir.path(), "short.pem", &[0u8; 43]);
        assert!(transfer::ed25519_public_from_pem_path(&path).is_err());
    }
}
