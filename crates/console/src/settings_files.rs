//! Private durable files shared by credential generation stores.
use anyhow::{Context, Result, ensure};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::Path,
};
pub(crate) fn directory(path: &Path) -> Result<()> {
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
        "TLS state directory must be owned and private"
    );
    sync_parent(path)
}
pub(crate) fn sync_parent(path: &Path) -> Result<()> {
    File::open(path.parent().context("TLS state needs a parent")?)?.sync_all()?;
    Ok(())
}
pub(crate) fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
pub(crate) fn read_private(path: &Path) -> Result<Vec<u8>> {
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC)
        .open(path)?;
    let m = file.metadata()?;
    ensure!(
        m.is_file()
            && m.uid() == unsafe { libc::geteuid() }
            && m.mode() & 0o077 == 0
            && m.len() <= 65536,
        "invalid TLS state file"
    );
    let mut bytes = Vec::new();
    Read::by_ref(&mut file)
        .take(65537)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 65536, "TLS state file too large");
    Ok(bytes)
}

pub(crate) fn lease(state: &Path) -> Result<File> {
    directory(state)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
        .open(state.join("tls-runtime.lock"))?;
    let meta = file.metadata()?;
    ensure!(
        meta.is_file() && meta.uid() == unsafe { libc::geteuid() } && meta.mode() & 0o077 == 0,
        "invalid console state lease"
    );
    ensure!(
        unsafe {
            libc::flock(
                std::os::fd::AsRawFd::as_raw_fd(&file),
                libc::LOCK_EX | libc::LOCK_NB,
            )
        } == 0,
        "console state is already in use"
    );
    Ok(file)
}

/// Publish only complete bytes, without replacing an existing pending action.
pub(crate) fn publish_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_file_name(format!(".settings-new-{}", uuid::Uuid::new_v4()));
    write_new(&temporary, bytes)?;
    fs::hard_link(&temporary, path)?;
    sync_parent(path)?;
    fs::remove_file(&temporary)?;
    sync_parent(&temporary)
}

pub(crate) fn publish_replace(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_file_name(format!(".settings-active-{}", uuid::Uuid::new_v4()));
    write_new(&temporary, bytes)?;
    fs::rename(&temporary, path)?;
    sync_parent(path)
}
