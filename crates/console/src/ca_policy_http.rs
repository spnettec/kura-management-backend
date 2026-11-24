use super::{ApiError, App, auth, require_admin};
use crate::ca_policy;
use axum::{
    Json,
    extract::{
        Path, State,
        rejection::{JsonRejection, PathRejection},
    },
    http::{HeaderMap, StatusCode},
};
use uuid::Uuid;

pub(super) async fn overview(
    State(app): State<App>,
    headers: HeaderMap,
) -> Result<Json<ca_policy::Overview>, ApiError> {
    let actor = auth::principal(&app.pool, &headers).await?;
    require_admin(&actor)?;
    Ok(Json(ca_policy::overview(&app.pool, &actor).await?))
}

pub(super) async fn status(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<ca_policy::Status>, ApiError> {
    let actor = auth::principal(&app.pool, &headers).await?;
    require_admin(&actor)?;
    let Path(id) = path?;
    let status = ca_policy::status(&app.pool, &actor, id)
        .await?
        .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?;
    Ok(Json(status))
}

pub(super) async fn propose(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<ca_policy::Change>, JsonRejection>,
) -> Result<Json<ca_policy::Status>, ApiError> {
    let actor = policy_writer(&app, &headers).await?;
    let Json(mut change) = input?;
    if !change.normalize() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_INPUT"));
    }
    let status = ca_policy::propose(&app.pool, &actor, change)
        .await?
        .ok_or(ApiError::new(StatusCode::CONFLICT, "CA_POLICY_CONFLICT"))?;
    Ok(Json(status))
}

pub(super) async fn approve(
    State(app): State<App>,
    headers: HeaderMap,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<ca_policy::Status>, ApiError> {
    let actor = policy_writer(&app, &headers).await?;
    let Path(id) = path?;
    let status = ca_policy::approve(&app.pool, &actor, id)
        .await?
        .ok_or(ApiError::new(StatusCode::CONFLICT, "CA_POLICY_CONFLICT"))?;
    Ok(Json(status))
}

async fn policy_writer(app: &App, headers: &HeaderMap) -> Result<auth::Principal, ApiError> {
    let actor = auth::principal(&app.pool, headers).await?;
    require_admin(&actor)?;
    auth::csrf(headers, &actor, &app.config.origin)?;
    if actor.must_change_password {
        return Err(ApiError::forbidden());
    }
    Ok(actor)
}
