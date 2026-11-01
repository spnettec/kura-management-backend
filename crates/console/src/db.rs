use anyhow::Result;
use serde_json::Value;
use sqlx::{PgExecutor, PgPool, postgres::PgPoolOptions};

pub async fn connect(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(12)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(url)
        .await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;
    Ok(pool)
}
pub async fn event<'e>(
    pool: impl PgExecutor<'e>,
    tenant: &str,
    kind: &str,
    target: &str,
    revision: i64,
    data: Value,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO mgmt_events(tenant_id,kind,target,revision,data) VALUES($1,$2,$3,$4,$5)",
    )
    .bind(tenant)
    .bind(kind)
    .bind(target)
    .bind(revision)
    .bind(data)
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn audit<'e>(
    pool: impl PgExecutor<'e>,
    tenant: &str,
    actor: &str,
    operation: &str,
    target: &str,
    outcome: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO mgmt_audit(tenant_id,actor,operation,target,outcome) VALUES($1,$2,$3,$4,$5)",
    )
    .bind(tenant)
    .bind(actor)
    .bind(operation)
    .bind(target)
    .bind(outcome)
    .execute(pool)
    .await?;
    Ok(())
}
