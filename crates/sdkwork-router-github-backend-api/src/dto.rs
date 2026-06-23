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

impl From<SyncResult> for SyncResponse {
    fn from(value: SyncResult) -> Self {
        Self {
            provider: value.provider,
            synced_count: value.synced_count,
        }
    }
}
