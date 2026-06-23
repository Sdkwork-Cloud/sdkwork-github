use sdkwork_utils_rust::string::is_blank;

use crate::domain::{Issue, Page, Plan, Repository};
use crate::error::ServiceError;
use crate::ports::GitHubStore;

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

fn validate_scope(tenant_id: &str, organization_id: &str) -> Result<(), ServiceError> {
    if is_blank(Some(tenant_id)) || is_blank(Some(organization_id)) {
        return Err(ServiceError::Validation(
            "tenant_id and organization_id are required".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_scope;
    use crate::error::ServiceError;

    #[test]
    fn rejects_blank_scope() {
        let error = validate_scope("", "org").unwrap_err();
        assert!(matches!(error, ServiceError::Validation(_)));
    }
}
