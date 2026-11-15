//! Durable recovery slots are written before changing the database binding.
//! Startup selects by the authenticated DB key-check, never by file order or
//! a blind "pending" rename. Old slots remain available for retained backups.
use crate::task_crypto::TaskCipher;
use anyhow::{Context, Result, ensure};
use sqlx::PgPool;
use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};

fn private_directory(path: &Path) -> Result<()> {
    match fs::create_dir(path) {
        Ok(()) => {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e.into()),
    }
    let m = fs::symlink_metadata(path)?;
    ensure!(
        m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
        "key recovery directory must be owned and private"
    );
    fs::File::open(path.parent().context("recovery directory needs a parent")?)?.sync_all()?;
    Ok(())
}

fn slots(state: &Path) -> PathBuf {
    state.join("pending").join("keys")
}

pub fn retain(state: &Path, cipher: &TaskCipher) -> Result<PathBuf> {
    private_directory(state)?;
    private_directory(&state.join("pending"))?;
    let directory = slots(state);
    private_directory(&directory)?;
    let target = directory.join(format!("{}.key", cipher.fingerprint()));
    if target.exists() {
        ensure!(
            TaskCipher::load(&target)?.fingerprint() == cipher.fingerprint(),
            "invalid recovery slot"
        );
        fs::File::open(&target)?.sync_all()?;
        fs::File::open(&directory)?.sync_all()?;
        return Ok(target);
    }
    let temporary = directory.join(format!(".new-{}", uuid::Uuid::new_v4()));
    cipher.persist_new(&temporary)?;
    fs::rename(&temporary, &target)?;
    fs::File::open(&directory)?.sync_all()?;
    Ok(target)
}

/// Both sides of a possible DB commit are durable before the first DB update.
pub fn prepare(state: &Path, old: &TaskCipher, new: &TaskCipher) -> Result<PathBuf> {
    // A staged restore must be explicitly applied/reviewed before a rotation.
    ensure!(
        !state.join("pending/task-data.key").exists(),
        "task key restore pending"
    );
    retain(state, old)?;
    retain(state, new)
}

pub async fn recover(pool: &PgPool, active: &Path, state: &Path) -> Result<TaskCipher> {
    let bound: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM mgmt_task_crypto WHERE singleton)")
            .fetch_one(pool)
            .await?;
    if !bound {
        return TaskCipher::load(active);
    }
    let mut candidates = Vec::new();
    if let Ok(cipher) = TaskCipher::load(active) {
        candidates.push(cipher);
    }
    let directory = slots(state);
    if directory.exists() {
        let m = fs::symlink_metadata(&directory)?;
        ensure!(
            m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
            "key recovery directory must be owned and private"
        );
        for entry in fs::read_dir(&directory)? {
            let path = entry?.path();
            if path.extension().is_some_and(|s| s == "key") {
                let cipher = TaskCipher::load(&path)?;
                ensure!(
                    path.file_stem()
                        .is_some_and(|s| s == cipher.fingerprint().as_str()),
                    "recovery slot fingerprint mismatch"
                );
                candidates.push(cipher);
            }
        }
    }
    let restore = state.join("pending/task-data.key");
    if restore.exists() {
        let cipher = TaskCipher::load(&restore)?;
        ensure!(
            cipher.matches_storage(pool).await?,
            "restored key does not match the current database; active key preserved"
        );
        candidates.push(cipher);
    }
    let mut selected: Option<TaskCipher> = None;
    for cipher in candidates {
        if cipher.matches_storage(pool).await? {
            ensure!(
                selected
                    .as_ref()
                    .is_none_or(|c| c.fingerprint() == cipher.fingerprint()),
                "ambiguous database key binding"
            );
            selected = Some(cipher);
        }
    }
    let selected = selected.context(
        "no retained key matches database; preserve state and restore the matching backup",
    )?;
    // Recovery need not overwrite the configured key. Keeping it and all slots
    // makes either DB backup generation recoverable; runtime always uses the
    // uniquely authenticated selected key. Backups use that runtime key too.
    if restore.exists() {
        retain(state, &selected)?;
        fs::rename(
            &restore,
            state.join(format!("restored-{}.key", selected.fingerprint())),
        )?;
        fs::File::open(state)?.sync_all()?;
        fs::File::open(restore.parent().unwrap())?.sync_all()?;
    }
    Ok(selected)
}
