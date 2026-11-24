//! Approval of a specific client CA activation. The CA signing key stays external.
use crate::{auth::Principal, db, tls_runtime::ActivationRequest};
use anyhow::{Result, ensure};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
pub(crate) struct Target {
    pub operation_id: Uuid,
    pub generation_id: Uuid,
    pub expected_generation: Uuid,
    pub pending_revision: String,
}

impl Target {
    pub fn new(request: &ActivationRequest, generation_id: Uuid) -> Self {
        Self {
            operation_id: request.operation_id,
            generation_id,
            expected_generation: request.expected_generation,
            pending_revision: request.pending_revision.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    operation_id: Uuid,
    generation_id: Uuid,
    expected_generation: Uuid,
    pending_revision: String,
    approved_count: i64,
    required_count: i16,
    quorum_met: bool,
}

/// Only an exact proposal with a current quorum can yield this grant.
pub(crate) struct Authorization(Target);

impl Authorization {
    pub fn matches(&self, target: &Target) -> bool {
        self.0 == *target
    }

    #[cfg(test)]
    pub fn for_fixture(target: Target) -> Self {
        Self(target)
    }
}

pub(crate) async fn approve(pool: &PgPool, actor: &Principal, target: &Target) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO ca_activation_proposals(operation_id,tenant_id,generation_id,expected_generation,pending_revision)
         VALUES($1,$2,$3,$4,$5) ON CONFLICT DO NOTHING",
    )
    .bind(target.operation_id)
    .bind(&actor.tenant_id)
    .bind(target.generation_id)
    .bind(target.expected_generation)
    .bind(&target.pending_revision)
    .execute(&mut *tx)
    .await?;
    let recorded: Option<Target> = sqlx::query_as(
        "SELECT operation_id,generation_id,expected_generation,pending_revision
         FROM ca_activation_proposals WHERE operation_id=$1 AND tenant_id=$2",
    )
    .bind(target.operation_id)
    .bind(&actor.tenant_id)
    .fetch_optional(&mut *tx)
    .await?;
    if recorded.as_ref() != Some(target) {
        return Ok(None);
    }
    let written = sqlx::query(
        "INSERT INTO ca_operation_approvals(tenant_id,operation_id,operation_kind,generation_id,approver,approver_id,status)
         SELECT tenant_id,$1,'generation-activate',$2,username,id,'approved'
         FROM mgmt_users WHERE id=$3 AND tenant_id=$4 AND enabled AND role='admin' AND NOT must_change_password
         ON CONFLICT DO NOTHING",
    )
    .bind(target.operation_id)
    .bind(target.generation_id)
    .bind(actor.id)
    .bind(&actor.tenant_id)
    .execute(&mut *tx)
    .await?;
    if written.rows_affected() == 1 {
        db::audit(
            &mut *tx,
            &actor.tenant_id,
            &actor.username,
            "settings.client-ca.approve",
            &target.operation_id.to_string(),
            "SUCCEEDED",
        )
        .await?;
    }
    tx.commit().await?;
    status(pool, &actor.tenant_id, target.operation_id).await
}

pub(crate) async fn status(pool: &PgPool, tenant: &str, id: Uuid) -> Result<Option<Status>> {
    let row: Option<(Uuid, Uuid, String, i16, i64)> = sqlx::query_as(
        "SELECT p.generation_id,p.expected_generation,p.pending_revision,
             GREATEST(COALESCE(q.quorum_size,2),2)::smallint,
             (SELECT COUNT(DISTINCT a.approver_id)
              FROM ca_operation_approvals a JOIN mgmt_users u ON u.id=a.approver_id
              WHERE a.tenant_id=p.tenant_id AND a.operation_id=p.operation_id
                AND a.generation_id=p.generation_id AND a.operation_kind='generation-activate'
                AND a.status='approved' AND u.tenant_id=p.tenant_id AND u.enabled
                AND u.role='admin' AND NOT u.must_change_password)
         FROM ca_activation_proposals p LEFT JOIN ca_quorum_policy q ON q.tenant_id=p.tenant_id
         WHERE p.operation_id=$1 AND p.tenant_id=$2",
    )
    .bind(id)
    .bind(tenant)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(generation_id, expected_generation, pending_revision, required_count, approved_count)| Status {
        operation_id: id,
        generation_id,
        expected_generation,
        pending_revision,
        approved_count,
        required_count,
        quorum_met: approved_count >= i64::from(required_count),
    }))
}

pub(crate) async fn check_activation_quorum(
    pool: &PgPool,
    tenant: &str,
    target: Target,
) -> Result<Option<Authorization>> {
    let Some(proposal) = status(pool, tenant, target.operation_id).await? else {
        return Ok(None);
    };
    ensure!(
        proposal.generation_id == target.generation_id
            && proposal.expected_generation == target.expected_generation
            && proposal.pending_revision == target.pending_revision,
        "CA activation approval targets another proposal"
    );
    Ok(proposal.quorum_met.then_some(Authorization(target)))
}
