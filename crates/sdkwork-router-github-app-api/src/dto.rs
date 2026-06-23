use serde::{Deserialize, Serialize};

use sdkwork_github_integration_service::domain::{Issue, Page, Plan, Repository};

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
