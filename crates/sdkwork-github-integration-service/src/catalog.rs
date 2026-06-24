use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::ServiceError;

#[derive(Debug, Clone, Deserialize)]
pub struct NotableRepositoryCatalog {
    pub repositories: Vec<NotableRepositoryEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotableRepositoryEntry {
    pub owner: String,
    pub name: String,
    pub plan_title: String,
    #[serde(default)]
    pub sync_issues: bool,
    #[serde(default = "default_max_issues")]
    pub max_issues: u32,
}

fn default_max_issues() -> u32 {
    10
}

pub fn load_notable_repository_catalog(app_root: &Path) -> Result<NotableRepositoryCatalog, ServiceError> {
    let path = app_root.join("database/catalog/notable-github-repositories.json");
    let raw = std::fs::read_to_string(&path).map_err(|error| {
        ServiceError::Configuration(format!(
            "read notable repository catalog at {} failed: {error}",
            path.display()
        ))
    })?;
    serde_json::from_str(&raw).map_err(|error| {
        ServiceError::Configuration(format!(
            "parse notable repository catalog at {} failed: {error}",
            path.display()
        ))
    })
}

pub fn resolve_catalog_app_root() -> PathBuf {
    std::env::var("SDKWORK_GITHUB_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        })
}

#[cfg(test)]
mod tests {
    use super::{load_notable_repository_catalog, resolve_catalog_app_root};

    #[test]
    fn loads_notable_repository_catalog_from_app_root() {
        let catalog = load_notable_repository_catalog(&resolve_catalog_app_root()).expect("catalog");
        assert!(catalog.repositories.len() >= 5);
        assert!(catalog.repositories.iter().any(|entry| entry.owner == "microsoft"));
    }
}
