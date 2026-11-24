//! Approval of a specific client CA activation. The CA signing key stays external.
use crate::{auth::Principal, ca_policy, db, tls_runtime::ActivationRequest};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
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
    policy_revision: i64,
    stale: bool,
    approved_count: i64,
    required_count: i16,
    quorum_met: bool,
}

impl Status {
    pub fn authorizes(&self, target: &Target) -> bool {
        self.operation_id == target.operation_id
            && self.generation_id == target.generation_id
            && self.expected_generation == target.expected_generation
            && self.pending_revision == target.pending_revision
            && self.quorum_met
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Evidence {
    tenant_id: String,
    required_count: i16,
    approver_ids: Vec<Uuid>,
}

impl Evidence {
    pub fn valid(&self) -> bool {
        self.required_count >= 2
            && self.approver_ids.len() >= self.required_count as usize
            && self.approver_ids.windows(2).all(|ids| ids[0] < ids[1])
    }
}

/// Owns the database locks through durable publication of the exact TLS target.
pub(crate) struct Authorization {
    target: Target,
    evidence: Option<Evidence>,
    _locks: Option<Transaction<'static, Postgres>>,
}

impl Authorization {
    pub fn matches(&self, target: &Target) -> bool {
        self.target == *target
    }

    pub fn evidence(&self) -> Option<&Evidence> {
        self.evidence.as_ref()
    }

    #[cfg(test)]
    pub fn for_fixture(target: Target) -> Self {
        Self { target, evidence: None, _locks: None }
    }
}

pub(crate) async fn approve(pool: &PgPool, actor: &Principal, target: &Target) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    let policy = ca_policy::lock_for_activation(&mut tx, &actor.tenant_id).await?;
    if !policy.permits(actor.id) || !ca_policy::eligible(&mut tx, &actor.tenant_id, &[actor.id]).await? {
        return Ok(None);
    }
    sqlx::query(
        "INSERT INTO ca_activation_proposals(operation_id,tenant_id,generation_id,expected_generation,pending_revision,policy_revision)
         VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT DO NOTHING",
    )
    .bind(target.operation_id)
    .bind(&actor.tenant_id)
    .bind(target.generation_id)
    .bind(target.expected_generation)
    .bind(&target.pending_revision)
    .bind(policy.revision)
    .execute(&mut *tx)
    .await?;
    let recorded = matching_revision(&mut tx, &actor.tenant_id, target.operation_id, policy.revision).await?;
    if recorded.as_ref() != Some(target) {
        return Ok(None);
    }
    let written = sqlx::query(
        "INSERT INTO ca_operation_approvals(tenant_id,operation_id,operation_kind,generation_id,approver,approver_id,status)
         SELECT tenant_id,$1,'generation-activate',$2,username,id,'approved'
         FROM mgmt_users WHERE id=$3 AND tenant_id=$4
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
    let row: Option<(Uuid, Uuid, String, i64, bool, i16, i64)> = sqlx::query_as(
        "SELECT p.generation_id,p.expected_generation,p.pending_revision,
             p.policy_revision,p.policy_revision<>COALESCE(q.revision,0),
             GREATEST(COALESCE(q.quorum_size,2),2)::smallint,
             (SELECT COUNT(DISTINCT a.approver_id)
              FROM ca_operation_approvals a JOIN mgmt_users u ON u.id=a.approver_id
              WHERE a.tenant_id=p.tenant_id AND a.operation_id=p.operation_id
                AND a.generation_id=p.generation_id AND a.operation_kind='generation-activate'
                AND a.status='approved' AND u.tenant_id=p.tenant_id AND u.enabled
                AND u.role='admin' AND NOT u.must_change_password
                AND (q.custodian_ids IS NULL OR u.id=ANY(q.custodian_ids)))
         FROM ca_activation_proposals p LEFT JOIN ca_quorum_policy q ON q.tenant_id=p.tenant_id
         WHERE p.operation_id=$1 AND p.tenant_id=$2",
    )
    .bind(id)
    .bind(tenant)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(generation_id, expected_generation, pending_revision, policy_revision, stale, required_count, approved_count)| Status {
        operation_id: id,
        generation_id,
        expected_generation,
        pending_revision,
        policy_revision,
        stale,
        approved_count,
        required_count,
        quorum_met: !stale && approved_count >= i64::from(required_count),
    }))
}

async fn matching_revision(
    conn: &mut sqlx::PgConnection, tenant: &str, id: Uuid, revision: i64,
) -> Result<Option<Target>> {
    Ok(sqlx::query_as(
        "SELECT operation_id,generation_id,expected_generation,pending_revision FROM ca_activation_proposals
         WHERE operation_id=$1 AND tenant_id=$2 AND policy_revision=$3 FOR SHARE",
    )
    .bind(id)
    .bind(tenant)
    .bind(revision)
    .fetch_optional(conn)
    .await?)
}

pub(crate) async fn check_activation_quorum(
    pool: &PgPool,
    actor: &Principal,
    target: Target,
) -> Result<Option<Authorization>> {
    let tenant = &actor.tenant_id;
    let mut tx = pool.begin().await?;
    let policy = ca_policy::lock_for_activation(&mut tx, tenant).await?;
    let proposal = matching_revision(&mut tx, tenant, target.operation_id, policy.revision).await?;
    if proposal.as_ref() != Some(&target) {
        return Ok(None);
    }
    let approver_ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT u.id FROM ca_operation_approvals a JOIN mgmt_users u ON u.id=a.approver_id
         WHERE a.tenant_id=$1 AND a.operation_id=$2 AND a.generation_id=$3
           AND a.operation_kind='generation-activate' AND a.status='approved'
           AND u.tenant_id=$1 AND u.enabled AND u.role='admin' AND NOT u.must_change_password
           AND ($4::uuid[] IS NULL OR u.id=ANY($4))
         ORDER BY u.id FOR SHARE OF a,u",
    )
    .bind(tenant)
    .bind(target.operation_id)
    .bind(target.generation_id)
    .bind(&policy.custodian_ids)
    .fetch_all(&mut *tx).await?;
    let required_count = policy.required_count;
    if approver_ids.len() < required_count as usize {
        return Ok(None);
    }
    let eligible_actor: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM mgmt_users WHERE id=$1 AND tenant_id=$2
         AND enabled AND role='admin' AND NOT must_change_password FOR SHARE",
    )
    .bind(actor.id)
    .bind(tenant)
    .fetch_optional(&mut *tx).await?;
    if eligible_actor.is_none() {
        return Ok(None);
    }
    Ok(Some(Authorization {
        target,
        evidence: Some(Evidence { tenant_id: tenant.clone(), required_count, approver_ids }),
        _locks: Some(tx),
    }))
}
