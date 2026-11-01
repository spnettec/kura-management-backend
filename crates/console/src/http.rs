use crate::{
    auth,
    config::Config,
    db,
    task_crypto::{TaskCipher, TaskContext},
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
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, Semaphore};
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct App {
    pub pool: PgPool,
    pub cipher: TaskCipher,
    pub config: Config,
    pub login_slots: Arc<Semaphore>,
    pub attempts: Arc<Mutex<HashMap<String, VecDeque<Instant>>>>,
    pub sse_slots: Arc<Semaphore>,
}
impl App {
    pub fn new(pool: PgPool, config: Config, cipher: TaskCipher) -> Self {
        Self {
            pool,
            cipher,
            config,
            login_slots: Arc::new(Semaphore::new(4)),
            attempts: Default::default(),
            sse_slots: Arc::new(Semaphore::new(128)),
        }
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
    fn from(_: anyhow::Error) -> Self {
        // Keep payloads and crypto internals out of response bodies and logs.
        tracing::error!("task storage operation failed");
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
        .route("/health/ready",get(ready))
        .route("/api/v1/auth/login",post(login))
        .route("/api/v1/auth/logout",post(logout))
        .route("/api/v1/session",get(session))
        .route("/api/v1/gateways",get(gateways))
        .route("/api/v1/gateways/{id}",get(gateway))
        .route("/api/v1/gateways/{id}/tasks",post(create_task))
        .route("/api/v1/tasks",get(tasks))
        .route("/api/v1/tasks/{id}",get(task))
        .route("/api/v1/events",get(events))
        .route_service("/",index.clone()).route_service("/login",index.clone())
        .route_service("/gateways",index.clone()).route_service("/commands",index)
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
    sqlx::query("SELECT 1").execute(&app.pool).await?;
    Ok(Json(json!({"status":"ready","protocolMajor":1})))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Login {
    username: String,
    password: String,
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
        "SELECT id,tenant_id,password_hash FROM mgmt_users WHERE username=$1 AND enabled",
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
        Json(json!({"csrf":csrf})),
    )
        .into_response())
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
fn gateway_json(r: sqlx::postgres::PgRow) -> Value {
    json!({"id":r.get::<String,_>("id"),"name":r.get::<String,_>("name"),"online":r.get::<bool,_>("online"),"enabled":r.get::<bool,_>("enabled"),"kuraAvailable":r.get::<bool,_>("kura_available"),"lastSeen":r.get::<Option<DateTime<Utc>>,_>("last_seen"),"agentVersion":r.get::<Option<String>,_>("agent_version"),"capabilities":r.get::<Value,_>("capabilities"),"revision":r.get::<i64,_>("revision"),"uptimeSeconds":r.get::<i64,_>("uptime_seconds")})
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
    operation: String,
    idempotency_key: String,
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
        _ => {
            return Err(ApiError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "UNSUPPORTED",
            ));
        }
    };
    if matches!(operation, 5..=7 | 10 | 12..=14) && p.role != "admin" {
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
    if (operation == 5) != input.network_change.is_some()
        || matches!(operation, 6..=8) != input.network_action.is_some()
        || matches!(operation, 9 | 10) != input.network_takeover.is_some()
        || (operation == 11) != input.kura_configuration.is_some()
        || (operation == 12) != input.kura_configuration_update.is_some()
        || (operation == 13) != input.kura_snapshot_save.is_some()
        || (operation == 14) != input.kura_snapshot_rollback.is_some()
        || input.kura_snapshot_save.as_ref().is_some_and(|v| v.parameters(i64::MAX).is_none())
        || input.kura_snapshot_rollback.as_ref().is_some_and(|v| v.parameters(i64::MAX).is_none())
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
    let payload = json!({"networkChange":input.network_change,"networkAction":input.network_action,"networkTakeover":input.network_takeover,"kuraConfiguration":input.kura_configuration,"kuraConfigurationUpdate":input.kura_configuration_update,"kuraSnapshotSave":input.kura_snapshot_save,"kuraSnapshotRollback":input.kura_snapshot_rollback});
    if serde_json::to_vec(&payload)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?
        .len()
        > 16384
    {
        return Err(ApiError::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            "RESOURCE_EXHAUSTED",
        ));
    }
    if input.idempotency_key.is_empty() || input.idempotency_key.len() > 64 {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let mut tx = app.pool.begin().await?;
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
    // Serialize admission only, not task execution, so limits hold under concurrent requests.
    sqlx::query("SELECT pg_advisory_xact_lock(794654,1)")
        .execute(&mut *tx)
        .await?;
    let previous=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,input FROM mgmt_tasks WHERE actor_id=$1 AND gateway_id=$2 AND idempotency_key=$3").bind(p.id).bind(&id).bind(&input.idempotency_key).fetch_optional(&mut *tx).await?;
    let replay = previous.is_some();
    if !replay {
        if operation == 12 || matches!(operation, 13 | 14) {
            let capability = if operation == 12 { "kura.configuration.update" } else { "kura.snapshot.write" };
            let capable: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND capabilities ? $2 AND last_seen>now()-interval '15 seconds')")
                .bind(&id).bind(capability).fetch_one(&mut *tx).await?;
            if !capable {
                return Err(ApiError::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "UNSUPPORTED",
                ));
            }
        }
        if yofc_management_protocol::has_side_effect(operation) && operation != 7 {
            let unresolved: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_tasks WHERE gateway_id=$1 AND state='UNKNOWN' AND operation NOT IN (1,2,3,4,8,9,11))").bind(&id).fetch_one(&mut *tx).await?;
            if unresolved {
                return Err(ApiError::new(StatusCode::CONFLICT, "OUTCOME_UNKNOWN"));
            }
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
        let context = TaskContext {
            id: Uuid::new_v4(),
            tenant: p.tenant_id.clone(),
            gateway: id.clone(),
            operation,
        };
        let encrypted = app.cipher.seal(&context, "input", &payload)?;
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,expires_at,input) VALUES($1,$2,$3,$4,$5,$6,now()+interval '30 seconds',$7) RETURNING id,tenant_id,gateway_id,operation,state,input")
        .bind(context.id).bind(&id).bind(&p.tenant_id).bind(p.id).bind(operation).bind(input.idempotency_key).bind(encrypted).fetch_one(&mut *tx).await?
    };
    let previous_input = app.cipher.open_row(&r, "input")?;
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
    let rows=sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,error,result,created_at,expires_at FROM mgmt_tasks WHERE tenant_id=$1 ORDER BY created_at DESC LIMIT 100 OFFSET $2").bind(p.tenant_id).bind(page.offset.unwrap_or(0).clamp(0,100000)).fetch_all(&app.pool).await?;
    let items = rows
        .into_iter()
        .map(|r| task_json(r, &app.cipher))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(json!({"items":items})))
}
fn task_json(r: sqlx::postgres::PgRow, cipher: &TaskCipher) -> Result<Value, ApiError> {
    let result = r
        .get::<Option<Value>, _>("result")
        .map(|value| cipher.open(&TaskContext::from_row(&r)?, "result", &value))
        .transpose()?;
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
    let row = sqlx::query("SELECT id,tenant_id,gateway_id,operation,state,error,result,created_at FROM mgmt_tasks WHERE tenant_id=$1 AND id=$2")
        .bind(p.tenant_id).bind(id).fetch_optional(&app.pool).await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND,"NOT_FOUND"))?;
    Ok(Json(task_json(row, &app.cipher)?))
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
