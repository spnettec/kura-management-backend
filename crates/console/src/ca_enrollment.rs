//! Bulk enrollment of external, public client certificates under the active CA and custody policy.
use crate::{auth::Principal, ca_policy, ca_signing, db, gateway_registration};
use anyhow::{Context, Result, ensure};
use rustls::{pki_types::UnixTime, server::danger::ClientCertVerifier};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, PgConnection, PgPool, Postgres, Transaction, types::Json};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

pub(crate) const MAX_ENTRIES: usize = 32;
pub(crate) const MAX_BODY: usize = 2 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    operation_id: Uuid,
    entries: Vec<Input>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Input {
    id: String,
    name: String,
    client_certificate_pem: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct Gateway {
    id: String,
    name: String,
    certificate_sha256: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    gateway: Gateway,
    pem: String,
}

pub(crate) struct Batch {
    operation_id: Uuid,
    entries: Vec<Entry>,
}

impl Request {
    pub(crate) fn operation_id(&self) -> Uuid {
        self.operation_id
    }

    pub(crate) fn validate(mut self, tenant: &str) -> Result<Batch> {
        ensure!(!self.operation_id.is_nil(), "non-nil operation ID required");
        ensure!(
            (2..=MAX_ENTRIES).contains(&self.entries.len()),
            "invalid batch size"
        );
        self.entries.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        ensure!(
            self.entries.windows(2).all(|pair| pair[0].id != pair[1].id),
            "duplicate gateway ID"
        );
        let mut fingerprints = HashSet::new();
        let mut entries = Vec::with_capacity(self.entries.len());
        for input in self.entries {
            gateway_registration::validate(&input.id, tenant, &input.name)?;
            let fingerprint =
                gateway_registration::certificate_sha256(&input.client_certificate_pem)?;
            ensure!(
                fingerprints.insert(fingerprint.clone()),
                "duplicate certificate"
            );
            entries.push(Entry {
                gateway: Gateway {
                    id: input.id,
                    name: input.name,
                    certificate_sha256: fingerprint,
                },
                pem: input.client_certificate_pem.trim().into(),
            });
        }
        Ok(Batch {
            operation_id: self.operation_id,
            entries,
        })
    }
}

pub(crate) struct Trust {
    generation: Option<Uuid>,
    fingerprint: String,
    verifier: Arc<dyn ClientCertVerifier>,
}

impl Trust {
    pub(crate) fn new(
        generation: Option<Uuid>,
        fingerprint: String,
        verifier: Arc<dyn ClientCertVerifier>,
    ) -> Self {
        Self {
            generation,
            fingerprint,
            verifier,
        }
    }

    fn verify(&self, entries: &[Entry]) -> Result<()> {
        let now = UnixTime::now();
        for entry in entries {
            let der = rustls_pemfile::certs(&mut entry.pem.as_bytes())
                .next()
                .context("client certificate required")??;
            // Each enrollment accepts one leaf, the same public PEM contract as ordinary enrollment.
            self.verifier.verify_client_cert(&der, &[], now)?;
        }
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct Proposal {
    operation_id: Uuid,
    ca_fingerprint: String,
    ca_generation: Option<Uuid>,
    policy_revision: i64,
    required_count: i16,
    entries: Json<Vec<Entry>>,
    decision_approver_ids: Option<Vec<Uuid>>,
    rejection_code: Option<String>,
    signing_manifest_sha256: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    operation_id: Uuid,
    ca_fingerprint: String,
    ca_generation: Option<Uuid>,
    policy_revision: i64,
    required_count: i16,
    entries: Vec<Gateway>,
    approved_count: usize,
    viewer_approved: bool,
    state: &'static str,
    decision_approver_ids: Option<Vec<Uuid>>,
    rejection_code: Option<String>,
    signing_manifest_sha256: Option<String>,
}

impl Proposal {
    fn terminal(&self) -> bool {
        self.decision_approver_ids.is_some()
    }

    fn stale(&self, policy: &ca_policy::Policy, trust: &Trust) -> bool {
        self.policy_revision != policy.revision
            || self.required_count != policy.required_count
            || self.ca_generation != trust.generation
            || self.ca_fingerprint != trust.fingerprint
    }

    fn status(self, count: usize, viewer_approved: bool, stale: bool) -> Status {
        let state = if self.rejection_code.is_some() {
            "REJECTED"
        } else if self.terminal() {
            "APPLIED"
        } else if stale {
            "STALE"
        } else {
            "PENDING"
        };
        Status {
            operation_id: self.operation_id,
            ca_fingerprint: self.ca_fingerprint,
            ca_generation: self.ca_generation,
            policy_revision: self.policy_revision,
            required_count: self.required_count,
            entries: self
                .entries
                .0
                .into_iter()
                .map(|entry| entry.gateway)
                .collect(),
            approved_count: self.decision_approver_ids.as_ref().map_or(count, Vec::len),
            viewer_approved,
            state,
            decision_approver_ids: self.decision_approver_ids,
            rejection_code: self.rejection_code,
            signing_manifest_sha256: self.signing_manifest_sha256,
        }
    }
}

async fn find(
    conn: &mut PgConnection,
    tenant: &str,
    id: Uuid,
    write: bool,
) -> Result<Option<Proposal>> {
    let query = "SELECT operation_id,ca_fingerprint,ca_generation,policy_revision,required_count,
                  entries,decision_approver_ids,rejection_code,signing_manifest_sha256
                 FROM ca_enrollment_batches WHERE tenant_id=$1 AND operation_id=$2";
    let locking;
    let query = if write {
        locking = format!("{query} FOR UPDATE");
        &locking
    } else {
        query
    };
    Ok(sqlx::query_as(query)
        .bind(tenant)
        .bind(id)
        .fetch_optional(conn)
        .await?)
}

async fn votes(
    conn: &mut PgConnection,
    tenant: &str,
    id: Uuid,
    policy: &ca_policy::Policy,
) -> Result<Vec<Uuid>> {
    Ok(sqlx::query_scalar(
        "SELECT u.id FROM ca_enrollment_batch_approvals a JOIN mgmt_users u ON u.id=a.approver_id
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

async fn observe(
    conn: &mut PgConnection,
    actor: &Principal,
    proposal: Proposal,
    policy: &ca_policy::Policy,
    trust: &Trust,
) -> Result<Status> {
    let stale = proposal.stale(policy, trust);
    let count = if !stale && !proposal.terminal() {
        votes(&mut *conn, &actor.tenant_id, proposal.operation_id, policy)
            .await?
            .len()
    } else {
        0
    };
    let viewer_approved = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM ca_enrollment_batch_approvals WHERE operation_id=$1 AND approver_id=$2)",
    ).bind(proposal.operation_id).bind(actor.id).fetch_one(conn).await?;
    Ok(proposal.status(count, viewer_approved, stale))
}

pub(crate) async fn status(
    pool: &PgPool,
    actor: &Principal,
    id: Uuid,
    trust: &Trust,
) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    let policy = ca_policy::lock_for_activation(&mut tx, &actor.tenant_id).await?;
    let Some(proposal) = find(&mut tx, &actor.tenant_id, id, false).await? else {
        return Ok(None);
    };
    Ok(Some(
        observe(&mut tx, actor, proposal, &policy, trust).await?,
    ))
}

pub(crate) enum Submission {
    Recorded(Status),
    Conflict,
    InvalidCertificate,
    SigningMismatch,
}

pub(crate) async fn propose(
    pool: &PgPool,
    actor: &Principal,
    batch: Batch,
    trust: &Trust,
    require_signing: bool,
) -> Result<Submission> {
    let id = batch.operation_id;
    let tenant = &actor.tenant_id;
    let mut tx = pool.begin().await?;
    let policy = ca_policy::lock_for_activation(&mut tx, tenant).await?;
    ca_signing::lock_id(&mut tx, id).await?;
    if let Some(original) = find(&mut tx, tenant, id, false).await? {
        if require_signing && original.signing_manifest_sha256.is_none() {
            return Ok(Submission::SigningMismatch);
        }
        if original
            .entries
            .0
            .iter()
            .map(|entry| &entry.gateway)
            .ne(batch.entries.iter().map(|entry| &entry.gateway))
        {
            return Ok(Submission::Conflict);
        }
        return Ok(Submission::Recorded(
            observe(&mut tx, actor, original, &policy, trust).await?,
        ));
    }
    if !policy.permits(actor.id) || !ca_policy::eligible(&mut tx, tenant, &[actor.id]).await? {
        return Ok(Submission::Conflict);
    }
    let signing_manifest = match ca_signing::lookup(&mut tx, tenant, id).await? {
        ca_signing::Lookup::Missing if !require_signing => None,
        ca_signing::Lookup::Missing => return Ok(Submission::SigningMismatch),
        ca_signing::Lookup::Foreign => return Ok(Submission::Conflict),
        ca_signing::Lookup::Original(original, digest) => {
            let signed = batch.entries.iter().map(|entry| {
                (
                    entry.gateway.id.as_str(),
                    entry.gateway.name.as_str(),
                    entry.pem.as_str(),
                )
            });
            if !original.matches(signed)? {
                return Ok(Submission::SigningMismatch);
            }
            Some(digest)
        }
    };
    if trust.verify(&batch.entries).is_err() {
        return Ok(Submission::InvalidCertificate);
    }
    let inserted = sqlx::query(
        "INSERT INTO ca_enrollment_batches(operation_id,tenant_id,ca_fingerprint,ca_generation,policy_revision,
            required_count,entries,signing_manifest_sha256)
         VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT DO NOTHING",
    ).bind(id).bind(tenant).bind(&trust.fingerprint).bind(trust.generation).bind(policy.revision).bind(policy.required_count)
        .bind(Json(&batch.entries)).bind(signing_manifest).execute(&mut *tx).await?;
    if inserted.rows_affected() != 1 {
        return Ok(Submission::Conflict);
    }
    db::audit(
        &mut *tx,
        tenant,
        &actor.username,
        "gateway.enrollment.batch.propose",
        &id.to_string(),
        "SUCCEEDED",
    )
    .await?;
    let proposal = find(&mut tx, tenant, id, false)
        .await?
        .context("batch proposal missing")?;
    tx.commit().await?;
    Ok(Submission::Recorded(proposal.status(0, false, false)))
}

/// This is called while holding the TLS runtime's change lock. Policy and voter
/// locks remain held until PostgreSQL commits the whole batch and its receipt.
pub(crate) async fn approve(
    pool: &PgPool,
    actor: &Principal,
    id: Uuid,
    trust: &Trust,
) -> Result<Option<Status>> {
    let tenant = &actor.tenant_id;
    let mut tx = pool.begin().await?;
    let policy = ca_policy::lock_for_activation(&mut tx, tenant).await?;
    let Some(mut proposal) = find(&mut tx, tenant, id, true).await? else {
        return Ok(None);
    };
    if proposal.terminal() || proposal.stale(&policy, trust) {
        return Ok(Some(
            observe(&mut tx, actor, proposal, &policy, trust).await?,
        ));
    }
    if !policy.permits(actor.id) || !ca_policy::eligible(&mut tx, tenant, &[actor.id]).await? {
        return Ok(None);
    }
    let inserted = sqlx::query(
        "INSERT INTO ca_enrollment_batch_approvals(operation_id,approver_id)
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
            "gateway.enrollment.batch.approve",
            &id.to_string(),
            "SUCCEEDED",
        )
        .await?;
    }
    let approvers = votes(&mut tx, tenant, id, &policy).await?;
    let count = approvers.len();
    if count >= proposal.required_count as usize {
        if trust.verify(&proposal.entries.0).is_err() {
            proposal.rejection_code = Some("CERTIFICATE_INVALID".into());
        } else if !register_all(&mut tx, actor, &proposal.entries.0).await? {
            proposal.rejection_code = Some("GATEWAY_CONFLICT".into());
        }
        proposal.decision_approver_ids = Some(approvers);
        sqlx::query("UPDATE ca_enrollment_batches SET decision_approver_ids=$2,rejection_code=$3 WHERE operation_id=$1")
            .bind(id).bind(&proposal.decision_approver_ids).bind(&proposal.rejection_code).execute(&mut *tx).await?;
        let outcome = if proposal.rejection_code.is_none() {
            "SUCCEEDED"
        } else {
            "REJECTED"
        };
        db::audit(
            &mut *tx,
            tenant,
            &actor.username,
            "gateway.enrollment.batch.apply",
            &id.to_string(),
            outcome,
        )
        .await?;
    }
    tx.commit().await?;
    Ok(Some(proposal.status(count, true, false)))
}

async fn register_all(
    tx: &mut Transaction<'_, Postgres>,
    actor: &Principal,
    entries: &[Entry],
) -> Result<bool> {
    // A uniqueness conflict aborts this savepoint, including any successful
    // prefix and its audit/events. The original batch keeps a rejection receipt.
    let mut batch = tx.begin().await?;
    let author = actor.id.to_string();
    for entry in entries {
        let gateway = &entry.gateway;
        let result = gateway_registration::register_in_transaction(
            &mut batch,
            &gateway.id,
            &actor.tenant_id,
            &gateway.name,
            &gateway.certificate_sha256,
            &author,
        )
        .await;
        if let Err(error) = result {
            if error
                .as_database_error()
                .is_some_and(|db| db.is_unique_violation())
            {
                batch.rollback().await?;
                return Ok(false);
            }
            return Err(error.into());
        }
    }
    batch.commit().await?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustls::{RootCertStore, server::WebPkiClientVerifier};

    const CLIENT_ONE: &str = include_str!("../tests/fixtures/tls-live/client-one.pem");
    const CLIENT_TWO: &str = include_str!("../tests/fixtures/tls-live/client-two.pem");
    const CA: &str = include_str!("../tests/fixtures/tls-live/ca-one.pem");

    fn request(ids: [&str; 2], certificates: [&str; 2]) -> Request {
        Request {
            operation_id: Uuid::new_v4(),
            entries: ids
                .into_iter()
                .zip(certificates)
                .map(|(id, pem)| Input {
                    id: id.into(),
                    name: id.into(),
                    client_certificate_pem: pem.into(),
                })
                .collect(),
        }
    }

    #[test]
    fn manifests_canonicalize_order_and_refuse_duplicate_entries_or_private_material() {
        let batch = request(["gateway-b", "gateway-a"], [CLIENT_ONE, CLIENT_TWO])
            .validate("tenant")
            .unwrap();
        assert_eq!(batch.entries[0].gateway.id, "gateway-a");
        assert!(
            request(["gateway-a", "gateway-a"], [CLIENT_ONE, CLIENT_TWO])
                .validate("tenant")
                .is_err()
        );
        assert!(
            request(["gateway-a", "gateway-b"], [CLIENT_ONE, CLIENT_ONE])
                .validate("tenant")
                .is_err()
        );
        let private =
            format!("{CLIENT_ONE}\n-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----");
        assert!(
            request(["gateway-a", "gateway-b"], [CLIENT_TWO, &private])
                .validate("tenant")
                .is_err()
        );
    }

    #[test]
    fn enrollment_requires_active_client_trust_and_the_exact_ca_generation() {
        let provider = rustls::crypto::ring::default_provider();
        let mut roots = RootCertStore::empty();
        roots
            .add(
                rustls_pemfile::certs(&mut CA.as_bytes())
                    .next()
                    .unwrap()
                    .unwrap(),
            )
            .unwrap();
        let verifier =
            WebPkiClientVerifier::builder_with_provider(Arc::new(roots), Arc::new(provider))
                .build()
                .unwrap();
        let fingerprint = crate::client_ca_store::validate(CA).unwrap();
        let trust = Trust::new(None, fingerprint.clone(), verifier.clone());
        let batch = request(["gateway-a", "gateway-b"], [CLIENT_ONE, CLIENT_TWO])
            .validate("tenant")
            .unwrap();
        assert!(trust.verify(&batch.entries[..1]).is_ok());
        assert!(trust.verify(&batch.entries).is_err());
        let proposal = Proposal {
            operation_id: batch.operation_id,
            ca_fingerprint: fingerprint.clone(),
            ca_generation: None,
            policy_revision: 0,
            required_count: 2,
            entries: Json(batch.entries),
            decision_approver_ids: None,
            rejection_code: None,
            signing_manifest_sha256: None,
        };
        let policy = ca_policy::Policy::default();
        assert!(!proposal.stale(&policy, &trust));
        let next = Trust::new(Some(Uuid::new_v4()), fingerprint, verifier);
        assert!(proposal.stale(&policy, &next));
    }
}
