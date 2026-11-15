//! The paired setup session has the same CSR workflow, without a database or CA key.
use super::*;
use crate::server_csr;

pub(super) async fn current(
    State(setup): State<Arc<Setup>>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    setup.session(&headers, false)?;
    Ok(Json(
        json!({"current":server_csr::current(&setup.root).map_err(private_error)?}),
    ))
}
pub(super) async fn read(
    State(setup): State<Arc<Setup>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    setup.session(&headers, false)?;
    Ok(Json(
        server_csr::observe(&setup.root, id)
            .map_err(private_error)?
            .ok_or(error(StatusCode::NOT_FOUND, "NOT_FOUND"))?,
    ))
}
pub(super) async fn read_import(
    State(setup): State<Arc<Setup>>,
    headers: HeaderMap,
    Path((id, operation)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    setup.session(&headers, false)?;
    Ok(Json(
        server_csr::import_operation(&setup.root, id, operation)
            .map_err(private_error)?
            .ok_or(error(StatusCode::NOT_FOUND, "NOT_FOUND"))?,
    ))
}
pub(super) async fn create(
    State(setup): State<Arc<Setup>>,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    input: Result<Json<server_csr::Request>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    setup.origin(&headers, &uri)?;
    setup.session(&headers, true)?;
    let Json(input) = input?;
    if input.request_id.is_nil() || server_csr::origin_host(&input.origin).is_err() {
        return Err(error(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let guard = setup.changes.clone().lock_owned().await;
    if setup.blocking().map_err(private_error)?.is_some() {
        return Err(error(StatusCode::CONFLICT, "CONFLICT"));
    }
    let permit = setup
        .gate
        .enter()
        .map_err(|_| error(StatusCode::SERVICE_UNAVAILABLE, "MAINTENANCE"))?;
    let root = setup.root.clone();
    let actor = format!("installation:{}", setup.credential.installation_id);
    let value = tokio::task::spawn_blocking(move || {
        let _guard = guard;
        let _permit = permit;
        server_csr::create(&root, input, &actor)
    })
    .await
    .map_err(private_error)?
    .map_err(private_error)?;
    Ok(Json(value))
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Certificate {
    server_pem: String,
    operation_id: Option<Uuid>,
}
pub(super) async fn import(
    State(setup): State<Arc<Setup>>,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    input: Result<Json<Certificate>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    setup.origin(&headers, &uri)?;
    setup.session(&headers, true)?;
    let Json(input) = input?;
    if input.operation_id.is_some_and(|id| id.is_nil()) {
        return Err(error(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let _guard = setup.changes.lock().await;
    if setup.blocking().map_err(private_error)?.is_some() {
        return Err(error(StatusCode::CONFLICT, "CONFLICT"));
    }
    let value = match input.operation_id {
        Some(operation) => server_csr::import_with_receipt(
            &setup.root,
            id,
            operation,
            &input.server_pem,
            &format!("installation:{}", setup.credential.installation_id),
        ),
        None => server_csr::import_certificate(&setup.root, id, &input.server_pem),
    }
    .map_err(|_| error(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))?;
    Ok(Json(value))
}
pub(super) async fn retire(
    State(setup): State<Arc<Setup>>,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    setup.origin(&headers, &uri)?;
    setup.session(&headers, true)?;
    let _guard = setup.changes.lock().await;
    if setup.blocking().map_err(private_error)?.is_some() {
        return Err(error(StatusCode::CONFLICT, "CONFLICT"));
    }
    Ok(Json(
        server_csr::retire(&setup.root, id).map_err(private_error)?,
    ))
}
