//! Cooperating writers hold a shared file lease through durable commit. A
//! maintenance coordinator takes the exclusive lease, verifies settlement, then
//! publishes a durable fence. A crash never silently removes that fence.
use anyhow::{Context,Result,ensure};
use serde_json::{Value,json};
use std::{fs::{self,File,OpenOptions},io::Read,os::{fd::AsRawFd,unix::fs::{MetadataExt,OpenOptionsExt}},path::{Path,PathBuf},sync::Arc};
use uuid::Uuid;

pub(crate) const HOST_DIRECTORY:&str="/var/lib/yofc-console-maintenance";
const PACKAGE_MARKER:&str="/usr/share/kura-management-console/package-v1";
const INSTALLED_BINARY:&str="/usr/bin/yofc-management-console";

#[derive(Clone)]
pub(crate) struct Gate { directory:PathBuf, owner:u32, native:bool }
#[derive(Debug)]
pub(crate) struct Permit { _file:File }

#[derive(Debug)]
pub(crate) enum Blocked { Busy,Frozen }
impl std::fmt::Display for Blocked {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"MAINTENANCE")}
}
impl std::error::Error for Blocked {}

fn check_directory(path:&Path,owner:u32,public:bool)->Result<()> {
    let metadata=fs::symlink_metadata(path)?;
    ensure!(metadata.is_dir() && metadata.uid()==owner && metadata.mode() & if public{0o022}else{0o077}==0,
        "invalid maintenance control directory");
    Ok(())
}

fn trusted_bytes(path:&Path,owner:u32,public:bool)->Result<Vec<u8>> {
    let mut file=OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW|libc::O_CLOEXEC|libc::O_NONBLOCK).open(path)?;
    let metadata=file.metadata()?;
    ensure!(metadata.is_file() && metadata.uid()==owner && metadata.mode() & if public{0o022}else{0o077}==0
        && metadata.len()<=65536,"invalid maintenance control record");
    let mut bytes=Vec::new();file.by_ref().take(65537).read_to_end(&mut bytes)?;
    ensure!(bytes.len()<=65536,"maintenance control record too large");Ok(bytes)
}

fn record_absent(path:&Path)->Result<bool> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind()==std::io::ErrorKind::NotFound=>Ok(true),
        Err(error)=>Err(error.into()),
        Ok(_)=>Ok(false), // A dangling link is an existing unresolved record.
    }
}

impl Gate {
    /// Only the installed native executable joins the root-controlled host
    /// cohort. Source/dev binaries and container images use their private state.
    pub(crate) fn for_state(root:&Path)->Result<Self> {
        let native=match (fs::metadata(std::env::current_exe()?),fs::metadata(INSTALLED_BINARY)) {
            (Ok(current),Ok(installed))=>current.dev()==installed.dev() && current.ino()==installed.ino(),
            _=>false,
        };
        if native {
              let name=root.file_name().and_then(|value|value.to_str()).unwrap_or("");
              ensure!(root.parent()==Some(Path::new("/var/lib")) && (name=="kura-management-console" ||
                  name.strip_prefix("kura-management-console-").is_some_and(|suffix|suffix.as_bytes().first().is_some_and(|v|v.is_ascii_lowercase() || v.is_ascii_digit()) && suffix.len()<=32 && suffix.bytes().all(|v|v.is_ascii_lowercase() || v.is_ascii_digit() || v==b'-'))),
                  "native state must belong to a packaged instance");
            ensure!(trusted_bytes(Path::new(PACKAGE_MARKER),0,true)?==b"kura-management-console-v1\n","invalid native package marker");
            let gate=Self{directory:HOST_DIRECTORY.into(),owner:0,native:true};
            check_directory(&gate.directory,0,true)?;
            return Ok(gate);
        }
        ensure!(root.is_absolute(),"console state directory must be absolute");
        crate::settings_files::directory(root)?;
        let directory=root.join(".lifecycle");crate::settings_files::directory(&directory)?;
        Ok(Self{directory,owner:unsafe{libc::geteuid()},native:false})
    }

    fn open(&self)->Result<File> {
        check_directory(&self.directory,self.owner,self.native)?;
        let mut options=OpenOptions::new();options.read(true).custom_flags(libc::O_NOFOLLOW|libc::O_CLOEXEC|libc::O_NONBLOCK);
        if !self.native { options.write(true).create(true).mode(0o600); }
        let file=options.open(self.directory.join("writers.lock"))?;
        let metadata=file.metadata()?;
        ensure!(metadata.is_file() && metadata.uid()==self.owner && metadata.mode() & if self.native{0o022}else{0o077}==0,
            "invalid maintenance writer lease");
        Ok(file)
    }

    fn lock(&self,exclusive:bool)->Result<File> {
        let file=self.open()?;
        if unsafe{libc::flock(file.as_raw_fd(),(if exclusive{libc::LOCK_EX}else{libc::LOCK_SH})|libc::LOCK_NB)}!=0 {
            let error=std::io::Error::last_os_error();
            if error.kind()==std::io::ErrorKind::WouldBlock{return Err(Blocked::Busy.into());}
            return Err(error.into());
        }
        Ok(file)
    }

    pub(crate) fn status(&self)->Result<Option<Value>> {
        let path=self.directory.join("active.json");
        match fs::symlink_metadata(&path){
            Err(error) if error.kind()==std::io::ErrorKind::NotFound=>return Ok(None),
            Err(error)=>return Err(error.into()),
            Ok(_)=>{},
        }
        let value:Value=serde_json::from_slice(&trusted_bytes(&path,self.owner,self.native)?)?;
        ensure!(value["schemaVersion"]==1 && value["phase"]=="FROZEN" &&
            Uuid::parse_str(value["operationId"].as_str().context("maintenance id missing")?).is_ok_and(|id|!id.is_nil()),
            "invalid maintenance fence; retain original record");
        Ok(Some(value))
    }

    pub(crate) fn enter(&self)->Result<Permit> {
        let file=self.lock(false)?;
        if self.status()?.is_some(){return Err(Blocked::Frozen.into());}
        Ok(Permit{_file:file})
    }

    /// An explicitly selected container generation may initialize its runtime
    /// under the original private fence. Only startup owns this exclusive lease;
    /// HTTP, gRPC and ordinary CLI writers still use enter() and remain blocked.
    /// The authenticated host verifies the running generation before completing
    /// that same maintenance. This method never removes or completes a fence.
    pub(crate) fn startup_standalone(&self,id:Uuid)->Result<Permit> {
        ensure!(!self.native,"native startup cannot bypass host maintenance");
        ensure!(!id.is_nil(),"maintenance id must not be nil");
        let exclusive=self.lock(true)?;
        if let Some(active)=self.status()? {
            ensure!(active["operationId"]==id.to_string() && active["scope"]=="installation",
                "startup must match the original installation fence");
            ensure!(record_absent(&self.directory.join(format!("{id}.complete.json")))?,
                "completed maintenance retains a fence; explicit review required");
            return Ok(Permit{_file:exclusive});
        }
        // A later ordinary daemon/host boot of this immutable container command
        // is allowed only after the matching maintenance was durably completed.
        let value:Value=serde_json::from_slice(&trusted_bytes(
            &self.directory.join(format!("{id}.complete.json")),self.owner,false)?)?;
        ensure!(value["schemaVersion"]==1 && value["operationId"]==id.to_string()
            && value["state"]=="APPLIED" && value["scope"]=="installation",
            "startup requires its original completed maintenance receipt");
        drop(exclusive);
        self.enter() // Recheck any competing fence after releasing EX.
    }

    /// A consumed gRPC receipt waits for a temporary exclusive check. It is not
    /// discarded just because the coordinator has not yet decided to freeze.
    pub(crate) async fn wait_enter(&self)->Result<Permit> {
        loop {
            match self.enter(){
                Ok(permit)=>return Ok(permit),
                Err(error) if matches!(error.downcast_ref::<Blocked>(),Some(Blocked::Busy))=>
                    tokio::time::sleep(std::time::Duration::from_millis(25)).await,
                Err(error)=>return Err(error),
            }
        }
    }

    pub(crate) async fn freeze_standalone(&self,root:&Path,id:Uuid)->Result<Value> {
        ensure!(!self.native,"native maintenance requires the authenticated host installer");
        ensure!(!id.is_nil(),"maintenance id must not be nil");
        let _exclusive=self.lock(true)?;
        if let Some(active)=self.status()? {
            ensure!(active["operationId"]==id.to_string(),"another original maintenance operation is active");
            return Ok(active); // Observe only; never repeat a stop or any application write.
        }
        ensure!(record_absent(&self.directory.join(format!("{id}.complete.json")))?,"completed maintenance id cannot be reused");
        crate::initial_setup::maintenance_check(root).await?;
        let active=json!({"schemaVersion":1,"operationId":id,"phase":"FROZEN","scope":"installation"});
        crate::settings_files::publish_new(&self.directory.join("active.json"),&serde_json::to_vec(&active)?)?;
        Ok(active)
    }

    pub(crate) async fn complete_standalone(&self,root:&Path,id:Uuid)->Result<Value> {
        ensure!(!self.native,"native maintenance requires the authenticated host installer");
        let _exclusive=self.lock(true)?;
        let receipt=self.directory.join(format!("{id}.complete.json"));
        let active=self.status()?;
        if active.is_none(){
            let value:Value=serde_json::from_slice(&trusted_bytes(&receipt,self.owner,false)?)?;
            ensure!(value["schemaVersion"]==1 && value["operationId"]==id.to_string() && value["state"]=="APPLIED","original maintenance receipt mismatch");
            return Ok(value);
        }
        ensure!(active.as_ref().unwrap()["operationId"]==id.to_string(),"maintenance id does not match original fence");
        crate::initial_setup::maintenance_check(root).await?;
        let result=json!({"schemaVersion":1,"operationId":id,"state":"APPLIED","scope":"installation"});
        if receipt.try_exists()?{ensure!(serde_json::from_slice::<Value>(&trusted_bytes(&receipt,self.owner,false)?)?==result,"maintenance receipt mismatch");}
        else{crate::settings_files::publish_new(&receipt,&serde_json::to_vec(&result)?)?;}
        fs::remove_file(self.directory.join("active.json"))?;
        File::open(&self.directory)?.sync_all()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    fn fixture()->(tempfile::TempDir,Gate){
        let dir=tempfile::tempdir().unwrap();fs::set_permissions(dir.path(),fs::Permissions::from_mode(0o700)).unwrap();
        let gate=Gate::for_state(dir.path()).unwrap();(dir,gate)
    }
    #[tokio::test]
    async fn freeze_excludes_active_writers_persists_and_requires_original_completion(){
        let (dir,gate)=fixture();let id=Uuid::new_v4();
        let permit=gate.enter().unwrap();
        assert!(matches!(gate.freeze_standalone(dir.path(),id).await.unwrap_err().downcast_ref::<Blocked>(),Some(Blocked::Busy)));
        assert!(gate.status().unwrap().is_none());drop(permit);
        gate.freeze_standalone(dir.path(),id).await.unwrap();
        let reopened=Gate::for_state(dir.path()).unwrap();
        assert!(matches!(reopened.enter().unwrap_err().downcast_ref::<Blocked>(),Some(Blocked::Frozen)));
        assert!(reopened.complete_standalone(dir.path(),Uuid::new_v4()).await.is_err());
        assert!(reopened.status().unwrap().is_some());
        reopened.complete_standalone(dir.path(),id).await.unwrap();
        assert!(reopened.enter().is_ok());
        assert_eq!(reopened.complete_standalone(dir.path(),id).await.unwrap()["state"],"APPLIED");
        assert!(reopened.freeze_standalone(dir.path(),id).await.is_err());
    }
    #[tokio::test]
    async fn unsettled_initialization_refuses_freeze_and_retains_original_bytes(){
        let (dir,gate)=fixture();let operations=dir.path().join("setup-operations");
        crate::settings_files::directory(&operations).unwrap();
        let path=operations.join(format!("{}.accepted.json",Uuid::new_v4()));
        crate::settings_files::write_new(&path,b"{}").unwrap();
        assert!(gate.freeze_standalone(dir.path(),Uuid::new_v4()).await.is_err());
        assert_eq!(fs::read(path).unwrap(),b"{}");assert!(gate.status().unwrap().is_none());
    }
    #[tokio::test]
    async fn consumed_receipt_waits_for_busy_check_without_bypassing_fence(){
        let (_dir,gate)=fixture();let exclusive=gate.lock(true).unwrap();
        assert!(tokio::time::timeout(std::time::Duration::from_millis(60),gate.wait_enter()).await.is_err());
        drop(exclusive);assert!(gate.wait_enter().await.is_ok());
    }
    #[test]
    fn malformed_dangling_and_writable_fences_fail_closed(){
        let (_dir,gate)=fixture();let path=gate.directory.join("active.json");
        std::os::unix::fs::symlink("missing",&path).unwrap();assert!(gate.enter().is_err());fs::remove_file(&path).unwrap();
        crate::settings_files::write_new(&path,b"{}").unwrap();assert!(gate.enter().is_err());
        fs::set_permissions(&path,fs::Permissions::from_mode(0o666)).unwrap();assert!(gate.enter().is_err());
    }
    #[tokio::test]
    async fn fenced_startup_excludes_writers_and_completion_without_releasing_original_fence(){
        let (dir,gate)=fixture();let id=Uuid::new_v4();gate.freeze_standalone(dir.path(),id).await.unwrap();
        let path=gate.directory.join("active.json");let before=fs::read(&path).unwrap();
        assert!(gate.startup_standalone(Uuid::new_v4()).is_err());
        let startup=gate.startup_standalone(id).unwrap();
        assert!(matches!(gate.enter().unwrap_err().downcast_ref::<Blocked>(),Some(Blocked::Busy)));
        assert!(matches!(gate.complete_standalone(dir.path(),id).await.unwrap_err().downcast_ref::<Blocked>(),Some(Blocked::Busy)));
        assert_eq!(fs::read(&path).unwrap(),before);drop(startup);
        assert!(matches!(gate.enter().unwrap_err().downcast_ref::<Blocked>(),Some(Blocked::Frozen)));
        assert_eq!(fs::read(&path).unwrap(),before);
        gate.complete_standalone(dir.path(),id).await.unwrap();assert!(gate.enter().is_ok());
    }
    #[tokio::test]
    async fn later_container_boot_requires_exact_completed_receipt_and_respects_a_new_fence(){
        let (dir,gate)=fixture();let id=Uuid::new_v4();assert!(gate.startup_standalone(id).is_err());
        gate.freeze_standalone(dir.path(),id).await.unwrap();gate.complete_standalone(dir.path(),id).await.unwrap();
        let startup=gate.startup_standalone(id).unwrap();let next=Uuid::new_v4();
        assert!(matches!(gate.freeze_standalone(dir.path(),next).await.unwrap_err().downcast_ref::<Blocked>(),Some(Blocked::Busy)));
        drop(startup);gate.freeze_standalone(dir.path(),next).await.unwrap();
        assert!(gate.startup_standalone(id).is_err());assert!(gate.status().unwrap().is_some());
    }
    #[tokio::test]
    async fn startup_never_overrides_native_nil_or_partially_completed_fences(){
        let (dir,gate)=fixture();let id=Uuid::new_v4();assert!(gate.startup_standalone(Uuid::nil()).is_err());
        gate.freeze_standalone(dir.path(),id).await.unwrap();
        let native=Gate{native:true,..gate.clone()};assert!(native.startup_standalone(id).is_err());
        let receipt=gate.directory.join(format!("{id}.complete.json"));
        std::os::unix::fs::symlink("missing",&receipt).unwrap();assert!(gate.startup_standalone(id).is_err());fs::remove_file(receipt).unwrap();
        crate::settings_files::publish_new(&gate.directory.join(format!("{id}.complete.json")),
            &serde_json::to_vec(&json!({"schemaVersion":1,"operationId":id,"state":"APPLIED","scope":"installation"})).unwrap()).unwrap();
        assert!(gate.startup_standalone(id).is_err());assert!(gate.status().unwrap().is_some());
    }
}

pub(crate) async fn admit(
    axum::extract::State(gate):axum::extract::State<Arc<Gate>>,
    request:axum::extract::Request,next:axum::middleware::Next,
)->axum::response::Response {
    use axum::response::IntoResponse;
    if !request.uri().path().starts_with("/api/") || request.uri().path()=="/api/setup/status" {
        return next.run(request).await;
    }
    let _permit=match gate.enter(){Ok(value)=>value,Err(_)=>return (
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        axum::Json(json!({"error":{"code":"MAINTENANCE","accepted":false}})),
    ).into_response()};
    next.run(request).await
}
