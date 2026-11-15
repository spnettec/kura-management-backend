//! Original public certificate-rotation receipts. Current gateway state never settles an old write.
use crate::http::ApiError;
use axum::http::StatusCode;
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct Request {
    pub operation: Uuid,
    pub gateway: String,
    pub tenant: String,
    pub previous: String,
    pub previous_generation: Option<i64>,
    pub certificate: Option<String>,
    pub pem_hash: String,
    pub bundle: bool,
}

pub(crate) fn operation(value: &str) -> Result<Uuid, ApiError> {
    Uuid::parse_str(value)
        .ok()
        .filter(|id| id.to_string() == value)
        .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"))
}

// Every certificate writer first locks the gateway, then reads history in a fresh statement.
// The separate statement matters when another reservation was committing while we waited.
async fn lock_gateway(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    gateway: &str,
) -> Result<sqlx::postgres::PgRow, ApiError> {
    sqlx::query("SELECT certificate_sha256,binding_generation,certificate_rotation_operation_id,certificate_rotation_pending_id,revision FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2 FOR UPDATE")
        .bind(gateway).bind(tenant).fetch_optional(&mut **tx).await?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"))
}

async fn history_intact(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    gateway: &str,
    row: &sqlx::postgres::PgRow,
) -> Result<bool, ApiError> {
    // Check both sides: a misassigned registry must still fence the receipt's original gateway.
    let pending: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateway_certificate_rotation_ids i LEFT JOIN mgmt_gateway_certificate_rotations r USING(operation_id) WHERE (i.gateway_id=$1 OR r.gateway_id=$1) AND (r.operation_id IS NULL OR r.gateway_id<>i.gateway_id OR r.tenant_id<>i.tenant_id OR r.state='PENDING'))")
        .bind(gateway).fetch_one(&mut **tx).await?;
    if pending
        || row
            .get::<Option<Uuid>, _>("certificate_rotation_pending_id")
            .is_some()
    {
        return Ok(false);
    }
    let Some(id) = row.get::<Option<Uuid>, _>("certificate_rotation_operation_id") else {
        return Ok(true); // Existing deployments are not retroactively given a receipt.
    };
    let valid: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateway_certificate_rotations r JOIN mgmt_gateway_certificate_rotation_ids i USING(operation_id) WHERE r.operation_id=$1 AND r.gateway_id=$2 AND r.tenant_id=$3 AND i.gateway_id=r.gateway_id AND i.tenant_id=r.tenant_id AND r.state='APPLIED' AND r.certificate_sha256=$4 AND r.accepted_revision<=$5 AND r.accepted_at IS NOT NULL)")
        .bind(id).bind(gateway).bind(tenant).bind(row.get::<String,_>("certificate_sha256"))
        .bind(row.get::<i64,_>("revision")).fetch_one(&mut **tx).await?;
    Ok(valid)
}

pub(crate) async fn maintenance_guard(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    gateway: &str,
) -> Result<(), ApiError> {
    let row = lock_gateway(tx, tenant, gateway).await?;
    if !history_intact(tx, tenant, gateway, &row).await? {
        return Err(ApiError::new(StatusCode::CONFLICT, "RECOVERY_REQUIRED"));
    }
    Ok(())
}

pub(crate) async fn pending_writes(
    tx: &mut Transaction<'_, Postgres>,
    gateway: &str,
) -> Result<bool, ApiError> {
    // Same read classification as task_lifecycle; queued or ambiguous writes must not lose their channel.
    Ok(sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_tasks WHERE gateway_id=$1 AND state IN ('QUEUED','RUNNING','UNKNOWN') AND operation NOT IN (1,2,3,4,8,9,11,15,20,23,30,34,38,39,45,46,48))")
        .bind(gateway).fetch_one(&mut **tx).await?)
}

/// Commit an intent before the credential transaction. An uncompleted intent fences every writer.
pub(crate) async fn reserve(
    pool: &PgPool,
    input: &Request,
    actor: &str,
    invalid: Option<&'static str>,
) -> Result<bool, ApiError> {
    let mut tx = pool.begin().await?;
    // Serialize only short reservations, so two gateways cannot both reserve one new credential.
    sqlx::query("SELECT pg_advisory_xact_lock(794654,2)")
        .execute(&mut *tx)
        .await?;
    let current = lock_gateway(&mut tx, &input.tenant, &input.gateway).await?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM mgmt_gateway_certificate_rotation_ids WHERE operation_id=$1 UNION ALL SELECT 1 FROM mgmt_gateway_certificate_rotations WHERE operation_id=$1 UNION ALL SELECT 1 FROM mgmt_gateways WHERE certificate_rotation_operation_id=$1 OR certificate_rotation_pending_id=$1)",
    )
    .bind(input.operation)
    .fetch_one(&mut *tx)
    .await?;
    if exists {
        return Err(ApiError::new(StatusCode::CONFLICT, "CONFLICT"));
    }
    let intact = history_intact(&mut tx, &input.tenant, &input.gateway, &current).await?;
    // A generation pin explicitly selects the new renewal contract. Existing
    // setup tools rotate certificates without changing their local generation.
    let previous_generation = input.previous_generation;
    if previous_generation.is_some_and(|value| value < 1) {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "INVALID_ARGUMENT"));
    }
    let generation = previous_generation.and_then(|value| value.checked_add(1));
    let receipt_version: i16 = if previous_generation.is_some() { 2 } else { 1 };
    let duplicate:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE certificate_sha256=$1 AND id<>$2 UNION ALL SELECT 1 FROM mgmt_gateway_certificate_rotations WHERE certificate_sha256=$1 AND gateway_id<>$2 AND state='PENDING')")
        .bind(&input.certificate).bind(&input.gateway).fetch_one(&mut *tx).await?;
    let reason = if !intact {
        Some("RECOVERY_REQUIRED")
    } else if current.get::<String, _>("certificate_sha256") != input.previous
        || previous_generation.is_some_and(|value| {
            value != current.get::<i64, _>("binding_generation") || generation.is_none()
        })
        || duplicate
        || input.certificate.as_deref() == Some(input.previous.as_str())
        || pending_writes(&mut tx, &input.gateway).await?
    {
        Some("CONFLICT")
    } else {
        invalid
    };
    sqlx::query("INSERT INTO mgmt_gateway_certificate_rotation_ids(operation_id,gateway_id,tenant_id) VALUES($1,$2,$3)")
        .bind(input.operation).bind(&input.gateway).bind(&input.tenant).execute(&mut *tx).await?;
    sqlx::query("INSERT INTO mgmt_gateway_certificate_rotations(operation_id,gateway_id,tenant_id,previous_certificate_sha256,certificate_sha256,certificate_pem_sha256,bundle_requested,state,rejection_code,actor,receipt_version,previous_binding_generation,binding_generation) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)")
        .bind(input.operation).bind(&input.gateway).bind(&input.tenant).bind(&input.previous)
        .bind(&input.certificate).bind(&input.pem_hash).bind(input.bundle)
        .bind(if reason.is_some() {"REJECTED"} else {"PENDING"}).bind(reason).bind(actor)
          .bind(receipt_version).bind(previous_generation).bind(generation)
        .execute(&mut *tx).await?;
    if reason.is_some() {
        crate::db::audit(
            &mut *tx,
            &input.tenant,
            actor,
            "gateway.certificate.rotate",
            &input.gateway,
            "REJECTED",
        )
        .await?;
    }
    if reason.is_none() {
        sqlx::query("UPDATE mgmt_gateways SET certificate_rotation_pending_id=$2 WHERE id=$1 AND tenant_id=$3")
            .bind(&input.gateway).bind(input.operation).bind(&input.tenant).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(reason.is_none())
}

pub(crate) async fn begin_application(
    tx: &mut Transaction<'_, Postgres>,
    input: &Request,
) -> Result<bool, ApiError> {
    let current = lock_gateway(tx, &input.tenant, &input.gateway).await?;
    let original: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_gateway_certificate_rotations WHERE operation_id=$1 AND gateway_id=$2 AND tenant_id=$3 AND state='PENDING' AND previous_certificate_sha256=$4 AND certificate_sha256=$5 AND certificate_pem_sha256=$6 AND bundle_requested=$7 AND receipt_version=CASE WHEN $8::bigint IS NULL THEN 1 ELSE 2 END AND previous_binding_generation IS NOT DISTINCT FROM $8)")
        .bind(input.operation).bind(&input.gateway).bind(&input.tenant).bind(&input.previous)
          .bind(&input.certificate).bind(&input.pem_hash).bind(input.bundle).bind(input.previous_generation).fetch_one(&mut **tx).await?;
    if !original
        || current.get::<Option<Uuid>, _>("certificate_rotation_pending_id")
            != Some(input.operation)
    {
        return Err(ApiError::new(StatusCode::CONFLICT, "RECOVERY_REQUIRED"));
    }
    let generation_matches: bool = sqlx::query_scalar("SELECT receipt_version=1 OR previous_binding_generation=$2 FROM mgmt_gateway_certificate_rotations WHERE operation_id=$1")
        .bind(input.operation).bind(current.get::<i64, _>("binding_generation")).fetch_one(&mut **tx).await?;
    if current.get::<String, _>("certificate_sha256") != input.previous
        || !generation_matches
        || pending_writes(tx, &input.gateway).await?
    {
        sqlx::query("UPDATE mgmt_gateway_certificate_rotations SET state='REJECTED',rejection_code='CONFLICT' WHERE operation_id=$1 AND state='PENDING'")
            .bind(input.operation).execute(&mut **tx).await?;
        sqlx::query("UPDATE mgmt_gateways SET certificate_rotation_pending_id=NULL WHERE id=$1 AND certificate_rotation_pending_id=$2")
            .bind(&input.gateway).bind(input.operation).execute(&mut **tx).await?;
        return Ok(false);
    }
    Ok(true)
}

pub(crate) async fn applied(
    tx: &mut Transaction<'_, Postgres>,
    input: &Request,
    revision: i64,
) -> Result<(), ApiError> {
    let result = sqlx::query("UPDATE mgmt_gateway_certificate_rotations SET state='APPLIED',accepted_revision=$2,accepted_at=now() WHERE operation_id=$1 AND state='PENDING'")
        .bind(input.operation).bind(revision).execute(&mut **tx).await?;
    if result.rows_affected() != 1 {
        return Err(ApiError::new(StatusCode::CONFLICT, "RECOVERY_REQUIRED"));
    }
    let changed = sqlx::query("UPDATE mgmt_gateways g SET certificate_rotation_operation_id=$2,certificate_rotation_pending_id=NULL,binding_generation=CASE WHEN r.receipt_version=2 THEN r.binding_generation ELSE g.binding_generation END FROM mgmt_gateway_certificate_rotations r WHERE g.id=$1 AND g.tenant_id=$3 AND g.certificate_rotation_pending_id=$2 AND r.operation_id=$2 AND r.gateway_id=g.id AND r.tenant_id=g.tenant_id AND r.state='APPLIED' AND (r.receipt_version=1 OR g.binding_generation=r.previous_binding_generation) AND g.certificate_sha256=r.certificate_sha256")
          .bind(&input.gateway).bind(input.operation).bind(&input.tenant).execute(&mut **tx).await?;
    if changed.rows_affected() != 1 {
        return Err(ApiError::new(StatusCode::CONFLICT, "RECOVERY_REQUIRED"));
    }
    Ok(())
}

pub(crate) async fn read(
    pool: &PgPool,
    tenant: &str,
    gateway: &str,
    operation: Uuid,
) -> Result<Value, ApiError> {
    let owned: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM mgmt_gateways WHERE id=$1 AND tenant_id=$2)",
    )
    .bind(gateway)
    .bind(tenant)
    .fetch_one(pool)
    .await?;
    if !owned {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "NOT_FOUND"));
    }
    let unknown = || json!({"id":gateway,"operationId":operation,"state":"UNKNOWN"});
    let Some(row) = sqlx::query("SELECT r.previous_certificate_sha256,r.certificate_sha256,r.certificate_pem_sha256,r.bundle_requested,r.state,r.rejection_code,r.accepted_revision,r.receipt_version,r.previous_binding_generation,r.binding_generation FROM mgmt_gateway_certificate_rotations r JOIN mgmt_gateway_certificate_rotation_ids i USING(operation_id) WHERE r.operation_id=$1 AND r.gateway_id=$2 AND r.tenant_id=$3 AND i.gateway_id=r.gateway_id AND i.tenant_id=r.tenant_id")
        .bind(operation).bind(gateway).bind(tenant).fetch_optional(pool).await? else { return Ok(unknown()); };
    let state: String = row.get("state");
    let state = match state.as_str() {
        "APPLIED" => "APPLIED",
        "REJECTED" => "REJECTED",
        _ => "UNKNOWN",
    };
    Ok(json!({"id":gateway,"operationId":operation,"state":state,
          "receiptVersion":row.get::<i16,_>("receipt_version"),
          "previousBindingGeneration":row.get::<Option<i64>,_>("previous_binding_generation"),
          "bindingGeneration":row.get::<Option<i64>,_>("binding_generation"),
        "previousCertificateSha256":row.get::<String,_>("previous_certificate_sha256"),
        "certificateSha256":row.get::<Option<String>,_>("certificate_sha256"),
        "certificatePemSha256":row.get::<String,_>("certificate_pem_sha256"),
        "bundleRequested":row.get::<bool,_>("bundle_requested"),
        "revision":row.get::<Option<i64>,_>("accepted_revision"),
        "rejectionCode":row.get::<Option<String>,_>("rejection_code"),
        "identityBundleTokenRecoverable":false}))
}
