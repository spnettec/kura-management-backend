use super::{ApiError, App, auth, ca_policy_http::policy_writer, require_admin};
use crate::{ca_enrollment, ca_signing, tls_runtime::TlsRuntime};
use axum::{
    Json,
    extract::{
        Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

fn runtime(app: &App) -> Result<&TlsRuntime, ApiError> {
    app.tls.as_deref().ok_or(ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "UNSUPPORTED",
    ))
}

fn current_trust(runtime: &TlsRuntime) -> Result<ca_enrollment::Trust, ApiError> {
    if !runtime.healthy() {
        return Err(ApiError::new(StatusCode::CONFLICT, "RECOVERY_REQUIRED"));
    }
    Ok(runtime.client_ca_authority())
}

pub(super) async fn status(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<ca_enrollment::Status>, ApiError> {
    let actor = auth::principal(&app.pool, &headers).await?;
    require_admin(&actor)?;
    let Path(id) = path?;
    let runtime = runtime(&app)?;
    let _tls = runtime.changes.lock().await;
    let trust = runtime.client_ca_authority();
    let result = ca_enrollment::status(&app.pool, &actor, id, &trust)
        .await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    Ok(Json(result))
}

pub(super) async fn propose(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<ca_enrollment::Request>, JsonRejection>,
) -> Result<Json<ca_enrollment::Status>, ApiError> {
    propose_with_signing(app, headers, input, None).await
}

pub(super) async fn signed(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
    input: Result<Json<ca_enrollment::Request>, JsonRejection>,
) -> Result<Json<ca_enrollment::Status>, ApiError> {
    let Path(id) = path?;
    propose_with_signing(app, headers, input, Some(id)).await
}

async fn propose_with_signing(
    app: App,
    headers: HeaderMap,
    input: Result<Json<ca_enrollment::Request>, JsonRejection>,
    id: Option<Uuid>,
) -> Result<Json<ca_enrollment::Status>, ApiError> {
    let actor = policy_writer(&app, &headers).await?;
    let Json(input) = input?;
    if id.is_some_and(|id| input.operation_id() != id) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let batch = input
        .validate(&actor.tenant_id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let runtime = runtime(&app)?;
    let _tls = runtime.changes.lock().await;
    let trust = current_trust(runtime)?;
    match ca_enrollment::propose(&app.pool, &actor, batch, &trust, id.is_some()).await? {
        ca_enrollment::Submission::Recorded(status) => Ok(Json(status)),
        ca_enrollment::Submission::Conflict => {
            Err(ApiError::new(StatusCode::CONFLICT, "CA_BATCH_CONFLICT"))
        }
        ca_enrollment::Submission::InvalidCertificate => Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "CA_BATCH_CERTIFICATE_INVALID",
        )),
        ca_enrollment::Submission::SigningMismatch => {
            Err(ApiError::new(StatusCode::CONFLICT, "CA_SIGNING_MISMATCH"))
        }
    }
}

pub(super) async fn signing_create(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<ca_signing::Request>, JsonRejection>,
) -> Result<Json<ca_signing::Status>, ApiError> {
    let actor = policy_writer(&app, &headers).await?;
    let Json(input) = input?;
    let batch = input
        .validate(&actor.tenant_id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "CA_SIGNING_CSR_INVALID"))?;
    let status = ca_signing::create(&app.pool, &actor, batch)
        .await?
        .ok_or(ApiError::new(StatusCode::CONFLICT, "CA_BATCH_CONFLICT"))?;
    Ok(Json(status))
}

async fn signing_original(
    app: &App,
    headers: &HeaderMap,
    id: Uuid,
) -> Result<ca_signing::Batch, ApiError> {
    let actor = auth::principal(&app.pool, headers).await?;
    require_admin(&actor)?;
    let mut conn = app.pool.acquire().await?;
    match ca_signing::lookup(&mut conn, &actor.tenant_id, id).await? {
        ca_signing::Lookup::Original(batch, _) => Ok(batch),
        _ => Err(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND")),
    }
}

pub(super) async fn signing_status(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<ca_signing::Status>, ApiError> {
    let Path(id) = path?;
    Ok(Json(signing_original(&app, &headers, id).await?.status()?))
}

pub(super) async fn signing_download(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Response, ApiError> {
    let Path(id) = path?;
    let batch = signing_original(&app, &headers, id).await?;
    let headers = [
        (header::CONTENT_TYPE, "application/x-tar".to_owned()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"gateway-csrs-{id}.tar\""),
        ),
    ];
    Ok((headers, batch.archive()?).into_response())
}

pub(super) async fn approve(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<ca_enrollment::Status>, ApiError> {
    let actor = policy_writer(&app, &headers).await?;
    let Path(id) = path?;
    let runtime = runtime(&app)?;
    let _tls = runtime.changes.lock().await;
    let trust = current_trust(runtime)?;
    let result = ca_enrollment::approve(&app.pool, &actor, id, &trust)
        .await?
        .ok_or(ApiError::new(StatusCode::CONFLICT, "CA_BATCH_CONFLICT"))?;
    Ok(Json(result))
}
