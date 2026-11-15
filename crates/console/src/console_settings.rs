//! Durable installation settings. Environment variables remain explicit advanced
//! overrides; a configured installation never falls back to admin/admin.
use crate::{auth, config::Config, settings_files as files};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SavedSettings {
    pub schema_version: u8,
    pub installation_id: Uuid,
    pub operation_id: Uuid,
    pub database_url: String,
    pub data_key_file: String,
    pub config: Config,
    pub material_sha256: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Selection {
    pub schema_version: u8,
    pub operation_id: Uuid,
    pub digest: String,
}

pub(crate) struct Loaded {
    pub settings: SavedSettings,
    pub selection: Selection,
}

pub(crate) fn state_dir() -> Result<PathBuf> {
    Ok(crate::config::setting("CONSOLE_STATE_DIR")?
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/var/lib/kura-management-console")))
}

pub(crate) fn generation(root: &Path, operation: Uuid) -> PathBuf {
    root.join("installation-generations")
        .join(operation.to_string())
}

pub(crate) fn load_at(root: &Path) -> Result<Option<Loaded>> {
    let active = root.join("console-config-active.json");
    if !active.try_exists()? {
        return Ok(None);
    }
    let selection: Selection = serde_json::from_slice(&files::read_private(&active)?)?;
    ensure!(
        selection.schema_version == 1,
        "unsupported console configuration schema"
    );
    let directory = generation(root, selection.operation_id);
    for path in [root, &root.join("installation-generations"), &directory] {
        use std::os::unix::fs::MetadataExt;
        let meta = fs::symlink_metadata(path)?;
        ensure!(
            meta.is_dir() && meta.uid() == unsafe { libc::geteuid() } && meta.mode() & 0o077 == 0,
            "invalid private installation directory"
        );
    }
    let bytes = files::read_private(&directory.join("settings.json"))?;
    ensure!(
        auth::hash(&bytes) == selection.digest,
        "console configuration digest mismatch"
    );
    let settings: SavedSettings = serde_json::from_slice(&bytes)?;
    ensure!(
        settings.schema_version == 1
            && settings.operation_id == selection.operation_id
            && Path::new(&settings.config.state_dir) == root,
        "invalid console configuration identity"
    );
    let paths = [
        (&settings.config.tls_cert, "server.pem"),
        (&settings.config.tls_key, "server.key"),
        (&settings.config.client_ca, "client-ca.pem"),
        (&settings.data_key_file, "task-data.key"),
    ];
    ensure!(
        settings.material_sha256.len() == paths.len(),
        "incomplete console material pins"
    );
    for ((path, name), expected) in paths.into_iter().zip(&settings.material_sha256) {
        ensure!(
            Path::new(path) == directory.join(name),
            "invalid installation material path"
        );
        ensure!(
            auth::hash(&files::read_private(Path::new(path))?) == *expected,
            "installation material changed"
        );
    }
    Ok(Some(Loaded {
        settings,
        selection,
    }))
}

pub(crate) fn load() -> Result<Option<Loaded>> {
    load_at(&state_dir()?)
}

pub(crate) fn connection() -> Result<(String, String, Option<Loaded>)> {
    let saved = load()?;
    ensure_legacy_allowed(&state_dir()?, saved.is_some())?;
    let database = crate::config::setting("DATABASE_URL")?
        .or_else(|| saved.as_ref().map(|s| s.settings.database_url.clone()))
        .context("database is not configured; complete the setup wizard")?;
    let key = crate::config::setting("CONSOLE_DATA_KEY_FILE")?
        .or_else(|| saved.as_ref().map(|s| s.settings.data_key_file.clone()))
        .context("data key is not configured; complete the setup wizard")?;
    Ok((database, key, saved))
}

pub(crate) fn ensure_legacy_allowed(root: &Path, configured: bool) -> Result<()> {
    if configured {
        return Ok(());
    }
    let history = root.join("setup-operations");
    if !history.try_exists()? {
        return Ok(());
    }
    use std::os::unix::fs::MetadataExt;
    let meta = fs::symlink_metadata(&history)?;
    ensure!(
        meta.is_dir() && meta.uid() == unsafe { libc::geteuid() } && meta.mode() & 0o077 == 0,
        "invalid private installation history"
    );
    for entry in fs::read_dir(history)? {
        let name = entry?.file_name();
        if name.to_str().is_some_and(|v| {
            [".accepted.json", ".intent.json", ".receipt.json"]
                .iter()
                .any(|suffix| v.ends_with(suffix))
        }) {
            anyhow::bail!(
                "an existing UI initialization record forbids legacy fallback; preserve and query the original operation"
            );
        }
    }
    Ok(())
}

pub(crate) async fn verify_database(pool: &sqlx::PgPool, saved: &Loaded) -> Result<()> {
    let matches: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_console_installation WHERE singleton AND installation_id=$1 AND setup_operation_id=$2 AND config_digest=$3)")
        .bind(saved.settings.installation_id).bind(saved.settings.operation_id).bind(&saved.selection.digest)
        .fetch_one(pool).await?;
    ensure!(
        matches,
        "configured installation is bound to a different database"
    );
    let admins: i64 =
        sqlx::query_scalar("SELECT count(*) FROM mgmt_users WHERE enabled AND role='admin'")
            .fetch_one(pool)
            .await?;
    ensure!(
        admins > 0,
        "configured installation has no administrator; explicit recovery required"
    );
    Ok(())
}

pub(crate) async fn connect_database(url: &str, saved: Option<&Loaded>) -> Result<sqlx::PgPool> {
    let Some(saved) = saved else {
        return crate::db::connect(url).await;
    };
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(12)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(url)
        .await?;
    // An advanced URL override must prove the existing installation identity
    // before any DDL or key binding touches the selected database.
    verify_database(&pool, saved).await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;
    Ok(pool)
}
