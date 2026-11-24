#[path = "server_csr_http.rs"]
mod server_csr_http;
#[path = "ca_policy_http.rs"]
mod ca_policy_http;
use crate::{
    auth,
    config::Config,
    db,
    task_crypto::{TaskCipher, TaskContext},
    transfer,
};
use axum::{
    Json, Router,
    extract::{
        ConnectInfo, DefaultBodyLimit, Path, Query, State,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{
        IntoResponse, Response, Sse,
        sse::{Event, KeepAlive},
    },
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use std::{
    collections::{HashMap, VecDeque},
    io::Write as _,
    os::unix::fs::OpenOptionsExt as _,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, Semaphore};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use uuid::Uuid;

#[cfg(test)]
#[path = "http_tls_tests.rs"]
mod http_tls_tests;

#[derive(Clone)]
pub struct App {
    pub pool: PgPool,
    // Shared so the runtime task-key rotation swaps it without a restart.
    pub cipher: Arc<tokio::sync::RwLock<TaskCipher>>,
    pub config: Config,
    pub(crate) tls: Option<Arc<crate::tls_runtime::TlsRuntime>>,
    pub(crate) gate: Arc<crate::maintenance::Gate>,
    pub login_slots: Arc<Semaphore>,
    pub attempts: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
    pub sse_slots: Arc<Semaphore>,
}
impl App {
    pub fn new(pool: PgPool, config: Config, cipher: TaskCipher) -> anyhow::Result<Self> {
        let gate = Arc::new(crate::maintenance::Gate::for_state(std::path::Path::new(&config.state_dir))?);
        Ok(Self {
            pool,
            cipher: Arc::new(tokio::sync::RwLock::new(cipher)),
            config,
            tls: None,
            gate,
            login_slots: Arc::new(Semaphore::new(4)),
            attempts: Default::default(),
            sse_slots: Arc::new(Semaphore::new(128)),
        })
    }
    fn effective_config(&self) -> Config {
        self.tls.as_ref().map(|runtime| runtime.config()).unwrap_or_else(|| self.config.clone())
    }
}
pub struct ApiError(StatusCode, &'static str);
impl ApiError {
    pub fn new(status: StatusCode, code: &'static str) -> Self {
        Self(status, code)
    }
    pub fn unauthorized() -> Self {
        Self(StatusCode::UNAUTHORIZED, "UNAUTHENTICATED")
    }
    fn forbidden() -> Self {
        Self(StatusCode::FORBIDDEN, "FORBIDDEN")
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({"error":{"code":self.1}}))).into_response()
    }
}
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!(error=%e,"database operation failed");
        Self(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL")
    }
}
impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        // Keep payloads and crypto internals out of response bodies and logs.
        tracing::error!(root = %e,"task storage operation failed");
        Self(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL")
    }
}
impl From<JsonRejection> for ApiError {
    fn from(error: JsonRejection) -> Self {
        let status = error.status();
        Self(
            status,
            if status == StatusCode::PAYLOAD_TOO_LARGE {
                "RESOURCE_EXHAUSTED"
            } else {
                "INVALID_ARGUMENT"
            },
        )
    }
}
impl From<QueryRejection> for ApiError {
    fn from(_: QueryRejection) -> Self {
        Self(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT")
    }
}
impl From<PathRejection> for ApiError {
    fn from(_: PathRejection) -> Self {
        Self(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT")
    }
}

pub fn router(app: App) -> Router {
    let index = ServeFile::new(format!("{}/index.html", app.config.web_dir));
    Router::new()
        .route("/health/live",get(||async{axum::Json(serde_json::json!({"status":"running"}))}))
        .route("/health/ready",get(ready))
          .route("/api/setup/status",get(configured_setup_status))
          .route("/api/v1/settings/initial-setup/{id}",get(configured_setup_operation))
        .route("/api/v1/auth/login",post(login))
        .route("/api/v1/auth/password",post(change_password))
        .route("/api/v1/auth/logout",post(logout))
        .route("/api/v1/session",get(session))
        .route("/api/v1/gateways",get(gateways).post(register_gateway))
        .route("/api/v1/gateways/{id}",get(gateway))
        .route("/api/v1/gateways/{id}/identity-bundle",get(identity_bundle))
        .route("/api/v1/gateways/{id}/certificate",post(rotate_gateway_certificate))
          .route("/api/v1/gateways/{id}/certificate-operations/{operation}",get(gateway_certificate_operation))
        .route("/api/v1/settings/tls",get(settings_tls).put(settings_tls_put))
        .route("/api/v1/settings/server-csr",get(server_csr_http::current).post(server_csr_http::create))
        .route("/api/v1/settings/server-csr/{id}",get(server_csr_http::read))
        .route("/api/v1/settings/server-csr/{id}/imports/{operation}",get(server_csr_http::read_import))
        .route("/api/v1/settings/server-csr/{id}/certificate",axum::routing::put(server_csr_http::import))
        .route("/api/v1/settings/server-csr/{id}/retire",post(server_csr_http::retire))
        .route("/api/v1/settings/server-csr/{id}/stage",post(server_csr_http::stage))
        .route("/api/v1/settings/client-ca",get(settings_client_ca).put(settings_client_ca_put))
          .route("/api/v1/settings/tls-activation",get(settings_tls_activation).post(settings_tls_activate))
          .route("/api/v1/settings/tls-activation/approvals",post(settings_tls_approve))
          .route("/api/v1/settings/tls-activation/approvals/{id}",get(settings_tls_approvals))
          .route("/api/v1/settings/tls-activation/{id}",get(settings_tls_operation))
          .route("/api/v1/settings/client-ca/policy",get(ca_policy_http::overview))
          .route("/api/v1/settings/client-ca/policy/proposals",post(ca_policy_http::propose))
          .route("/api/v1/settings/client-ca/policy/proposals/{id}",get(ca_policy_http::status))
          .route("/api/v1/settings/client-ca/policy/proposals/{id}/approve",post(ca_policy_http::approve))
        .route("/api/v1/settings/task-key",get(settings_task_key).post(settings_task_key_backup))
        .route("/api/v1/settings/task-key/rotate",post(settings_task_key_rotate))
        .route("/api/v1/settings/task-key/restore",post(settings_task_key_restore))
        .route("/api/v1/gateways/{id}/tasks",post(create_task))
        .route("/api/v1/gateways/{id}/task-intents/{key}",get(task_intent))
        .route("/api/v1/tasks",get(tasks))
        .route("/api/v1/tasks/{id}",get(task))
        .route("/api/v1/transfer/signers",get(transfer_signers).post(register_transfer_signer))
        .route("/api/v1/transfer/signers/{keyId}/revoke",post(revoke_transfer_signer))
        .route("/api/v1/transfer/approvals",get(transfer_approvals))
        .route("/api/v1/gateways/{id}/transfer-approval",post(request_transfer_approval))
        .route("/api/v1/transfer/approvals/{id}/approve",post(approve_transfer_approval))
        .route("/api/v1/transfer/approvals/{id}/reject",post(reject_transfer_approval))
        .route("/api/v1/transfer/approvals/{id}/revoke",post(revoke_transfer_approval))
        .route("/api/v1/transfer/issuances",get(transfer_issuances))
        .route("/api/v1/transfer/issuances/{transferId}/revoke",post(revoke_transfer_issuance))
        .route("/api/v1/events",get(events))
        .layer(axum::middleware::from_fn_with_state(app.clone(), enforce_password_change))
        .layer(axum::middleware::from_fn_with_state(app.gate.clone(), crate::maintenance::admit))
        .route_service("/",index.clone()).route_service("/login",index.clone())
          .route_service("/gateways",index.clone()).route_service("/commands",index.clone()).route_service("/setup",index.clone()).route_service("/settings",index)
        .nest_service("/assets",ServeDir::new(format!("{}/assets",app.config.web_dir)))
        .fallback(||async{ApiError::new(StatusCode::NOT_FOUND,"NOT_FOUND")})
        .method_not_allowed_fallback(||async{ApiError::new(StatusCode::METHOD_NOT_ALLOWED,"UNSUPPORTED")})
        .layer(DefaultBodyLimit::max(65536))
        .layer(SetResponseHeaderLayer::overriding(header::CACHE_CONTROL,HeaderValue::from_static("no-store")))
        .layer(SetResponseHeaderLayer::overriding(header::X_CONTENT_TYPE_OPTIONS,HeaderValue::from_static("nosniff")))
        .layer(SetResponseHeaderLayer::overriding(header::CONTENT_SECURITY_POLICY,HeaderValue::from_static("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'")))
        .with_state(app)
}
async fn ready(State(app): State<App>) -> Result<Json<Value>, ApiError> {
    if app.tls.as_ref().is_some_and(|runtime| !runtime.healthy()) {
        return Err(ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "CONFLICT"));
    }
    sqlx::query("SELECT 1").execute(&app.pool).await?;
    Ok(Json(json!({"status":"ready","protocolMajor":1})))
}

async fn configured_setup_status(State(app): State<App>) -> Result<Json<Value>, ApiError> {
    let saved = crate::console_settings::load_at(std::path::Path::new(&app.config.state_dir))?;
    Ok(Json(json!({"configured":true,"maintenance":app.gate.status()?.is_some(),"origin":app.config.origin,
        "operationId":saved.as_ref().map(|s|s.settings.operation_id),
        "installationId":saved.map(|s|s.settings.installation_id)})))
}

async fn configured_setup_operation(State(app): State<App>, headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let Path(id) = path?;
    Ok(Json(crate::initial_setup::operation(std::path::Path::new(&app.config.state_dir),id,false)?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND,"NOT_FOUND"))?))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ChangePassword {
    current_password: Option<String>,
    new_password: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Login {
    username: String,
    password: String,
}
/// Server-side enforcement of first-login rotation: a session flagged
/// must_change_password may only reach the session probe, the rotation
/// endpoint, and logout until the password is actually changed.
async fn enforce_password_change(
    State(app): State<App>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<Response, ApiError> {
    let path = req.uri().path();
    let exempt = matches!(
        path,
        "/api/v1/auth/login"
            | "/api/v1/auth/password"
            | "/api/v1/auth/logout"
            | "/api/v1/session"
            | "/health/ready"
            | "/health/live"
              | "/api/setup/status"
    );
    if exempt {
        return Ok(next.run(req).await);
    }
    let p = auth::principal(&app.pool, req.headers()).await?;
    if p.must_change_password {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "PASSWORD_CHANGE_REQUIRED",
        ));
    }
    Ok(next.run(req).await)
}

async fn login(
    State(app): State<App>,
    ConnectInfo(peer): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    input: Result<Json<Login>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(input) = input?;
    auth::origin(&headers, &app.config.origin)?;
    if input.username.len() > 128 || input.password.len() > 256 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let _permit = app
        .login_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError::new(StatusCode::TOO_MANY_REQUESTS, "RESOURCE_EXHAUSTED"))?;
    {
        let mut attempts = app.attempts.lock().await;
        attempts.retain(|_, q| {
            q.back()
                .is_some_and(|t| t.elapsed() < Duration::from_secs(60))
        });
        if attempts.len() >= 4096 {
            return Err(ApiError::new(
                StatusCode::TOO_MANY_REQUESTS,
                "RESOURCE_EXHAUSTED",
            ));
        }
        // IP limit prevents username rotation from evading the expensive password-work limit.
        let q = attempts.entry(peer.ip().to_string()).or_default();
        while q
            .front()
            .is_some_and(|t| t.elapsed() > Duration::from_secs(60))
        {
            q.pop_front();
        }
        if q.len() >= 10 {
            return Err(ApiError::new(
                StatusCode::TOO_MANY_REQUESTS,
                "RESOURCE_EXHAUSTED",
            ));
        }
        q.push_back(Instant::now());
    }
    let row = sqlx::query(
        "SELECT id,tenant_id,password_hash,must_change_password FROM mgmt_users WHERE username=$1 AND enabled",
    )
    .bind(&input.username)
    .fetch_optional(&app.pool)
    .await?;
    // A fixed valid hash gives unknown users the same password work as registered users.
    let encoded=row.as_ref().map(|r|r.get::<String,_>("password_hash")).unwrap_or_else(||"$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHRzb21lc2FsdA$2R2KCd50Fv5Qf8eij/GpOWYQW5Q8eP7kJxZaQHqWbIU".into());
    let valid =
        tokio::task::spawn_blocking(move || auth::password_valid(&input.password, &encoded))
            .await
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?;
    let Some(row) = row.filter(|_| valid) else {
        return Err(ApiError::unauthorized());
    };
    let token = auth::random_token();
    let csrf = auth::random_token();
    let id: Uuid = row.get("id");
    let mut tx = app.pool.begin().await?;
    sqlx::query("DELETE FROM mgmt_sessions WHERE expires_at<now() OR user_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,$3,now()+interval '8 hours')")
        .bind(auth::hash(token.as_bytes())).bind(id).bind(&csrf).execute(&mut *tx).await?;
    sqlx::query("INSERT INTO mgmt_audit(tenant_id,actor,operation,target,outcome) VALUES($1,$2,'session.login',$2,'SUCCEEDED')").bind(row.get::<String,_>("tenant_id")).bind(id.to_string()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok((
        [(
            header::SET_COOKIE,
            format!(
                "kura_session={token}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=28800"
            ),
        )],
        Json(json!({"csrf":csrf,"mustChangePassword":row.get::<bool,_>("must_change_password")})),
    )
        .into_response())
}

/// First-login rotation for the seeded bootstrap administrator: verifies the
/// current password, enforces the normal strength rules on the new one, clears
/// the must-change flag, and revokes every other session of the user.
async fn change_password(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<ChangePassword>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(input) = input?;
    let p = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let _permit = app
        .login_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError::new(StatusCode::TOO_MANY_REQUESTS, "RESOURCE_EXHAUSTED"))?;
    let mut tx = app.pool.begin().await?;
    let row=sqlx::query("SELECT password_hash,must_change_password FROM mgmt_users WHERE id=$1 AND enabled FOR UPDATE")
        .bind(p.id).fetch_optional(&mut *tx).await?.ok_or(ApiError::unauthorized())?;
    let forced: bool = row.get("must_change_password");
    let encoded: String = row.get("password_hash");
    // Initial setup already authenticated admin/admin. Ordinary changes prove
    // the current password. Locking prevents two initial sessions from racing.
    let valid = match input.current_password {
        None => forced,
        Some(current) => {
            tokio::task::spawn_blocking(move || auth::password_valid(&current, &encoded))
                .await
                .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?
        }
    };
    if !valid {
        sqlx::query("INSERT INTO mgmt_audit(tenant_id,actor,operation,target,outcome) VALUES($1,$2,'user.password.change',$2,'UNAUTHENTICATED')")
            .bind(&p.tenant_id).bind(p.id.to_string()).execute(&mut *tx).await?;
        tx.commit().await?;
        return Err(ApiError::unauthorized());
    }
    let hashed = tokio::task::spawn_blocking(move || auth::password_hash(&input.new_password))
        .await
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    sqlx::query("UPDATE mgmt_users SET password_hash=$2,must_change_password=false WHERE id=$1")
        .bind(p.id)
        .bind(&hashed)
        .execute(&mut *tx)
        .await?;
    // The verified current session stays; every other session of the user dies.
    let current_hash = auth::cookie(&headers).map(|t| auth::hash(t.as_bytes()));
    sqlx::query("DELETE FROM mgmt_sessions WHERE user_id=$1 AND token_hash<>COALESCE($2,'')")
        .bind(p.id)
        .bind(current_hash)
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO mgmt_audit(tenant_id,actor,operation,target,outcome) VALUES($1,$2,'user.password.change',$2,'SUCCEEDED')")
        .bind(&p.tenant_id).bind(p.id.to_string()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok((StatusCode::OK, Json(json!({"ok":true}))).into_response())
}
async fn session(
    State(app): State<App>,
    headers: HeaderMap,
) -> Result<Json<auth::Principal>, ApiError> {
    Ok(Json(auth::principal(&app.pool, &headers).await?))
}
async fn logout(State(app): State<App>, headers: HeaderMap) -> Result<Response, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    if let Some(token) = auth::cookie(&headers) {
        sqlx::query("DELETE FROM mgmt_sessions WHERE token_hash=$1")
            .bind(auth::hash(token.as_bytes()))
            .execute(&app.pool)
            .await?;
    }
    Ok((
        [(
            header::SET_COOKIE,
            "kura_session=; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0",
        )],
        StatusCode::NO_CONTENT,
    )
        .into_response())
}
#[derive(Deserialize, Default)]
struct Page {
    offset: Option<i64>,
}
fn unique_conflict(error: sqlx::Error) -> ApiError {
    if error
        .as_database_error()
        .is_some_and(|database| database.is_unique_violation())
    {
        ApiError::new(StatusCode::CONFLICT, "CONFLICT")
    } else {
        error.into()
    }
}

fn require_admin(p: &auth::Principal) -> Result<(), ApiError> {
    if p.role != "admin" {
        return Err(ApiError::forbidden());
    }
    Ok(())
}

fn certificate_pem_sha256(value: &str) -> Result<String, ApiError> {
    if value.is_empty() || value.len() > 65536 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let mut pem = value.as_bytes();
    let mut certificates = rustls_pemfile::certs(&mut pem);
    let certificate = match certificates.next() {
        Some(Ok(certificate)) => certificate,
        _ => return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT")),
    };
    if certificates.next().is_some() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    Ok(auth::hash(certificate.as_ref()))
}

fn signer_json(row: sqlx::postgres::PgRow) -> serde_json::Value {
    json!({
        "keyId": row.get::<String, _>("key_id"),
        "publicKeySha256": row.get::<String, _>("public_key_sha256"),
        "status": row.get::<String, _>("status"),
        "createdAt": row.get::<DateTime<Utc>, _>("created_at"),
        "revokedAt": row.get::<Option<DateTime<Utc>>, _>("revoked_at"),
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RegisterSigner {
    key_id: String,
    public_key_pem: String,
}

async fn transfer_signers(
    State(app): State<App>,
    headers: HeaderMap,
    page: Result<Query<Page>, QueryRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Query(page) = page?;
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    let rows = sqlx::query(
        "SELECT key_id,public_key_sha256,status,created_at,revoked_at \
         FROM mgmt_transfer_signers WHERE tenant_id=$1 \
         ORDER BY created_at DESC,key_id LIMIT 100 OFFSET $2",
    )
    .bind(&principal.tenant_id)
    .bind(page.offset.unwrap_or(0).clamp(0, 100000))
    .fetch_all(&app.pool)
    .await?;
    Ok(Json(
        json!({"items":rows.into_iter().map(signer_json).collect::<Vec<_>>()}),
    ))
}

async fn register_transfer_signer(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<RegisterSigner>, JsonRejection>,
) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
    let Json(input) = input?;
    let principal = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    require_admin(&principal)?;
    if !transfer::valid_identifier(&input.key_id)
        || input.public_key_pem.is_empty()
        || input.public_key_pem.len() > 4096
    {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let raw = transfer::ed25519_public_from_pem(&input.public_key_pem)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let digest = auth::hash(&transfer::ed25519_public_der(&raw)?);
    let mut transaction = app.pool.begin().await?;
    let inserted = sqlx::query(
        "INSERT INTO mgmt_transfer_signers(key_id,tenant_id,public_key_sha256) \
         VALUES($1,$2,$3) ON CONFLICT(key_id) DO NOTHING RETURNING key_id",
    )
    .bind(&input.key_id)
    .bind(&principal.tenant_id)
    .bind(&digest)
    .fetch_optional(&mut *transaction)
    .await
    .map_err(unique_conflict)?;
    if inserted.is_none() {
        let existing = sqlx::query(
            "SELECT tenant_id,public_key_sha256,status FROM mgmt_transfer_signers WHERE key_id=$1",
        )
        .bind(&input.key_id)
        .fetch_optional(&mut *transaction)
        .await?;
        let same = existing.is_some_and(|row| {
            row.get::<String, _>("tenant_id") == principal.tenant_id
                && row.get::<String, _>("public_key_sha256") == digest
                && row.get::<String, _>("status") == "ACTIVE"
        });
        if !same {
            return Err(ApiError::new(StatusCode::CONFLICT, "CONFLICT"));
        }
    }
    db::audit(
        &mut *transaction,
        &principal.tenant_id,
        &principal.id.to_string(),
        "transfer.signer.register",
        &input.key_id,
        "SUCCEEDED",
    )
    .await?;
    transaction.commit().await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"keyId":input.key_id,"publicKeySha256":digest,"status":"ACTIVE"})),
    ))
}

async fn revoke_transfer_signer(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Path(key_id) = path?;
    let principal = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    require_admin(&principal)?;
    let mut transaction = app.pool.begin().await?;
    let row = sqlx::query(
        "UPDATE mgmt_transfer_signers SET status='REVOKED',revoked_at=now() \
         WHERE key_id=$1 AND tenant_id=$2 AND status='ACTIVE' \
         RETURNING public_key_sha256",
    )
    .bind(&key_id)
    .bind(&principal.tenant_id)
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    db::audit(
        &mut *transaction,
        &principal.tenant_id,
        &principal.id.to_string(),
        "transfer.signer.revoke",
        &key_id,
        "SUCCEEDED",
    )
    .await?;
    transaction.commit().await?;
    Ok(Json(json!({
        "keyId":key_id,
        "publicKeySha256":row.get::<String,_>("public_key_sha256"),
        "status":"REVOKED"
    })))
}

fn issuance_json(row: sqlx::postgres::PgRow) -> serde_json::Value {
    json!({
        "transferId": row.get::<Uuid, _>("transfer_id"),
        "approvalId": row.get::<Uuid, _>("approval_id"),
        "gatewayId": row.get::<String, _>("gateway_id"),
        "expectedGeneration": row.get::<i64, _>("expected_generation"),
        "toGeneration": row.get::<i64, _>("to_generation"),
        "newGatewayId": row.get::<String, _>("new_gateway_id"),
        "newEndpoint": row.get::<String, _>("new_endpoint"),
        "signerKeyId": row.get::<String, _>("signer_key_id"),
        "issuedBy": row.get::<Uuid, _>("issued_by"),
        "issuedByName": row.get::<String, _>("issued_by_name"),
        "issuedAt": row.get::<DateTime<Utc>, _>("issued_at"),
        // Only a recorded, non-revoked issuance is ACTIVE; consumed tokens cannot
        // be told apart here and revocation never re-opens the approval.
        "status": if row.get::<Option<DateTime<Utc>>, _>("revoked_at").is_some() { "REVOKED" } else { "ACTIVE" },
        "revokedAt": row.get::<Option<DateTime<Utc>>, _>("revoked_at"),
        "revokedByName": row.get::<Option<String>, _>("revoked_by_name"),
    })
}

async fn transfer_issuances(
    State(app): State<App>,
    headers: HeaderMap,
    page: Result<Query<Page>, QueryRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Query(page) = page?;
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    let rows = sqlx::query(
        "SELECT i.transfer_id,i.approval_id,a.gateway_id,a.expected_generation,a.to_generation, \
                a.new_gateway_id,a.new_endpoint,i.signer_key_id,i.issued_by,u.username AS issued_by_name, \
                i.issued_at,i.revoked_at,ru.username AS revoked_by_name \
         FROM mgmt_transfer_issuances i \
         JOIN mgmt_transfer_approvals a ON a.id=i.approval_id \
         JOIN mgmt_users u ON u.id=i.issued_by \
         LEFT JOIN mgmt_users ru ON ru.id=i.revoked_by \
         WHERE a.tenant_id=$1 \
         ORDER BY i.issued_at DESC,i.transfer_id LIMIT 100 OFFSET $2",
    )
    .bind(&principal.tenant_id)
    .bind(page.offset.unwrap_or(0).clamp(0, 100000))
    .fetch_all(&app.pool)
    .await?;
    Ok(Json(
        json!({"items":rows.into_iter().map(issuance_json).collect::<Vec<_>>()}),
    ))
}

async fn revoke_transfer_issuance(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Path(transfer_id) = path?;
    let transfer_id = Uuid::parse_str(&transfer_id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let principal = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    require_admin(&principal)?;
    let mut transaction = app.pool.begin().await?;
    sqlx::query(
        "SELECT 1 FROM mgmt_transfer_issuances i \
         JOIN mgmt_transfer_approvals a ON a.id=i.approval_id \
         WHERE i.transfer_id=$1 AND a.tenant_id=$2",
    )
    .bind(transfer_id)
    .bind(&principal.tenant_id)
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    let row = sqlx::query(
        "UPDATE mgmt_transfer_issuances i SET revoked_at=now(),revoked_by=$2 \
         WHERE i.transfer_id=$1 AND i.revoked_at IS NULL \
         RETURNING i.signer_key_id",
    )
    .bind(transfer_id)
    .bind(principal.id)
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::new(StatusCode::CONFLICT, "CONFLICT"))?;
    db::audit(
        &mut *transaction,
        &principal.tenant_id,
        &principal.id.to_string(),
        "transfer.revocation",
        &transfer_id.to_string(),
        "SUCCEEDED",
    )
    .await?;
    transaction.commit().await?;
    Ok(Json(json!({
        "transferId":transfer_id,
        "signerKeyId":row.get::<String,_>("signer_key_id"),
        "status":"REVOKED"
    })))
}

fn approval_json(row: sqlx::postgres::PgRow) -> serde_json::Value {
    json!({
        "id": row.get::<Uuid, _>("id"),
        "gatewayId": row.get::<String, _>("gateway_id"),
        "expectedGeneration": row.get::<i64, _>("expected_generation"),
        "toGeneration": row.get::<i64, _>("to_generation"),
        "newGatewayId": row.get::<String, _>("new_gateway_id"),
        "newEndpoint": row.get::<String, _>("new_endpoint"),
        "newClientCaSha256": row.get::<String, _>("new_client_ca_sha256"),
        "newClientCertificateSha256": row.get::<String, _>("new_client_certificate_sha256"),
        "requestedBy": row.get::<Uuid, _>("requested_by"),
        "requestedByName": row.get::<String, _>("requested_by_name"),
        "approvedBy": row.get::<Option<Uuid>, _>("approved_by"),
        "approvedByName": row.get::<Option<String>, _>("approved_by_name"),
        "status": row.get::<String, _>("status"),
        "createdAt": row.get::<DateTime<Utc>, _>("created_at"),
        "approvedAt": row.get::<Option<DateTime<Utc>>, _>("approved_at"),
        "closedAt": row.get::<Option<DateTime<Utc>>, _>("closed_at"),
    })
}

const TRANSFER_APPROVALS: &str = "SELECT a.*,r.username AS requested_by_name,p.username AS approved_by_name \
     FROM mgmt_transfer_approvals a \
     JOIN mgmt_users r ON r.id=a.requested_by \
     LEFT JOIN mgmt_users p ON p.id=a.approved_by";

async fn transfer_approvals(
    State(app): State<App>,
    headers: HeaderMap,
    page: Result<Query<Page>, QueryRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Query(page) = page?;
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    let rows = sqlx::query(&format!(
        "{TRANSFER_APPROVALS} WHERE a.tenant_id=$1 \
         ORDER BY a.created_at DESC,a.id LIMIT 100 OFFSET $2"
    ))
    .bind(&principal.tenant_id)
    .bind(page.offset.unwrap_or(0).clamp(0, 100000))
    .fetch_all(&app.pool)
    .await?;
    Ok(Json(
        json!({"items":rows.into_iter().map(approval_json).collect::<Vec<_>>()}),
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TransferRequest {
    new_endpoint: String,
    new_gateway_id: String,
    new_client_ca_pem: String,
    new_client_certificate_pem: String,
}

async fn request_transfer_approval(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
    input: Result<Json<TransferRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
    let Json(input) = input?;
    let Path(gateway_id) = path?;
    let principal = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    require_admin(&principal)?;
    if !transfer::valid_transfer_endpoint(&input.new_endpoint)
        || !transfer::valid_identifier(&input.new_gateway_id)
    {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let ca_hash = certificate_pem_sha256(&input.new_client_ca_pem)?;
    let certificate_hash = certificate_pem_sha256(&input.new_client_certificate_pem)?;
    let approval_id = Uuid::new_v4();
    let mut transaction = app.pool.begin().await?;
    let gateway = sqlx::query(
        "SELECT binding_generation FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 AND enabled",
    )
    .bind(&gateway_id)
    .bind(&principal.tenant_id)
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    let generation = gateway.get::<i64, _>("binding_generation");
    sqlx::query(
        "INSERT INTO mgmt_transfer_approvals( \
            id,tenant_id,gateway_id,expected_generation,to_generation,new_gateway_id, \
            new_endpoint,new_client_ca_sha256,new_client_certificate_sha256,requested_by \
         ) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
    )
    .bind(approval_id)
    .bind(&principal.tenant_id)
    .bind(&gateway_id)
    .bind(generation)
    .bind(generation + 1)
    .bind(&input.new_gateway_id)
    .bind(&input.new_endpoint)
    .bind(&ca_hash)
    .bind(&certificate_hash)
    .bind(principal.id)
    .execute(&mut *transaction)
    .await
    .map_err(unique_conflict)?;
    db::audit(
        &mut *transaction,
        &principal.tenant_id,
        &principal.id.to_string(),
        "transfer.approval.request",
        &gateway_id,
        "SUCCEEDED",
    )
    .await?;
    transaction.commit().await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"id":approval_id,"status":"PENDING"})),
    ))
}

async fn transfer_approval_action(
    app: App,
    headers: HeaderMap,
    approval_id: Uuid,
    action: &str,
) -> Result<Json<serde_json::Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    require_admin(&principal)?;
    let action = transfer::ApprovalAction::parse(action)
        .ok_or(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let gateway_id = transfer::decide_approval(
        &app.pool,
        Some(&principal.tenant_id),
        approval_id,
        principal.id,
        action,
    )
    .await?
    .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    Ok(Json(
        json!({"id":approval_id,"gatewayId":gateway_id,"status":action.status()}),
    ))
}

async fn approve_transfer_approval(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Path(id) = path?;
    transfer_approval_action(app, headers, id, "approve").await
}

async fn reject_transfer_approval(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Path(id) = path?;
    transfer_approval_action(app, headers, id, "reject").await
}

async fn revoke_transfer_approval(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let Path(id) = path?;
    transfer_approval_action(app, headers, id, "revoke").await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RegisterGateway {
    id: String,
    name: String,
    client_certificate_pem: String,
    #[serde(default)]
    client_key_pem: Option<String>,
}

async fn register_gateway(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<RegisterGateway>, JsonRejection>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    crate::gateway_registration::validate(&input.id, &p.tenant_id, &input.name)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let fingerprint =
        crate::gateway_registration::certificate_sha256(&input.client_certificate_pem)
            .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    // Scheme A: an uploaded private key is packaged once and handed back once.
    // It never appears in a response or log again, and is wiped from storage at
    // first download.
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let mut bundle = None;
    if let Some(key) = &input.client_key_pem {
        crate::tls_store::validate_pair(&input.client_certificate_pem, key)
            .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
        let token = crate::provisioning::bundle_token();
        let issued_at = Utc::now();
        // The certificate PEM is public but stored only here, sealed next to the
        // key, so mgmt_gateways keeps holding just the fingerprint.
        let contents = json!({"kind":"identity","certificatePem": input.client_certificate_pem, "keyPem": key});
        let sealed = cipher.seal_identity_bundle(
                &input.id,
                issued_at.timestamp_millis(),
                serde_json::to_vec(&contents).map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?.as_slice(),
            )
            .map_err(ApiError::from)?;
        bundle = Some((
            token.clone(),
            issued_at,
            serde_json::to_string(&sealed).map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?,
            auth::hash(token.as_bytes()),
        ));
    }
    crate::gateway_registration::register_in_transaction(
        &mut tx,
        &input.id,
        &p.tenant_id,
        &input.name,
        &fingerprint,
        &p.id.to_string(),
    )
    .await
    .map_err(unique_conflict)?;
    let mut response = json!({"id":input.id,"name":input.name,"certificateSha256":fingerprint,"bindingGeneration":1});
    if let Some((token, issued_at, ciphertext, token_hash)) = bundle {
        let result = sqlx::query(
            "UPDATE mgmt_gateways SET identity_bundle_token_hash=$1,identity_bundle_ciphertext=$2,identity_bundle_issued_at=$3 \
             WHERE id=$4 AND tenant_id=$5 AND identity_bundle_token_hash IS NULL",
        )
        .bind(&token_hash)
        .bind(&ciphertext)
        .bind(issued_at)
        .bind(&input.id)
        .bind(&p.tenant_id)
        .execute(&mut *tx)
        .await?;
        if result.rows_affected() != 1 {
            db::audit(&mut *tx, &p.tenant_id, &p.username, "gateway.identity-bundle.issue", &input.id, "FAILED")
                .await?;
            return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"));
        }
        db::audit(&mut *tx, &p.tenant_id, &p.username, "gateway.identity-bundle.issue", &input.id, "SUCCEEDED")
            .await?;
        response["identityBundleToken"] = json!(token);
        response["identityBundleUrl"] =
            json!(format!("/api/v1/gateways/{}/identity-bundle", input.id));
    }
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct IdentityBundleQuery {
    token: String,
    // Required for identity bundles (the agent's console endpoint); ignored
    // for rotation bundles, which never change the endpoint.
    endpoint: Option<String>,
}

async fn identity_bundle(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
    query: Result<Query<IdentityBundleQuery>, QueryRejection>,
) -> Result<Response, ApiError> {
    let Path(id) = path?;
    let Query(query) = query?;
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let row = sqlx::query(
        "SELECT identity_bundle_token_hash,identity_bundle_ciphertext,identity_bundle_issued_at,identity_bundle_delivered_at \
         FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2",
    )
    .bind(&id)
    .bind(&p.tenant_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    let token_hash: Option<String> = row.get("identity_bundle_token_hash");
    let ciphertext: Option<String> = row.get("identity_bundle_ciphertext");
    let issued_at: Option<DateTime<Utc>> = row.get("identity_bundle_issued_at");
    let delivered: Option<DateTime<Utc>> = row.get("identity_bundle_delivered_at");
    let (token_hash, ciphertext, issued_at) = match (token_hash, ciphertext, issued_at) {
        (Some(hash), Some(value), Some(issued_at)) if delivered.is_none() => (hash, value, issued_at),
        _ => return Err(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND")),
    };
    if !crate::provisioning::token_matches(&token_hash, &query.token) {
        // Constant-ish response: unknown token is never distinguished from an
        // already-delivered bundle.
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"));
    }
    let sealed: Value = serde_json::from_str(&ciphertext)
        .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?;
    let sealed = cipher.open_identity_bundle(&id, issued_at.timestamp_millis(), &sealed)
        .map_err(ApiError::from)?;
    let contents: Value =
        serde_json::from_slice(&sealed).map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?;
    let cert_pem = contents
        .get("certificatePem")
        .and_then(Value::as_str)
        .ok_or(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?;
    let key_pem = contents
        .get("keyPem")
        .and_then(Value::as_str)
        .ok_or(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?;
    let ca = crate::settings::live_client_ca(&app.effective_config())?;
    // Plain HTTPS endpoint for the agent; mirrors identity_settings on the agent
    // (scheme, host/port characters only, at most scheme://host:port, no path).
    let endpoint_ok = |endpoint: &str| {
        endpoint
            .strip_prefix("https://")
            .is_some_and(|rest| {
                (rest.ends_with(|c: char| c.is_ascii_alphanumeric() || c == ']'))
                    && rest.len() <= 193
                    && rest.chars().all(|c| {
                        c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':' | '[' | ']')
                    })
                    && rest.matches(':').count() <= 1
                    && !rest.contains("..")
            })
    };
    let (bundle, filename) = if contents.get("kind").and_then(Value::as_str) == Some("rotation") {
        let previous = contents
            .get("previousCertificateSha256")
            .and_then(Value::as_str)
            .ok_or(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?;
        (
            crate::provisioning::build_identity_bundle(
                &id,
                crate::provisioning::BundleKind::Rotation,
                "",
                &ca,
                cert_pem,
                key_pem,
                Some(previous),
            )?,
            format!("{id}-rotation-bundle.tar"),
        )
    } else {
        let endpoint = query.endpoint.as_deref().filter(|value| endpoint_ok(value));
        let endpoint = match endpoint {
            Some(value) => value,
            None => return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT")),
        };
        (
            crate::provisioning::build_identity_bundle(
                &id,
                crate::provisioning::BundleKind::Identity,
                endpoint,
                &ca,
                cert_pem,
                key_pem,
                None,
            )?,
            format!("{id}-identity-bundle.tar"),
        )
    };
    let result = sqlx::query(
        "UPDATE mgmt_gateways SET identity_bundle_delivered_at=now(),identity_bundle_token_hash=NULL,identity_bundle_ciphertext=NULL \
         WHERE id=$1 AND tenant_id=$2 AND identity_bundle_delivered_at IS NULL AND identity_bundle_token_hash=$3",
    )
    .bind(&id)
    .bind(&p.tenant_id)
    .bind(&token_hash)
    .execute(&mut *tx)
    .await?;
    if result.rows_affected() != 1 {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"));
    }
    db::audit(&mut *tx, &p.tenant_id, &p.username, "gateway.identity-bundle.deliver", &id, "SUCCEEDED")
        .await?;
    tx.commit().await?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/x-tar"),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{filename}\"")),
        ],
        bundle,
    )
        .into_response())
}

async fn settings_tls(
    State(app): State<App>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    if let Some(runtime) = &app.tls {
        let status = runtime.status()?;
        return Ok(Json(json!({"origin":app.effective_config().origin,"certificateFingerprint":status["certificateFingerprint"],
            "pendingFingerprint":status["pendingTlsFingerprint"],"restartRequired":false,
            "activationRequired":status["pendingTlsFingerprint"].is_string()})));
    }
    let config=app.effective_config();
    let mut value=crate::settings::tls_status(&config)?;
    value["origin"]=json!(config.origin);
    Ok(Json(value))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TlsUpload {
    server_pem: String,
    server_key: String,
}

async fn settings_tls_put(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<TlsUpload>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    crate::server_csr::ensure_no_uncertain_stage(std::path::Path::new(&app.config.state_dir))?;
    crate::tls_store::validate_pair(&input.server_pem, &input.server_key)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let _guard = match &app.tls { Some(runtime) => Some(runtime.changes.lock().await), None => None };
    let config = match &app.tls { Some(runtime) => runtime.prepare_stage()?, None => app.config.clone() };
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.tls.stage", "tls-server", "REQUESTED").await?;
    crate::tls_store::stage(&config, &input.server_pem, &input.server_key, &p.username)?;
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.tls.update", "tls-server", "SUCCEEDED")
        .await?;
    Ok(Json(
        json!({"staged":true,"restartRequired":app.tls.is_none(),"activationRequired":app.tls.is_some(),"pendingFingerprint":crate::settings::tls_status(&config)?.get("pendingFingerprint")}),
    ))
}

async fn settings_client_ca(
    State(app): State<App>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let mut status = if let Some(runtime) = &app.tls {
        let status = runtime.status()?;
        json!({"caFingerprint":status["caFingerprint"],"pendingFingerprint":status["pendingCaFingerprint"],
            "restartRequired":false,"activationRequired":status["pendingCaFingerprint"].is_string()})
    } else { crate::settings::client_ca_status(&app.effective_config())? };
    let affected: i64 = sqlx::query_scalar("SELECT count(*) FROM mgmt_gateways")
        .fetch_one(&app.pool)
        .await?;
    status["registeredGateways"] = json!(affected);
    Ok(Json(status))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ClientCaUpload {
    client_ca_pem: String,
}

async fn settings_client_ca_put(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<ClientCaUpload>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    let fingerprint = crate::client_ca_store::validate(&input.client_ca_pem)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let affected: i64 = sqlx::query_scalar("SELECT count(*) FROM mgmt_gateways")
        .fetch_one(&app.pool)
        .await?;
    let _guard = match &app.tls { Some(runtime) => Some(runtime.changes.lock().await), None => None };
    let config = match &app.tls { Some(runtime) => runtime.prepare_stage()?, None => app.config.clone() };
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.client-ca.stage", "client-ca", "REQUESTED").await?;
    crate::client_ca_store::stage(&config, &input.client_ca_pem, &p.username)?;
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.client-ca.update", "client-ca", "SUCCEEDED")
        .await?;
    Ok(Json(json!({"staged":true,"restartRequired":app.tls.is_none(),"activationRequired":app.tls.is_some(),"pendingFingerprint":fingerprint,"registeredGateways":affected})))
}

async fn settings_tls_activation(State(app): State<App>, headers: HeaderMap) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let runtime = app.tls.as_ref().ok_or(ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "UNSUPPORTED"))?;
    let _guard = runtime.changes.lock().await;
    let mut status = runtime.status()?;
    status["registeredGateways"] = json!(sqlx::query_scalar::<_, i64>("SELECT count(*) FROM mgmt_gateways")
        .fetch_one(&app.pool).await?);
    Ok(Json(status))
}

async fn settings_tls_operation(State(app): State<App>, headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let Path(id) = path?;
    let runtime = app.tls.as_ref().ok_or(ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "UNSUPPORTED"))?;
    Ok(Json(runtime.operation(id)?.ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?))
}

async fn settings_tls_approvals(State(app): State<App>, headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>) -> Result<Json<crate::ca_quorum::Status>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let Path(id) = path?;
    let status = crate::ca_quorum::status(&app.pool, &p.tenant_id, id).await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    Ok(Json(status))
}

async fn settings_tls_approve(State(app): State<App>, headers: HeaderMap,
    input: Result<Json<crate::tls_runtime::ActivationRequest>, JsonRejection>)
    -> Result<Json<crate::ca_quorum::Status>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    if p.must_change_password {
        return Err(ApiError::forbidden());
    }
    let Json(input) = input?;
    let runtime = app.tls.as_ref().ok_or(ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "UNSUPPORTED"))?;
    let _guard = runtime.changes.lock().await;
    let conflict = || ApiError::new(StatusCode::CONFLICT, "CONFLICT");
    if runtime.operation(input.operation_id)?.is_some() {
        return Err(conflict());
    }
    let target = runtime.activation_approval_target(&input)
        .map_err(|_| conflict())?.ok_or_else(conflict)?;
    let status = crate::ca_quorum::approve(&app.pool, &p, &target).await?.ok_or_else(conflict)?;
    Ok(Json(status))
}

async fn settings_tls_activate(State(app): State<App>, headers: HeaderMap,
    input: Result<Json<crate::tls_runtime::ActivationRequest>, JsonRejection>) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    let runtime = app.tls.as_ref().ok_or(ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "UNSUPPORTED"))?;
    let _guard = runtime.changes.lock().await;
    // Duplicate ids are observation only and do not append a second action audit.
    if let Some(record) = runtime.operation(input.operation_id)? {
        if record["request"] != serde_json::to_value(&input).map_err(anyhow::Error::from)? {
            return Err(ApiError::new(StatusCode::CONFLICT, "CONFLICT"));
        }
        return Ok(Json(record));
    }
    // A stale or invalid request still reaches the existing durable rejection.
    let target = runtime.activation_approval_target(&input).ok().flatten();
    let quorum_required = || ApiError::new(StatusCode::CONFLICT, "CA_QUORUM_REQUIRED");
    if let Some(target) = &target {
        let status = crate::ca_quorum::status(&app.pool, &p.tenant_id, target.operation_id).await?;
        if !status.is_some_and(|status| status.authorizes(target)) {
            return Err(quorum_required());
        }
    }
    // Commit REQUESTED first so it survives a crash before publishing the TLS
    // intent. Only the subsequent locked quorum can authorize publication.
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.tls.activate", &input.operation_id.to_string(), "REQUESTED").await?;
    let grant = match target {
        Some(target) => Some(crate::ca_quorum::check_activation_quorum(&app.pool, &p, target).await?
            .ok_or_else(quorum_required)?),
        None => None,
    };
    let result = runtime.activate_authorized(input, &p.username, grant.as_ref())
        .map_err(|_| ApiError::new(StatusCode::CONFLICT, "CONFLICT"))?;
    Ok(Json(result))
}

async fn settings_task_key(
    State(app): State<App>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    let cipher = app.cipher.read().await;
    Ok(Json(crate::settings::task_key_status(&app.config, &cipher)?))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TaskKeyBackup {
    passphrase: String,
}

fn cipher_backup(state: &std::path::Path, cipher: &TaskCipher, passphrase: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key_path = crate::task_key_store::retain(state, cipher)?;
    let backup_file = state.join(format!(".task-key-backup.{}", Uuid::new_v4()));
    let passphrase_file = state.join(format!(".task-key-pass.{}", Uuid::new_v4()));
    let result = (|| -> anyhow::Result<Vec<u8>> {
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC).open(&passphrase_file)?;
        file.write_all(passphrase)?;
        file.sync_all()?;
        crate::task_crypto::backup_key(&key_path, &backup_file, &passphrase_file)?;
        Ok(std::fs::read(&backup_file)?)
    })();
    let _ = std::fs::remove_file(&passphrase_file);
    let _ = std::fs::remove_file(&backup_file);
    result
}

async fn settings_task_key_backup(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<TaskKeyBackup>, JsonRejection>,
) -> Result<Response, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    let passphrase = input.passphrase.into_bytes();
    if !(16..=1024).contains(&passphrase.len()) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let payload = cipher_backup(std::path::Path::new(&app.config.state_dir), &cipher, &passphrase)?;
    tx.commit().await?;
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.task-key.backup", "task-key", "SUCCEEDED")
        .await?;
    Ok((
        [
            (header::CONTENT_TYPE, "application/octet-stream"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"task-key.backup\""),
        ],
        payload,
    )
        .into_response())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TaskKeyRestore {
    backup_base64: String,
    passphrase: String,
}

async fn settings_task_key_restore(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<TaskKeyRestore>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    if input.backup_base64.len() > 8192 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let payload = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &input.backup_base64,
    )
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let passphrase = input.passphrase.into_bytes();
    if !(16..=1024).contains(&passphrase.len()) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let scratch = std::path::Path::new(&app.config.state_dir);
    crate::task_key_store::retain(scratch, &cipher)?;
    let candidate_file = scratch.join(format!(".task-key-candidate.{}", Uuid::new_v4()));
    let backup_file = scratch.join(format!(".task-key-restore.{}", Uuid::new_v4()));
    let passphrase_file = scratch.join(format!(".task-key-pass.{}", Uuid::new_v4()));
    let target = std::path::Path::new(&app.config.state_dir)
        .join("pending")
        .join("task-data.key");
    let result = (|| -> anyhow::Result<()> {
        std::fs::create_dir_all(scratch)?;
        {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
                .open(&backup_file)?;
            file.write_all(&payload)?;
        }
        {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
                .open(&passphrase_file)?;
            file.write_all(&passphrase)?;
        }
        // Keep candidate private until its authenticated DB binding is checked.
        crate::task_crypto::restore_key(&backup_file, &candidate_file, &passphrase_file)
    })();
    let _ = std::fs::remove_file(&passphrase_file);
    let _ = std::fs::remove_file(&backup_file);
    let validation = async {
        result?;
        let candidate = TaskCipher::load(&candidate_file)?;
        anyhow::ensure!(candidate.matches_storage(&app.pool).await?, "restored key does not match the current database");
        candidate.persist_new(&target)?;
        Ok::<_, anyhow::Error>(())
    }.await;
    let _ = std::fs::remove_file(&candidate_file);
    validation.map_err(ApiError::from)?;
    tx.commit().await?;
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.task-key.restore", "task-key", "SUCCEEDED")
        .await?;
    Ok(Json(json!({"staged":true,"restartRequired":true})))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RotateGatewayCertificate {
    #[serde(default)]
    operation_id: Option<String>,
    previous_certificate_sha256: String,
    #[serde(default)]
    previous_binding_generation: Option<i64>,
    client_certificate_pem: String,
    #[serde(default)]
    client_key_pem: Option<String>,
}

async fn rotate_gateway_certificate(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
    input: Result<Json<RotateGatewayCertificate>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let Json(input) = input?;
    let Path(id) = path?;
    if input.previous_certificate_sha256.len() != 64
        || input.previous_binding_generation.is_some_and(|value| value < 1)
        || !input.previous_certificate_sha256.chars().all(|c| c.is_ascii_hexdigit())
    {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let client_operation = input.operation_id.is_some();
    let operation = match &input.operation_id {
        Some(value) => crate::gateway_certificate::operation(value)?,
        None => Uuid::new_v4(),
    };
    let certificate = crate::gateway_registration::certificate_sha256(&input.client_certificate_pem).ok();
    let invalid = if certificate.is_none() || input.client_key_pem.as_ref().is_some_and(|key|
        crate::tls_store::validate_pair(&input.client_certificate_pem, key).is_err()) {
        Some("INVALID_ARGUMENT")
    } else { None };
    let original = crate::gateway_certificate::Request {
        operation, gateway:id.clone(), tenant:p.tenant_id.clone(),
        previous:input.previous_certificate_sha256.to_ascii_lowercase(),
          previous_generation:input.previous_binding_generation,
        certificate, pem_hash:auth::hash(input.client_certificate_pem.as_bytes()),
        bundle:input.client_key_pem.is_some(),
    };
    if !crate::gateway_certificate::reserve(&app.pool, &original, &p.username, invalid).await? {
        let receipt=crate::gateway_certificate::read(&app.pool,&p.tenant_id,&id,operation).await?;
        if client_operation { return Ok(Json(receipt)); }
        return Err(ApiError::new(StatusCode::CONFLICT,"CONFLICT"));
    }
    let new_fingerprint=original.certificate.clone().ok_or_else(||ApiError::new(StatusCode::BAD_REQUEST,"INVALID_ARGUMENT"))?;
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    if !crate::gateway_certificate::begin_application(&mut tx,&original).await? {
        db::audit(&mut *tx,&p.tenant_id,&p.username,"gateway.certificate.rotate",&id,"REJECTED").await?;
        tx.commit().await?;
        // Legacy clients do not inspect a state field. A refused credential transaction must stay non-2xx.
        return Err(ApiError::new(StatusCode::CONFLICT,"CONFLICT"));
    }
    let mut bundle = None;
    if let Some(key) = &input.client_key_pem {
        crate::tls_store::validate_pair(&input.client_certificate_pem, key)
            .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
        let token = crate::provisioning::bundle_token();
        let issued_at = Utc::now();
        let contents = json!({
            "kind": "rotation",
            "certificatePem": input.client_certificate_pem,
            "keyPem": key,
            "previousCertificateSha256": original.previous,
        });
        let sealed = cipher.seal_identity_bundle(
                &id,
                issued_at.timestamp_millis(),
                serde_json::to_vec(&contents).map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?.as_slice(),
            )
            .map_err(ApiError::from)?;
        bundle = Some((
            token.clone(),
            issued_at,
            serde_json::to_string(&sealed).map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"))?,
            auth::hash(token.as_bytes()),
        ));
    }
    // Same admission as the CLI rotate-gateway-certificate: fingerprint swap,
    // connection reset, audit + identityChanged event in one transaction.
    let row = sqlx::query(
        "UPDATE mgmt_gateways SET certificate_sha256=$2,connection_epoch=NULL,last_seen=NULL,online_reported=false,kura_available=false,revision=revision+1 \
         WHERE id=$1 AND tenant_id=$3 AND certificate_sha256=$4 AND certificate_sha256<>$2 RETURNING revision",
    )
    .bind(&id)
    .bind(&new_fingerprint)
    .bind(&p.tenant_id)
    .bind(&original.previous)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::new(StatusCode::CONFLICT, "CONFLICT"))?;
    let revision: i64 = row.get("revision");
    crate::gateway_certificate::applied(&mut tx,&original,revision).await?;
    db::audit(&mut *tx, &p.tenant_id, &p.username, "gateway.certificate.rotate", &id, "SUCCEEDED").await?;
    db::event(&mut *tx, &p.tenant_id, "gateway", &id, revision, json!({"online":false,"identityChanged":true})).await?;
    let mut issued_token = None;
    if let Some((token, issued_at, ciphertext, token_hash)) = bundle {
        issued_token = Some(token);
        // A new rotation package replaces any un-delivered package; the old
        // token/ciphertext are wiped so they can never be delivered.
        sqlx::query(
            "UPDATE mgmt_gateways SET identity_bundle_token_hash=$1,identity_bundle_ciphertext=$2,identity_bundle_issued_at=$3,identity_bundle_delivered_at=NULL \
             WHERE id=$4 AND tenant_id=$5",
        )
        .bind(&token_hash)
        .bind(&ciphertext)
        .bind(issued_at)
        .bind(&id)
        .bind(&p.tenant_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    let mut response = crate::gateway_certificate::read(&app.pool,&p.tenant_id,&id,operation).await?;
    if let Some(token) = issued_token {
        response["identityBundleToken"] = json!(token);
        response["identityBundleUrl"] = json!(format!("/api/v1/gateways/{}/identity-bundle", id));
    }
    Ok(Json(response))
}

async fn gateway_certificate_operation(
    State(app):State<App>, headers:HeaderMap,
    path:Result<Path<(String,String)>,PathRejection>,
)->Result<Json<Value>,ApiError> {
    let Path((gateway,operation))=path?;
    let p=auth::principal(&app.pool,&headers).await?;require_admin(&p)?;
    let operation=crate::gateway_certificate::operation(&operation)?;
    Ok(Json(crate::gateway_certificate::read(&app.pool,&p.tenant_id,&gateway,operation).await?))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TaskKeyRotate {
    passphrase: String,
    confirmed: bool,
}

async fn settings_task_key_rotate(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<TaskKeyRotate>, JsonRejection>,
) -> Result<Response, ApiError> {
    let p = auth::principal(&app.pool, &headers).await?;
    require_admin(&p)?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    let Json(input) = input?;
    if !input.confirmed {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let passphrase = input.passphrase.into_bytes();
    if !(16..=1024).contains(&passphrase.len()) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    // One shared in-process barrier covers HTTP and each gRPC DB operation.
    // DB binding locks also fence a stale second process/CLI. Durable old/new
    // slots and the encrypted download are prepared before DB mutation.
    let mut live = app.cipher.write().await;
    let mut new_key = [0u8; 32];
    use rand_core::RngCore;
    rand_core::OsRng.fill_bytes(&mut new_key);
    let replacement = TaskCipher::from_key_bytes(&new_key);
    let scratch = std::path::Path::new(&app.config.state_dir);
    crate::task_key_store::prepare(scratch, &live, &replacement)?;
    let payload = cipher_backup(scratch, &replacement, &passphrase)?;
    let rotated = match live.rotate_storage(&replacement, &app.pool).await {
        Ok(count) => { *live = replacement; count }
        Err(error) => {
            // Commit errors can be ambiguous. Select by authenticated binding;
            // never repeat the rotation or delete either recovery slot.
            if replacement.matches_storage(&app.pool).await.unwrap_or(false) {
                *live = replacement;
            }
            return Err(ApiError::from(error));
        }
    };
    db::audit(&app.pool, &p.tenant_id, &p.username, "settings.task-key.rotate", "task-key", "SUCCEEDED")
        .await?;
    tracing::info!(rotated_tasks = rotated, "task storage key rotated from the web UI");
    Ok((
        [
            (header::CONTENT_TYPE, "application/octet-stream"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"task-key-rotated.backup\""),
        ],
        payload,
    )
        .into_response())
}

fn gateway_json(r: sqlx::postgres::PgRow) -> Value {
    json!({"id":r.get::<String,_>("id"),"name":r.get::<String,_>("name"),"online":r.get::<bool,_>("online"),"enabled":r.get::<bool,_>("enabled"),"kuraAvailable":r.get::<bool,_>("kura_available"),"lastSeen":r.get::<Option<DateTime<Utc>>,_>("last_seen"),"agentVersion":r.get::<Option<String>,_>("agent_version"),"capabilities":r.get::<Value,_>("capabilities"),"revision":r.get::<i64,_>("revision"),"uptimeSeconds":r.get::<i64,_>("uptime_seconds"),"bindingGeneration":r.get::<i64,_>("binding_generation"),"certificateSha256":r.get::<String,_>("certificate_sha256")})
}
const GATEWAYS: &str = "SELECT *,coalesce(last_seen>now()-interval '15 seconds' AND enabled,false) AS online FROM mgmt_gateways";
async fn gateways(
    State(app): State<App>,
    headers: HeaderMap,
    page: Result<Query<Page>, QueryRejection>,
) -> Result<Json<Value>, ApiError> {
    let Query(page) = page?;
    let p = auth::principal(&app.pool, &headers).await?;
    if !p.can_query() {
        return Err(ApiError::forbidden());
    }
    let rows = sqlx::query(&format!(
        "{GATEWAYS} WHERE tenant_id=$1 ORDER BY id LIMIT 100 OFFSET $2"
    ))
    .bind(&p.tenant_id)
    .bind(page.offset.unwrap_or(0).clamp(0, 100000))
    .fetch_all(&app.pool)
    .await?;
    Ok(Json(
        json!({"items":rows.into_iter().map(gateway_json).collect::<Vec<_>>(),"limit":100}),
    ))
}
async fn gateway(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
) -> Result<Json<Value>, ApiError> {
    let Path(id) = path?;
    let p = auth::principal(&app.pool, &headers).await?;
    let row = sqlx::query(&format!("{GATEWAYS} WHERE tenant_id=$1 AND id=$2"))
        .bind(&p.tenant_id)
        .bind(id)
        .fetch_optional(&app.pool)
        .await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    Ok(Json(gateway_json(row)))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CreateTask {
    #[serde(default)]
    imported_firewall: Option<yofc_management_protocol::imported::Request>,
    #[serde(default)]
    firewall_change: Option<yofc_management_protocol::firewall::Change>,
    #[serde(default)]
    firewall_action: Option<yofc_management_protocol::v1::FirewallAction>,
    #[serde(default)]
    nm_change: Option<yofc_management_protocol::nm::Change>,
    #[serde(default)]
    nm_action: Option<yofc_management_protocol::v1::NmAction>,
    operation: String,
    idempotency_key: String,
    #[serde(default)]
    ordinary_boot_action: Option<yofc_management_protocol::v1::OrdinaryBootAction>,
    #[serde(default)]
    ordinary_handoff_action: Option<yofc_management_protocol::v1::OrdinaryHandoffAction>,
    #[serde(default)]
    system_control_action: Option<yofc_management_protocol::v1::SystemControlAction>,
    #[serde(default)]
    network_change: Option<yofc_management_protocol::v1::NetworkChange>,
    #[serde(default)]
    network_action: Option<yofc_management_protocol::v1::NetworkAction>,
    #[serde(default)]
    network_takeover: Option<yofc_management_protocol::v1::NetworkTakeover>,
    #[serde(default)]
    kura_configuration: Option<yofc_management_protocol::v1::KuraConfigurationTarget>,
    #[serde(default)]
    kura_configuration_update: Option<yofc_management_protocol::v1::KuraConfigurationUpdate>,
    #[serde(default)]
    kura_snapshot_save: Option<yofc_management_protocol::v1::KuraSnapshotSave>,
    #[serde(default)]
    kura_snapshot_rollback: Option<yofc_management_protocol::v1::KuraSnapshotRollback>,
}
async fn create_task(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<String>, PathRejection>,
    input: Result<Json<CreateTask>, JsonRejection>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let Json(input) = input?;
    let Path(id) = path?;
    let p = auth::principal(&app.pool, &headers).await?;
    auth::csrf(&headers, &p, &app.config.origin)?;
    if !p.can_command() {
        db::audit(
            &app.pool,
            &p.tenant_id,
            &p.id.to_string(),
            "task.create",
            &id,
            "FORBIDDEN",
        )
        .await
        .map_err(|_| ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL"))?;
        return Err(ApiError::forbidden());
    }
    let operation = match input.operation.as_str() {
          "firewall.imported.view" => 38,
          "firewall.imported.review" => 39,
          "firewall.imported.prepare" => 40,
          "firewall.imported.apply" => 41,
          "firewall.imported.confirm" => 42,
          "firewall.imported.rollback" => 43,
          "firewall.imported.cancel" => 44,
          "firewall.imported.outcome" => 45,
          "firewall.imported.source.review" => 46,
          "firewall.imported.source.observe" => 47,
          "firewall.imported.source.outcome" => 48,
        "firewall.view" => 30,
        "firewall.prepare" => 31,
        "firewall.confirm" => 32,
        "firewall.rollback" => 33,
        "nm.view" => 34,
        "nm.prepare" => 35,
        "nm.confirm" => 36,
        "nm.rollback" => 37,
        "agent.status" => 1,
        "kura.configuration.list" => 2,
        "kura.snapshot.list" => 3,
        "network.inspect" => 4,
        "network.prepare" => 5,
        "network.confirm" => 6,
        "network.rollback" => 7,
        "network.transaction" => 8,
        "network.takeover.plan" => 9,
        "network.takeover.prepare" => 10,
        "kura.configuration.get" => 11,
        "kura.configuration.update" => 12,
        "kura.snapshot.save" => 13,
        "kura.snapshot.rollback" => 14,
        "ordinary.boot.status" => 15,
        "ordinary.boot.prepare" => 16,
        "ordinary.boot.execute" => 17,
        "ordinary.boot.cancel" => 18,
        "ordinary.boot.reconcile" => 19,
        "recovery.ordinary.status" => 20,
        "recovery.ordinary.prepare" => 21,
        "recovery.ordinary.execute" => 22,
        "recovery.ordinary.cancel" => 29,
        "service.status" => 23,
        "service.restart.prepare" => 24,
        "service.restart" => 25,
        "system.reboot.prepare" => 26,
        "system.reboot" => 27,
        "system.control.cancel" => 28,
        _ => {
            return Err(ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "UNSUPPORTED",
            ));
        }
    };
    if matches!(operation, 5..=7 | 10 | 12..=48) && p.role != "admin" {
        db::audit(
            &app.pool,
            &p.tenant_id,
            &p.id.to_string(),
            &input.operation,
            &id,
            "FORBIDDEN",
        )
        .await
        .map_err(|_| ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL"))?;
        return Err(ApiError::forbidden());
    }
    if matches!(operation,38..=48) != input.imported_firewall.is_some()
          || input.imported_firewall.as_ref().is_some_and(|v|v.operation()!=operation || !v.valid())
          || (operation == 31) != input.firewall_change.is_some()
        || matches!(operation, 32 | 33) != input.firewall_action.is_some()
        || (operation == 35) != input.nm_change.is_some()
        || matches!(operation, 36 | 37) != input.nm_action.is_some()
        || input
            .firewall_change
            .as_ref()
            .is_some_and(|v| v.to_wire().is_none())
        || input.firewall_action.as_ref().is_some_and(|v| !v.valid())
        || input
            .nm_change
            .as_ref()
            .is_some_and(|v| v.to_wire().is_none())
        || input.nm_action.as_ref().is_some_and(|v| !v.valid())
        || (operation == 5) != input.network_change.is_some()
        || matches!(operation, 6..=8) != input.network_action.is_some()
        || matches!(operation, 9 | 10) != input.network_takeover.is_some()
        || (operation == 11) != input.kura_configuration.is_some()
        || (operation == 12) != input.kura_configuration_update.is_some()
        || (operation == 13) != input.kura_snapshot_save.is_some()
        || (operation == 14) != input.kura_snapshot_rollback.is_some()
        || matches!(operation, 17 | 18) != input.ordinary_boot_action.is_some()
        || matches!(operation, 22 | 29) != input.ordinary_handoff_action.is_some()
        || matches!(operation, 25 | 27 | 28) != input.system_control_action.is_some()
        || input
            .system_control_action
            .as_ref()
            .is_some_and(|v| !v.valid())
        || input
            .ordinary_handoff_action
            .as_ref()
            .is_some_and(|v| !v.valid())
        || input
            .ordinary_boot_action
            .as_ref()
            .is_some_and(|v| !v.valid())
        || input
            .kura_snapshot_save
            .as_ref()
            .is_some_and(|v| v.parameters(i64::MAX).is_none())
        || input
            .kura_snapshot_rollback
            .as_ref()
            .is_some_and(|v| v.parameters(i64::MAX).is_none())
        || input
            .kura_configuration_update
            .as_ref()
            .is_some_and(|v| v.parameters(i64::MAX).is_none())
        || input
            .kura_configuration
            .as_ref()
            .is_some_and(|target| !target.valid())
    {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let mut payload = json!({"importedFirewall":input.imported_firewall.as_ref().map(|request|json!({"request":request})),"firewallChange":input.firewall_change,"firewallAction":input.firewall_action,"nmChange":input.nm_change,"nmAction":input.nm_action,"networkChange":input.network_change,"networkAction":input.network_action,"networkTakeover":input.network_takeover,"kuraConfiguration":input.kura_configuration,"kuraConfigurationUpdate":input.kura_configuration_update,"kuraSnapshotSave":input.kura_snapshot_save,"kuraSnapshotRollback":input.kura_snapshot_rollback,"ordinaryBootAction":input.ordinary_boot_action,"ordinaryHandoffAction":input.ordinary_handoff_action,"systemControlAction":input.system_control_action});
    if serde_json::to_vec(&payload)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?
        .len()
        > if matches!(operation,38..=48) {yofc_management_protocol::imported::MAX_PAYLOAD+512} else {16384}
    {
        return Err(ApiError::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            "RESOURCE_EXHAUSTED",
        ));
    }
    if input.idempotency_key.is_empty() || input.idempotency_key.len() > 64 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 AND enabled)",
    )
    .bind(&id)
    .bind(&p.tenant_id)
    .fetch_one(&mut *tx)
    .await?;
    if !exists {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"));
    }
    if yofc_management_protocol::has_side_effect(operation) {
        crate::gateway_certificate::maintenance_guard(&mut tx,&p.tenant_id,&id).await?;
    }
    // Serialize admission only, not task execution, so limits hold under concurrent requests.
    sqlx::query("SELECT pg_advisory_xact_lock(794654,1)")
        .execute(&mut *tx)
        .await?;
    let previous=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,input FROM mgmt_tasks WHERE actor_id=$1 AND gateway_id=$2 AND idempotency_key=$3").bind(p.id).bind(&id).bind(&input.idempotency_key).fetch_optional(&mut *tx).await?;
    let replay = previous.is_some();
    let previous_payload=previous.as_ref().map(|row|cipher.open_row(row,"input")).transpose()?;
    crate::imported_tasks::materialize(&mut payload,previous_payload.as_ref()).map_err(|_|ApiError::new(StatusCode::CONFLICT,"CONFLICT"))?;
    if !replay {
        if operation == 12 || matches!(operation, 13..=48) {
            let capability = if matches!(operation,38..=48) {
                  if matches!(operation,40..=43|47) {yofc_management_protocol::imported::WRITE_CAPABILITY} else {yofc_management_protocol::imported::CAPABILITY}
              } else if matches!(operation, 30..=33) {
                "firewall.remote.v1"
            } else if matches!(operation, 34..=37) {
                "nm.remote.v1"
            } else if operation == 12 {
                "kura.configuration.update"
            } else if matches!(operation, 15..=19) {
                "ordinary.boot.v1"
            } else if operation == 29 {
                "recovery.ordinary.cancel.v1"
            } else if matches!(operation, 20..=22) {
                "recovery.ordinary.v1"
            } else if matches!(operation, 24 | 25) {
                "service.restart.v1"
            } else if matches!(operation, 26 | 27) {
                "system.reboot.v1"
            } else if matches!(operation, 23 | 28) {
                "system.control.v1"
            } else {
                "kura.snapshot.write"
            };
            let capable: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND capabilities ? $2 AND ($3<>29 OR capabilities ? 'recovery.ordinary.v1') AND last_seen>now()-interval '15 seconds')")
                .bind(&id).bind(capability).bind(operation).fetch_one(&mut *tx).await?;
            if !capable {
                return Err(ApiError::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "UNSUPPORTED",
                ));
            }
            if matches!(operation, 23..=28) {
                let online:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND online_reported AND connection_epoch IS NOT NULL AND capabilities ? 'system.control.v1' AND last_seen>now()-interval '15 seconds')").bind(&id).fetch_one(&mut *tx).await?;
                if !online {
                    return Err(ApiError::new(StatusCode::CONFLICT, "CONFLICT"));
                }
            }
        }
        if yofc_management_protocol::has_side_effect(operation)
            && !matches!(operation, 7 | 18 | 19 | 28 | 33 | 43 | 44)
        {
            let unresolved: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_tasks WHERE gateway_id=$1 AND state='UNKNOWN' AND operation NOT IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48))").bind(&id).fetch_one(&mut *tx).await?;
            if unresolved {
                return Err(ApiError::new(StatusCode::CONFLICT, "OUTCOME_UNKNOWN"));
            }
        }
        if let Some(original)=input.imported_firewall.as_ref().and_then(|request|request.original_task()) {
            let generation:i64=sqlx::query_scalar("SELECT binding_generation FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 AND enabled FOR SHARE")
                .bind(&id).bind(&p.tenant_id).fetch_one(&mut *tx).await?;
            let original_operation:Option<i32>=sqlx::query_scalar("SELECT operation FROM mgmt_tasks WHERE id=$1 AND actor_id=$2 AND gateway_id=$3 AND tenant_id=$4 AND binding_generation=$5")
                .bind(original).bind(p.id).bind(&id).bind(&p.tenant_id).bind(generation).fetch_optional(&mut *tx).await?;
            let allowed=match operation {45=>original_operation.is_some_and(|v|matches!(v,40..=44)),48=>original_operation==Some(47),_=>original_operation==Some(40)};
            if !allowed {return Err(ApiError::new(StatusCode::NOT_FOUND,"NOT_FOUND"));}
        }
        let counts=sqlx::query("SELECT count(*) AS total,count(*) FILTER(WHERE tenant_id=$1) AS tenant,count(*) FILTER(WHERE gateway_id=$2) AS gateway FROM mgmt_tasks WHERE state IN ('QUEUED','RUNNING','UNKNOWN')").bind(&p.tenant_id).bind(&id).fetch_one(&mut *tx).await?;
        if counts.get::<i64, _>("total") >= 10000
            || counts.get::<i64, _>("tenant") >= 1000
            || counts.get::<i64, _>("gateway") >= 100
        {
            return Err(ApiError::new(
                StatusCode::TOO_MANY_REQUESTS,
                "RESOURCE_EXHAUSTED",
            ));
        }
    }
    let r = if let Some(previous) = previous {
        previous
    } else {
        let gateway_row=sqlx::query("SELECT binding_generation,connection_epoch,online_reported,last_seen FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 AND enabled FOR SHARE").bind(&id).bind(&p.tenant_id).fetch_optional(&mut *tx).await.map_err(|_| ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL"))?.ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
        let binding_generation: i64 = gateway_row.get("binding_generation");
        let power_epoch = if matches!(operation, 24..=29 | 31..=37 | 40..=43 | 47) {
            let online: bool = gateway_row.get("online_reported");
            let seen: Option<DateTime<Utc>> = gateway_row.get("last_seen");
            let epoch: Option<Uuid> = gateway_row.get("connection_epoch");
            if !online
                || epoch.is_none()
                || seen.is_none_or(|at| Utc::now() - at > chrono::Duration::seconds(15))
            {
                return Err(ApiError::new(StatusCode::CONFLICT, "CONFLICT"));
            }
            epoch
        } else {
            None
        };
        let context = TaskContext {
            id: Uuid::new_v4(),
            tenant: p.tenant_id.clone(),
            gateway: id.clone(),
            operation,
        };
        let encrypted = cipher.seal(&context, "input", &payload)?;
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,binding_generation,expires_at,input,dispatch_epoch) VALUES($1,$2,$3,$4,$5,$6,$7,now()+CASE WHEN $5 IN (25,27) THEN interval '5 minutes' ELSE interval '30 seconds' END,$8,$9) RETURNING id,tenant_id,gateway_id,operation,state,input")
        .bind(context.id).bind(&id).bind(&p.tenant_id).bind(p.id).bind(operation).bind(input.idempotency_key).bind(binding_generation).bind(encrypted).bind(power_epoch).fetch_one(&mut *tx).await?
    };
    let previous_input = cipher.open_row(&r, "input")?;
    // Additive optional payloads do not invalidate pre-upgrade idempotency receipts.
    let without_nulls = |mut v: Value| {
        if let Some(m) = v.as_object_mut() {
            m.retain(|_, v| !v.is_null());
        }
        v
    };
    if r.get::<i32, _>("operation") != operation
        || without_nulls(previous_input) != without_nulls(payload)
    {
        return Err(ApiError::new(StatusCode::CONFLICT, "CONFLICT"));
    }
    let task_id = r.get::<Uuid, _>("id");
    if !replay {
        db::event(
            &mut *tx,
            &p.tenant_id,
            "task",
            &task_id.to_string(),
            0,
            json!({"state":"QUEUED"}),
        )
        .await
        .map_err(|_| ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL"))?;
    }
    sqlx::query("INSERT INTO mgmt_audit(tenant_id,actor,operation,target,outcome) VALUES($1,$2,$3,$4,'ACCEPTED')").bind(&p.tenant_id).bind(p.id.to_string()).bind(&input.operation).bind(task_id.to_string()).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok((
        StatusCode::ACCEPTED,
        Json(json!({"id":task_id,"state":r.get::<String,_>("state")})),
    ))
}
async fn tasks(
    State(app): State<App>,
    headers: HeaderMap,
    page: Result<Query<Page>, QueryRejection>,
) -> Result<Json<Value>, ApiError> {
    let Query(page) = page?;
    let p = auth::principal(&app.pool, &headers).await?;
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let rows=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,error,result,created_at,expires_at FROM mgmt_tasks WHERE tenant_id=$1 ORDER BY created_at DESC LIMIT 100 OFFSET $2").bind(p.tenant_id).bind(page.offset.unwrap_or(0).clamp(0,100000)).fetch_all(&mut *tx).await?;
    let items = rows
        .into_iter()
        .map(|r| task_json(r, &cipher, p.role == "admin"))
        .collect::<Result<Vec<_>, _>>()?;
    tx.commit().await?;
    Ok(Json(json!({"items":items})))
}
pub(crate) async fn find_task_intent(
    pool: &PgPool,
    tenant: &str,
    actor: Uuid,
    gateway: &str,
    key: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar("SELECT id FROM mgmt_tasks WHERE tenant_id=$1 AND actor_id=$2 AND gateway_id=$3 AND idempotency_key=$4")
        .bind(tenant).bind(actor).bind(gateway).bind(key).fetch_optional(pool).await
}
async fn task_intent(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<(String, String)>, PathRejection>,
) -> Result<Json<Value>, ApiError> {
    let Path((gateway, key)) = path?;
    let p = auth::principal(&app.pool, &headers).await?;
    if p.role != "admin" {
        return Err(ApiError::forbidden());
    }
    if key.is_empty() || key.len() > 64 || key.chars().any(char::is_control) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let id = find_task_intent(&app.pool, &p.tenant_id, p.id, &gateway, &key)
        .await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    Ok(Json(json!({"id":id})))
}
fn task_json(
    r: sqlx::postgres::PgRow,
    cipher: &TaskCipher,
    admin: bool,
) -> Result<Value, ApiError> {
    let mut result = r
        .get::<Option<Value>, _>("result")
        .map(|value| cipher.open(&TaskContext::from_row(&r)?, "result", &value))
        .transpose()?;
    if !admin
        && matches!(r.get::<i32, _>("operation"), 16 | 21 | 24 | 26 | 31)
        && let Some(object) = result.as_mut().and_then(Value::as_object_mut)
    {
        object.remove("confirmToken");
        object.remove("confirmationToken");
    }
    Ok(
        json!({"id":r.get::<Uuid,_>("id"),"gatewayId":r.get::<String,_>("gateway_id"),"operation":r.get::<i32,_>("operation"),"state":r.get::<String,_>("state"),"error":r.get::<Option<String>,_>("error"),"result":result,"createdAt":r.get::<DateTime<Utc>,_>("created_at")}),
    )
}
async fn task(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<Value>, ApiError> {
    let Path(id) = path?;
    let p = auth::principal(&app.pool, &headers).await?;
    let cipher = app.cipher.read().await;
    let mut tx = app.pool.begin().await?;
    cipher.lock_storage(&mut tx).await?;
    let row = sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,error,result,created_at FROM mgmt_tasks WHERE tenant_id=$1 AND id=$2")
        .bind(p.tenant_id).bind(id).fetch_optional(&mut *tx).await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND,"NOT_FOUND"))?;
    let value = task_json(row, &cipher, p.role == "admin")?;
    tx.commit().await?;
    Ok(Json(value))
}
#[derive(Deserialize, Default)]
struct Cursor {
    cursor: Option<i64>,
}
async fn events(
    State(app): State<App>,
    headers: HeaderMap,
    query: Result<Query<Cursor>, QueryRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Query(query) = query?;
    let p = auth::principal(&app.pool, &headers).await?;
    let permit = app
        .sse_slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError::new(StatusCode::TOO_MANY_REQUESTS, "RESOURCE_EXHAUSTED"))?;
    let mut cursor = headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<i64>().ok())
        .or(query.cursor)
        .unwrap_or(0)
        .max(0);
    let stream = async_stream::stream! {
        let _permit=permit;
        loop {
            match auth::principal(&app.pool,&headers).await {
                Ok(current) if current.can_query() && current.tenant_id==p.tenant_id => {},
                _ => break,
            }
            let bounds=sqlx::query("SELECT coalesce(min(id),0) AS lo,coalesce(max(id),0) AS hi FROM mgmt_events").fetch_one(&app.pool).await;
            let Ok(bounds)=bounds else{break};
            if cursor<bounds.get::<i64,_>("lo").saturating_sub(1) || cursor>bounds.get::<i64,_>("hi") {
                cursor=bounds.get("hi");
                yield Ok::<_,std::convert::Infallible>(Event::default().id(cursor.to_string()).event("resync").data("{}"));
            }
            let rows=sqlx::query("SELECT id,kind,target,revision,data,created_at FROM mgmt_events WHERE tenant_id=$1 AND id>$2 ORDER BY id LIMIT 100").bind(&p.tenant_id).bind(cursor).fetch_all(&app.pool).await;
            let Ok(rows)=rows else{break};
            for r in rows {
                cursor=r.get("id");
                let body=json!({"target":r.get::<String,_>("target"),"revision":r.get::<i64,_>("revision"),"data":r.get::<Value,_>("data"),"time":r.get::<DateTime<Utc>,_>("created_at")});
                yield Ok(Event::default().id(cursor.to_string()).event(r.get::<String,_>("kind")).data(body.to_string()));
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    };
    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(5))))
}

#[cfg(test)]
#[path = "system_control_http_tests.rs"]
mod system_control_http_tests;
#[cfg(test)]
#[path = "firewall_http_tests.rs"]
mod firewall_http_tests;
#[cfg(test)]
#[path = "nm_http_tests.rs"]
mod nm_http_tests;

#[cfg(test)]
mod password_setup_tests {
    use super::*;
    #[tokio::test]
    #[ignore = "requires explicit isolated PostgreSQL URL; uses its own disposable schema"]
    async fn postgres_initial_password_setup_is_authenticated_atomic_and_cannot_bypass_later_changes()
     {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("password_setup_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect_with(options)
            .await
            .unwrap();
        sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
        auth::ensure_bootstrap_admin(&pool).await.unwrap();
        let row = sqlx::query(
            "SELECT id,password_hash,must_change_password FROM mgmt_users WHERE username='admin'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        let user: Uuid = row.get("id");
        assert!(row.get::<bool, _>("must_change_password"));
        assert!(auth::password_valid(
            "admin",
            &row.get::<String, _>("password_hash")
        ));
        let directory = tempfile::tempdir().unwrap();
        std::fs::set_permissions(
            directory.path(),
            std::os::unix::fs::PermissionsExt::from_mode(0o700),
        )
        .unwrap();
        let key = directory.path().join("key");
        crate::task_crypto::create_key(&key).unwrap();
        let config = Config {
            http_addr: "127.0.0.1:0".parse().unwrap(),
            grpc_addr: "127.0.0.1:0".parse().unwrap(),
            origin: "https://fixture.invalid".into(),
            tls_cert: String::new(),
            tls_key: String::new(),
            client_ca: String::new(),
            web_dir: String::new(),
            state_dir: String::new(),
        };
        let app = App::new(pool.clone(), config, TaskCipher::load(&key).unwrap()).unwrap();
        let mut sessions = Vec::new();
        for _ in 0..2 {
            let token = auth::random_token();
            sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(user).execute(&pool).await.unwrap();
            let mut headers = HeaderMap::new();
            headers.insert("cookie", format!("kura_session={token}").parse().unwrap());
            headers.insert("origin", "https://fixture.invalid".parse().unwrap());
            headers.insert("x-csrf-token", "csrf".parse().unwrap());
            sessions.push(headers);
        }
        let request = |password: &str| {
            Ok(Json(ChangePassword {
                current_password: None,
                new_password: password.into(),
            }))
        };
        assert_eq!(
            change_password(
                State(app.clone()),
                HeaderMap::new(),
                request("initial-password-1")
            )
            .await
            .err()
            .unwrap()
            .0,
            StatusCode::UNAUTHORIZED
        );
        let mut no_csrf = sessions[0].clone();
        no_csrf.remove("x-csrf-token");
        assert_eq!(
            change_password(State(app.clone()), no_csrf, request("initial-password-1"))
                .await
                .err()
                .unwrap()
                .0,
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            change_password(State(app.clone()), sessions[0].clone(), request("admin"))
                .await
                .err()
                .unwrap()
                .0,
            StatusCode::BAD_REQUEST
        );
        let (first, second) = tokio::join!(
            change_password(
                State(app.clone()),
                sessions[0].clone(),
                request("initial-password-1")
            ),
            change_password(
                State(app.clone()),
                sessions[1].clone(),
                request("initial-password-2")
            )
        );
        assert_ne!(first.is_ok(), second.is_ok());
        let winner = if first.is_ok() { 0 } else { 1 };
        let principal = auth::principal(&pool, &sessions[winner])
            .await
            .ok()
            .unwrap();
        assert!(!principal.must_change_password);
        assert!(auth::principal(&pool, &sessions[1 - winner]).await.is_err());
        assert_eq!(
            change_password(
                State(app.clone()),
                sessions[winner].clone(),
                request("bypass-password-3")
            )
            .await
            .err()
            .unwrap()
            .0,
            StatusCode::UNAUTHORIZED
        );
        let hash: String = sqlx::query_scalar("SELECT password_hash FROM mgmt_users WHERE id=$1")
            .bind(user)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(auth::password_valid(
            &format!("initial-password-{}", winner + 1),
            &hash
        ));
        assert!(!auth::password_valid("admin", &hash));
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM mgmt_audit WHERE operation='user.password.change' AND outcome='SUCCEEDED'").fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}

#[cfg(test)]
mod gateway_registration_tests {
    use super::*;
    #[tokio::test]
    #[ignore = "requires explicit isolated DATABASE_URL"]
    async fn postgres_registration_http_authority_atomicity_and_no_replacement() {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("registration_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(6)
            .connect_with(options)
            .await
            .unwrap();
        sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
        let directory = tempfile::tempdir().unwrap();
        std::fs::set_permissions(
            directory.path(),
            std::os::unix::fs::PermissionsExt::from_mode(0o700),
        )
        .unwrap();
        let key = directory.path().join("key");
        crate::task_crypto::create_key(&key).unwrap();
        let config = Config {
            http_addr: "127.0.0.1:0".parse().unwrap(),
            grpc_addr: "127.0.0.1:0".parse().unwrap(),
            origin: "https://fixture.invalid".into(),
            tls_cert: String::new(),
            tls_key: String::new(),
            client_ca: String::new(),
            web_dir: String::new(),
            state_dir: String::new(),
        };
        let app = App::new(pool.clone(), config, TaskCipher::load(&key).unwrap()).unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            axum::serve(
                listener,
                router(app).into_make_service_with_connect_info::<std::net::SocketAddr>(),
            )
            .await
            .unwrap();
        });
        let client = reqwest::Client::new();
        let mut tokens = Vec::new();
        for (index, (role, tenant, initial)) in [
            ("admin", "tenant-a", false),
            ("operator", "tenant-a", false),
            ("viewer", "tenant-a", false),
            ("recovery", "tenant-a", false),
            ("admin", "tenant-b", false),
            ("admin", "tenant-a", true),
        ]
        .into_iter()
        .enumerate()
        {
            let id = Uuid::new_v4();
            let token = auth::random_token();
            sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role,must_change_password) VALUES($1,$2,$3,'unused',$4,$5)").bind(id).bind(tenant).bind(format!("user-{index}")).bind(role).bind(initial).execute(&pool).await.unwrap();
            sqlx::query("INSERT INTO mgmt_sessions(token_hash,user_id,csrf,expires_at) VALUES($1,$2,'csrf',now()+interval '1 hour')").bind(auth::hash(token.as_bytes())).bind(id).execute(&pool).await.unwrap();
            tokens.push((token, id));
        }
        let certificate = include_str!("../tests/fixtures/registration-client.pem");
        let body = json!({"id":"registered-gateway","name":"网关 Gateway","clientCertificatePem":certificate});
        let request = |index: usize| {
            client
                .post(format!("{base}/api/v1/gateways"))
                .header("cookie", format!("kura_session={}", tokens[index].0))
                .header("origin", "https://fixture.invalid")
                .header("x-csrf-token", "csrf")
        };
        assert_eq!(
            client
                .post(format!("{base}/api/v1/gateways"))
                .json(&body)
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::UNAUTHORIZED
        );
        for index in [1, 2, 3, 5] {
            assert_eq!(
                request(index).json(&body).send().await.unwrap().status(),
                StatusCode::FORBIDDEN
            );
        }
        assert_eq!(
            client
                .post(format!("{base}/api/v1/gateways"))
                .header("cookie", format!("kura_session={}", tokens[0].0))
                .header("origin", "https://fixture.invalid")
                .header("x-csrf-token", "wrong")
                .json(&body)
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            client
                .post(format!("{base}/api/v1/gateways"))
                .header("cookie", format!("kura_session={}", tokens[0].0))
                .header("x-csrf-token", "csrf")
                .header("origin", "https://other.invalid")
                .json(&body)
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::FORBIDDEN
        );
        for (field, value) in [
            ("tenantId", json!("tenant-b")),
            ("bindingGeneration", json!(9)),
            ("privateKeyPem", json!("secret")),
        ] {
            let mut extra = body.clone();
            extra[field] = value;
            assert_eq!(
                request(0).json(&extra).send().await.unwrap().status(),
                StatusCode::UNPROCESSABLE_ENTITY
            );
        }
        let mut invalid = body.clone();
        invalid["clientCertificatePem"] = json!(format!("{certificate}{certificate}"));
        assert_eq!(
            request(0).json(&invalid).send().await.unwrap().status(),
            StatusCode::BAD_REQUEST
        );
        // Concurrent requests must yield one durable row/audit/event, never an UPSERT.
        let (left, right) =
            tokio::join!(request(0).json(&body).send(), request(0).json(&body).send());
        let statuses = [left.unwrap().status(), right.unwrap().status()];
        assert!(statuses.contains(&StatusCode::CREATED));
        assert!(statuses.contains(&StatusCode::CONFLICT));
        let fingerprint = crate::gateway_registration::certificate_sha256(certificate).unwrap();
        let get = |index: usize| {
            client
                .get(format!("{base}/api/v1/gateways/registered-gateway"))
                .header("cookie", format!("kura_session={}", tokens[index].0))
        };
        let receipt: Value = get(0).send().await.unwrap().json().await.unwrap();
        assert_eq!(receipt["certificateSha256"], fingerprint);
        assert_eq!(receipt["bindingGeneration"], 1);
        assert_eq!(receipt["online"], false);
        assert!(receipt.get("clientCertificatePem").is_none());
        assert_eq!(get(4).send().await.unwrap().status(), StatusCode::NOT_FOUND);
        let mut duplicate = body.clone();
        duplicate["id"] = json!("another-gateway");
        assert_eq!(
            request(4).json(&duplicate).send().await.unwrap().status(),
            StatusCode::CONFLICT
        );
        let audit = sqlx::query(
            "SELECT actor,tenant_id FROM mgmt_audit WHERE operation='gateway.register'",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(audit.len(), 1);
        assert_eq!(audit[0].get::<String, _>("actor"), tokens[0].1.to_string());
        assert_eq!(audit[0].get::<String, _>("tenant_id"), "tenant-a");
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM mgmt_events WHERE kind='gateway'")
                .fetch_one(&pool)
                .await
                .unwrap(),
            1
        );
        // Registration cannot re-enable or change an existing advanced binding.
        sqlx::query("UPDATE mgmt_gateways SET enabled=false,binding_generation=7,revision=21 WHERE id='registered-gateway'").execute(&pool).await.unwrap();
        assert_eq!(
            request(0).json(&body).send().await.unwrap().status(),
            StatusCode::CONFLICT
        );
        let saved=sqlx::query("SELECT enabled,binding_generation,revision FROM mgmt_gateways WHERE id='registered-gateway'").fetch_one(&pool).await.unwrap();
        assert!(!saved.get::<bool, _>("enabled"));
        assert_eq!(saved.get::<i64, _>("binding_generation"), 7);
        assert_eq!(saved.get::<i64, _>("revision"), 21);
        // A failed durable audit rolls back registration and event together.
        sqlx::query("CREATE FUNCTION reject_registration_audit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'test audit failure'; END $$").execute(&pool).await.unwrap();
        sqlx::query("CREATE TRIGGER reject_registration_audit BEFORE INSERT ON mgmt_audit FOR EACH ROW EXECUTE FUNCTION reject_registration_audit()").execute(&pool).await.unwrap();
        let other = include_str!("../tests/fixtures/registration-other.pem");
        let fail =
            json!({"id":"audit-failure","name":"Audit failure","clientCertificatePem":other});
        assert_eq!(
            request(0).json(&fail).send().await.unwrap().status(),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM mgmt_gateways WHERE id='audit-failure'"
            )
            .fetch_one(&pool)
            .await
            .unwrap(),
            0
        );
        server.abort();
        pool.close().await;
        admin.close().await;
        // Isolated schema retained as evidence, not cleaned or reused.
    }
}

#[cfg(test)]
#[path = "key_lifecycle_tests.rs"]
mod key_lifecycle_tests;
