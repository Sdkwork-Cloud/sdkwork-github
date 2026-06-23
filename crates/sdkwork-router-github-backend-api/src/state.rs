use sdkwork_github_integration_service::ports::GitHubSyncStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

#[derive(Clone)]
pub struct GitHubBackendState<S: GitHubSyncStore> {
    pub service: GitHubIntegrationService<S>,
}

impl<S: GitHubSyncStore> GitHubBackendState<S> {
    pub fn new(service: GitHubIntegrationService<S>) -> Self {
        Self { service }
    }
}
