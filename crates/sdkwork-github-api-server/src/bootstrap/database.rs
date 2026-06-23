use sdkwork_database_config::DatabaseConfig;
use sdkwork_github_integration_repository_sqlx::bootstrap::{
    connect_sqlite_runtime_pool, install_sqlite_schema,
};
use sdkwork_github_integration_repository_sqlx::SqlGitHubStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

pub async fn build_github_service() -> Result<GitHubIntegrationService<SqlGitHubStore>, String> {
    let config = DatabaseConfig::from_env("github")
        .map_err(|error| format!("resolve github database config failed: {error}"))?;
    let pool = connect_sqlite_runtime_pool(&config.url).await?;
    install_sqlite_schema(&pool)
        .await
        .map_err(|error| format!("install github schema failed: {error}"))?;
    Ok(GitHubIntegrationService::new(SqlGitHubStore::new(pool)))
}
