use serde::{Deserialize, Serialize};

use sdkwork_github_integration_service::domain::{
    IntegrationStatus, Issue, LinkIntegrationCommand, Page, Plan, Repository, SyncResult,
};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub tenant_id: Option<String>,
    pub organization_id: Option<String>,
    pub operator_id: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub repository_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RepositoryPageResponse {
    pub items: Vec<Repository>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct IssuePageResponse {
    pub items: Vec<Issue>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct PlanPageResponse {
    pub items: Vec<Plan>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

impl From<Page<Repository>> for RepositoryPageResponse {
    fn from(page: Page<Repository>) -> Self {
        Self {
            items: page.items,
            page: page.page,
            page_size: page.page_size,
            total: page.total,
        }
    }
}

impl From<Page<Issue>> for IssuePageResponse {
    fn from(page: Page<Issue>) -> Self {
        Self {
            items: page.items,
            page: page.page,
            page_size: page.page_size,
            total: page.total,
        }
    }
}

impl From<Page<Plan>> for PlanPageResponse {
    fn from(page: Page<Plan>) -> Self {
        Self {
            items: page.items,
            page: page.page,
            page_size: page.page_size,
            total: page.total,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub provider: String,
    pub synced_count: u64,
}

impl From<SyncResult> for SyncResponse {
    fn from(value: SyncResult) -> Self {
        Self {
            provider: value.provider,
            synced_count: value.synced_count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IntegrationStatusResponse {
    pub provider: String,
    pub linked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<String>,
}

impl From<IntegrationStatus> for IntegrationStatusResponse {
    fn from(value: IntegrationStatus) -> Self {
        Self {
            provider: value.provider,
            linked: value.linked,
            status: value.status,
            external_account_id: value.external_account_id,
            scopes: value.scopes,
            last_synced_at: value
                .last_synced_at
                .map(|timestamp| timestamp.to_rfc3339()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LinkIntegrationRequest {
    pub access_token: String,
    pub external_account_id: Option<String>,
    pub scopes: Option<String>,
}

impl From<LinkIntegrationRequest> for LinkIntegrationCommand {
    fn from(value: LinkIntegrationRequest) -> Self {
        Self {
            access_token: value.access_token,
            external_account_id: value.external_account_id,
            scopes: value.scopes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OAuthBeginResponse {
    pub provider: String,
    pub authorization_url: String,
    pub state: String,
}

impl From<sdkwork_github_integration_service::domain::OAuthBeginResult> for OAuthBeginResponse {
    fn from(value: sdkwork_github_integration_service::domain::OAuthBeginResult) -> Self {
        Self {
            provider: value.provider,
            authorization_url: value.authorization_url,
            state: value.state,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub state: String,
    pub code: String,
}
