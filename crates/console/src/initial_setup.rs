//! First installation UI. No database, CA, data key, or default administrator
//! is needed to reach the HTTPS wizard. Installer credentials are private files.
#[path = "initial_setup_csr.rs"]
mod csr;
use crate::{
    auth,
    config::Config,
    console_settings::{self, SavedSettings, Selection},
    http::ApiError,
    settings_files as files,
    task_crypto::TaskCipher,
};
use anyhow::{Context, Result, ensure};
use axum::{
    Json, Router,
    extract::{
        DefaultBodyLimit, Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fs,
    net::{SocketAddr, TcpListener},
    path::{Path as FsPath, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Mutex as AsyncMutex, Semaphore, watch};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Credential {
    schema_version: u8,
    installation_id: Uuid,
    token: String,
    certificate_fingerprint: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Session {
    token_hash: String,
    csrf: String,
    expires_at: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SetupInput {
    operation_id: Uuid,
    origin: String,
    grpc_address: SocketAddr,
    database_url: String,
    username: String,
    tenant_id: String,
    password: String,
    server_pem: String,
    #[serde(default)]
    server_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    server_csr_id: Option<Uuid>,
    client_ca_pem: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Intent {
    schema_version: u8,
    operation_id: Uuid,
    request_digest: String,
    origin: String,
    username: String,
    selection: Option<Selection>,
}
struct Candidate {
    settings: SavedSettings,
    selection: Selection,
    password_hash: String,
    pool: sqlx::PgPool,
}
struct Job {
    id: Uuid,
    handle: tokio::task::JoinHandle<()>,
}
struct Setup {
    root: PathBuf,
    gate: Arc<crate::maintenance::Gate>,
    credential: Credential,
    listen: SocketAddr,
    web: String,
    changes: Arc<AsyncMutex<()>>,
    slots: Semaphore,
    job: Mutex<Option<Job>>,
    ready: watch::Sender<bool>,
    pairing_attempts: Mutex<std::collections::VecDeque<std::time::Instant>>,
    uncertain_receipt: Mutex<Option<Uuid>>,
}

fn error(status: StatusCode, code: &'static str) -> ApiError {
    ApiError::new(status, code)
}
fn private_error(_: impl std::fmt::Display) -> ApiError {
    error(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL")
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn constant_match(a: &str, b: &str) -> bool {
    let expected = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, a.as_bytes());
    let supplied = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, b.as_bytes());
    ring::hmac::verify(
        &expected,
        b"console-setup-v1",
        ring::hmac::sign(&supplied, b"console-setup-v1").as_ref(),
    )
    .is_ok()
}
fn operation_path(root: &FsPath, id: Uuid, kind: &str) -> PathBuf {
    root.join("setup-operations")
        .join(format!("{id}.{kind}.json"))
}

pub(crate) fn operation(root: &FsPath, id: Uuid, live: bool) -> Result<Option<Value>> {
    let intent = operation_path(root, id, "intent");
    let rejected = operation_path(root, id, "rejected");
    let (path, state) = if rejected.try_exists()? {
        ensure!(!intent.try_exists()?, "conflicting setup records");
        (rejected, "REJECTED")
    } else if intent.try_exists()? {
        (intent, if live { "PENDING" } else { "UNKNOWN" })
    } else if operation_path(root, id, "accepted").try_exists()? {
        (
            operation_path(root, id, "accepted"),
            if live { "PENDING" } else { "UNKNOWN" },
        )
    } else {
        return Ok(None);
    };
    let record: Intent = serde_json::from_slice(&files::read_private(&path)?)?;
    ensure!(
        record.schema_version == 1 && record.operation_id == id,
        "invalid setup record"
    );
    let receipt = operation_path(root, id, "receipt");
    let applied = files::read_private(&receipt)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Selection>(&bytes).ok())
        .is_some_and(|r| {
            record
                .selection
                .as_ref()
                .is_some_and(|s| r.operation_id == s.operation_id && r.digest == s.digest)
        });
    let selected = console_settings::load_at(root)?;
    let observed = record.selection.as_ref().is_some_and(|s| {
        selected.as_ref().is_some_and(|v| {
            v.selection.operation_id == s.operation_id && v.selection.digest == s.digest
        })
    });
    Ok(Some(
        json!({"operationId":id,"state":if applied{"APPLIED"}else{state},"origin":record.origin,
        "username":record.username,"observedConfiguration":observed}),
    ))
}

impl Setup {
    fn live(&self, id: Uuid) -> bool {
        self.job
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(|j| j.id == id && !j.handle.is_finished())
    }
    fn read(&self, id: Uuid) -> Result<Option<Value>> {
        let mut record = operation(&self.root, id, self.live(id))?;
        if *self.uncertain_receipt.lock().unwrap() == Some(id) {
            if let Some(value) = &mut record {
                value["state"] = json!("UNKNOWN");
            }
        }
        Ok(record)
    }
    fn blocking(&self) -> Result<Option<Uuid>> {
        let mut ids = Vec::new();
        for entry in fs::read_dir(self.root.join("setup-operations"))? {
            let name = entry?.file_name();
            if let Some(id) = name.to_str().and_then(|s| {
                s.strip_suffix(".intent.json")
                    .or_else(|| s.strip_suffix(".accepted.json"))
            }) {
                ids.push(Uuid::parse_str(id)?);
            }
        }
        ids.sort();
        ids.dedup();
        for id in ids {
            if self.read(id)?.context("setup record disappeared")?["state"] != "REJECTED" {
                return Ok(Some(id));
            }
        }
        Ok(None)
    }
    fn origin(&self, headers: &HeaderMap, uri: &axum::http::Uri) -> Result<(), ApiError> {
        let from_uri = uri.authority().map(|a| a.as_str());
        let from_header = headers.get(header::HOST).and_then(|v| v.to_str().ok());
        if from_uri.zip(from_header).is_some_and(|(a, b)| a != b) {
            return Err(error(StatusCode::FORBIDDEN, "FORBIDDEN"));
        }
        let host = from_uri
            .or(from_header)
            .ok_or(error(StatusCode::FORBIDDEN, "FORBIDDEN"))?;
        let authority = host
            .parse::<axum::http::uri::Authority>()
            .map_err(|_| error(StatusCode::FORBIDDEN, "FORBIDDEN"))?;
        if authority.as_str() != host
            || headers.get(header::ORIGIN).and_then(|v| v.to_str().ok())
                != Some(format!("https://{host}").as_str())
        {
            return Err(error(StatusCode::FORBIDDEN, "FORBIDDEN"));
        }
        Ok(())
    }
    fn session(&self, headers: &HeaderMap, write: bool) -> Result<Session, ApiError> {
        let token = headers
            .get(header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| {
                s.split(';')
                    .find_map(|v| v.trim().strip_prefix("yofc_setup="))
            })
            .ok_or(error(StatusCode::UNAUTHORIZED, "UNAUTHENTICATED"))?;
        if token.len() != 64 {
            return Err(error(StatusCode::UNAUTHORIZED, "UNAUTHENTICATED"));
        }
        let session: Session = serde_json::from_slice(
            &files::read_private(&self.root.join("setup/session.json")).map_err(private_error)?,
        )
        .map_err(private_error)?;
        if session.expires_at < now()
            || !constant_match(&session.token_hash, &auth::hash(token.as_bytes()))
        {
            return Err(error(StatusCode::UNAUTHORIZED, "UNAUTHENTICATED"));
        }
        if write {
            if !headers
                .get("x-setup-csrf")
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| constant_match(&session.csrf, v))
            {
                return Err(error(StatusCode::FORBIDDEN, "FORBIDDEN"));
            }
        }
        Ok(session)
    }
    fn digest(&self, input: &SetupInput) -> Result<String> {
        Ok(hex::encode(
            ring::hmac::sign(
                &ring::hmac::Key::new(ring::hmac::HMAC_SHA256, self.credential.token.as_bytes()),
                &serde_json::to_vec(input)?,
            )
            .as_ref(),
        ))
    }
    fn reject(&self, input: &SetupInput, digest: String) -> Result<Value> {
        let r = Intent {
            schema_version: 1,
            operation_id: input.operation_id,
            request_digest: digest,
            origin: input.origin.clone(),
            username: input.username.clone(),
            selection: None,
        };
        files::publish_new(
            &operation_path(&self.root, input.operation_id, "rejected"),
            &serde_json::to_vec(&r)?,
        )?;
        self.read(input.operation_id)?
            .context("rejected setup record missing")
    }
    async fn prepare(&self, mut input: SetupInput) -> Result<Candidate> {
        ensure!(
            !input.operation_id.is_nil() && input.database_url.len() <= 4096,
            "invalid setup identity"
        );
        ensure!(
            (input.database_url.starts_with("postgres://")
                || input.database_url.starts_with("postgresql://"))
                && !input.database_url.chars().any(char::is_control),
            "invalid database URL"
        );
        for name in [&input.username, &input.tenant_id] {
            ensure!(
                !name.is_empty() && name.len() <= 128 && !name.chars().any(char::is_control),
                "invalid organization identity"
            );
        }
        ensure!(
            input.password.as_bytes().len() >= 12 && input.password.as_bytes().len() <= 256,
            "invalid administrator password"
        );
        ensure!(
            input.grpc_address.port() != 0 && input.grpc_address.port() != self.listen.port(),
            "HTTP and gRPC listeners must use separate ports"
        );
        if let Some(id) = input.server_csr_id {
            ensure!(input.server_key.is_empty(), "select a generated CSR or an uploaded key");
            let (certificate,key) = crate::server_csr::identity(&self.root,id,&input.origin)?;
            ensure!(input.server_pem == certificate, "CSR certificate selection changed");
            input.server_key = key;
        } else {
            ensure!(crate::server_csr::current(&self.root)?.is_none(), "review the original CSR before using a different identity");
        }
        crate::tls_store::validate_pair(&input.server_pem, &input.server_key)?;
        crate::client_ca_store::validate(&input.client_ca_pem)?;
        let directory = console_settings::generation(&self.root, input.operation_id);
        files::directory(&directory)?;
        for (name, value) in [
            ("server.pem", &input.server_pem),
            ("server.key", &input.server_key),
            ("client-ca.pem", &input.client_ca_pem),
        ] {
            files::write_new(&directory.join(name), value.as_bytes())?;
        }
        crate::task_crypto::create_key(&directory.join("task-data.key"))?;
        let config = Config {
            http_addr: self.listen,
            grpc_addr: input.grpc_address,
            origin: input.origin,
            tls_cert: directory.join("server.pem").to_string_lossy().into(),
            tls_key: directory.join("server.key").to_string_lossy().into(),
            client_ca: directory.join("client-ca.pem").to_string_lossy().into(),
            web_dir: self.web.clone(),
            state_dir: self.root.to_string_lossy().into(),
        };
        crate::tls_runtime::validate_config(&config)?;
        let settings = SavedSettings {
            schema_version: 1,
            installation_id: self.credential.installation_id,
            operation_id: input.operation_id,
            database_url: input.database_url,
            data_key_file: directory.join("task-data.key").to_string_lossy().into(),
            config,
            material_sha256: ["server.pem", "server.key", "client-ca.pem", "task-data.key"]
                .iter()
                .map(|n| files::read_private(&directory.join(n)).map(|v| auth::hash(&v)))
                .collect::<Result<_>>()?,
        };
        let bytes = serde_json::to_vec(&settings)?;
        files::write_new(&directory.join("settings.json"), &bytes)?;
        files::sync_parent(&directory.join("settings.json"))?;
        let selection = Selection {
            schema_version: 1,
            operation_id: settings.operation_id,
            digest: auth::hash(&bytes),
        };
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&settings.database_url)
            .await?;
        let mut conn = pool.acquire().await?;
        ensure!(
            empty_schema(&mut conn).await?,
            "setup requires a dedicated empty database"
        );
        drop(conn);
        let password_hash =
            tokio::task::spawn_blocking(move || auth::password_hash(&input.password)).await??;
        Ok(Candidate {
            settings,
            selection,
            password_hash,
            pool,
        })
    }
    async fn apply(&self, candidate: Candidate, username: String, tenant: String) -> Result<()> {
        let pool = &candidate.pool;
        let mut conn = pool.acquire().await?;
        let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock(794654,24)")
            .fetch_one(&mut *conn)
            .await?;
        ensure!(acquired, "database installation is already in use");
        let result:Result<()>=async {
            ensure!(empty_schema(&mut conn).await?,"database changed before installation");
            sqlx::migrate!("../../migrations").run(pool).await?;
            let cipher=TaskCipher::load(FsPath::new(&candidate.settings.data_key_file))?;
            ensure!(cipher.prepare_storage(pool).await?==0,"unexpected existing task payloads");
            let mut tx=pool.begin().await?;
            sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,$2,$3,$4,'admin',false)")
                .bind(Uuid::new_v4()).bind(&tenant).bind(&username).bind(&candidate.password_hash).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO mgmt_console_installation(singleton,installation_id,setup_operation_id,config_digest) VALUES(true,$1,$2,$3)")
                .bind(candidate.settings.installation_id).bind(candidate.settings.operation_id).bind(&candidate.selection.digest).execute(&mut *tx).await?;
            crate::db::audit(&mut *tx,&tenant,&username,"console.initial-setup",&candidate.settings.operation_id.to_string(),"COMMITTED").await?;
            tx.commit().await?;
            files::publish_new(&self.root.join("console-config-active.json"),&serde_json::to_vec(&candidate.selection)?)?;
            if files::publish_new(&operation_path(&self.root,candidate.settings.operation_id,"receipt"),&serde_json::to_vec(&candidate.selection)?).is_err() {
                *self.uncertain_receipt.lock().unwrap()=Some(candidate.settings.operation_id);
                anyhow::bail!("setup receipt durability is uncertain");
            }
            self.ready.send_replace(true);
            Ok(())
        }.await;
        // Session advisory locks must never be returned to a connection pool.
        let unlocked = sqlx::query("SELECT pg_advisory_unlock(794654,24)")
            .execute(&mut *conn)
            .await;
        if unlocked.is_err() {
            let _ = conn.close().await;
        } else {
            drop(conn);
        }
        pool.close().await;
        result
    }
}

async fn empty_schema(conn: &mut sqlx::PgConnection) -> Result<bool> {
    Ok(!sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname=current_schema() AND c.relkind IN ('r','p','v','m','S','f'))").fetch_one(conn).await?)
}

async fn info(
    State(setup): State<Arc<Setup>>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let mut body = json!({"configured":false,"installationId":setup.credential.installation_id,"certificateFingerprint":setup.credential.certificate_fingerprint,"listenAddress":setup.listen,"authenticated":false});
    body["maintenance"]=json!(setup.gate.status().map_err(private_error)?.is_some());
    if let Ok(session) = setup.session(&headers, false) {
        body["authenticated"] = json!(true);
        body["csrf"] = json!(session.csrf);
        body["blockingOperationId"] = json!(setup.blocking().map_err(private_error)?);
    }
    Ok(Json(body))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PairInput {
    installation_id: Uuid,
    token: String,
}
async fn pair(
    State(setup): State<Arc<Setup>>,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    input: Result<Json<PairInput>, JsonRejection>,
) -> Result<Response, ApiError> {
    setup.origin(&headers, &uri)?;
    let _slot = setup
        .slots
        .try_acquire()
        .map_err(|_| error(StatusCode::TOO_MANY_REQUESTS, "RESOURCE_EXHAUSTED"))?;
    {
        let mut attempts = setup.pairing_attempts.lock().unwrap();
        let now = std::time::Instant::now();
        while attempts
            .front()
            .is_some_and(|t| now.duration_since(*t) > Duration::from_secs(60))
        {
            attempts.pop_front();
        }
        if attempts.len() >= 32 {
            return Err(error(StatusCode::TOO_MANY_REQUESTS, "RESOURCE_EXHAUSTED"));
        }
        attempts.push_back(now);
    }
    let Json(input) = input?;
    if input.installation_id != setup.credential.installation_id
        || input.token.len() != 64
        || !constant_match(&setup.credential.token, &input.token)
    {
        return Err(error(StatusCode::UNAUTHORIZED, "UNAUTHENTICATED"));
    }
    let token = auth::random_token();
    let session = Session {
        token_hash: auth::hash(token.as_bytes()),
        csrf: auth::random_token(),
        expires_at: now() + 3600,
    };
    files::publish_replace(
        &setup.root.join("setup/session.json"),
        &serde_json::to_vec(&session).map_err(private_error)?,
    )
    .map_err(private_error)?;
    Ok(([(header::SET_COOKIE,format!("yofc_setup={token}; HttpOnly; Secure; SameSite=Strict; Path=/api/setup; Max-Age=3600"))],Json(json!({"csrf":session.csrf}))).into_response())
}
async fn read_operation(
    State(setup): State<Arc<Setup>>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<Value>, ApiError> {
    setup.session(&headers, false)?;
    let Path(id) = path?;
    Ok(Json(
        setup
            .read(id)
            .map_err(private_error)?
            .ok_or(error(StatusCode::NOT_FOUND, "NOT_FOUND"))?,
    ))
}
async fn submit(
    State(setup): State<Arc<Setup>>,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    input: Result<Json<SetupInput>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    setup.origin(&headers, &uri)?;
    setup.session(&headers, true)?;
    let Json(input) = input?;
    let _slot = setup
        .slots
        .try_acquire()
        .map_err(|_| error(StatusCode::TOO_MANY_REQUESTS, "RESOURCE_EXHAUSTED"))?;
    if input.operation_id.is_nil()
        || input.origin.len() > 256
        || input.username.len() > 128
        || input.tenant_id.len() > 128
        || input.database_url.len() > 4096
        || input.password.len() > 256
        || input.server_pem.len() > 65536
        || input.server_key.len() > 32768
        || input.client_ca_pem.len() > 65536
    {
        return Err(error(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let digest = setup.digest(&input).map_err(private_error)?;
    let _guard = setup.changes.clone().lock_owned().await;
    if let Some(record) = setup.read(input.operation_id).map_err(private_error)? {
        let kind = if operation_path(&setup.root, input.operation_id, "intent").exists() {
            "intent"
        } else if operation_path(&setup.root, input.operation_id, "rejected").exists() {
            "rejected"
        } else {
            "accepted"
        };
        let old: Intent = serde_json::from_slice(
            &files::read_private(&operation_path(&setup.root, input.operation_id, kind))
                .map_err(private_error)?,
        )
        .map_err(private_error)?;
        if !constant_match(&digest, &old.request_digest) {
            return Err(error(StatusCode::CONFLICT, "CONFLICT"));
        }
        return Ok(Json(record));
    }

    if setup.blocking().map_err(private_error)?.is_some()
        || setup
            .root
            .join("console-config-active.json")
            .try_exists()
            .map_err(private_error)?
    {
        return Ok(Json(setup.reject(&input, digest).map_err(private_error)?));
    }
    let id = input.operation_id;
    // The independent job owns its lease before acceptance becomes durable.
    // Dropping the HTTP request must not release an accepted initialization.
    let job_permit = setup.gate.enter().map_err(|_|error(StatusCode::SERVICE_UNAVAILABLE,"MAINTENANCE"))?;
    let origin = input.origin.clone();
    let username = input.username.clone();
    let tenant = input.tenant_id.clone();
    // Invalid material is known unentered. Keep a durable rejected record without
    // returning private PEM, password, connection URL or parser details.
    let rejected = Intent {
        schema_version: 1,
        operation_id: id,
        request_digest: digest.clone(),
        origin: origin.clone(),
        username: username.clone(),
        selection: None,
    };
    files::publish_new(
        &operation_path(&setup.root, id, "accepted"),
        &serde_json::to_vec(&rejected).map_err(private_error)?,
    )
    .map_err(private_error)?;
    let owner = setup.clone();
    let handle = tokio::spawn(async move {
        let _permit = job_permit;
        let candidate = match owner.prepare(input).await {
            Ok(candidate) => candidate,
            Err(_) => {
                if let Ok(bytes) = serde_json::to_vec(&rejected) {
                    let _ =
                        files::publish_new(&operation_path(&owner.root, id, "rejected"), &bytes);
                }
                return;
            }
        };
        let intent = Intent {
            selection: Some(candidate.selection.clone()),
            ..rejected
        };
        let published = serde_json::to_vec(&intent).ok().is_some_and(|bytes| {
            files::publish_new(&operation_path(&owner.root, id, "intent"), &bytes).is_ok()
        });
        if !published {
            return;
        }
        if owner.apply(candidate, username, tenant).await.is_err() {
            tracing::error!(operation_id=%id,"initial setup outcome requires original-record review");
        }
    });
    *setup.job.lock().unwrap() = Some(Job { id, handle });
    Ok(Json(
        json!({"operationId":id,"state":"PENDING","origin":origin,"observedConfiguration":false}),
    ))
}

fn router(setup: Arc<Setup>) -> Router {
    Router::new().route("/",get(||async{Redirect::temporary("/setup")}))
        .route("/health/live",get(||async{Json(json!({"status":"setup-required"}))}))
        .route("/health/ready",get(||async{(StatusCode::SERVICE_UNAVAILABLE,Json(json!({"status":"setup-required"})))}))
        .route_service("/setup",ServeFile::new(format!("{}/index.html",setup.web)))
        .nest_service("/assets",ServeDir::new(format!("{}/assets",setup.web)))
        .route("/api/setup/status",get(info)).route("/api/setup/pair",post(pair))
        .route("/api/setup/server-csr",get(csr::current).post(csr::create))
        .route("/api/setup/server-csr/{id}",get(csr::read))
        .route("/api/setup/server-csr/{id}/imports/{operation}",get(csr::read_import))
        .route("/api/setup/server-csr/{id}/certificate",axum::routing::put(csr::import))
        .route("/api/setup/server-csr/{id}/retire",post(csr::retire))
        .route("/api/setup/operations",post(submit)).route("/api/setup/operations/{id}",get(read_operation))
        .fallback(||async{error(StatusCode::NOT_FOUND,"NOT_FOUND")})
        .layer(axum::middleware::from_fn_with_state(setup.gate.clone(),crate::maintenance::admit))
        .layer(DefaultBodyLimit::max(196608))
        .layer(SetResponseHeaderLayer::overriding(header::CACHE_CONTROL,HeaderValue::from_static("no-store")))
        .layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS,HeaderValue::from_static("nosniff")))
        .layer(SetResponseHeaderLayer::overriding(header::CONTENT_SECURITY_POLICY,HeaderValue::from_static("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'")))
        .with_state(setup)
}

async fn openssl(args: &[&str]) -> Result<Vec<u8>> {
    let command = tokio::process::Command::new("openssl")
        .args(args)
        .env("OPENSSL_CONF", "/dev/null")
        .kill_on_drop(true)
        .output();
    let output = tokio::time::timeout(Duration::from_secs(10), command).await??;
    ensure!(
        output.status.success() && output.stdout.len() <= 65536,
        "bootstrap certificate generator failed"
    );
    Ok(output.stdout)
}
async fn credential(root: &FsPath) -> Result<Credential> {
    let setup = root.join("setup");
    files::directory(&setup)?;
    let path = setup.join("installation-credential.json");
    if path.try_exists()? {
        return load_credential(root);
    }
    ensure!(
        !setup.join("bootstrap.key").try_exists()? && !setup.join("bootstrap.pem").try_exists()?,
        "incomplete bootstrap generation must be preserved and reviewed"
    );
    let key = bootstrap_key()?;
    files::write_new(&setup.join("bootstrap.key"), &key)?;
    let keypath = setup.join("bootstrap.key");
    let request_config = setup.join("bootstrap-request.cnf");
    files::write_new(
        &request_config,
        b"[req]\nprompt=no\ndistinguished_name=dn\n[dn]\nCN=YOFC installation setup\n",
    )?;
    let cert = openssl(&[
        "req",
        "-new",
        "-x509",
        "-key",
        keypath.to_str().context("non-UTF8 state")?,
        "-config",
        request_config.to_str().context("non-UTF8 state")?,
        "-days",
        "7",
        "-subj",
        "/CN=YOFC installation setup",
        "-addext",
        "basicConstraints=critical,CA:FALSE",
        "-addext",
        "extendedKeyUsage=serverAuth",
        "-addext",
        "subjectAltName=DNS:localhost,IP:127.0.0.1",
    ])
    .await?;
    files::write_new(&setup.join("bootstrap.pem"), &cert)?;
    let c = Credential {
        schema_version: 1,
        installation_id: Uuid::new_v4(),
        token: auth::random_token(),
        certificate_fingerprint: crate::tls_store::validate_pair(
            std::str::from_utf8(&cert)?,
            std::str::from_utf8(&key)?,
        )?,
    };
    files::publish_new(&path, &serde_json::to_vec(&c)?)?;
    Ok(c)
}

fn private_directory(path: &FsPath) -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    let meta = fs::symlink_metadata(path)?;
    ensure!(
        meta.is_dir() && meta.uid() == unsafe { libc::geteuid() } && meta.mode() & 0o077 == 0,
        "installation directory must be owned and private"
    );
    Ok(())
}

fn load_credential(root: &FsPath) -> Result<Credential> {
    private_directory(root)?;
    let setup = root.join("setup");
    private_directory(&setup)?;
    let credential: Credential = serde_json::from_slice(&files::read_private(
        &setup.join("installation-credential.json"),
    )?)?;
    ensure!(
        credential.schema_version == 1
            && credential.token.len() == 64
            && credential.token.bytes().all(|v| v.is_ascii_hexdigit()),
        "invalid installation credential"
    );
    let fingerprint = crate::tls_store::validate_pair(
        std::str::from_utf8(&files::read_private(&setup.join("bootstrap.pem"))?)?,
        std::str::from_utf8(&files::read_private(&setup.join("bootstrap.key"))?)?,
    )?;
    ensure!(
        fingerprint == credential.certificate_fingerprint,
        "bootstrap identity changed"
    );
    Ok(credential)
}

/// Local OS-administrator delivery channel only. Never generate, pair, reset or
/// write anything when an installer asks to download an existing credential.
pub(crate) fn installer_credential(root: &FsPath) -> Result<Value> {
    ensure!(
        console_settings::load_at(root)?.is_none(),
        "installation is already configured; the installer credential is retired"
    );
    Ok(serde_json::to_value(load_credential(root)?)?)
}

pub(crate) fn setup_port() -> Result<u16> {
    let port=crate::config::setting("CONSOLE_SETUP_PORT")?.unwrap_or_else(||"18443".into()).parse::<u16>()?;
    ensure!(port>=1024,"setup port must be between 1024 and 65535");
    Ok(port)
}

/// Private host-administrator observation. Never create identity, access the
/// database, acquire the serving lease, or return installation secrets.
pub(crate) fn installation_status(root: &FsPath) -> Result<Value> {
    let configured=console_settings::load_at(root)?;
    let config=if let Some(saved)=configured {
        Some(Config::overrides(saved.settings.config)?)
    }else if crate::config::setting("CONSOLE_ORIGIN")?.is_some(){
        Some(Config::load()?)
    }else{None};
    if let Some(config)=config {
        let fingerprint=crate::tls_runtime::readiness_fingerprint(&config)?;
        return Ok(json!({"schemaVersion":1,"configured":true,"httpAddress":config.http_addr.to_string(),
            "httpPort":config.http_addr.port(),"certificateFingerprint":fingerprint}));
    }
    let credential=load_credential(root)?;
    let address=crate::config::setting("CONSOLE_HTTP_ADDR")?.map(|v|v.parse::<SocketAddr>()).transpose()?;
    let port=match address{Some(value)=>value.port(),None=>setup_port()?};
    Ok(json!({"schemaVersion":1,"configured":false,"httpAddress":address.map(|v|v.to_string()),
        "httpPort":port,"certificateFingerprint":credential.certificate_fingerprint}))
}

pub(crate) fn maintenance_records(root: &FsPath) -> Result<()> {
    private_directory(root)?;
    let history = root.join("setup-operations");
    if history.try_exists()? {
        private_directory(&history)?;
        let mut ids = std::collections::BTreeSet::new();
        for entry in fs::read_dir(&history)? {
            let name = entry?
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("invalid installation journal name"))?;
            for suffix in ["accepted", "intent", "receipt", "rejected"] {
                if let Some(id) = name.strip_suffix(&format!(".{suffix}.json")) {
                    ids.insert(Uuid::parse_str(id)?);
                }
            }
        }
        for id in ids {
            let original =
                operation(root, id, false)?.context("incomplete installation journal")?;
            ensure!(
                original["state"] == "REJECTED"
                    || (original["state"] == "APPLIED"
                        && original["observedConfiguration"] == true),
                "unsettled installation operation forbids package maintenance; preserve and query the original operation"
            );
        }
    }
    let tls = root.join("tls-operations");
    if tls.try_exists()? {
        private_directory(&tls)?;
        crate::tls_runtime::maintenance_records(&tls)?;
    }
    Ok(())
}

/// Read-only maintenance guard: no migrations, task-key recovery, seeding or
/// shutdown. The package must pass this before attempting to stop any center.
pub(crate) async fn maintenance_check(root: &FsPath) -> Result<()> {
    maintenance_records(root)?;
    let Some(saved) = console_settings::load_at(root)? else {
        ensure!(
            crate::config::setting("DATABASE_URL")?.is_none(),
            "legacy installations require their explicit database maintenance guard"
        );
        return Ok(());
    };
    let database = crate::config::setting("DATABASE_URL")?
        .unwrap_or_else(|| saved.settings.database_url.clone());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database)
        .await?;
    console_settings::verify_database(&pool, &saved).await?;
    let unsettled: i64 = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        sqlx::query_scalar(
            "SELECT count(*) FROM mgmt_tasks WHERE state IN ('QUEUED','RUNNING','UNKNOWN')",
        )
        .fetch_one(&pool),
    )
    .await??;
    ensure!(
        unsettled == 0,
        "unsettled console tasks forbid package maintenance"
    );
    pool.close().await;
    Ok(())
}

fn bootstrap_key() -> Result<Vec<u8>> {
    use base64::Engine;
    let document = ring::signature::EcdsaKeyPair::generate_pkcs8(
        &ring::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        &ring::rand::SystemRandom::new(),
    )
    .map_err(|_| anyhow::anyhow!("bootstrap key generation failed"))?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(document.as_ref());
    let mut pem = String::from("-----BEGIN PRIVATE KEY-----\n");
    for chunk in encoded.as_bytes().chunks(64) {
        pem.push_str(std::str::from_utf8(chunk)?);
        pem.push('\n');
    }
    pem.push_str("-----END PRIVATE KEY-----\n");
    Ok(pem.into_bytes())
}

pub(crate) async fn run_if_needed(root: PathBuf) -> Result<Option<TcpListener>> {
    let gate = Arc::new(crate::maintenance::Gate::for_state(&root)?);
    let startup_permit = gate.enter()?;
    if console_settings::load_at(&root)?.is_some() {
        return Ok(None);
    }
    let required = [
        "DATABASE_URL",
        "CONSOLE_DATA_KEY_FILE",
        "CONSOLE_ORIGIN",
        "CONSOLE_TLS_CERT",
        "CONSOLE_TLS_KEY",
        "CONSOLE_CLIENT_CA",
    ];
    let mut present = 0;
    for name in required {
        if crate::config::setting(name)?.is_some() {
            present += 1;
        }
    }
    if present == required.len() {
        return Ok(None);
    }
    ensure!(
        present == 0,
        "partial legacy environment configuration; preserve it and complete the explicit configuration"
    );
    let web = if let Some(path) = crate::config::setting("CONSOLE_WEB_DIR")? {
        PathBuf::from(path)
    } else {
        [
            "react",
            "frontend/react/dist",
            "/usr/share/kura-management-console/react",
        ]
        .iter()
        .map(PathBuf::from)
        .find(|p| p.join("index.html").is_file())
        .context("setup web payload missing")?
    };
    let web = fs::canonicalize(web)?.to_string_lossy().into_owned();
    ensure!(
        FsPath::new(&web).join("index.html").is_file(),
        "setup web payload missing"
    );
    files::directory(&root)?;
    files::directory(&root.join("setup-operations"))?;
    files::directory(&root.join("installation-generations"))?;
    let credential = credential(&root).await?;
    let listener = match crate::config::setting("CONSOLE_HTTP_ADDR")? {
        Some(address)=>crate::listeners::bind(address.parse()?)?,
        None=>crate::listeners::automatic(setup_port()?)?,
    };
    listener.set_nonblocking(true)?;
    let listen = listener.local_addr()?;
    let (ready, mut changed) = watch::channel(false);
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        root.join("setup/bootstrap.pem"),
        root.join("setup/bootstrap.key"),
    )
    .await?;
    tracing::info!(https=%listen,certificate_fingerprint=%credential.certificate_fingerprint,"first setup ready; use the private installer credential, never a default password");
    let setup = Arc::new(Setup {
        root,
        gate,
        credential,
        listen,
        web,
        changes: Arc::new(AsyncMutex::new(())),
        slots: Semaphore::new(2),
        job: Mutex::new(None),
        ready,
        pairing_attempts: Mutex::new(Default::default()),
        uncertain_receipt: Mutex::new(None),
    });
    let handle = axum_server::Handle::new();
    drop(startup_permit);
    let server = axum_server::from_tcp(listener.try_clone()?)?
        .acceptor(axum_server::tls_rustls::RustlsAcceptor::new(tls))
        .handle(handle.clone())
        .serve(router(setup.clone()).into_make_service());
    tokio::pin!(server);
    tokio::select! {r=&mut server=>{r?;anyhow::bail!("setup listener stopped before configuration")},
    r=changed.changed()=>{r?;ensure!(*changed.borrow(),"setup result missing");},
    _=tokio::signal::ctrl_c()=>{anyhow::bail!("setup interrupted; preserve original records");}}
    handle.shutdown();
    server.await?;
    // The same listening socket moves into the normal HTTPS server. No manual
    // process restart and no binding gap are needed for first configuration.
    Ok(Some(listener))
}

#[cfg(test)]
#[path = "initial_setup_tests.rs"]
mod tests;
