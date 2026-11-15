//! The CLI's complete admission contract runs before state creation, migration,
//! key recovery or admin creation. Business authorization still runs in its DB
//! transaction. Errors never echo private argument values.
use anyhow::{Result,ensure};
use std::{fs,path::Path};
use uuid::Uuid;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Kind{Serve,ServeMaintenance,ReadOnly,Maintenance,Offline,Database}

fn identifier(value:&str)->Result<()>{ensure!(crate::transfer::valid_identifier(value),"invalid identifier");Ok(())}
fn uuid(value:&str)->Result<()>{ensure!(Uuid::parse_str(value).is_ok_and(|id|!id.is_nil()),"expected non-nil UUID");Ok(())}
fn generation(value:&str)->Result<()>{ensure!(value.parse::<i64>().is_ok_and(|n|n>=1 && n<i64::MAX),"expected positive generation");Ok(())}
fn validity(value:&str)->Result<()>{ensure!(value.parse::<u64>().is_ok_and(|n|(1..=900).contains(&n)),"validity must be 1..=900 seconds");Ok(())}
fn endpoint(value:&str)->Result<()>{ensure!(value.starts_with("https://") && value.len()<=512 && !value.contains(['\r','\n','\0']),"invalid HTTPS endpoint");Ok(())}
fn output(value:&str)->Result<()>{
    ensure!(!value.is_empty(),"output path required");
    match fs::symlink_metadata(value){Err(error) if error.kind()==std::io::ErrorKind::NotFound=>Ok(()),Err(error)=>Err(error.into()),Ok(_)=>anyhow::bail!("output must be a new file")}
}

pub(crate) fn validate(a:&[String])->Result<Kind>{
    let name=a.get(1).map(String::as_str).unwrap_or("serve");
    let (arity,kind)=match name{
        "serve"=>(2,Kind::Serve),
        "serve-maintenance"=>(3,Kind::ServeMaintenance),
        "installation-status"|"installation-credential"|"installation-maintenance-check"=>(2,Kind::ReadOnly),
        "maintenance-status"=>(2,Kind::Maintenance),
        "maintenance-begin"|"maintenance-complete"=>(3,Kind::Maintenance),
        "generate-transfer-key"=>(4,Kind::Offline),
        "issue-transfer-signer-rotation"=>(10,Kind::Offline),
        "task-key-init"=>(3,Kind::Offline),
        "task-key-backup"|"task-key-restore"=>(5,Kind::Offline),
        "task-key-rotate"=>(4,Kind::Database),
        "migrate"=>(2,Kind::Database),
        "create-user"|"register-transfer-signer"|"register-gateway"=>(6,Kind::Database),
        "issue-gateway-transfer"=>(15,Kind::Database),
        "request-gateway-transfer-approval"=>(9,Kind::Database),
        "approve-gateway-transfer-approval"|"reject-gateway-transfer-approval"|"revoke-gateway-transfer-approval"|
        "revoke-transfer-signer"|"revoke-transfer-issuance"|"set-gateway-enabled"|
        "rotate-gateway-certificate"|"set-gateway-binding-generation"=>(4,Kind::Database),
        _=>anyhow::bail!("unknown subcommand"),
    };
    ensure!(a.len()==arity || name=="serve" && a.len()==1 || name=="rotate-gateway-certificate" && a.len()==5,"invalid command argument count");
    match name{
        "maintenance-begin"|"maintenance-complete"|"serve-maintenance"=>uuid(&a[2])?,
        "generate-transfer-key"=>{output(&a[2])?;output(&a[3])?;ensure!(a[2]!=a[3],"distinct output paths required");},
        "task-key-init"=>{ensure!(Path::new(&a[2]).is_absolute(),"absolute task key path required");output(&a[2])?;},
        "task-key-backup"=>{crate::task_crypto::TaskCipher::load(Path::new(&a[2]))?;output(&a[3])?;fs::read(&a[4])?;},
        "task-key-restore"=>{fs::read(&a[2])?;fs::read(&a[4])?;output(&a[3])?;},
        "task-key-rotate"=>{
            let old=crate::task_crypto::TaskCipher::load(Path::new(&a[2]))?;
            let new=crate::task_crypto::TaskCipher::load(Path::new(&a[3]))?;
            ensure!(fs::read(&a[2])?!=fs::read(&a[3])?,"new task key must differ");drop((old,new));
        },
        "create-user"=>{
            ensure!(!a[2].is_empty() && a[2].len()<=128 && !a[3].is_empty(),"invalid identity");
            ensure!(["admin","operator","viewer","recovery"].contains(&a[4].as_str()),"invalid role");
            let password=fs::read_to_string(&a[5])?;
            crate::auth::password_hash(password.trim_end_matches(['\r','\n']))?;
        },
        "issue-transfer-signer-rotation"=>{
            identifier(&a[2])?;generation(&a[3])?;validity(&a[6])?;
            ensure!(!a[5].is_empty() && a[5].len()<=128 && !a[5].contains(['\r','\n','\0']),"invalid operator");
            let key=crate::transfer_key_pair(&a[7])?;
            let public=crate::transfer::ed25519_public_from_pem_path(&a[8])?;
            let new=crate::transfer::ed25519_public_from_pem_path(&a[4])?;
            ensure!(crate::transfer::ed25519_public_raw(&key)?==public && public!=new,"invalid signer rotation");output(&a[9])?;
        },
        "issue-gateway-transfer"=>{
            uuid(&a[2])?;uuid(&a[3])?;identifier(&a[4])?;generation(&a[5])?;endpoint(&a[6])?;identifier(&a[7])?;
            crate::certificate_hash(&a[8])?;crate::certificate_hash(&a[9])?;validity(&a[10])?;identifier(&a[11])?;
            let key=crate::transfer_key_pair(&a[12])?;
            ensure!(crate::auth::hash(crate::transfer::ed25519_public_raw(&key)?)==crate::signer_sha256(&a[13])?,"signer key mismatch");output(&a[14])?;
        },
        "request-gateway-transfer-approval"=>{
            identifier(&a[2])?;generation(&a[3])?;endpoint(&a[4])?;identifier(&a[5])?;
            crate::certificate_hash(&a[6])?;crate::certificate_hash(&a[7])?;uuid(&a[8])?;
        },
        "approve-gateway-transfer-approval"|"reject-gateway-transfer-approval"|"revoke-gateway-transfer-approval"|"revoke-transfer-issuance"=>{uuid(&a[2])?;uuid(&a[3])?;},
        "register-transfer-signer"=>{identifier(&a[2])?;crate::transfer::ed25519_public_from_pem_path(&a[3])?;ensure!(!a[4].is_empty(),"tenant required");uuid(&a[5])?;},
        "revoke-transfer-signer"=>{identifier(&a[2])?;uuid(&a[3])?;},
        "register-gateway"=>{crate::gateway_registration::validate(&a[2],&a[3],&a[4])?;crate::gateway_registration::certificate_sha256(&fs::read_to_string(&a[5])?)?;},
        "set-gateway-enabled"=>{identifier(&a[2])?;ensure!(a[3].parse::<bool>().is_ok(),"expected true or false");},
        "rotate-gateway-certificate"=>{identifier(&a[2])?;crate::certificate_hash(&a[3])?;if let Some(value)=a.get(4){generation(value)?;}},
        "set-gateway-binding-generation"=>{identifier(&a[2])?;generation(&a[3])?;},
        _=>{},
    }
    Ok(kind)
}

#[cfg(test)]
mod tests{
    use super::*;
    fn args(values:&[&str])->Vec<String>{std::iter::once("console").chain(values.iter().copied()).map(str::to_owned).collect()}
    #[test]fn certificate_generation_rotation_is_explicit_and_optional(){
        let path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registration-client.pem");
        let path=path.to_str().unwrap();
        assert_eq!(validate(&args(&["rotate-gateway-certificate","gateway",path])).unwrap(),Kind::Database);
        assert_eq!(validate(&args(&["rotate-gateway-certificate","gateway",path,"7"])).unwrap(),Kind::Database);
        for value in ["0","-1","invalid","9223372036854775808"]{assert!(validate(&args(&["rotate-gateway-certificate","gateway",path,value])).is_err());}
        assert!(validate(&args(&["rotate-gateway-certificate","gateway",path,"7","extra"])).is_err());
    }
    #[test]fn invalid_commands_fail_before_any_installation_or_database_access(){
        for values in [vec!["unknown"],vec!["migrate","unexpected"],vec!["serve","unexpected"],
            vec!["create-user"],vec!["set-gateway-enabled","gateway","invalid"],
            vec!["set-gateway-binding-generation","gateway","0"],vec!["maintenance-begin","nil"],
            vec!["serve-maintenance"],vec!["serve-maintenance","00000000-0000-0000-0000-000000000000"],
            vec!["serve-maintenance","08271197-47a6-4b3e-a123-853925071244","extra"],
            vec!["revoke-transfer-issuance","invalid","invalid"]]{assert!(validate(&args(&values)).is_err());}
        assert_eq!(validate(&args(&[])).unwrap(),Kind::Serve);
        assert_eq!(validate(&args(&["installation-status"])).unwrap(),Kind::ReadOnly);
        assert_eq!(validate(&args(&["serve-maintenance","08271197-47a6-4b3e-a123-853925071244"])).unwrap(),Kind::ServeMaintenance);
    }
}
