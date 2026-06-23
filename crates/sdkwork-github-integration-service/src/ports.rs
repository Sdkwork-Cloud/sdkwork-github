use async_trait::async_trait;

use crate::domain::{
    IntegrationStatus, Issue, LinkIntegrationCommand, Page, Plan, ProviderAccount, Repository,
};
use crate::error::ServiceError;

#[async_trait]
pub trait GitHubStore: Send + Sync + Clone {
    async fn list_repositories(
        &self,
        tenant_id: &str,
        organization_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Repository>, ServiceError>;

    async fn list_issues(
        &self,
        tenant_id: &str,
        organization_id: &str,
        repository_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Issue>, ServiceError>;

    async fn list_plans(
        &self,
        tenant_id: &str,
        organization_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Plan>, ServiceError>;
}

#[async_trait]
pub trait GitHubSyncStore: GitHubStore {
    async fn upsert_repository(&self, repository: &Repository) -> Result<(), ServiceError>;

    async fn upsert_issue(&self, issue: &Issue) -> Result<(), ServiceError>;

    async fn find_active_provider_account(
        &self,
        tenant_id: &str,
        organization_id: &str,
        provider: &str,
    ) -> Result<Option<ProviderAccount>, ServiceError>;

    async fn upsert_provider_account(
        &self,
        account: &ProviderAccount,
    ) -> Result<(), ServiceError>;

    async fn revoke_provider_account(
        &self,
        tenant_id: &str,
        organization_id: &str,
        provider: &str,
    ) -> Result<(), ServiceError>;

    async fn get_integration_status(
        &self,
        tenant_id: &str,
        organization_id: &str,
        provider: &str,
    ) -> Result<IntegrationStatus, ServiceError>;

    async fn link_integration(
        &self,
        tenant_id: &str,
        organization_id: &str,
        provider: &str,
        command: &LinkIntegrationCommand,
        access_token_cipher: &str,
    ) -> Result<IntegrationStatus, ServiceError>;

    async fn touch_provider_last_synced(
        &self,
        tenant_id: &str,
        organization_id: &str,
        provider: &str,
    ) -> Result<(), ServiceError>;

    async fn create_oauth_pending(
        &self,
        state: &str,
        tenant_id: &str,
        organization_id: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), ServiceError>;

    async fn consume_oauth_pending(
        &self,
        state: &str,
    ) -> Result<Option<(String, String)>, ServiceError>;

    async fn list_admin_integrations(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<Page<crate::domain::AdminIntegrationView>, ServiceError>;
}
