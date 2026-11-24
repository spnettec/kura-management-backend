//! CA custody policy proposals are approved under the current policy before publication.
use crate::{auth::Principal, db};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Policy {
    pub revision: i64,
    pub required_count: i16,
    pub custodian_ids: Option<Vec<Uuid>>,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            revision: 0,
            required_count: 2,
            custodian_ids: None,
        }
    }
}

impl Policy {
    pub(crate) fn permits(&self, id: Uuid) -> bool {
        self.custodian_ids
            .as_ref()
            .is_none_or(|ids| ids.contains(&id))
    }
}

pub(crate) async fn load(conn: &mut PgConnection, tenant: &str) -> Result<Policy> {
    Ok(sqlx::query_as(
        "SELECT revision,GREATEST(quorum_size,2)::smallint AS required_count,custodian_ids
        FROM ca_quorum_policy WHERE tenant_id=$1",
    )
    .bind(tenant)
    .fetch_optional(conn)
    .await?
    .unwrap_or_default())
}

pub(crate) async fn lock_for_activation(conn: &mut PgConnection, tenant: &str) -> Result<Policy> {
    // This lock also covers the initial INSERT for a tenant using the default.
    sqlx::query("LOCK TABLE ca_quorum_policy IN SHARE MODE")
        .execute(&mut *conn)
        .await?;
    load(conn, tenant).await
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Change {
    operation_id: Uuid,
    expected_revision: i64,
    required_count: i16,
    custodian_ids: Vec<Uuid>,
}

impl Change {
    pub(crate) fn normalize(&mut self) -> bool {
        self.custodian_ids.sort_unstable();
        self.operation_id != Uuid::nil()
            && self.expected_revision >= 0
            && self.expected_revision < i64::MAX
            && self.required_count >= 2
            && self.custodian_ids.len() >= self.required_count as usize
            && i16::try_from(self.custodian_ids.len()).is_ok()
            && !self.custodian_ids.contains(&Uuid::nil())
            && self.custodian_ids.windows(2).all(|pair| pair[0] < pair[1])
    }
}

#[derive(sqlx::FromRow)]
struct Proposal {
    #[sqlx(flatten)]
    change: Change,
    approval_required_count: i16,
    applied_approver_ids: Option<Vec<Uuid>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    change: Change,
    approval_required_count: i16,
    approved_count: usize,
    applied_approver_ids: Option<Vec<Uuid>>,
    applied_revision: Option<i64>,
    viewer_approved: bool,
    stale: bool,
}

impl Proposal {
    fn status(self, revision: i64, eligible_count: usize, viewer_approved: bool) -> Status {
        let applied_revision = self
            .applied_approver_ids
            .as_ref()
            .map(|_| self.change.expected_revision + 1);
        Status {
            stale: applied_revision.is_none() && self.change.expected_revision != revision,
            approved_count: self
                .applied_approver_ids
                .as_ref()
                .map_or(eligible_count, Vec::len),
            change: self.change,
            approval_required_count: self.approval_required_count,
            applied_approver_ids: self.applied_approver_ids,
            applied_revision,
            viewer_approved,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Overview {
    policy: Policy,
    eligible_admins: Vec<Admin>,
    viewer_can_propose: bool,
}

#[derive(Serialize, sqlx::FromRow)]
struct Admin {
    id: Uuid,
    username: String,
}

pub(crate) async fn overview(pool: &PgPool, actor: &Principal) -> Result<Overview> {
    let mut tx = pool.begin().await?;
    let tenant = &actor.tenant_id;
    let policy = lock_for_activation(&mut tx, tenant).await?;
    let eligible_admins: Vec<Admin> = sqlx::query_as("SELECT id,username FROM mgmt_users
        WHERE tenant_id=$1 AND enabled AND role='admin' AND NOT must_change_password ORDER BY username,id")
        .bind(tenant).fetch_all(&mut *tx).await?;
    let viewer_can_propose =
        policy.permits(actor.id) && eligible_admins.iter().any(|admin| admin.id == actor.id);
    Ok(Overview {
        policy,
        eligible_admins,
        viewer_can_propose,
    })
}

async fn find(conn: &mut PgConnection, tenant: &str, id: Uuid) -> Result<Option<Proposal>> {
    Ok(sqlx::query_as(
        "SELECT operation_id,expected_revision,required_count,custodian_ids,
            approval_required_count,applied_approver_ids
        FROM ca_policy_proposals WHERE tenant_id=$1 AND operation_id=$2",
    )
    .bind(tenant)
    .bind(id)
    .fetch_optional(conn)
    .await?)
}

async fn votes(
    conn: &mut PgConnection,
    tenant: &str,
    id: Uuid,
    policy: &Policy,
) -> Result<Vec<Uuid>> {
    Ok(sqlx::query_scalar(
        "SELECT u.id FROM ca_policy_approvals a JOIN mgmt_users u ON u.id=a.approver_id
        WHERE a.operation_id=$1 AND u.tenant_id=$2 AND u.enabled AND u.role='admin'
        AND NOT u.must_change_password AND ($3::uuid[] IS NULL OR u.id=ANY($3))
        ORDER BY u.id FOR SHARE OF u",
    )
    .bind(id)
    .bind(tenant)
    .bind(&policy.custodian_ids)
    .fetch_all(conn)
    .await?)
}

pub(crate) async fn eligible(conn: &mut PgConnection, tenant: &str, ids: &[Uuid]) -> Result<bool> {
    let active: Vec<Uuid> = sqlx::query_scalar(
        "SELECT id FROM mgmt_users
        WHERE tenant_id=$1 AND enabled AND role='admin' AND NOT must_change_password AND id=ANY($2)
        ORDER BY id FOR SHARE",
    )
    .bind(tenant)
    .bind(ids)
    .fetch_all(conn)
    .await?;
    Ok(active.len() == ids.len())
}

pub(crate) async fn status(pool: &PgPool, actor: &Principal, id: Uuid) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    let tenant = &actor.tenant_id;
    let policy = lock_for_activation(&mut tx, tenant).await?;
    let Some(proposal) = find(&mut tx, tenant, id).await? else {
        return Ok(None);
    };
    Ok(Some(observe(&mut tx, actor, proposal, &policy).await?))
}

async fn observe(
    conn: &mut PgConnection,
    actor: &Principal,
    proposal: Proposal,
    policy: &Policy,
) -> Result<Status> {
    let count = if proposal.change.expected_revision == policy.revision {
        votes(
            &mut *conn,
            &actor.tenant_id,
            proposal.change.operation_id,
            policy,
        )
        .await?
        .len()
    } else {
        0
    };
    let viewer_approved = sqlx::query_scalar(
        "SELECT EXISTS(
        SELECT 1 FROM ca_policy_approvals WHERE operation_id=$1 AND approver_id=$2)",
    )
    .bind(proposal.change.operation_id)
    .bind(actor.id)
    .fetch_one(conn)
    .await?;
    Ok(proposal.status(policy.revision, count, viewer_approved))
}

pub(crate) async fn propose(
    pool: &PgPool,
    actor: &Principal,
    change: Change,
) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    let tenant = &actor.tenant_id;
    let policy = lock_for_activation(&mut tx, tenant).await?;
    let id = change.operation_id;
    // Observation of the original immutable proposal never modifies it.
    if let Some(original) = find(&mut tx, tenant, id).await? {
        if original.change != change {
            return Ok(None);
        }
        return Ok(Some(observe(&mut tx, actor, original, &policy).await?));
    }
    if policy.revision != change.expected_revision
        || !policy.permits(actor.id)
        || !eligible(&mut tx, tenant, &[actor.id]).await?
        || !eligible(&mut tx, tenant, &change.custodian_ids).await?
    {
        return Ok(None);
    }
    let inserted = sqlx::query("INSERT INTO ca_policy_proposals
        (operation_id,tenant_id,expected_revision,required_count,custodian_ids,approval_required_count)
        VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT DO NOTHING")
        .bind(id).bind(tenant).bind(change.expected_revision).bind(change.required_count)
        .bind(&change.custodian_ids).bind(policy.required_count).execute(&mut *tx).await?;
    if inserted.rows_affected() == 1 {
        db::audit(
            &mut *tx,
            tenant,
            &actor.username,
            "settings.client-ca.policy.propose",
            &id.to_string(),
            "SUCCEEDED",
        )
        .await?;
    }
    let original = find(&mut tx, tenant, id).await?;
    tx.commit().await?;
    Ok(original
        .filter(|original| original.change == change)
        .map(|original| original.status(policy.revision, 0, false)))
}

pub(crate) async fn approve(pool: &PgPool, actor: &Principal, id: Uuid) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    let tenant = &actor.tenant_id;
    // Serializes publication (also when the default has no tenant row) and
    // conflicts with the lock held through synchronous TLS publication.
    sqlx::query("LOCK TABLE ca_quorum_policy IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await?;
    let policy = load(&mut tx, tenant).await?;
    let Some(mut proposal) = find(&mut tx, tenant, id).await? else {
        return Ok(None);
    };
    if proposal.applied_approver_ids.is_some() {
        return Ok(Some(observe(&mut tx, actor, proposal, &policy).await?));
    }
    let change = &proposal.change;
    if change.expected_revision != policy.revision
        || proposal.approval_required_count != policy.required_count
        || !policy.permits(actor.id)
        || !eligible(&mut tx, tenant, &[actor.id]).await?
        || !eligible(&mut tx, tenant, &change.custodian_ids).await?
    {
        return Ok(None);
    }
    let inserted = sqlx::query(
        "INSERT INTO ca_policy_approvals(operation_id,approver_id)
        VALUES($1,$2) ON CONFLICT DO NOTHING",
    )
    .bind(id)
    .bind(actor.id)
    .execute(&mut *tx)
    .await?;
    if inserted.rows_affected() == 1 {
        db::audit(
            &mut *tx,
            tenant,
            &actor.username,
            "settings.client-ca.policy.approve",
            &id.to_string(),
            "SUCCEEDED",
        )
        .await?;
    }
    let approvals = votes(&mut tx, tenant, id, &policy).await?;
    let count = approvals.len();
    if count >= policy.required_count as usize {
        let revision = change.expected_revision + 1;
        let roster_size = i16::try_from(change.custodian_ids.len())?;
        sqlx::query("INSERT INTO ca_quorum_policy(tenant_id,revision,quorum_size,total_approvers,custodian_ids)
            VALUES($1,$2,$3,$4,$5) ON CONFLICT (tenant_id) DO UPDATE SET revision=EXCLUDED.revision,
            quorum_size=EXCLUDED.quorum_size,total_approvers=EXCLUDED.total_approvers,custodian_ids=EXCLUDED.custodian_ids")
            .bind(tenant).bind(revision).bind(change.required_count).bind(roster_size)
            .bind(&change.custodian_ids).execute(&mut *tx).await?;
        sqlx::query("UPDATE ca_policy_proposals SET applied_approver_ids=$2 WHERE operation_id=$1")
            .bind(id)
            .bind(&approvals)
            .execute(&mut *tx)
            .await?;
        db::audit(
            &mut *tx,
            tenant,
            &actor.username,
            "settings.client-ca.policy.publish",
            &id.to_string(),
            "SUCCEEDED",
        )
        .await?;
        proposal.applied_approver_ids = Some(approvals);
    }
    tx.commit().await?;
    Ok(Some(proposal.status(policy.revision, count, true)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custody_change_requires_a_canonical_distinct_roster_and_two_approvers() {
        let first = Uuid::from_u128(1);
        let second = Uuid::from_u128(2);
        let mut change = Change {
            operation_id: Uuid::from_u128(3),
            expected_revision: 0,
            required_count: 2,
            custodian_ids: vec![second, first],
        };
        assert!(change.normalize());
        assert_eq!(change.custodian_ids, vec![first, second]);
        change.required_count = 1;
        assert!(!change.normalize());
        change.required_count = 3;
        assert!(!change.normalize());
        change.required_count = 2;
        change.custodian_ids = vec![first, first];
        assert!(!change.normalize());
    }

    #[test]
    fn applied_policy_retains_original_quorum_proof_after_later_revisions() {
        let first = Uuid::from_u128(1);
        let second = Uuid::from_u128(2);
        let change = Change {
            operation_id: Uuid::from_u128(3),
            expected_revision: 0,
            required_count: 2,
            custodian_ids: vec![first, second],
        };
        let proof = Proposal {
            change,
            approval_required_count: 2,
            applied_approver_ids: Some(vec![first, second]),
        };
        let status = proof.status(2, 0, true);
        assert!(!status.stale);
        assert_eq!(status.applied_revision, Some(1));
        assert_eq!(status.approved_count, 2);
    }
}
