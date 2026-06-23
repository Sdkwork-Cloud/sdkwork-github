use axum::Router;
use sdkwork_github_integration_service::ports::GitHubStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

pub fn build_router<S>(_: GitHubIntegrationService<S>) -> Router
where
    S: GitHubStore + Clone + Send + Sync + 'static,
{
    Router::new()
}
