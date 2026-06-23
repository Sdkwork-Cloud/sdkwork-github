use std::time::Duration;

use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::StatusCode;

use crate::error::ProviderError;
use crate::types::{GitHubApiIssue, GitHubApiRepository};

const DEFAULT_API_BASE: &str = "https://api.github.com";
pub(crate) const DEFAULT_USER_AGENT: &str = "sdkwork-github-integration/0.1.0";

#[derive(Clone)]
pub struct GitHubRestProvider {
    client: reqwest::Client,
    token: String,
    api_base: String,
}

impl GitHubRestProvider {
    pub fn from_env() -> Option<Self> {
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

    pub async fn fetch_repositories(&self) -> Result<Vec<GitHubApiRepository>, ProviderError> {
        let url = format!("{}/user/repos?per_page=100&sort=updated", self.api_base);
        self.get_json(&url).await
    }

    pub async fn fetch_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<GitHubApiIssue>, ProviderError> {
        let url = format!(
            "{}/repos/{owner}/{repo}/issues?per_page=100&state=all",
            self.api_base
        );
        let issues: Vec<GitHubApiIssue> = self.get_json(&url).await?;
        Ok(issues.into_iter().filter(|issue| issue.number > 0).collect())
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

#[cfg(test)]
mod tests {
    use super::GitHubRestProvider;

    #[test]
    fn from_env_returns_none_without_token() {
        std::env::remove_var("SDKWORK_GITHUB_INTEGRATION_PAT");
        assert!(GitHubRestProvider::from_env().is_none());
    }
}
