mod auth;
mod config;
mod db;
mod grpc;
mod http;
mod task_crypto;
use anyhow::{Context, Result, ensure};
use std::{env, fs, io::Write, path::Path, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use ring::signature::{Ed25519KeyPair, KeyPair};
use tokio::sync::Semaphore;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
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
        if chunk.len() > 1 { out.push(ALPHABET[(number >> 6) as usize & 63] as char); }
        if chunk.len() > 2 { out.push(ALPHABET[number as usize & 63] as char); }
    }
    out
}

fn write_new(path: &Path, value: &[u8], mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    if path.exists() {
        anyhow::bail!("refusing to overwrite {}", path.display());
    }
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path)
        .with_context(|| format!("create {}", path.display()))?;
    file.write_all(value)?;
    file.sync_all()?;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
    Ok(())
}

fn certificate_hash(path: &str) -> Result<String> {
    let data = fs::read(path)?;
    let cert = rustls_pemfile::certs(&mut data.as_slice()).next()
        .context("no certificate")??;
    Ok(auth::hash(cert.as_ref()))
}

fn transfer_key_pair(path: &str) -> Result<Ed25519KeyPair> {
    let document = fs::read(path)?;
    Ed25519KeyPair::from_pkcs8_maybe_unchecked(&document)
        .map_err(|_| anyhow::anyhow!("read Ed25519 transfer signer {}", path))
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
        ensure!(args.len() == 4, "usage: generate-transfer-key PRIVATE_KEY_PEM PUBLIC_KEY_PEM");
        let rng = ring::rand::SystemRandom::new();
        let document = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| anyhow::anyhow!("generate Ed25519 transfer signer"))?;
        write_new(Path::new(&args[2]), document.as_ref(), 0o600)?;
        let key = Ed25519KeyPair::from_pkcs8_maybe_unchecked(document.as_ref())
            .map_err(|_| anyhow::anyhow!("read generated Ed25519 transfer signer"))?;
        let mut public = vec![0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00];
        public.extend_from_slice(key.public_key().as_ref());
        write_new(Path::new(&args[3]), &public, 0o644)?;
        println!("Transfer signer generated; import the public key with agent import-identity");
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
        let pool = db::connect(&env::var("DATABASE_URL").context("DATABASE_URL is required")?).await?;
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
    let cipher = task_crypto::TaskCipher::load(std::path::Path::new(
        &env::var("CONSOLE_DATA_KEY_FILE").context("CONSOLE_DATA_KEY_FILE is required")?,
    ))?;
    let pool = db::connect(&env::var("DATABASE_URL").context("DATABASE_URL is required")?).await?;
    let migrated = cipher.prepare_storage(&pool).await?;
    tracing::info!(migrated_tasks = migrated, "task storage key verified");
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
                args.len() == 12,
                "usage: issue-gateway-transfer CURRENT_ENDPOINT GATEWAY_ID EXPECTED_GENERATION NEW_ENDPOINT NEW_GATEWAY_ID NEW_CA_PEM NEW_CERT_PEM EXPIRES_IN_SECONDS PRIVATE_KEY_PEM TOKEN_FILE"
            );
            let _current_endpoint=&args[2]; let gateway_id=&args[3];
            let expected_generation=args[4].parse::<i64>().context("expected generation must be an integer")?;
            ensure!(expected_generation >= 1, "expected generation must be positive");
            let new_endpoint=&args[5]; let new_gateway=&args[6];
            let expires_in=args[9].parse::<u64>().context("expires seconds must be an integer")?;
            ensure!(expires_in > 0 && expires_in <= 900, "transfer token validity must be 1..=900 seconds");
            let ca_hash=certificate_hash(&args[7])?;
            let cert_hash=certificate_hash(&args[8])?;
            let key_pair=transfer_key_pair(&args[10])?;
            let now_ms=SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;
            let expires_ms=now_ms + expires_in as i64;
            let transfer_id=Uuid::new_v4();
            let payload=serde_json::json!({
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
            let payload_bytes=serde_json::to_vec(&payload)?;
            let signature=key_pair.sign(&payload_bytes);
            let token=format!("YOFC-AGENT-TRANSFER-v1\n{}\n{}\n",
                base64_url_no_pad(&payload_bytes), hex::encode(signature.as_ref()));
            write_new(Path::new(&args[11]), token.as_bytes(), 0o600)?;
            println!("One-time transfer token written; deliver out of logs and command history");
            return Ok(());
        }
        "register-gateway" => {
            ensure!(
                args.len() == 6,
                "usage: register-gateway GATEWAY_ID TENANT NAME CLIENT_CERT_PEM"
            );
            let data = fs::read(&args[5])?;
            let cert = rustls_pemfile::certs(&mut data.as_slice())
                .next()
                .context("no certificate")??;
            ensure!(
                !args[2].is_empty()
                    && args[2].len() <= 128
                    && args[2]
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
                "invalid gateway id"
            );
            sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256,binding_generation) VALUES($1,$2,$3,$4,1)").bind(&args[2]).bind(&args[3]).bind(&args[4]).bind(auth::hash(cert.as_ref())).execute(&pool).await?;
            db::audit(
                &pool,
                &args[3],
                "local-admin",
                "gateway.register",
                &args[2],
                "SUCCEEDED",
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
    let cfg = config::Config::load()?;
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
    let control = grpc::ControlService {
        pool: pool.clone(),
        cipher: cipher.clone(),
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
        http::router(http::App::new(pool.clone(), cfg.clone(), cipher))
            .into_make_service_with_connect_info::<std::net::SocketAddr>(),
    );
    tokio::spawn(grpc::maintenance(pool));
    tracing::info!(https=%cfg.http_addr,grpc=%cfg.grpc_addr,"console ready");
    tokio::select! {result=grpc=>result?,result=web=>result?,_=tokio::signal::ctrl_c()=>{}}
    Ok(())
}
