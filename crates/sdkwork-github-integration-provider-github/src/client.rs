use std::time::Duration;

use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::StatusCode;

use crate::error::ProviderError;
use crate::types::{GitHubApiIssue, GitHubApiRepository, GitHubApiUser};

const DEFAULT_API_BASE: &str = "https://api.github.com";
pub(crate) const DEFAULT_USER_AGENT: &str = "sdkwork-github-integration/0.1.0";
const MAX_PAGES: u32 = 50;

#[derive(Clone)]
pub struct GitHubRestProvider {
    client: reqwest::Client,
    token: String,
    api_base: String,
}

impl GitHubRestProvider {
    pub fn from_env() -> Option<Self> {
        if !pat_fallback_allowed() {
            return None;
        }
        let token = std::env::var("SDKWORK_GITHUB_INTEGRATION_PAT")
            .ok()
            .filter(|value| !value.trim().is_empty())?;
        let api_base = std::env::var("SDKWORK_GITHUB_INTEGRATION_API_BASE")
            .unwrap_or_else(|_| DEFAULT_API_BASE.to_string());
        Some(Self::new(token, api_base))
    }

    pub fn new(token: impl Into<String>, api_base: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("build GitHub HTTP client");
        let api_base = api_base.into();
        Self {
            client,
            token: token.into(),
            api_base: api_base.trim_end_matches('/').to_string(),
        }
    }

    pub fn provider_key(&self) -> &'static str {
        "github"
    }

    pub async fn fetch_current_user(&self) -> Result<GitHubApiUser, ProviderError> {
        let url = format!("{}/user", self.api_base);
        self.get_json(&url).await
    }

    pub async fn fetch_repositories(&self) -> Result<Vec<GitHubApiRepository>, ProviderError> {
        let initial = format!("{}/user/repos?per_page=100&sort=updated", self.api_base);
        self.fetch_paginated(&initial).await
    }

    pub async fn fetch_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<GitHubApiIssue>, ProviderError> {
        let initial = format!(
            "{}/repos/{owner}/{repo}/issues?per_page=100&state=all",
            self.api_base
        );
        let issues: Vec<GitHubApiIssue> = self.fetch_paginated(&initial).await?;
        Ok(issues
            .into_iter()
            .filter(|issue| issue.number > 0 && !issue.is_pull_request())
            .collect())
    }

    async fn fetch_paginated<T>(&self, initial_url: &str) -> Result<Vec<T>, ProviderError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut items = Vec::new();
        let mut next_url = Some(initial_url.to_string());
        let mut pages = 0_u32;

        while let Some(url) = next_url.take() {
            pages += 1;
            if pages > MAX_PAGES {
                return Err(ProviderError::Request(format!(
                    "GitHub pagination exceeded {MAX_PAGES} pages for {initial_url}"
                )));
            }

            let response = self
                .client
                .get(&url)
                .header(USER_AGENT, DEFAULT_USER_AGENT)
                .header(ACCEPT, "application/vnd.github+json")
                .header(AUTHORIZATION, format!("Bearer {}", self.token))
                .send()
                .await
                .map_err(|error| ProviderError::Request(error.to_string()))?;

            let status = response.status();
            if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
                crate::metrics::record_failure();
                return Err(ProviderError::Configuration(
                    "GitHub integration token is invalid or lacks required scopes".to_string(),
                ));
            }
            if !status.is_success() {
                crate::metrics::record_failure();
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "<unreadable body>".to_string());
                return Err(ProviderError::Request(format!(
                    "GitHub API returned {status}: {body}"
                )));
            }

            let link = response
                .headers()
                .get(reqwest::header::LINK)
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned);
            let page_items = response
                .json::<Vec<T>>()
                .await
                .map_err(|error| ProviderError::Response(error.to_string()))?;
            items.extend(page_items);
            next_url = parse_next_link(link.as_deref());
            crate::metrics::record_success(Duration::from_millis(0));
        }

        Ok(items)
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, ProviderError> {
        let started = std::time::Instant::now();
        let response = self
            .client
            .get(url)
            .header(USER_AGENT, DEFAULT_USER_AGENT)
            .header(ACCEPT, "application/vnd.github+json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .await
            .map_err(|error| ProviderError::Request(error.to_string()))?;

        let status = response.status();
        if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
            crate::metrics::record_failure();
            return Err(ProviderError::Configuration(
                "GitHub integration token is invalid or lacks required scopes".to_string(),
            ));
        }
        if !status.is_success() {
            crate::metrics::record_failure();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable body>".to_string());
            return Err(ProviderError::Request(format!(
                "GitHub API returned {status}: {body}"
            )));
        }

        let payload = response
            .json::<T>()
            .await
            .map_err(|error| ProviderError::Response(error.to_string()))?;

        tracing::info!(
            provider = "github",
            operation = "http_get",
            status = %status,
            latency_ms = started.elapsed().as_millis(),
            "github provider request completed"
        );
        crate::metrics::record_success(started.elapsed());

        Ok(payload)
    }
}

pub fn pat_fallback_allowed() -> bool {
    if std::env::var("SDKWORK_GITHUB_INTEGRATION_PAT_FALLBACK_ENABLED")
        .map(|value| matches!(value.trim(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
    {
        return true;
    }
    !matches!(
        std::env::var("SDKWORK_GITHUB_ENVIRONMENT")
            .unwrap_or_default()
            .trim(),
        "production"
    )
}

fn parse_next_link(link_header: Option<&str>) -> Option<String> {
    let header = link_header?;
    for segment in header.split(',') {
        let segment = segment.trim();
        if segment.contains("rel=\"next\"") {
            let start = segment.find('<')? + 1;
            let end = segment.find('>')?;
            return Some(segment[start..end].to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{parse_next_link, pat_fallback_allowed, GitHubRestProvider};

    static ENV_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_env_lock<T>(run: impl FnOnce() -> T) -> T {
        let _guard = ENV_TEST_LOCK.lock().expect("env test lock");
        run()
    }

    #[test]
    fn from_env_returns_none_without_token() {
        with_env_lock(|| {
            std::env::remove_var("SDKWORK_GITHUB_INTEGRATION_PAT");
            std::env::remove_var("SDKWORK_GITHUB_ENVIRONMENT");
            std::env::remove_var("SDKWORK_GITHUB_INTEGRATION_PAT_FALLBACK_ENABLED");
            assert!(GitHubRestProvider::from_env().is_none());
        });
    }

    #[test]
    fn from_env_returns_none_in_production_without_explicit_fallback() {
        with_env_lock(|| {
            std::env::set_var("SDKWORK_GITHUB_INTEGRATION_PAT", "ghp_test");
            std::env::set_var("SDKWORK_GITHUB_ENVIRONMENT", "production");
            std::env::remove_var("SDKWORK_GITHUB_INTEGRATION_PAT_FALLBACK_ENABLED");
            assert!(GitHubRestProvider::from_env().is_none());
            std::env::remove_var("SDKWORK_GITHUB_INTEGRATION_PAT");
            std::env::remove_var("SDKWORK_GITHUB_ENVIRONMENT");
        });
    }

    #[test]
    fn parse_next_link_extracts_url() {
        let next = parse_next_link(Some(
            "<https://api.github.com/user/repos?page=2>; rel=\"next\", <https://api.github.com/user/repos?page=5>; rel=\"last\"",
        ));
        assert_eq!(
            next.as_deref(),
            Some("https://api.github.com/user/repos?page=2")
        );
    }

    #[test]
    fn production_environment_disallows_pat_fallback_by_default() {
        with_env_lock(|| {
            std::env::set_var("SDKWORK_GITHUB_ENVIRONMENT", "production");
            std::env::remove_var("SDKWORK_GITHUB_INTEGRATION_PAT_FALLBACK_ENABLED");
            assert!(!pat_fallback_allowed());
            std::env::remove_var("SDKWORK_GITHUB_ENVIRONMENT");
        });
    }
}
