use axum::routing::get;
use axum::Router;
use sdkwork_github_integration_service::ports::GitHubStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

use crate::handlers;
use crate::paths;
use crate::state::GitHubAppState;

pub fn build_router<S>(service: GitHubIntegrationService<S>) -> Router
where
    S: GitHubStore + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::REPOSITORIES, get(handlers::list_repositories::<S>))
        .route(paths::ISSUES, get(handlers::list_issues::<S>))
        .route(paths::PLANS, get(handlers::list_plans::<S>))
        .with_state(GitHubAppState::new(service))
}
