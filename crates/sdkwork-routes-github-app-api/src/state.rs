use sdkwork_github_integration_service::ports::GitHubStore;
use sdkwork_github_integration_service::GitHubIntegrationService;

#[derive(Clone)]
pub struct GitHubAppState<S: GitHubStore> {
    pub service: GitHubIntegrationService<S>,
}

impl<S: GitHubStore> GitHubAppState<S> {
    pub fn new(service: GitHubIntegrationService<S>) -> Self {
        Self { service }
    }
}
