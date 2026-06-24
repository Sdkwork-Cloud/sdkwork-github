use serde::{Deserialize, Serialize};

use sdkwork_github_integration_service::domain::{AdminIntegrationView, Page, SyncResult};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AdminSyncRequest {
    pub tenant_id: String,
    pub organization_id: String,
}

#[derive(Debug, Serialize)]
pub struct AdminIntegrationPageResponse {
    pub items: Vec<AdminIntegrationView>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

impl From<Page<AdminIntegrationView>> for AdminIntegrationPageResponse {
    fn from(page: Page<AdminIntegrationView>) -> Self {
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

#[derive(Debug, Serialize)]
pub struct CatalogBootstrapResponse {
    pub repositories_synced: u64,
    pub issues_synced: u64,
    pub plans_created: u64,
    pub plan_items_created: u64,
}

impl From<sdkwork_github_integration_service::domain::CatalogBootstrapResult>
    for CatalogBootstrapResponse
{
    fn from(value: sdkwork_github_integration_service::domain::CatalogBootstrapResult) -> Self {
        Self {
            repositories_synced: value.repositories_synced,
            issues_synced: value.issues_synced,
            plans_created: value.plans_created,
            plan_items_created: value.plan_items_created,
        }
    }
}

impl From<SyncResult> for SyncResponse {
    fn from(value: SyncResult) -> Self {
        Self {
            provider: value.provider,
            synced_count: value.synced_count,
        }
    }
}
