use sdkwork_utils_rust::string::is_blank;

use crate::domain::{
    IntegrationStatus, Issue, LinkIntegrationCommand, Page, Plan, Repository, SyncResult,
};
use crate::error::ServiceError;
use crate::ports::{GitHubStore, GitHubSyncStore};

const GITHUB_PROVIDER: &str = "github";

pub struct GitHubIntegrationService<S: GitHubStore> {
    store: S,
}

impl<S: GitHubStore + Clone> Clone for GitHubIntegrationService<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

impl<S: GitHubStore> GitHubIntegrationService<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn list_repositories(
        &self,
        tenant_id: &str,
        organization_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Repository>, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        self.store
            .list_repositories(tenant_id, organization_id, page, page_size)
            .await
    }

    pub async fn list_issues(
        &self,
        tenant_id: &str,
        organization_id: &str,
        repository_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Issue>, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        self.store
            .list_issues(tenant_id, organization_id, repository_id, page, page_size)
            .await
    }

    pub async fn list_plans(
        &self,
        tenant_id: &str,
        organization_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Plan>, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        self.store
            .list_plans(tenant_id, organization_id, page, page_size)
            .await
    }
}

impl<S: GitHubSyncStore> GitHubIntegrationService<S> {
    pub async fn get_integration_status(
        &self,
        tenant_id: &str,
        organization_id: &str,
    ) -> Result<IntegrationStatus, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        self.store
            .get_integration_status(tenant_id, organization_id, GITHUB_PROVIDER)
            .await
    }

    pub async fn link_integration(
        &self,
        tenant_id: &str,
        organization_id: &str,
        command: LinkIntegrationCommand,
    ) -> Result<IntegrationStatus, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        if is_blank(Some(command.access_token.as_str())) {
            return Err(ServiceError::Validation(
                "access_token is required".to_string(),
            ));
        }

        let cipher = sdkwork_github_integration_provider_github::GitHubCredentialCipher::from_env()
            .map_err(|error| ServiceError::Configuration(error.to_string()))?;
        let encrypted = cipher
            .encrypt(&command.access_token)
            .map_err(|error| ServiceError::Configuration(error.to_string()))?;

        self.store
            .link_integration(
                tenant_id,
                organization_id,
                GITHUB_PROVIDER,
                &command,
                &encrypted,
            )
            .await
    }

    pub async fn unlink_integration(
        &self,
        tenant_id: &str,
        organization_id: &str,
    ) -> Result<IntegrationStatus, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        self.store
            .revoke_provider_account(tenant_id, organization_id, GITHUB_PROVIDER)
            .await?;
        self.get_integration_status(tenant_id, organization_id)
            .await
    }

    pub async fn begin_oauth_integration(
        &self,
        tenant_id: &str,
        organization_id: &str,
    ) -> Result<crate::domain::OAuthBeginResult, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        let oauth = sdkwork_github_integration_provider_github::GitHubOAuthClient::from_env()
            .map_err(|error| ServiceError::Configuration(error.to_string()))?;
        let state = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);
        self.store
            .create_oauth_pending(&state, tenant_id, organization_id, expires_at)
            .await?;
        Ok(crate::domain::OAuthBeginResult {
            provider: GITHUB_PROVIDER.to_string(),
            authorization_url: oauth.build_authorization_url(&state),
            state,
        })
    }

    pub async fn complete_oauth_integration(
        &self,
        state: &str,
        code: &str,
    ) -> Result<IntegrationStatus, ServiceError> {
        if is_blank(Some(state)) || is_blank(Some(code)) {
            return Err(ServiceError::Validation(
                "state and code are required".to_string(),
            ));
        }
        let (tenant_id, organization_id) = self
            .store
            .consume_oauth_pending(state)
            .await?
            .ok_or_else(|| ServiceError::Validation("oauth state is invalid or expired".to_string()))?;
        let oauth = sdkwork_github_integration_provider_github::GitHubOAuthClient::from_env()
            .map_err(|error| ServiceError::Configuration(error.to_string()))?;
        let access_token = oauth
            .exchange_code(code)
            .await
            .map_err(|error| ServiceError::Integration(error.to_string()))?;
        self.link_integration(
            &tenant_id,
            &organization_id,
            crate::domain::LinkIntegrationCommand {
                access_token,
                external_account_id: None,
                scopes: Some("read:user,repo".to_string()),
            },
        )
        .await
    }

    pub async fn list_admin_integrations(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<Page<crate::domain::AdminIntegrationView>, ServiceError> {
        self.store.list_admin_integrations(page, page_size).await
    }

    pub async fn sync_repositories(
        &self,
        tenant_id: &str,
        organization_id: &str,
    ) -> Result<SyncResult, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        let provider = self
            .resolve_provider(tenant_id, organization_id)
            .await?;

        let remote_repositories = provider
            .fetch_repositories()
            .await
            .map_err(|error| ServiceError::Integration(error.to_string()))?;

        let now = chrono::Utc::now();
        let mut synced_count = 0_u64;
        for remote in remote_repositories {
            let repository = Repository {
                id: format!("github-repo-{}", remote.id),
                tenant_id: tenant_id.to_string(),
                organization_id: organization_id.to_string(),
                full_name: remote.full_name,
                owner: remote.owner.login,
                description: remote.description,
                default_branch: remote.default_branch,
                html_url: remote.html_url,
                is_private: remote.private,
                created_at: now,
                updated_at: now,
            };
            self.store.upsert_repository(&repository).await?;
            synced_count += 1;
        }

        self.store
            .touch_provider_last_synced(tenant_id, organization_id, GITHUB_PROVIDER)
            .await?;

        Ok(SyncResult {
            provider: provider.provider_key().to_string(),
            synced_count,
        })
    }

    pub async fn sync_issues(
        &self,
        tenant_id: &str,
        organization_id: &str,
        repository_id: Option<&str>,
    ) -> Result<SyncResult, ServiceError> {
        validate_scope(tenant_id, organization_id)?;
        let provider = self
            .resolve_provider(tenant_id, organization_id)
            .await?;

        let repositories = if let Some(repository_id) = repository_id {
            let page = self
                .store
                .list_repositories(tenant_id, organization_id, 1, 100)
                .await?;
            page.items
                .into_iter()
                .filter(|item| item.id == repository_id)
                .collect::<Vec<_>>()
        } else {
            self.store
                .list_repositories(tenant_id, organization_id, 1, 100)
                .await?
                .items
        };

        if repositories.is_empty() {
            return Err(ServiceError::Validation(
                "sync issues requires at least one tracked repository".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        let mut synced_count = 0_u64;
        for repository in repositories {
            let (owner, repo) = split_full_name(&repository.full_name)?;
            let remote_issues = provider
                .fetch_issues(owner, repo)
                .await
                .map_err(|error| ServiceError::Integration(error.to_string()))?;

            for remote in remote_issues {
                let issue = Issue {
                    id: format!("github-issue-{}", remote.id),
                    tenant_id: tenant_id.to_string(),
                    organization_id: organization_id.to_string(),
                    repository_id: repository.id.clone(),
                    number: remote.number,
                    title: remote.title,
                    state: remote.state,
                    html_url: remote.html_url,
                    created_at: now,
                    updated_at: now,
                };
                self.store.upsert_issue(&issue).await?;
                synced_count += 1;
            }
        }

        self.store
            .touch_provider_last_synced(tenant_id, organization_id, GITHUB_PROVIDER)
            .await?;

        Ok(SyncResult {
            provider: provider.provider_key().to_string(),
            synced_count,
        })
    }

    async fn resolve_provider(
        &self,
        tenant_id: &str,
        organization_id: &str,
    ) -> Result<sdkwork_github_integration_provider_github::GitHubRestProvider, ServiceError> {
        if let Some(account) = self
            .store
            .find_active_provider_account(tenant_id, organization_id, GITHUB_PROVIDER)
            .await?
        {
            let cipher =
                sdkwork_github_integration_provider_github::GitHubCredentialCipher::from_env()
                    .map_err(|error| ServiceError::Configuration(error.to_string()))?;
            let token = cipher
                .decrypt(&account.access_token_cipher)
                .map_err(|error| ServiceError::Configuration(error.to_string()))?;
            return Ok(build_provider(token));
        }

        if let Some(provider) =
            sdkwork_github_integration_provider_github::GitHubRestProvider::from_env()
        {
            tracing::warn!(
                tenant_id,
                organization_id,
                "using SDKWORK_GITHUB_INTEGRATION_PAT fallback; link tenant integration for production"
            );
            return Ok(provider);
        }

        Err(ServiceError::Configuration(
            "GitHub integration is not linked and SDKWORK_GITHUB_INTEGRATION_PAT is not configured"
                .to_string(),
        ))
    }
}

fn build_provider(token: String) -> sdkwork_github_integration_provider_github::GitHubRestProvider {
    let api_base = std::env::var("SDKWORK_GITHUB_INTEGRATION_API_BASE")
        .unwrap_or_else(|_| "https://api.github.com".to_string());
    sdkwork_github_integration_provider_github::GitHubRestProvider::new(token, api_base)
}

fn validate_scope(tenant_id: &str, organization_id: &str) -> Result<(), ServiceError> {
    if is_blank(Some(tenant_id)) || is_blank(Some(organization_id)) {
        return Err(ServiceError::Validation(
            "tenant_id and organization_id are required".to_string(),
        ));
    }
    Ok(())
}

fn split_full_name(full_name: &str) -> Result<(&str, &str), ServiceError> {
    let (owner, repo) = full_name.split_once('/').ok_or_else(|| {
        ServiceError::Validation(format!("invalid repository full_name: {full_name}"))
    })?;
    if is_blank(Some(owner)) || is_blank(Some(repo)) {
        return Err(ServiceError::Validation(format!(
            "invalid repository full_name: {full_name}"
        )));
    }
    Ok((owner, repo))
}

#[cfg(test)]
mod tests {
    use super::{split_full_name, validate_scope};
    use crate::error::ServiceError;

    #[test]
    fn rejects_blank_scope() {
        let error = validate_scope("", "org").unwrap_err();
        assert!(matches!(error, ServiceError::Validation(_)));
    }

    #[test]
    fn splits_repository_full_name() {
        let (owner, repo) = split_full_name("sdkwork/demo").unwrap();
        assert_eq!(owner, "sdkwork");
        assert_eq!(repo, "demo");
    }
}
