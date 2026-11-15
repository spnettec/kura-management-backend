//! Authenticated certificate requests; the private key never enters an HTTP DTO.
use super::*;
use crate::server_csr;

pub(super) async fn current(
    State(app): State<App>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    Ok(Json(
        json!({"current":server_csr::current(std::path::Path::new(&app.config.state_dir))?}),
    ))
}
pub(super) async fn read(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    Ok(Json(
        server_csr::observe(std::path::Path::new(&app.config.state_dir), id)?
            .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?,
    ))
}
pub(super) async fn read_import(
    State(app): State<App>,
    headers: HeaderMap,
    Path((id, operation)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    Ok(Json(
        server_csr::import_operation(std::path::Path::new(&app.config.state_dir), id, operation)?
            .ok_or(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))?,
    ))
}
pub(super) async fn create(
    State(app): State<App>,
    headers: HeaderMap,
    input: Result<Json<server_csr::Request>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    let Json(input) = input?;
    if input.origin != app.config.origin
        || input.request_id.is_nil()
        || server_csr::origin_host(&input.origin).is_err()
    {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let permit = app
        .gate
        .enter()
        .map_err(|_| ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "MAINTENANCE"))?;
    if let Some(runtime) = &app.tls {
        let _guard = runtime.changes.lock().await;
        runtime.prepare_stage()?;
    }
    db::audit(
        &app.pool,
        &principal.tenant_id,
        &principal.username,
        "settings.server-csr.create",
        &input.request_id.to_string(),
        "REQUESTED",
    )
    .await?;
    let root = std::path::PathBuf::from(&app.config.state_dir);
    let actor = format!("{}:{}", principal.tenant_id, principal.username);
    // A dropped HTTP observer cannot cancel an accepted cryptographic operation.
    let value = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        server_csr::create(&root, input, &actor)
    })
    .await
    .map_err(|_| ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "INTERNAL"))??;
    Ok(Json(value))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Certificate {
    pub(super) server_pem: String,
    pub(super) operation_id: Option<Uuid>,
}
pub(super) async fn import(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    input: Result<Json<Certificate>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    let Json(input) = input?;
    if input.operation_id.is_some_and(|id| id.is_nil()) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let _guard = match &app.tls {
        Some(runtime) => Some(runtime.changes.lock().await),
        None => None,
    };
    if let Some(runtime) = &app.tls {
        runtime.prepare_stage()?;
    }
    let audit_target = input.operation_id.map(|operation| format!("{id}/imports/{operation}"))
        .unwrap_or_else(|| id.to_string());
    db::audit(
        &app.pool,
        &principal.tenant_id,
        &principal.username,
        "settings.server-csr.import",
        &audit_target,
        "REQUESTED",
    )
    .await?;
    let state = std::path::Path::new(&app.config.state_dir);
    let value = match input.operation_id {
        Some(operation) => server_csr::import_with_receipt(
            state,
            id,
            operation,
            &input.server_pem,
            &format!("{}:{}", principal.tenant_id, principal.username),
        ),
        None => server_csr::import_certificate(state, id, &input.server_pem),
    }
    .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    Ok(Json(value))
}
pub(super) async fn retire(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    db::audit(
        &app.pool,
        &principal.tenant_id,
        &principal.username,
        "settings.server-csr.retire",
        &id.to_string(),
        "REQUESTED",
    )
    .await?;
    Ok(Json(server_csr::retire(
        std::path::Path::new(&app.config.state_dir),
        id,
    )?))
}
pub(super) async fn stage(
    State(app): State<App>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let principal = auth::principal(&app.pool, &headers).await?;
    require_admin(&principal)?;
    auth::csrf(&headers, &principal, &app.config.origin)?;
    let _guard = match &app.tls {
        Some(runtime) => Some(runtime.changes.lock().await),
        None => None,
    };
    let config = match &app.tls {
        Some(runtime) => runtime.prepare_stage()?,
        None => app.config.clone(),
    };
    db::audit(
        &app.pool,
        &principal.tenant_id,
        &principal.username,
        "settings.server-csr.stage",
        &id.to_string(),
        "REQUESTED",
    )
    .await?;
    Ok(Json(server_csr::stage(&config, id, &principal.username)?))
}
