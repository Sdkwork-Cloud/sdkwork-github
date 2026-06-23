use sdkwork_github_database_host::GitHubDatabaseHost;
use sdkwork_github_integration_repository_sqlx::SqlGitHubStore;
use sdkwork_github_integration_service::GitHubIntegrationService;
use sdkwork_database_sqlx::DatabasePool;

pub struct GitHubBootstrap {
    pub service: GitHubIntegrationService<SqlGitHubStore>,
    pub pool: DatabasePool,
}

pub async fn build_github_bootstrap() -> Result<GitHubBootstrap, String> {
    let host = bootstrap_github_database_from_env().await?;
    Ok(GitHubBootstrap {
        service: GitHubIntegrationService::new(SqlGitHubStore::new(host.pool().clone())),
        pool: host.pool().clone(),
    })
}

pub async fn build_github_service(
) -> Result<GitHubIntegrationService<SqlGitHubStore>, String> {
    Ok(build_github_bootstrap().await?.service)
}

async fn bootstrap_github_database_from_env(
) -> Result<GitHubDatabaseHost, String> {
    sdkwork_github_database_host::bootstrap_github_database_from_env().await
}
