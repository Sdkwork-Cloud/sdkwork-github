use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::AnyPool;

use sdkwork_github_integration_service::domain::{Issue, Page, Plan, Repository};
use sdkwork_github_integration_service::error::ServiceError;
use sdkwork_github_integration_service::ports::GitHubStore;

#[derive(Clone)]
pub struct SqlGitHubStore {
    pool: AnyPool,
}

impl SqlGitHubStore {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GitHubStore for SqlGitHubStore {
    async fn list_repositories(
        &self,
        tenant_id: &str,
        organization_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Repository>, ServiceError> {
        let offset = ((page.saturating_sub(1)) * page_size) as i64;
        let limit = page_size as i64;
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM github_repository WHERE tenant_id = ? AND organization_id = ?",
        )
        .bind(tenant_id)
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|error| ServiceError::Repository(error.to_string()))?;

        let rows = sqlx::query_as::<_, RepositoryRow>(
            "SELECT id, tenant_id, organization_id, full_name, owner, description, default_branch, html_url, is_private, created_at, updated_at
             FROM github_repository WHERE tenant_id = ? AND organization_id = ?
             ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        )
        .bind(tenant_id)
        .bind(organization_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| ServiceError::Repository(error.to_string()))?;

        Ok(Page {
            items: rows.into_iter().map(Into::into).collect(),
            page,
            page_size,
            total: total.0 as u64,
        })
    }

    async fn list_issues(
        &self,
        tenant_id: &str,
        organization_id: &str,
        repository_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Issue>, ServiceError> {
        let offset = ((page.saturating_sub(1)) * page_size) as i64;
        let limit = page_size as i64;
        let (total, rows) = if let Some(repository_id) = repository_id {
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM github_issue WHERE tenant_id = ? AND organization_id = ? AND repository_id = ?",
            )
            .bind(tenant_id)
            .bind(organization_id)
            .bind(repository_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| ServiceError::Repository(error.to_string()))?;
            let rows = sqlx::query_as::<_, IssueRow>(
                "SELECT id, tenant_id, organization_id, repository_id, number, title, state, html_url, created_at, updated_at
                 FROM github_issue WHERE tenant_id = ? AND organization_id = ? AND repository_id = ?
                 ORDER BY number DESC LIMIT ? OFFSET ?",
            )
            .bind(tenant_id)
            .bind(organization_id)
            .bind(repository_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|error| ServiceError::Repository(error.to_string()))?;
            (total, rows)
        } else {
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM github_issue WHERE tenant_id = ? AND organization_id = ?",
            )
            .bind(tenant_id)
            .bind(organization_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| ServiceError::Repository(error.to_string()))?;
            let rows = sqlx::query_as::<_, IssueRow>(
                "SELECT id, tenant_id, organization_id, repository_id, number, title, state, html_url, created_at, updated_at
                 FROM github_issue WHERE tenant_id = ? AND organization_id = ?
                 ORDER BY updated_at DESC LIMIT ? OFFSET ?",
            )
            .bind(tenant_id)
            .bind(organization_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|error| ServiceError::Repository(error.to_string()))?;
            (total, rows)
        };

        Ok(Page {
            items: rows.into_iter().map(Into::into).collect(),
            page,
            page_size,
            total: total.0 as u64,
        })
    }

    async fn list_plans(
        &self,
        tenant_id: &str,
        organization_id: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Page<Plan>, ServiceError> {
        let offset = ((page.saturating_sub(1)) * page_size) as i64;
        let limit = page_size as i64;
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM github_plan WHERE tenant_id = ? AND organization_id = ?",
        )
        .bind(tenant_id)
        .bind(organization_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|error| ServiceError::Repository(error.to_string()))?;

        let rows = sqlx::query_as::<_, PlanRow>(
            "SELECT id, tenant_id, organization_id, repository_id, title, status, created_at, updated_at
             FROM github_plan WHERE tenant_id = ? AND organization_id = ?
             ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        )
        .bind(tenant_id)
        .bind(organization_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| ServiceError::Repository(error.to_string()))?;

        Ok(Page {
            items: rows.into_iter().map(Into::into).collect(),
            page,
            page_size,
            total: total.0 as u64,
        })
    }
}

#[derive(sqlx::FromRow)]
struct RepositoryRow {
    id: String,
    tenant_id: String,
    organization_id: String,
    full_name: String,
    owner: String,
    description: Option<String>,
    default_branch: Option<String>,
    html_url: Option<String>,
    is_private: i64,
    created_at: String,
    updated_at: String,
}

impl From<RepositoryRow> for Repository {
    fn from(row: RepositoryRow) -> Self {
        Self {
            id: row.id,
            tenant_id: row.tenant_id,
            organization_id: row.organization_id,
            full_name: row.full_name,
            owner: row.owner,
            description: row.description,
            default_branch: row.default_branch,
            html_url: row.html_url,
            is_private: row.is_private != 0,
            created_at: parse_ts(&row.created_at),
            updated_at: parse_ts(&row.updated_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct IssueRow {
    id: String,
    tenant_id: String,
    organization_id: String,
    repository_id: String,
    number: i64,
    title: String,
    state: String,
    html_url: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<IssueRow> for Issue {
    fn from(row: IssueRow) -> Self {
        Self {
            id: row.id,
            tenant_id: row.tenant_id,
            organization_id: row.organization_id,
            repository_id: row.repository_id,
            number: row.number,
            title: row.title,
            state: row.state,
            html_url: row.html_url,
            created_at: parse_ts(&row.created_at),
            updated_at: parse_ts(&row.updated_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct PlanRow {
    id: String,
    tenant_id: String,
    organization_id: String,
    repository_id: Option<String>,
    title: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<PlanRow> for Plan {
    fn from(row: PlanRow) -> Self {
        Self {
            id: row.id,
            tenant_id: row.tenant_id,
            organization_id: row.organization_id,
            repository_id: row.repository_id,
            title: row.title,
            status: row.status,
            created_at: parse_ts(&row.created_at),
            updated_at: parse_ts(&row.updated_at),
        }
    }
}

fn parse_ts(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}
