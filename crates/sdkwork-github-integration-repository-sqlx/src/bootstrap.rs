use std::path::Path;

use sqlx::AnyPool;

pub async fn install_sqlite_schema(pool: &AnyPool) -> Result<(), sqlx::Error> {
    let migration = include_str!("../../../database/migrations/0001_baseline.sql");
    for statement in migration.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        sqlx::query(statement).execute(pool).await?;
    }
    Ok(())
}

pub async fn connect_sqlite_runtime_pool(
    database_url: &str,
) -> Result<AnyPool, String> {
    sqlx::any::install_default_drivers();
    let path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("create database directory failed: {error}"))?;
        }
    }
    sqlx::any::AnyPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|error| format!("connect sqlite database failed: {error}"))
}
