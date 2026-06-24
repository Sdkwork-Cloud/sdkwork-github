use axum::extract::{Query, State};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};
use sdkwork_github_integration_repository_sqlx::SqlGitHubStore;
use sdkwork_github_integration_service::domain::Repository;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_github_integration_service::GitHubIntegrationService;
use sdkwork_router_github_app_api::dto::PageQuery;
use sdkwork_router_github_app_api::handlers;
use sdkwork_router_github_app_api::state::GitHubAppState;
use sdkwork_web_core::{
    ServerRequestId, WebApiSurface, WebAuthMode, WebEnvironment, WebLoginScope, WebRequestContext,
    WebRequestPrincipal, WebTransportFacts,
};

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

fn test_context(tenant_id: &str, organization_id: &str) -> WebRequestContext {
    WebRequestContext {
        request_id: ServerRequestId("req-test".to_owned()),
        api_surface: WebApiSurface::AppApi,
        auth_mode: WebAuthMode::DualToken,
        transport: WebTransportFacts {
            path: "/app/v3/api/github/repositories".to_owned(),
            method: "GET".to_owned(),
            auth_token_present: true,
            access_token_present: true,
            api_key_present: false,
            oauth_bearer_present: false,
        },
        principal: Some(
            WebRequestPrincipal::builder()
                .tenant_id(tenant_id)
                .organization_id(Some(organization_id.to_owned()))
                .login_scope(WebLoginScope::Organization)
                .user_id("user-test")
                .session_id(Some("session-test".to_owned()))
                .app_id("sdkwork-github")
                .environment(WebEnvironment::Test)
                .build(),
        ),
        locale: None,
        client_kind: None,
        operation: None,
        trace_id: None,
    }
}

#[tokio::test]
async fn list_repositories_returns_tenant_scoped_rows() {
    let store = migrated_store().await;
    let now = chrono::Utc::now();
    store
        .upsert_repository(&Repository {
            id: "github-repo-test-1".to_owned(),
            tenant_id: "tenant-a".to_owned(),
            organization_id: "org-a".to_owned(),
            full_name: "sdkwork/test".to_owned(),
            owner: "sdkwork".to_owned(),
            description: None,
            default_branch: Some("main".to_owned()),
            html_url: None,
            is_private: false,
            created_at: now,
            updated_at: now,
        })
        .await
        .expect("seed repository");

    let service = GitHubIntegrationService::new(store);
    let state = GitHubAppState::new(service);
    let response = handlers::list_repositories(
        State(state),
        test_context("tenant-a", "org-a"),
        Query(PageQuery {
            tenant_id: None,
            organization_id: None,
            operator_id: None,
            page: Some(1),
            page_size: Some(20),
            repository_id: None,
        }),
    )
    .await
    .expect("list repositories");

    assert_eq!(response.0.items.len(), 1);
    assert_eq!(response.0.items[0].full_name, "sdkwork/test");
}

#[tokio::test]
async fn integration_status_is_unlinked_by_default() {
    let store = migrated_store().await;
    let service = GitHubIntegrationService::new(store);
    let state = GitHubAppState::new(service);
    let response = handlers::get_integration_status(
        State(state),
        test_context("tenant-a", "org-a"),
        Query(PageQuery {
            tenant_id: None,
            organization_id: None,
            operator_id: None,
            page: None,
            page_size: None,
            repository_id: None,
        }),
    )
    .await
    .expect("integration status");

    assert_eq!(response.0.provider, "github");
    assert!(!response.0.linked);
}

#[tokio::test]
async fn oauth_begin_requires_oauth_configuration() {
    std::env::remove_var("SDKWORK_GITHUB_OAUTH_CLIENT_ID");
    std::env::remove_var("SDKWORK_GITHUB_OAUTH_CLIENT_SECRET");
    std::env::remove_var("SDKWORK_GITHUB_OAUTH_REDIRECT_URI");

    let store = migrated_store().await;
    let service = GitHubIntegrationService::new(store);
    let state = GitHubAppState::new(service);
    let error = handlers::begin_oauth_integration(
        State(state),
        test_context("tenant-a", "org-a"),
        Query(PageQuery {
            tenant_id: None,
            organization_id: None,
            operator_id: None,
            page: None,
            page_size: None,
            repository_id: None,
        }),
    )
    .await
    .expect_err("oauth begin should fail without configuration");

    assert_eq!(error.0, http::StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn list_plans_returns_nested_checklist_items() {
    let store = migrated_store().await;
    let now = chrono::Utc::now();
    store
        .upsert_plan(&sdkwork_github_integration_service::domain::Plan {
            id: "github-plan-test-1".to_owned(),
            tenant_id: "tenant-a".to_owned(),
            organization_id: "org-a".to_owned(),
            repository_id: Some("github-repo-test-1".to_owned()),
            title: "Launch checklist".to_owned(),
            status: "active".to_owned(),
            created_at: now,
            updated_at: now,
        })
        .await
        .expect("seed plan");
    store
        .upsert_plan_item(&sdkwork_github_integration_service::domain::PlanItem {
            id: "github-plan-item-test-1".to_owned(),
            plan_id: "github-plan-test-1".to_owned(),
            title: "Verify issue linkage".to_owned(),
            status: "pending".to_owned(),
            sort_order: 1,
            issue_id: Some("github-issue-test-1".to_owned()),
            created_at: now,
            updated_at: now,
        })
        .await
        .expect("seed plan item");

    let service = GitHubIntegrationService::new(store);
    let state = GitHubAppState::new(service);
    let response = handlers::list_plans(
        State(state),
        test_context("tenant-a", "org-a"),
        Query(PageQuery {
            tenant_id: None,
            organization_id: None,
            operator_id: None,
            page: Some(1),
            page_size: Some(20),
            repository_id: None,
        }),
    )
    .await
    .expect("list plans");

    assert_eq!(response.0.items.len(), 1);
    assert_eq!(response.0.items[0].title, "Launch checklist");
    assert_eq!(response.0.items[0].items.len(), 1);
    assert_eq!(
        response.0.items[0].items[0].issue_id.as_deref(),
        Some("github-issue-test-1")
    );
}
