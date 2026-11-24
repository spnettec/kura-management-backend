//! Joint, explicitly approved HTTPS/mTLS generations. An original activation
//! intent is never replayed, including after a crash or a lost HTTP response.
use crate::{auth, ca_quorum, client_ca_store, config::Config, settings_files as files, tls_store};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Manifest {
    schema_version: u8,
    id: Uuid,
    tls: Option<tls_store::Generation>,
    client_ca: Option<client_ca_store::Generation>,
    material_digest: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ActivationRequest {
    pub operation_id: Uuid,
    pub expected_generation: Uuid,
    pub pending_revision: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Intent {
    schema_version: u8,
    request: ActivationRequest,
    actor: String,
    target: Manifest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ca_approval: Option<ca_quorum::Evidence>,
}

impl Intent {
    fn valid(&self, id: Uuid) -> bool {
        self.request.operation_id == id && self.target.id == id
            && match (self.schema_version, &self.ca_approval) {
                (1, None) => true,
                (2, Some(proof)) => self.target.client_ca.is_some() && proof.valid(),
                _ => false,
            }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Rejected {
    schema_version: u8,
    request: ActivationRequest,
    actor: String,
}

struct Active {
    manifest: Manifest,
    config: Config,
    web: Arc<rustls::ServerConfig>,
    grpc: Arc<rustls::ServerConfig>,
    web_epoch: CancellationToken,
    grpc_epoch: CancellationToken,
    certificate_fingerprint: String,
    ca_fingerprint: String,
}

pub(crate) struct TlsRuntime {
    base: Config,
    active: RwLock<Active>,
    pub changes: tokio::sync::Mutex<()>,
    degraded: AtomicBool,
    uncertain_receipts: std::sync::Mutex<HashSet<Uuid>>,
    // Exactly one process can publish/serve generations for this state directory.
    _lease: File,
}

fn read_material(path: &str) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?.take(65537).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 65536, "TLS material too large");
    Ok(bytes)
}

fn resolve(base: &Config, manifest: &Manifest) -> Result<Active> {
    ensure!(
        manifest.schema_version == 1,
        "unsupported joint TLS generation"
    );
    let mut config = base.clone();
    if let Some(g) = &manifest.tls {
        tls_store::select(&mut config, g)?;
    }
    if let Some(g) = &manifest.client_ca {
        client_ca_store::select(&mut config, g)?;
    }
    let cert = read_material(&config.tls_cert)?;
    let key = read_material(&config.tls_key)?;
    let ca = read_material(&config.client_ca)?;
    let certificate_fingerprint =
        tls_store::validate_server_identity(std::str::from_utf8(&cert)?, std::str::from_utf8(&key)?, &config.origin)?;
    let ca_fingerprint = client_ca_store::validate(std::str::from_utf8(&ca)?)?;
    let digest = auth::hash(&serde_json::to_vec(&[
        auth::hash(&cert),
        auth::hash(&key),
        auth::hash(&ca),
    ])?);
    ensure!(
        manifest.material_digest.is_empty() || manifest.material_digest == digest,
        "joint TLS material changed"
    );
    let certs = rustls_pemfile::certs(&mut cert.as_slice()).collect::<std::io::Result<Vec<_>>>()?;
    let parse_key =
        || rustls_pemfile::private_key(&mut key.as_slice())?.context("TLS private key required");
    let mut web = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs.clone(), parse_key()?)?;
    web.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    let mut roots = rustls::RootCertStore::empty();
    for certificate in rustls_pemfile::certs(&mut ca.as_slice()) {
        roots.add(certificate?)?;
    }
    let verifier = rustls::server::WebPkiClientVerifier::builder(Arc::new(roots)).build()?;
    let mut grpc = rustls::ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(certs, parse_key()?)?;
    grpc.alpn_protocols = vec![b"h2".to_vec()];
    let mut manifest = manifest.clone();
    manifest.material_digest = digest;
    Ok(Active {
        manifest,
        config,
        web: Arc::new(web),
        grpc: Arc::new(grpc),
        web_epoch: CancellationToken::new(),
        grpc_epoch: CancellationToken::new(),
        certificate_fingerprint,
        ca_fingerprint,
    })
}

/// Observe only the selected immutable generation. No lease, directories,
/// pending selection, operation replay, or pointer publication is performed.
pub(crate) fn readiness_fingerprint(base: &Config) -> Result<String> {
    rustls::crypto::ring::default_provider().install_default().ok();
    tls_store::reject_legacy(base)?;
    client_ca_store::reject_legacy(base)?;
    let pointer=Path::new(&base.state_dir).join("tls-runtime-active.json");
    let manifest=if pointer.try_exists()? {
        serde_json::from_slice(&files::read_private(&pointer)?)?
    } else {
        Manifest{schema_version:1,id:Uuid::nil(),tls:tls_store::selection(base,false)?,
            client_ca:client_ca_store::selection(base,false)?,material_digest:String::new()}
    };
    Ok(resolve(base,&manifest)?.certificate_fingerprint)
}

impl TlsRuntime {
    #[cfg(test)]
    pub fn open(base: Config) -> Result<Arc<Self>> {
        let lease = files::lease(Path::new(&base.state_dir))?;
        Self::open_leased(base, lease)
    }

    pub(crate) fn open_leased(base: Config, lease: File) -> Result<Arc<Self>> {
        rustls::crypto::ring::default_provider()
            .install_default()
            .ok();
        let root = Path::new(&base.state_dir);
        files::directory(root)?;
        files::directory(&root.join("tls-operations"))?;
        // Validate legacy markers even after migration. Pending entries remain
        // staged; starting a process is never approval to activate them.
        tls_store::reject_legacy(&base)?;
        client_ca_store::reject_legacy(&base)?;
        let pointer = root.join("tls-runtime-active.json");
        let manifest = if pointer.try_exists()? {
            serde_json::from_slice(&files::read_private(&pointer)?)?
        } else {
            Manifest {
                schema_version: 1,
                id: Uuid::new_v4(),
                tls: tls_store::selection(&base, false)?,
                client_ca: client_ca_store::selection(&base, false)?,
                material_digest: String::new(),
            }
        };
        let active = resolve(&base, &manifest)?;
        if !pointer.try_exists()? {
            files::publish_new(&pointer, &serde_json::to_vec(&active.manifest)?)?;
        }
        Ok(Arc::new(Self {
            base,
            active: RwLock::new(active),
            changes: tokio::sync::Mutex::new(()),
            degraded: AtomicBool::new(false),
            uncertain_receipts: std::sync::Mutex::new(HashSet::new()),
            _lease: lease,
        }))
    }

    pub fn config(&self) -> Config {
        self.active.read().unwrap().config.clone()
    }

    pub fn healthy(&self) -> bool {
        !self.degraded.load(Ordering::Acquire)
    }

    pub fn transport(
        &self,
        grpc: bool,
    ) -> std::io::Result<(Arc<rustls::ServerConfig>, CancellationToken)> {
        if grpc && !self.healthy() {
            return Err(std::io::Error::other("TLS generation requires review"));
        }
        let active = self.active.read().unwrap();
        Ok(if grpc {
            (active.grpc.clone(), active.grpc_epoch.clone())
        } else {
            (active.web.clone(), active.web_epoch.clone())
        })
    }

    fn pending(
        &self,
    ) -> Result<(
        Option<tls_store::Generation>,
        Option<client_ca_store::Generation>,
        String,
    )> {
        let active = self.active.read().unwrap();
        let tls = tls_store::selection(&self.base, true)?
            .filter(|g| Some(g) != active.manifest.tls.as_ref());
        let ca = client_ca_store::selection(&self.base, true)?
            .filter(|g| Some(g) != active.manifest.client_ca.as_ref());
        let revision = auth::hash(&serde_json::to_vec(&(&tls, &ca))?);
        Ok((tls, ca, revision))
    }

    pub fn status(&self) -> Result<Value> {
        let (tls, ca, revision) = self.pending()?;
        let blocking_operation = self.blocking_operation()?;
        let active = self.active.read().unwrap();
        let tls_fp = |config: &Config| -> Result<String> {
            crate::settings::certificate_fingerprint(std::str::from_utf8(&read_material(
                &config.tls_cert,
            )?)?)
        };
        let ca_fp = |config: &Config| -> Result<String> {
            crate::settings::certificate_fingerprint(&crate::settings::live_client_ca(config)?)
        };
        let mut proposed = active.config.clone();
        if let Some(g) = &tls {
            tls_store::select(&mut proposed, g)?;
        }
        if let Some(g) = &ca {
            client_ca_store::select(&mut proposed, g)?;
        }
        Ok(
            json!({"generation":active.manifest.id,"healthy":self.healthy(),
            "certificateFingerprint":active.certificate_fingerprint,"caFingerprint":active.ca_fingerprint,
            "pendingTlsFingerprint":if tls.is_some(){Some(tls_fp(&proposed)?)}else{None},
            "pendingCaFingerprint":if ca.is_some(){Some(ca_fp(&proposed)?)}else{None},
            "pendingRevision":revision,"activationRequired":tls.is_some() || ca.is_some(),
            "blockingOperationId":blocking_operation,
            "blockingCsrRequestId":crate::server_csr::uncertain_stage(Path::new(&self.base.state_dir))?}),
        )
    }

    // Caller holds changes, including while auditing. Cleanup only consumes the
    // exact generations already named by the durable joint active manifest.
    pub fn prepare_stage(&self) -> Result<Config> {
        crate::server_csr::ensure_no_uncertain_stage(Path::new(&self.base.state_dir))?;
        ensure!(self.healthy(), "TLS generation requires review");
        ensure!(
            self.blocking_operation()?.is_none(),
            "an original TLS activation is unresolved"
        );
        let active = self.active.read().unwrap();
        tls_store::consume(&self.base, &active.manifest.tls)?;
        client_ca_store::consume(&self.base, &active.manifest.client_ca)?;
        Ok(active.config.clone())
    }

    fn operation_path(&self, id: Uuid, kind: &str) -> PathBuf {
        Path::new(&self.base.state_dir)
            .join("tls-operations")
            .join(format!("{id}.{kind}.json"))
    }

    fn blocking_operation(&self) -> Result<Option<Uuid>> {
        let mut ids = Vec::new();
        for entry in std::fs::read_dir(Path::new(&self.base.state_dir).join("tls-operations"))? {
            let name = entry?.file_name();
            if let Some(id) = name.to_str().and_then(|s| s.strip_suffix(".intent.json")) {
                ids.push(Uuid::parse_str(id)?);
            }
        }
        ids.sort();
        for id in ids {
            if self
                .operation(id)?
                .context("activation intent disappeared")?["state"]
                == "UNKNOWN"
            {
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    pub fn operation(&self, id: Uuid) -> Result<Option<Value>> {
        let path = self.operation_path(id, "intent");
        let rejected = self.operation_path(id, "rejected");
        if rejected.try_exists()? {
            ensure!(!path.try_exists()?, "conflicting TLS operation records");
            let recorded: Rejected = serde_json::from_slice(&files::read_private(&rejected)?)?;
            ensure!(
                recorded.schema_version == 1 && recorded.request.operation_id == id,
                "invalid rejected TLS activation record"
            );
            return Ok(Some(
                json!({"operationId":id,"state":"REJECTED","observedActive":false,
                "actor":recorded.actor,"request":recorded.request,"reasonCode":"PRECONDITION_FAILED"}),
            ));
        }
        if !path.try_exists()? {
            return Ok(None);
        }
        let intent: Intent = serde_json::from_slice(&files::read_private(&path)?)?;
        ensure!(intent.valid(id), "invalid TLS activation intent");
        let receipt = self.operation_path(id, "receipt");
        let applied = files::read_private(&receipt)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<ActivationRequest>(&bytes).ok())
            .is_some_and(|recorded| recorded == intent.request)
            && !self.uncertain_receipts.lock().unwrap().contains(&id);
        Ok(Some(
            json!({"operationId":id,"state":if applied{"APPLIED"}else{"UNKNOWN"},
            "targetGeneration":intent.target.id,"actor":intent.actor,
            "observedActive":self.healthy() && self.active.read().unwrap().manifest == intent.target,
            "request":intent.request}),
        ))
    }

    pub fn activation_approval_target(
        &self,
        request: &ActivationRequest,
    ) -> Result<Option<ca_quorum::Target>> {
        let (_, pending_ca, _) = self.pending()?;
        let Some(ca) = pending_ca else {
            return Ok(None);
        };
        ensure!(!request.operation_id.is_nil(), "activation operation id required");
        self.candidate(request)?;
        Ok(Some(ca_quorum::Target::new(request, ca.id)))
    }

    #[cfg(test)]
    pub(crate) fn activate(&self, request: ActivationRequest, actor: &str) -> Result<Value> {
        let grant = self.activation_approval_target(&request).ok().flatten()
            .map(ca_quorum::Authorization::for_fixture);
        self.activate_authorized(request, actor, grant.as_ref())
    }

    pub fn activate_authorized(
        &self,
        request: ActivationRequest,
        actor: &str,
        grant: Option<&ca_quorum::Authorization>,
    ) -> Result<Value> {
        if let Some(record) = self.operation(request.operation_id)? {
            ensure!(
                record["request"] == serde_json::to_value(&request)?,
                "operation id already used"
            );
            return Ok(record); // Observation only; never repeat an existing intent.
        }
        let candidate = match self.candidate(&request) {
            Ok(candidate) => candidate,
            Err(_) => {
                let rejected = Rejected {
                    schema_version: 1,
                    request: request.clone(),
                    actor: actor.into(),
                };
                files::publish_new(
                    &self.operation_path(request.operation_id, "rejected"),
                    &serde_json::to_vec(&rejected)?,
                )?;
                return self
                    .operation(request.operation_id)?
                    .context("rejected operation missing");
            }
        };
        let active_ca = self.active.read().unwrap().manifest.client_ca.clone();
        if candidate.manifest.client_ca != active_ca {
            let ca = candidate.manifest.client_ca.as_ref().context("missing client CA generation")?;
            let target = ca_quorum::Target::new(&request, ca.id);
            ensure!(
                grant.is_some_and(|grant| grant.matches(&target)),
                "client CA activation requires current quorum for this operation"
            );
        }
        let ca_approval = grant.and_then(ca_quorum::Authorization::evidence).cloned();
        let intent = Intent {
            schema_version: if ca_approval.is_some() { 2 } else { 1 },
            request: request.clone(),
            actor: actor.into(),
            target: candidate.manifest.clone(),
            ca_approval,
        };
        if files::publish_new(
            &self.operation_path(request.operation_id, "intent"),
            &serde_json::to_vec(&intent)?,
        )
        .is_err()
        {
            self.degraded.store(true, Ordering::Release);
            self.active.read().unwrap().grpc_epoch.cancel();
            return self
                .operation(request.operation_id)?
                .context("activation intent publication is uncertain");
        }
        let pointer = Path::new(&self.base.state_dir).join("tls-runtime-active.json");
        if files::publish_replace(&pointer, &serde_json::to_vec(&candidate.manifest)?).is_err() {
            // Publication may have happened. Keep the original intent UNKNOWN,
            // stop gateway dispatch and retain HTTPS for authenticated review.
            self.degraded.store(true, Ordering::Release);
            self.active.read().unwrap().grpc_epoch.cancel();
            return self
                .operation(request.operation_id)?
                .context("activation intent missing");
        }
        let previous = std::mem::replace(&mut *self.active.write().unwrap(), candidate);
        // Persist the receipt before closing old connections. A failed receipt
        // remains UNKNOWN even if the new generation is currently observable.
        if files::publish_new(
            &self.operation_path(request.operation_id, "receipt"),
            &serde_json::to_vec(&request)?,
        )
        .is_err()
        {
            // A visible link is not proof that all durability steps succeeded.
            self.uncertain_receipts
                .lock()
                .unwrap()
                .insert(request.operation_id);
        }
        previous.grpc_epoch.cancel();
        previous.web_epoch.cancel();
        self.operation(request.operation_id)?
            .context("activation intent missing")
    }

    fn candidate(&self, request: &ActivationRequest) -> Result<Active> {
        crate::server_csr::ensure_no_uncertain_stage(Path::new(&self.base.state_dir))?;
        ensure!(self.healthy(), "TLS generation requires review");
        ensure!(
            self.blocking_operation()?.is_none(),
            "an original TLS activation is unresolved"
        );
        let (tls, ca, revision) = self.pending()?;
        ensure!(
            (tls.is_some() || ca.is_some()) && revision == request.pending_revision,
            "pending TLS selection changed"
        );
        let active = self.active.read().unwrap();
        ensure!(
            active.manifest.id == request.expected_generation,
            "active TLS generation changed"
        );
        resolve(&self.base, &active.manifest)?; // Reject out-of-band material drift.
        if let Some(g) = &ca {
            client_ca_store::approved_for(
                g,
                &crate::settings::certificate_fingerprint(&crate::settings::live_client_ca(
                    &active.config,
                )?)?,
            )?;
        }
        let target = Manifest {
            schema_version: 1,
            id: request.operation_id,
            tls: tls.or_else(|| active.manifest.tls.clone()),
            client_ca: ca.or_else(|| active.manifest.client_ca.clone()),
            material_digest: String::new(),
        };
        resolve(&self.base, &target) // Both TLS configurations before intent.
    }
}

pub(crate) fn maintenance_records(directory: &Path) -> Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let name = entry?
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("invalid TLS journal name"))?;
        if let Some(value) = name.strip_suffix(".intent.json") {
            let id = Uuid::parse_str(value)?;
            let intent: Intent =
                serde_json::from_slice(&files::read_private(&directory.join(&name))?)?;
            let receipt: ActivationRequest = serde_json::from_slice(
                &files::read_private(&directory.join(format!("{id}.receipt.json")))
                    .context("unsettled TLS activation forbids package maintenance")?,
            )?;
            ensure!(
                intent.valid(id) && receipt == intent.request,
                "invalid TLS receipt forbids package maintenance"
            );
        }
    }
    Ok(())
}

#[test]
fn platform_maintenance_requires_the_exact_tls_receipt_not_just_a_file() {
    for outcome in ["missing", "different", "matching"] {
        let directory = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        let request = ActivationRequest {
            operation_id: id,
            expected_generation: Uuid::new_v4(),
            pending_revision: "test-revision".into(),
        };
        let intent = Intent {
            schema_version: 1,
            request: request.clone(),
            actor: "test-admin".into(),
            target: Manifest {
                schema_version: 1,
                id,
                tls: None,
                client_ca: None,
                material_digest: "test-only-digest".into(),
            },
            ca_approval: None,
        };
        files::publish_new(
            &directory.path().join(format!("{id}.intent.json")),
            &serde_json::to_vec(&intent).unwrap(),
        )
        .unwrap();
        if outcome != "missing" {
            let mut receipt = request;
            if outcome == "different" {
                receipt.pending_revision = "other-revision".into();
            }
            files::publish_new(
                &directory.path().join(format!("{id}.receipt.json")),
                &serde_json::to_vec(&receipt).unwrap(),
            )
            .unwrap();
        }
        assert_eq!(
            maintenance_records(directory.path()).is_ok(),
            outcome == "matching"
        );
    }
}

pub(crate) fn validate_config(config: &Config) -> Result<()> {
    resolve(
        config,
        &Manifest {
            schema_version: 1,
            id: Uuid::nil(),
            tls: None,
            client_ca: None,
            material_digest: String::new(),
        },
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "tls_runtime_tests.rs"]
mod tests;
