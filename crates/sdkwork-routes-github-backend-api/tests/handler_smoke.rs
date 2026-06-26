use axum::extract::{Query, State};
use http::StatusCode;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};
use sdkwork_github_integration_repository_sqlx::SqlGitHubStore;
use sdkwork_github_integration_service::domain::LinkIntegrationCommand;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_github_integration_service::GitHubIntegrationService;
use sdkwork_routes_github_backend_api::dto::PageQuery;
use sdkwork_routes_github_backend_api::handlers;
use sdkwork_routes_github_backend_api::state::GitHubBackendState;

async fn migrated_store() -> SqlGitHubStore {
    let config = DatabaseConfig {
        engine: DatabaseEngine::Sqlite,
        url: "sqlite::memory:".to_string(),
        max_connections: 1,
        ..Default::default()
    };
    let pool = create_pool_from_config(config)
        .await
        .expect("create sqlite memory pool");
    install_schema(pool.clone()).await;
    SqlGitHubStore::new(pool)
}

async fn install_schema(pool: DatabasePool) {
    let sqlite = pool.as_sqlite().expect("sqlite pool");
    let baseline = include_str!("../../../database/ddl/baseline/sqlite/0001_github_legacy_baseline.sql");
    let migration = include_str!("../../../database/migrations/sqlite/0002_github_provider_account.sql");
    let oauth_migration = include_str!("../../../database/migrations/sqlite/0003_github_oauth_pending.sql");
    let integrity_migration =
        include_str!("../../../database/migrations/sqlite/0004_github_referential_integrity.sql");
    for script in [baseline, migration, oauth_migration, integrity_migration] {
        for statement in script.split(';').map(str::trim).filter(|value| !value.is_empty()) {
            sqlx::query(statement)
                .execute(sqlite)
                .await
                .expect("execute schema statement");
        }
    }
}

#[tokio::test]
async fn list_integrations_returns_linked_accounts() {
    let store = migrated_store().await;
    store
        .link_integration(
            "tenant-a",
            "org-a",
            "github",
            &LinkIntegrationCommand {
                access_token: "ghp_test".to_string(),
                external_account_id: Some("12345".to_string()),
                scopes: Some("repo".to_string()),
            },
            "cipher-test",
        )
        .await
        .expect("seed integration");

    let service = GitHubIntegrationService::new(store);
    let state = GitHubBackendState::new(service);
    let response = handlers::list_integrations(
        State(state),
        Query(PageQuery {
            page: Some(1),
            page_size: Some(20),
        }),
    )
    .await
    .expect("list integrations");

    assert_eq!(response.0.items.len(), 1);
    assert_eq!(response.0.items[0].tenant_id, "tenant-a");
    assert_eq!(response.0.items[0].organization_id, "org-a");
}

#[tokio::test]
async fn sync_integration_requires_linked_provider() {
    let store = migrated_store().await;
    let service = GitHubIntegrationService::new(store);
    let state = GitHubBackendState::new(service);
    let error = handlers::sync_integration_repositories(
        State(state),
        axum::Json(sdkwork_routes_github_backend_api::dto::AdminSyncRequest {
            tenant_id: "tenant-a".to_string(),
            organization_id: "org-a".to_string(),
        }),
    )
    .await
    .expect_err("sync should fail without linked integration");

    assert_eq!(error.0, StatusCode::SERVICE_UNAVAILABLE);
}
