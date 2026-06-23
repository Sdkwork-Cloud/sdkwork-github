use axum::routing::{delete, get, post};
use axum::Router;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

use crate::handlers;
use crate::paths;
use crate::state::GitHubAppState;

pub fn build_router<S>(service: GitHubIntegrationService<S>) -> Router
where
    S: GitHubSyncStore + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::REPOSITORIES, get(handlers::list_repositories::<S>))
        .route(
            paths::REPOSITORIES_SYNC,
            post(handlers::sync_repositories::<S>),
        )
        .route(paths::ISSUES, get(handlers::list_issues::<S>))
        .route(paths::ISSUES_SYNC, post(handlers::sync_issues::<S>))
        .route(paths::PLANS, get(handlers::list_plans::<S>))
        .route(paths::INTEGRATION, get(handlers::get_integration_status::<S>))
        .route(paths::INTEGRATION, post(handlers::link_integration::<S>))
        .route(paths::INTEGRATION, delete(handlers::unlink_integration::<S>))
        .route(
            paths::INTEGRATION_OAUTH_BEGIN,
            post(handlers::begin_oauth_integration::<S>),
        )
        .route(
            paths::INTEGRATION_OAUTH_CALLBACK,
            get(handlers::oauth_callback::<S>),
        )
        .with_state(GitHubAppState::new(service))
}
