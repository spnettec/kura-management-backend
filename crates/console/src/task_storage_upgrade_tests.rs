//! Staged PostgreSQL backup/restore acceptance. The harness restores an actual
//! pg_dump between these ignored tests; no running installation is selected.
use crate::{
    auth, db,
    task_crypto::{self, TaskCipher},
    task_key_store,
};
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use std::{
    borrow::Cow,
    fs,
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};
use uuid::Uuid;

const COUNT: i64 = 103;
const HISTORY: &str = "SELECT jsonb_agg(to_jsonb(t)-'input'-'result'-'binding_generation' ORDER BY id) FROM mgmt_tasks t";
const PAYLOADS: &str = "SELECT jsonb_agg(jsonb_build_object('id',id,'input',input,'result',result) ORDER BY id) FROM mgmt_tasks";

fn directory() -> PathBuf {
    let path = PathBuf::from(
        std::env::var("YOFC_STORAGE_UPGRADE_FIXTURE")
            .expect("explicit private fixture directory required"),
    );
    let metadata = fs::symlink_metadata(&path).unwrap();
    assert!(
        metadata.is_dir()
            && metadata.uid() == unsafe { libc::geteuid() }
            && metadata.mode() & 0o077 == 0
    );
    path
}

fn private_new(path: &Path, bytes: &[u8]) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
    fs::File::open(path.parent().unwrap())
        .unwrap()
        .sync_all()
        .unwrap();
}

fn save(path: &Path, value: &Value) {
    private_new(path, &serde_json::to_vec(value).unwrap());
}
fn load(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn protected(path: &Path) -> Value {
    let metadata = fs::symlink_metadata(path).unwrap();
    json!({"sha256":auth::hash(&fs::read(path).unwrap()),"uid":metadata.uid(),"gid":metadata.gid(),
        "mode":metadata.mode(),"inode":metadata.ino(),"size":metadata.size(),"mtime":metadata.mtime(),
        "mtimeNsec":metadata.mtime_nsec(),"ctime":metadata.ctime(),"ctimeNsec":metadata.ctime_nsec()})
}

async fn pool() -> PgPool {
    PgPool::connect(&std::env::var("DATABASE_URL").expect("explicit isolated database required"))
        .await
        .unwrap()
}

async fn baseline(pool: &PgPool) -> Value {
    let history: Value = sqlx::query_scalar(HISTORY).fetch_one(pool).await.unwrap();
    let payloads: Value = sqlx::query_scalar(PAYLOADS).fetch_one(pool).await.unwrap();
    let audit: Value =
        sqlx::query_scalar("SELECT jsonb_agg(to_jsonb(a) ORDER BY id) FROM mgmt_audit a")
            .fetch_one(pool)
            .await
            .unwrap();
    let binding: Value = sqlx::query_scalar("SELECT to_jsonb(c) FROM mgmt_task_crypto c")
        .fetch_one(pool)
        .await
        .unwrap();
    json!({"history":history,"payloads":payloads,"audit":audit,"binding":binding})
}

async fn assert_clear(pool: &PgPool, cipher: &TaskCipher) {
    let rows = sqlx::query(
        "SELECT id,tenant_id,gateway_id,operation,input,result FROM mgmt_tasks ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(rows.len() as i64, COUNT);
    for row in rows {
        let n = row.get::<Uuid, _>("id").as_u128();
        assert_eq!(
            cipher.open_row(&row, "input").unwrap(),
            json!({"private":"legacy-input-fixture","ordinal":n.to_string()})
        );
        if n % 2 == 0 {
            assert_eq!(
                cipher.open_row(&row, "result").unwrap(),
                json!({"private":"legacy-result-fixture","ordinal":n.to_string()})
            );
        } else {
            assert!(row.get::<Option<Value>, _>("result").is_none());
        }
    }
}

/// Produce a real schema-9 encrypted database, including UNKNOWN and live states.
#[tokio::test]
#[ignore = "requires fresh isolated PostgreSQL and private backup/restore harness directory"]
async fn seed_legacy_encrypted_database_for_dump() {
    let root = directory();
    let pool = pool().await;
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM information_schema.tables WHERE table_schema='public'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    // Use the original migration text/checksums, with no fabricated SQLx history.
    let mut legacy = sqlx::migrate!("../../migrations");
    legacy.migrations = Cow::Owned(legacy.iter().filter(|m| m.version <= 9).cloned().collect());
    legacy.run(&pool).await.unwrap();
    let actor = Uuid::from_u128(9001);
    sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'legacy','legacy','fixture-only','admin')").bind(actor).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES('legacy','legacy','legacy','fixture-only')").execute(&pool).await.unwrap();
    for n in 1..=COUNT as u128 {
        let state = [
            "SUCCEEDED",
            "FAILED",
            "UNKNOWN",
            "EXPIRED",
            "QUEUED",
            "RUNNING",
        ][(n % 6) as usize];
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,expires_at,input,result,dispatch_epoch,lease_until,error) VALUES($1,'legacy','legacy',$2,5,$3,$4,'2020-01-02 03:04:05+00',$5,$6,$7,'2020-01-02 03:04:04+00',$8)")
            .bind(Uuid::from_u128(n)).bind(actor).bind(n.to_string()).bind(state)
            .bind(json!({"private":"legacy-input-fixture","ordinal":n.to_string()}))
            .bind((n % 2 == 0).then(||json!({"private":"legacy-result-fixture","ordinal":n.to_string()})))
            .bind(Uuid::from_u128(9002)).bind((state=="UNKNOWN").then_some("OUTCOME_UNKNOWN"))
            .execute(&pool).await.unwrap();
    }
    db::audit(
        &pool,
        "legacy",
        "fixture-only",
        "legacy.before-upgrade",
        "legacy",
        "UNKNOWN",
    )
    .await
    .unwrap();
    task_crypto::create_key(&root.join("original.key")).unwrap();
    let original = TaskCipher::load(&root.join("original.key")).unwrap();
    assert_eq!(original.prepare_storage(&pool).await.unwrap(), COUNT as u64);
    private_new(
        &root.join("passphrase"),
        b"synthetic-upgrade-backup-passphrase",
    );
    task_crypto::backup_key(
        &root.join("original.key"),
        &root.join("original.backup"),
        &root.join("passphrase"),
    )
    .unwrap();
    assert_clear(&pool, &original).await;
    let saved = baseline(&pool).await;
    assert!(
        !saved["payloads"]
            .to_string()
            .contains("legacy-input-fixture")
    );
    assert!(
        !saved["payloads"]
            .to_string()
            .contains("legacy-result-fixture")
    );
    save(&root.join("legacy-baseline.json"), &saved);
    save(
        &root.join("protected-original.json"),
        &json!({"key":protected(&root.join("original.key")),"backup":protected(&root.join("original.backup"))}),
    );
    pool.close().await;
}

/// Run only after pg_restore into a separate database. Exercise current startup,
/// wrong/missing material, retained slots, rekey, and both encrypted key backups.
#[tokio::test]
#[ignore = "requires pg_restore of the seed dump into a different isolated database"]
async fn restored_legacy_database_upgrades_and_rekeys_without_losing_history() {
    let root = directory();
    let pool = pool().await;
    let old = load(&root.join("legacy-baseline.json"));
    assert_eq!(
        baseline(&pool).await,
        old,
        "pg_restore must reproduce original ciphertext and binding"
    );
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    assert_eq!(
        baseline(&pool).await,
        old,
        "SQL upgrade must not rewrite historical payloads or UNKNOWNs"
    );
    let state = root.join("upgraded-state");
    fs::create_dir(&state).unwrap();
    fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).unwrap();
    assert!(
        task_key_store::recover(&pool, &root.join("missing.key"), &state)
            .await
            .is_err()
    );
    assert!(!root.join("missing.key").exists());
    task_crypto::create_key(&root.join("unrelated.key")).unwrap();
    let unrelated = TaskCipher::load(&root.join("unrelated.key")).unwrap();
    assert!(unrelated.prepare_storage(&pool).await.is_err());
    assert_eq!(baseline(&pool).await, old);
    private_new(
        &root.join("wrong-passphrase"),
        b"synthetic-but-wrong-passphrase",
    );
    assert!(
        task_crypto::restore_key(
            &root.join("original.backup"),
            &root.join("bad-restore.key"),
            &root.join("wrong-passphrase")
        )
        .is_err()
    );
    assert!(!root.join("bad-restore.key").exists());
    task_crypto::restore_key(
        &root.join("original.backup"),
        &root.join("restored-original.key"),
        &root.join("passphrase"),
    )
    .unwrap();
    let original = task_key_store::recover(&pool, &root.join("restored-original.key"), &state)
        .await
        .unwrap();
    assert_eq!(
        original.fingerprint(),
        TaskCipher::load(&root.join("original.key"))
            .unwrap()
            .fingerprint()
    );
    assert_eq!(original.prepare_storage(&pool).await.unwrap(), 0);
    assert_clear(&pool, &original).await;
    task_crypto::create_key(&root.join("replacement.key")).unwrap();
    let replacement = TaskCipher::load(&root.join("replacement.key")).unwrap();
    task_key_store::prepare(&state, &original, &replacement).unwrap();
    task_crypto::backup_key(
        &root.join("replacement.key"),
        &root.join("replacement.backup"),
        &root.join("passphrase"),
    )
    .unwrap();
    assert_eq!(
        original.rotate_storage(&replacement, &pool).await.unwrap(),
        COUNT as u64
    );
    assert_clear(&pool, &replacement).await;
    let after = baseline(&pool).await;
    assert_eq!(after["history"], old["history"]);
    assert_eq!(after["audit"], old["audit"]);
    assert_ne!(after["payloads"], old["payloads"]);
    assert_eq!(after["binding"]["version"], 2);
    // A stale configured file must select the durable authenticated new slot.
    let selected = task_key_store::recover(&pool, &root.join("original.key"), &state)
        .await
        .unwrap();
    assert_eq!(selected.fingerprint(), replacement.fingerprint());
    assert_eq!(selected.prepare_storage(&pool).await.unwrap(), 0);
    assert_clear(&pool, &selected).await;
    task_crypto::restore_key(
        &root.join("replacement.backup"),
        &root.join("restored-replacement.key"),
        &root.join("passphrase"),
    )
    .unwrap();
    let restored = TaskCipher::load(&root.join("restored-replacement.key")).unwrap();
    assert!(restored.matches_storage(&pool).await.unwrap());
    assert!(!original.matches_storage(&pool).await.unwrap());
    assert_clear(&pool, &restored).await;
    assert_eq!(baseline(&pool).await, after);
    save(&root.join("upgraded-baseline.json"), &after);
    let protected = load(&root.join("protected-original.json"));
    assert_eq!(
        self::protected(&root.join("original.key")),
        protected["key"]
    );
    assert_eq!(
        self::protected(&root.join("original.backup")),
        protected["backup"]
    );
    pool.close().await;
}

/// After a second pg_restore of the pre-upgrade dump, newer retained material
/// must not strand the old backup or cause an automatic rekey/re-dispatch.
#[tokio::test]
#[ignore = "requires a second fresh pg_restore of the original schema-9 dump"]
async fn historical_database_backup_selects_its_old_key_after_later_rekey() {
    let root = directory();
    let pool = pool().await;
    let old = load(&root.join("legacy-baseline.json"));
    assert_eq!(baseline(&pool).await, old);
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
    let selected = task_key_store::recover(
        &pool,
        &root.join("replacement.key"),
        &root.join("upgraded-state"),
    )
    .await
    .unwrap();
    assert_eq!(
        selected.fingerprint(),
        TaskCipher::load(&root.join("original.key"))
            .unwrap()
            .fingerprint()
    );
    assert_eq!(selected.prepare_storage(&pool).await.unwrap(), 0);
    assert_clear(&pool, &selected).await;
    assert!(
        !TaskCipher::load(&root.join("restored-replacement.key"))
            .unwrap()
            .matches_storage(&pool)
            .await
            .unwrap()
    );
    assert_eq!(
        baseline(&pool).await,
        old,
        "reading/restoring old backup must preserve every original task outcome"
    );
    let protected = load(&root.join("protected-original.json"));
    assert_eq!(
        self::protected(&root.join("original.key")),
        protected["key"]
    );
    assert_eq!(
        self::protected(&root.join("original.backup")),
        protected["backup"]
    );
    pool.close().await;
}
