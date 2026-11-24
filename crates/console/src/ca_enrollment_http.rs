use super::{ApiError, App, auth, ca_policy_http::policy_writer, require_admin};
use crate::{ca_enrollment, tls_runtime::TlsRuntime};
use axum::{
    Json,
    extract::{
        Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::{HeaderMap, StatusCode},
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
    let actor = policy_writer(&app, &headers).await?;
    let Json(input) = input?;
    let batch = input
        .validate(&actor.tenant_id)
        .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    let runtime = runtime(&app)?;
    let _tls = runtime.changes.lock().await;
    let trust = current_trust(runtime)?;
    match ca_enrollment::propose(&app.pool, &actor, batch, &trust).await? {
        ca_enrollment::Submission::Recorded(status) => Ok(Json(status)),
        ca_enrollment::Submission::Conflict => {
            Err(ApiError::new(StatusCode::CONFLICT, "CA_BATCH_CONFLICT"))
        }
        ca_enrollment::Submission::InvalidCertificate => Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "CA_BATCH_CERTIFICATE_INVALID",
        )),
    }
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
