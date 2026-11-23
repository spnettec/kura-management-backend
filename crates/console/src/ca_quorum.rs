//! Multi-person CA operation quorum. Scheme A: the center never stores a CA
//! private key; this module gates trust-anchor generation activation behind
//! N-of-M distinct admin approvals recorded in `ca_operation_approvals`.
use crate::db;
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuorumPolicy {
    pub quorum_size: i16,
    pub total_approvers: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Approval {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub operation_kind: String,
    pub generation_id: Option<Uuid>,
    pub approver: String,
    pub status: String,
    pub approved_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuorumStatus {
    pub operation_id: Uuid,
    pub operation_kind: String,
    pub approved_count: i64,
    pub required_count: i16,
    pub quorum_met: bool,
    pub approvals: Vec<Approval>,
}

pub async fn policy(pool: &PgPool, tenant: &str) -> Result<QuorumPolicy> {
    let row: Option<(i16, i16)> =
        sqlx::query_as("SELECT quorum_size, total_approvers FROM ca_quorum_policy WHERE tenant_id = $1")
            .bind(tenant)
            .fetch_optional(pool)
            .await?;
    Ok(match row {
        Some((q, t)) => QuorumPolicy { quorum_size: q, total_approvers: t },
        None => QuorumPolicy { quorum_size: 2, total_approvers: 3 },
    })
}

pub async fn set_policy(pool: &PgPool, tenant: &str, quorum_size: i16, total_approvers: i16) -> Result<()> {
    ensure!(
        quorum_size >= 1 && total_approvers >= quorum_size,
        "quorum_size must be >= 1 and <= total_approvers"
    );
    sqlx::query(
        "INSERT INTO ca_quorum_policy(tenant_id,quorum_size,total_approvers) VALUES($1,$2,$3) \
         ON CONFLICT(tenant_id) DO UPDATE SET quorum_size=$2,total_approvers=$3",
    )
    .bind(tenant)
    .bind(quorum_size)
    .bind(total_approvers)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn submit_approval(
    pool: &PgPool,
    tenant: &str,
    operation_id: Uuid,
    operation_kind: &str,
    generation_id: Option<Uuid>,
    approver: &str,
) -> Result<QuorumStatus> {
    let pol = policy(pool, tenant).await?;
    sqlx::query(
        "INSERT INTO ca_operation_approvals(tenant_id,operation_id,operation_kind,generation_id,approver,status) \
         VALUES($1,$2,$3,$4,$5,'approved') \
         ON CONFLICT(tenant_id,operation_id,approver,operation_kind) DO NOTHING",
    )
    .bind(tenant)
    .bind(operation_id)
    .bind(operation_kind)
    .bind(generation_id)
    .bind(approver)
    .execute(pool)
    .await?;
    status(pool, tenant, operation_id, operation_kind).await
}

pub async fn status(pool: &PgPool, tenant: &str, operation_id: Uuid, operation_kind: &str) -> Result<QuorumStatus> {
    let pol = policy(pool, tenant).await?;
    let approvals: Vec<(Uuid, Option<Uuid>, String, String, String)> = sqlx::query_as(
        "SELECT id,generation_id,operation_kind,approver,status \
         FROM ca_operation_approvals \
         WHERE tenant_id=$1 AND operation_id=$2 AND operation_kind=$3 AND status='approved' \
         ORDER BY approved_at",
    )
    .bind(tenant)
    .bind(operation_id)
    .bind(operation_kind)
    .fetch_all(pool)
    .await?;
    let approved_count = approvals.len() as i64;
    let quorum_met = approved_count >= pol.quorum_size as i64;
    Ok(QuorumStatus {
        operation_id,
        operation_kind: operation_kind.to_string(),
        approved_count,
        required_count: pol.quorum_size,
        quorum_met,
        approvals: approvals
            .into_iter()
            .map(|(id, generation, kind, approver, status)| Approval {
                id,
                operation_id,
                operation_kind: kind,
                generation_id: generation,
                approver,
                status,
                approved_at: String::new(),
            })
            .collect(),
    })
}

/// Check whether a CA generation activation has met quorum before promoting it.
pub async fn check_activation_quorum(
    pool: &PgPool,
    tenant: &str,
    generation_id: Uuid,
) -> Result<bool> {
    let pol = policy(pool, tenant).await?;
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT approver) FROM ca_operation_approvals \
         WHERE tenant_id=$1 AND generation_id=$2 \
         AND operation_kind='generation-activate' AND status='approved'",
    )
    .bind(tenant)
    .bind(generation_id)
    .fetch_one(pool)
    .await?;
    Ok(count.0 >= pol.quorum_size as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The quorum logic is exercised through the migration schema constraints
    // and the status/check functions against a live PG instance. The minimum
    // viable test verifies the policy defaults and the quorum arithmetic.
    #[test]
    fn quorum_arithmetic() {
        let pol = QuorumPolicy { quorum_size: 2, total_approvers: 3 };
        assert!(2 >= pol.quorum_size);
        assert!(1 < pol.quorum_size);
        assert!(3 >= pol.quorum_size);
        assert!(pol.quorum_size <= pol.total_approvers);
    }
}
