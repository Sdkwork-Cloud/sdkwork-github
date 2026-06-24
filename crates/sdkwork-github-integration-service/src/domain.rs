use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub full_name: String,
    pub owner: String,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub html_url: Option<String>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub repository_id: String,
    pub number: i64,
    pub title: String,
    pub state: String,
    pub html_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub repository_id: Option<String>,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItem {
    pub id: String,
    pub plan_id: String,
    pub title: String,
    pub status: String,
    pub sort_order: i32,
    pub issue_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanView {
    pub id: String,
    pub title: String,
    pub status: String,
    pub repository_id: Option<String>,
    pub items: Vec<PlanItemView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItemView {
    pub id: String,
    pub title: String,
    pub status: String,
    pub sort_order: i32,
    pub issue_id: Option<String>,
}

impl PlanView {
    pub fn from_plan(plan: Plan, items: Vec<PlanItem>) -> Self {
        Self {
            id: plan.id,
            title: plan.title,
            status: plan.status,
            repository_id: plan.repository_id,
            items: items.into_iter().map(PlanItemView::from).collect(),
        }
    }
}

impl From<PlanItem> for PlanItemView {
    fn from(item: PlanItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            status: item.status,
            sort_order: item.sort_order,
            issue_id: item.issue_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogBootstrapResult {
    pub repositories_synced: u64,
    pub issues_synced: u64,
    pub plans_created: u64,
    pub plan_items_created: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub provider: String,
    pub synced_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAccount {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub provider: String,
    pub external_account_id: Option<String>,
    pub access_token_cipher: String,
    pub scopes: Option<String>,
    pub status: String,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    pub provider: String,
    pub linked: bool,
    pub status: Option<String>,
    pub external_account_id: Option<String>,
    pub scopes: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkIntegrationCommand {
    pub access_token: String,
    pub external_account_id: Option<String>,
    pub scopes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthBeginResult {
    pub provider: String,
    pub authorization_url: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminIntegrationView {
    pub tenant_id: String,
    pub organization_id: String,
    pub provider: String,
    pub linked: bool,
    pub status: Option<String>,
    pub external_account_id: Option<String>,
    pub scopes: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
}
