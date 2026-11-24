//! Public PKCS#10 handoffs from gateway-local identities to an external signer.
use crate::{
    auth::{self, Principal},
    ca_policy, db, gateway_registration, provisioning,
};
use anyhow::{Context, Result, anyhow, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use ring::signature;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool, types::Json};
use std::collections::HashSet;
use uuid::Uuid;
use x509_cert::{
    Certificate,
    der::{Decode, Encode, Tag, Tagged, asn1::ObjectIdentifier},
    name::Name,
    request::CertReq,
};

const COMMON_NAME: ObjectIdentifier = ObjectIdentifier::new_unwrap("2.5.4.3");
const EC: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.10045.2.1");
const P256: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.10045.3.1.7");
const ECDSA_SHA256: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.10045.4.3.2");
const RSA: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1");
const RSA_SHA256: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.11");
const EXTENSION_REQUEST: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.9.14");

fn public_der(pem: &str, label: &str) -> Result<Vec<u8>> {
    ensure!(pem.len() <= 32768, "public PEM exceeds its size limit");
    let body = pem
        .trim()
        .strip_prefix(&format!("-----BEGIN {label}-----"))
        .and_then(|body| body.strip_suffix(&format!("-----END {label}-----")))
        .context("one public PEM block required")?;
    let encoded: String = body.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let der = STANDARD.decode(&encoded)?;
    ensure!(STANDARD.encode(&der) == encoded, "noncanonical public PEM");
    Ok(der)
}

fn exact_subject(subject: &Name, gateway_id: &str) -> bool {
    let mut values = subject.as_ref().iter().flat_map(|rdn| rdn.iter());
    values.next().is_some_and(|attribute| {
        attribute.oid == COMMON_NAME
            && matches!(
                attribute.value.tag(),
                Tag::Utf8String | Tag::PrintableString
            )
            && attribute.value.value() == gateway_id.as_bytes()
    }) && values.next().is_none()
}

fn check_csr(gateway_id: &str, pem: &str) -> Result<(String, String)> {
    let der = public_der(pem, "CERTIFICATE REQUEST")?;
    let csr = CertReq::from_der(&der)?;
    ensure!(
        exact_subject(&csr.info.subject, gateway_id),
        "CSR subject differs from gateway ID"
    );
    // Do not retain or export password-bearing request attributes.
    ensure!(
        csr.info
            .attributes
            .iter()
            .all(|attribute| attribute.oid == EXTENSION_REQUEST),
        "unsupported CSR attribute"
    );
    let key = &csr.info.public_key;
    let algo = match (key.algorithm.oid, csr.algorithm.oid) {
        (EC, ECDSA_SHA256)
            if key
                .algorithm
                .parameters
                .as_ref()
                .and_then(|p| p.decode_as::<ObjectIdentifier>().ok())
                == Some(P256)
                && csr.algorithm.parameters.is_none() =>
        {
            &signature::ECDSA_P256_SHA256_ASN1 as &dyn signature::VerificationAlgorithm
        }
        (RSA, RSA_SHA256)
            if [&key.algorithm.parameters, &csr.algorithm.parameters]
                .iter()
                .all(|parameters| {
                    parameters
                        .as_ref()
                        .is_none_or(|value| value.tag() == Tag::Null)
                }) =>
        {
            &signature::RSA_PKCS1_2048_8192_SHA256
        }
        _ => return Err(anyhow!("unsupported CSR signature algorithm")),
    };
    let public = key
        .subject_public_key
        .as_bytes()
        .context("invalid CSR public key bits")?;
    let proof = csr
        .signature
        .as_bytes()
        .context("invalid CSR signature bits")?;
    signature::UnparsedPublicKey::new(algo, public)
        .verify(&csr.info.to_der()?, proof)
        .map_err(|_| anyhow!("CSR signature is invalid"))?;
    Ok((auth::hash(&der), auth::hash(&key.to_der()?)))
}

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
    csr_pem: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PublicEntry {
    id: String,
    name: String,
    path: String,
    csr_sha256: String,
    public_key_sha256: String,
}
#[derive(Deserialize, Serialize)]
struct Entry {
    public: PublicEntry,
    pem: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    schema_version: u8,
    operation_id: Uuid,
    entries: Vec<PublicEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    #[serde(flatten)]
    manifest: Manifest,
    manifest_sha256: String,
}

pub(crate) struct Batch {
    id: Uuid,
    entries: Vec<Entry>,
}

impl Request {
    pub(crate) fn validate(mut self, tenant: &str) -> Result<Batch> {
        ensure!(!self.operation_id.is_nil(), "batch operation ID required");
        ensure!(
            (2..=crate::ca_enrollment::MAX_ENTRIES).contains(&self.entries.len()),
            "invalid signing batch size"
        );
        self.entries.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        ensure!(
            self.entries.windows(2).all(|pair| pair[0].id != pair[1].id),
            "duplicate gateway ID"
        );
        let mut entries = Vec::with_capacity(self.entries.len());
        let mut public_keys = HashSet::new();
        for (index, input) in self.entries.into_iter().enumerate() {
            gateway_registration::validate(&input.id, tenant, &input.name)?;
            let (csr_sha256, public_key_sha256) = check_csr(&input.id, &input.csr_pem)?;
            ensure!(
                public_keys.insert(public_key_sha256.clone()),
                "duplicate CSR key"
            );
            entries.push(Entry {
                public: PublicEntry {
                    id: input.id,
                    name: input.name,
                    path: format!("requests/{:02}.csr.pem", index + 1),
                    csr_sha256,
                    public_key_sha256,
                },
                pem: input.csr_pem.trim().into(),
            });
        }
        Ok(Batch {
            id: self.operation_id,
            entries,
        })
    }
}

impl Batch {
    fn manifest(&self) -> Manifest {
        Manifest {
            schema_version: 1,
            operation_id: self.id,
            entries: self.entries.iter().map(|e| e.public.clone()).collect(),
        }
    }

    pub(crate) fn status(&self) -> Result<Status> {
        let manifest = self.manifest();
        let manifest_sha256 = auth::hash(&serde_json::to_vec(&manifest)?);
        Ok(Status {
            manifest,
            manifest_sha256,
        })
    }

    pub(crate) fn archive(&self) -> Result<Vec<u8>> {
        let manifest = serde_json::to_vec(&self.manifest())?;
        provisioning::archive_members(
            std::iter::once(("manifest.json", manifest.as_slice())).chain(
                self.entries
                    .iter()
                    .map(|entry| (entry.public.path.as_str(), entry.pem.as_bytes())),
            ),
        )
    }

    pub(crate) fn matches<'a>(
        &self,
        signed: impl Iterator<Item = (&'a str, &'a str, &'a str)>,
    ) -> Result<bool> {
        let mut signed = signed;
        for entry in &self.entries {
            let Some((id, name, pem)) = signed.next() else {
                return Ok(false);
            };
            if entry.public.id != id || entry.public.name != name {
                return Ok(false);
            }
            let cert = Certificate::from_der(&public_der(pem, "CERTIFICATE")?)?;
            let leaf = cert.tbs_certificate();
            if !exact_subject(leaf.subject(), id)
                || auth::hash(&leaf.subject_public_key_info().to_der()?)
                    != entry.public.public_key_sha256
            {
                return Ok(false);
            }
        }
        Ok(signed.next().is_none())
    }
}

// A signing request and its enrollment share the UUID namespace. Lock the
// original identifier across both inserts, including when neither row exists.
pub(crate) async fn lock_id(conn: &mut PgConnection, id: Uuid) -> Result<()> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 1031))")
        .bind(id.to_string())
        .execute(conn)
        .await?;
    Ok(())
}

pub(crate) enum Lookup {
    Missing,
    Foreign,
    Original(Batch, String),
}

pub(crate) async fn lookup(conn: &mut PgConnection, tenant: &str, id: Uuid) -> Result<Lookup> {
    let row: Option<(String, String, Json<Vec<Entry>>)> = sqlx::query_as(
        "SELECT tenant_id,manifest_sha256,entries FROM ca_signing_requests WHERE operation_id=$1",
    )
    .bind(id)
    .fetch_optional(conn)
    .await?;
    let Some((owner, digest, Json(entries))) = row else {
        return Ok(Lookup::Missing);
    };
    if owner != tenant {
        return Ok(Lookup::Foreign);
    }
    let batch = Batch { id, entries };
    ensure!(
        batch.status()?.manifest_sha256 == digest,
        "CSR manifest changed"
    );
    for entry in &batch.entries {
        let der = public_der(&entry.pem, "CERTIFICATE REQUEST")?;
        ensure!(
            auth::hash(&der) == entry.public.csr_sha256,
            "CSR material changed"
        );
    }
    Ok(Lookup::Original(batch, digest))
}

pub(crate) async fn create(
    pool: &PgPool,
    actor: &Principal,
    batch: Batch,
) -> Result<Option<Status>> {
    let mut tx = pool.begin().await?;
    let tenant = &actor.tenant_id;
    let policy = ca_policy::lock_for_activation(&mut tx, tenant).await?;
    lock_id(&mut tx, batch.id).await?;
    let status = batch.status()?;
    match lookup(&mut tx, tenant, batch.id).await? {
        Lookup::Original(_, original) => {
            return Ok((original == status.manifest_sha256).then_some(status));
        }
        Lookup::Foreign => return Ok(None),
        Lookup::Missing => {}
    }
    if !policy.permits(actor.id) || !ca_policy::eligible(&mut tx, tenant, &[actor.id]).await? {
        return Ok(None);
    }
    let occupied: bool = sqlx::query_scalar(
        "SELECT EXISTS(
        SELECT 1 FROM ca_enrollment_batches WHERE operation_id=$1)",
    )
    .bind(batch.id)
    .fetch_one(&mut *tx)
    .await?;
    if occupied {
        return Ok(None);
    }
    sqlx::query(
        "INSERT INTO ca_signing_requests(operation_id,tenant_id,manifest_sha256,entries)
        VALUES($1,$2,$3,$4)",
    )
    .bind(batch.id)
    .bind(tenant)
    .bind(&status.manifest_sha256)
    .bind(Json(&batch.entries))
    .execute(&mut *tx)
    .await?;
    db::audit(
        &mut *tx,
        tenant,
        &actor.username,
        "gateway.enrollment.signing.prepare",
        &batch.id.to_string(),
        "SUCCEEDED",
    )
    .await?;
    tx.commit().await?;
    Ok(Some(status))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSR: &str = include_str!("../tests/fixtures/tls-live/client-one.csr.pem");
    const LEAF: &str = include_str!("../tests/fixtures/tls-live/client-one.pem");
    const OTHER_KEY: &str = include_str!("../tests/fixtures/tls-live/client-two.pem");

    #[test]
    fn csr_requires_the_exact_subject_and_a_valid_signature() {
        assert!(check_csr("localhost", CSR).is_ok());
        assert!(check_csr("LOCALHOST", CSR).is_err());
        let mut der = public_der(CSR, "CERTIFICATE REQUEST").unwrap();
        *der.last_mut().unwrap() ^= 1;
        let tampered = format!(
            "-----BEGIN CERTIFICATE REQUEST-----\n{}\n-----END CERTIFICATE REQUEST-----",
            STANDARD.encode(der)
        );
        assert!(check_csr("localhost", &tampered).is_err());
        assert!(check_csr("localhost", &format!("{CSR}{CSR}")).is_err());
    }

    #[test]
    fn signed_import_requires_the_original_key_and_the_complete_manifest() {
        let (csr_sha256, public_key_sha256) = check_csr("localhost", CSR).unwrap();
        let batch = Batch {
            id: Uuid::new_v4(),
            entries: vec![Entry {
                public: PublicEntry {
                    id: "localhost".into(),
                    name: "Fixture".into(),
                    path: "requests/01.csr.pem".into(),
                    csr_sha256,
                    public_key_sha256,
                },
                pem: CSR.to_owned(),
            }],
        };
        assert!(
            batch
                .matches([("localhost", "Fixture", LEAF)].into_iter())
                .unwrap()
        );
        assert!(
            !batch
                .matches([("localhost", "Fixture", OTHER_KEY)].into_iter())
                .unwrap()
        );
        assert!(
            !batch
                .matches([("localhost", "changed", LEAF)].into_iter())
                .unwrap()
        );
        assert!(!batch.matches(std::iter::empty()).unwrap());
        assert_eq!(batch.archive().unwrap(), batch.archive().unwrap());
    }
}
