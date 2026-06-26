use axum::routing::{get, post};
use axum::Router;
use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

use crate::handlers;
use crate::paths;
use crate::state::GitHubBackendState;

pub fn build_router<S>(service: GitHubIntegrationService<S>) -> Router
where
    S: GitHubSyncStore + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::INTEGRATIONS, get(handlers::list_integrations::<S>))
        .route(
            paths::INTEGRATIONS_SYNC,
            post(handlers::sync_integration_repositories::<S>),
        )
        .route(
            paths::CATALOG_SYNC,
            post(handlers::sync_notable_catalog::<S>),
        )
        .with_state(GitHubBackendState::new(service))
}
