use std::time::Duration;

use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::StatusCode;

use crate::client::DEFAULT_USER_AGENT;
use crate::error::ProviderError;
use crate::types::GitHubApiRepository;

const DEFAULT_API_BASE: &str = "https://api.github.com";

#[derive(Clone)]
pub struct GitHubPublicApiClient {
    client: reqwest::Client,
    api_base: String,
}

impl GitHubPublicApiClient {
    pub fn new() -> Self {
        Self::with_api_base(DEFAULT_API_BASE)
    }

    pub fn with_api_base(api_base: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("build GitHub public HTTP client"),
            api_base: api_base.into().trim_end_matches('/').to_string(),
        }
    }

    pub async fn fetch_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<GitHubApiRepository, ProviderError> {
        let url = format!("{}/repos/{owner}/{repo}", self.api_base);
        let started = std::time::Instant::now();
        let response = self
            .client
            .get(&url)
            .header(USER_AGENT, DEFAULT_USER_AGENT)
            .header(ACCEPT, "application/vnd.github+json")
            .send()
            .await
            .map_err(|error| ProviderError::Request(error.to_string()))?;

        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            crate::metrics::record_failure();
            return Err(ProviderError::Request(format!(
                "GitHub repository {owner}/{repo} was not found"
            )));
        }
        if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
            crate::metrics::record_failure();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable body>".to_string());
            return Err(ProviderError::Request(format!(
                "GitHub API rate limit or access denied ({status}): {body}"
            )));
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

        let repository = response
            .json::<GitHubApiRepository>()
            .await
            .map_err(|error| ProviderError::Response(error.to_string()))?;
        crate::metrics::record_success(started.elapsed());
        Ok(repository)
    }
}

impl Default for GitHubPublicApiClient {
    fn default() -> Self {
        Self::new()
    }
}
