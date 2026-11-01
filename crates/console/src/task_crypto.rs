use aes_gcm::{
    Aes256Gcm, KeyInit, Nonce,
    aead::{Aead, Payload},
};
use anyhow::{Context, Result, bail, ensure};
use rand_core::{OsRng, RngCore};
use serde_json::{Value, json};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::Path,
    sync::Arc,
};
use uuid::Uuid;

const LIMIT: usize = 1024 * 1024;
const CHECK_AAD: &[u8] = b"yofc-console-task-key-check-v1";
const CHECK_VALUE: &[u8] = b"yofc-console-task-storage-v1";

// Deliberately no Debug: neither key material nor plaintext belongs in logs.
#[derive(Clone)]
pub struct TaskCipher(Arc<Aes256Gcm>);

pub struct TaskContext {
    pub id: Uuid,
    pub tenant: String,
    pub gateway: String,
    pub operation: i32,
}
impl TaskContext {
    pub fn from_row(row: &PgRow) -> Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant: row.try_get("tenant_id")?,
            gateway: row.try_get("gateway_id")?,
            operation: row.try_get("operation")?,
        })
    }
    fn aad(&self, field: &str) -> Result<Vec<u8>> {
        ensure!(matches!(field, "input" | "result"), "invalid task field");
        Ok(serde_json::to_vec(&json!([
            "yofc-console-task-v1",
            self.id,
            self.tenant,
            self.gateway,
            self.operation,
            field
        ]))?)
    }
}

fn private_parent(path: &Path) -> Result<&Path> {
    ensure!(path.is_absolute(), "task data key path must be absolute");
    let parent = path
        .parent()
        .context("task data key needs a private directory")?;
    let m = fs::symlink_metadata(parent)?;
    ensure!(
        m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
        "task data key directory must be owned and private"
    );
    Ok(parent)
}

pub fn create_key(path: &Path) -> Result<()> {
    let parent = private_parent(path)?;
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)?;
    file.write_all(&key)?;
    file.sync_all()?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

impl TaskCipher {
    pub fn load(path: &Path) -> Result<Self> {
        private_parent(path)?;
        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
            .open(path)
            .context("task data key unavailable; restore the original key, do not regenerate")?;
        let m = file.metadata()?;
        ensure!(
            m.is_file()
                && m.uid() == unsafe { libc::geteuid() }
                && m.mode() & 0o777 == 0o600
                && m.nlink() == 1
                && m.len() == 32,
            "task data key must be a single owned regular 32-byte file with mode 0600"
        );
        let mut key = [0u8; 32];
        file.read_exact(&mut key)?;
        Ok(Self(Arc::new(
            Aes256Gcm::new_from_slice(&key)
                .map_err(|_| anyhow::anyhow!("invalid task data key"))?,
        )))
    }

    fn seal_bytes(&self, aad: &[u8], clear: &[u8]) -> Result<Value> {
        ensure!(clear.len() <= LIMIT, "task payload exceeds storage bound");
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let encrypted = self
            .0
            .encrypt(Nonce::from_slice(&nonce), Payload { msg: clear, aad })
            .map_err(|_| anyhow::anyhow!("task payload encryption failed"))?;
        Ok(json!({"$yofcTask":1,"nonce":hex::encode(nonce),"ciphertext":hex::encode(encrypted)}))
    }

    fn open_bytes(&self, aad: &[u8], encrypted: &Value) -> Result<Vec<u8>> {
        let object = encrypted
            .as_object()
            .context("encrypted task envelope required")?;
        ensure!(
            object.len() == 3 && object.get("$yofcTask") == Some(&json!(1)),
            "invalid task envelope"
        );
        let nonce = object
            .get("nonce")
            .and_then(Value::as_str)
            .context("invalid task nonce")?;
        let data = object
            .get("ciphertext")
            .and_then(Value::as_str)
            .context("invalid task ciphertext")?;
        ensure!(
            nonce.len() == 24 && (32..=(LIMIT + 16) * 2).contains(&data.len()),
            "invalid task ciphertext length"
        );
        let nonce = hex::decode(nonce).context("invalid task nonce")?;
        let data = hex::decode(data).context("invalid task ciphertext")?;
        self.0
            .decrypt(Nonce::from_slice(&nonce), Payload { msg: &data, aad })
            .map_err(|_| anyhow::anyhow!("task payload authentication failed"))
    }

    pub fn seal(&self, context: &TaskContext, field: &str, clear: &Value) -> Result<Value> {
        self.seal_bytes(&context.aad(field)?, &serde_json::to_vec(clear)?)
    }

    pub fn open(&self, context: &TaskContext, field: &str, encrypted: &Value) -> Result<Value> {
        Ok(serde_json::from_slice(
            &self.open_bytes(&context.aad(field)?, encrypted)?,
        )?)
    }

    pub fn open_row(&self, row: &PgRow, field: &str) -> Result<Value> {
        self.open(
            &TaskContext::from_row(row)?,
            field,
            &row.try_get::<Value, _>(field)?,
        )
    }

    // First upgrade: one transaction locks out old writers, encrypts retained
    // rows in bounded pages, validates the guard, and binds the database to key.
    // Later starts verify the key without rewriting any ciphertext or history.
    pub async fn prepare_storage(&self, pool: &PgPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        sqlx::query("SET LOCAL lock_timeout='10s'")
            .execute(&mut *tx)
            .await?;
        sqlx::query("SELECT pg_advisory_xact_lock(794654,2)")
            .execute(&mut *tx)
            .await?;
        sqlx::query("LOCK TABLE mgmt_tasks IN ACCESS EXCLUSIVE MODE")
            .execute(&mut *tx)
            .await?;
        let check: Option<Value> =
            sqlx::query_scalar("SELECT key_check FROM mgmt_task_crypto WHERE singleton")
                .fetch_optional(&mut *tx)
                .await?;
        if let Some(check) = check {
            ensure!(
                self.open_bytes(CHECK_AAD, &check)? == CHECK_VALUE,
                "incorrect task data key"
            );
            tx.commit().await?;
            return Ok(0);
        }
        let mut last: Option<Uuid> = None;
        let mut migrated = 0;
        loop {
            let rows = sqlx::query("SELECT id,tenant_id,gateway_id,operation,input,result FROM mgmt_tasks WHERE ($1::uuid IS NULL OR id>$1) ORDER BY id LIMIT 100")
                .bind(last).fetch_all(&mut *tx).await?;
            if rows.is_empty() {
                break;
            }
            for row in rows {
                let context = TaskContext::from_row(&row)?;
                let input: Value = row.try_get("input")?;
                let result: Option<Value> = row.try_get("result")?;
                // Never guess how to recover partially converted or corrupt data.
                if input.get("$yofcTask").is_some()
                    || result
                        .as_ref()
                        .is_some_and(|r| r.get("$yofcTask").is_some())
                {
                    bail!(
                        "encrypted task found without database key binding; restore storage metadata"
                    );
                }
                let input = self.seal(&context, "input", &input)?;
                let result = result
                    .map(|r| self.seal(&context, "result", &r))
                    .transpose()?;
                sqlx::query("UPDATE mgmt_tasks SET input=$2,result=$3 WHERE id=$1")
                    .bind(context.id)
                    .bind(input)
                    .bind(result)
                    .execute(&mut *tx)
                    .await?;
                last = Some(context.id);
                migrated += 1;
            }
        }
        sqlx::query("ALTER TABLE mgmt_tasks ALTER COLUMN input DROP DEFAULT")
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"ALTER TABLE mgmt_tasks ADD CONSTRAINT mgmt_tasks_encrypted_payloads CHECK(
            (input->'$yofcTask' = '1'::jsonb AND jsonb_typeof(input->'nonce')='string' AND jsonb_typeof(input->'ciphertext')='string') IS TRUE
            AND (result IS NULL OR (result->'$yofcTask' = '1'::jsonb AND jsonb_typeof(result->'nonce')='string' AND jsonb_typeof(result->'ciphertext')='string') IS TRUE)
        )"#).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO mgmt_task_crypto(singleton,version,key_check) VALUES(true,1,$1)")
            .bind(self.seal_bytes(CHECK_AAD, CHECK_VALUE)?)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(migrated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::{PermissionsExt, symlink};

    fn fixture() -> (tempfile::TempDir, TaskCipher) {
        let d = tempfile::tempdir().unwrap();
        fs::set_permissions(d.path(), fs::Permissions::from_mode(0o700)).unwrap();
        let p = d.path().join("task.key");
        create_key(&p).unwrap();
        let cipher = TaskCipher::load(&p).unwrap();
        (d, cipher)
    }
    fn context() -> TaskContext {
        TaskContext {
            id: Uuid::new_v4(),
            tenant: "tenant".into(),
            gateway: "gateway".into(),
            operation: 5,
        }
    }
    #[test]
    fn randomized_ciphertext_roundtrips_json_without_plaintext() {
        let (_d, cipher) = fixture();
        let ctx = context();
        let value =
            json!({"password":"test-only-private-value","unset":null,"large":"9007199254740993"});
        let one = cipher.seal(&ctx, "input", &value).unwrap();
        assert_ne!(one, cipher.seal(&ctx, "input", &value).unwrap());
        assert!(!one.to_string().contains("test-only-private-value"));
        assert_eq!(cipher.open(&ctx, "input", &one).unwrap(), value);
    }
    #[test]
    fn ciphertext_is_bound_to_every_context_field_and_key() {
        let (_d, cipher) = fixture();
        let ctx = context();
        let encrypted = cipher
            .seal(&ctx, "input", &json!({"private":true}))
            .unwrap();
        for changed in [
            TaskContext {
                id: Uuid::new_v4(),
                ..context()
            },
            TaskContext {
                tenant: "other".into(),
                id: ctx.id,
                ..context()
            },
            TaskContext {
                gateway: "other".into(),
                id: ctx.id,
                ..context()
            },
            TaskContext {
                operation: 6,
                id: ctx.id,
                ..context()
            },
        ] {
            assert!(cipher.open(&changed, "input", &encrypted).is_err());
        }
        assert!(cipher.open(&ctx, "result", &encrypted).is_err());
        let (_d2, wrong) = fixture();
        assert!(wrong.open(&ctx, "input", &encrypted).is_err());
    }
    #[test]
    fn invalid_or_plaintext_envelopes_fail_without_fallback() {
        let (_d, cipher) = fixture();
        let ctx = context();
        let mut value = cipher
            .seal(&ctx, "result", &json!({"private":true}))
            .unwrap();
        value["ciphertext"] = json!("00".repeat(32));
        for bad in [
            value,
            json!({"password":"plaintext"}),
            json!({"$yofcTask":2}),
            Value::Null,
        ] {
            assert!(cipher.open(&ctx, "result", &bad).is_err());
        }
        assert!(
            cipher
                .seal(&ctx, "input", &json!("x".repeat(LIMIT)))
                .is_err()
        );
    }
    #[test]
    fn missing_insecure_linked_and_wrong_size_keys_are_rejected() {
        let (d, _) = fixture();
        let p = d.path().join("task.key");
        assert!(TaskCipher::load(&d.path().join("missing")).is_err());
        let link = d.path().join("link");
        symlink(&p, &link).unwrap();
        assert!(TaskCipher::load(&link).is_err());
        fs::set_permissions(&p, fs::Permissions::from_mode(0o640)).unwrap();
        assert!(TaskCipher::load(&p).is_err());
        fs::set_permissions(&p, fs::Permissions::from_mode(0o600)).unwrap();
        fs::hard_link(&p, d.path().join("hard")).unwrap();
        assert!(TaskCipher::load(&p).is_err());
        fs::remove_file(d.path().join("hard")).unwrap();
        fs::write(&p, b"short").unwrap();
        assert!(TaskCipher::load(&p).is_err());
    }
    #[test]
    fn explicit_key_creation_never_overwrites_or_uses_public_directory() {
        let (d, _) = fixture();
        let p = d.path().join("task.key");
        let before = fs::read(&p).unwrap();
        assert!(create_key(&p).is_err());
        assert_eq!(fs::read(&p).unwrap(), before);
        fs::set_permissions(d.path(), fs::Permissions::from_mode(0o755)).unwrap();
        assert!(create_key(&d.path().join("new")).is_err());
        assert!(TaskCipher::load(&p).is_err());
    }

    #[tokio::test]
    #[ignore = "requires isolated PostgreSQL; creates and drops a dedicated schema"]
    async fn postgres_backfill_is_atomic_key_bound_and_preserves_task_history() {
        let url = std::env::var("DATABASE_URL").unwrap();
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("task_crypto_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .options([("search_path", schema.as_str())]);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect_with(options)
            .await
            .unwrap();
        sqlx::migrate!("../../migrations").run(&pool).await.unwrap();
        let user = Uuid::new_v4();
        sqlx::query("INSERT INTO mgmt_users(id,tenant_id,username,password_hash,role) VALUES($1,'crypto','crypto','unused','admin')")
            .bind(user).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO mgmt_gateways(id,tenant_id,name,certificate_sha256) VALUES('crypto','crypto','crypto','unused')")
            .execute(&pool).await.unwrap();
        for n in 1..=101 {
            let state = if n % 3 == 0 { "UNKNOWN" } else { "SUCCEEDED" };
            sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,state,expires_at,input,result) VALUES($1,'crypto','crypto',$2,5,$3,$4,now()-interval '1 hour',$5,$6)")
                .bind(Uuid::from_u128(n)).bind(user).bind(n.to_string()).bind(state)
                .bind(json!({"private":"test-only-input-secret"}))
                .bind(if n % 2 == 0 {Some(json!({"private":"test-only-result-secret"}))} else {None})
                .execute(&pool).await.unwrap();
        }
        // Force a failure after the first bounded page has been processed.
        let bad = Uuid::from_u128(200);
        sqlx::query("INSERT INTO mgmt_tasks(id,gateway_id,tenant_id,actor_id,operation,idempotency_key,expires_at,input) VALUES($1,'crypto','crypto',$2,1,'bad',now(),$3)")
            .bind(bad).bind(user).bind(json!({"$yofcTask":1})).execute(&pool).await.unwrap();
        let (_dir, cipher) = fixture();
        assert!(cipher.prepare_storage(&pool).await.is_err());
        let untouched: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM mgmt_tasks WHERE input->>'private'='test-only-input-secret'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(untouched, 101);
        let no_binding: i64 = sqlx::query_scalar("SELECT count(*) FROM mgmt_task_crypto")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(no_binding, 0);
        sqlx::query("UPDATE mgmt_tasks SET input='{}' WHERE id=$1")
            .bind(bad)
            .execute(&pool)
            .await
            .unwrap();
        let history_sql =
            "SELECT jsonb_agg(to_jsonb(t)-'input'-'result' ORDER BY id) FROM mgmt_tasks t";
        let history: Value = sqlx::query_scalar(history_sql)
            .fetch_one(&pool)
            .await
            .unwrap();
        let (first, concurrent) =
            tokio::join!(cipher.prepare_storage(&pool), cipher.prepare_storage(&pool));
        let mut migrated = vec![first.unwrap(), concurrent.unwrap()];
        migrated.sort();
        assert_eq!(migrated, vec![0, 102]);
        let after: Value = sqlx::query_scalar(history_sql)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(history, after);
        let stored:Value=sqlx::query_scalar("SELECT jsonb_agg(jsonb_build_object('input',input,'result',result) ORDER BY id) FROM mgmt_tasks")
            .fetch_one(&pool).await.unwrap();
        assert!(!stored.to_string().contains("test-only-input-secret"));
        assert!(!stored.to_string().contains("test-only-result-secret"));
        for row in sqlx::query(
            "SELECT id,tenant_id,gateway_id,operation,input,result FROM mgmt_tasks WHERE id<>$1",
        )
        .bind(bad)
        .fetch_all(&pool)
        .await
        .unwrap()
        {
            assert_eq!(
                cipher.open_row(&row, "input").unwrap(),
                json!({"private":"test-only-input-secret"})
            );
            if row.get::<Option<Value>, _>("result").is_some() {
                assert_eq!(
                    cipher.open_row(&row, "result").unwrap(),
                    json!({"private":"test-only-result-secret"})
                );
            }
        }
        assert!(
            sqlx::query("UPDATE mgmt_tasks SET input='{}' WHERE id=$1")
                .bind(bad)
                .execute(&pool)
                .await
                .is_err()
        );
        assert!(
            sqlx::query("UPDATE mgmt_tasks SET result='{\"private\":true}' WHERE id=$1")
                .bind(bad)
                .execute(&pool)
                .await
                .is_err()
        );
        let (_wrong_dir, wrong) = fixture();
        assert!(wrong.prepare_storage(&pool).await.is_err());
        assert_eq!(cipher.prepare_storage(&pool).await.unwrap(), 0);
        let restarted:Value=sqlx::query_scalar("SELECT jsonb_agg(jsonb_build_object('input',input,'result',result) ORDER BY id) FROM mgmt_tasks")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(stored, restarted);
        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}
