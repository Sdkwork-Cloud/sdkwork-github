use std::time::Duration;

use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

use crate::client::DEFAULT_USER_AGENT;
use crate::error::ProviderError;

const GITHUB_AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
pub const DEFAULT_SCOPES: &str = "read:user,repo";

#[derive(Clone, Debug)]
pub struct OAuthExchangeResult {
    pub access_token: String,
    pub scope: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GitHubOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: String,
}

impl GitHubOAuthConfig {
    pub fn from_env() -> Result<Self, ProviderError> {
        let client_id = std::env::var("SDKWORK_GITHUB_OAUTH_CLIENT_ID")
            .map_err(|_| ProviderError::Configuration("SDKWORK_GITHUB_OAUTH_CLIENT_ID is not configured".to_string()))?;
        let client_secret = std::env::var("SDKWORK_GITHUB_OAUTH_CLIENT_SECRET")
            .map_err(|_| ProviderError::Configuration("SDKWORK_GITHUB_OAUTH_CLIENT_SECRET is not configured".to_string()))?;
        let redirect_uri = std::env::var("SDKWORK_GITHUB_OAUTH_REDIRECT_URI")
            .map_err(|_| ProviderError::Configuration("SDKWORK_GITHUB_OAUTH_REDIRECT_URI is not configured".to_string()))?;
        let scopes = std::env::var("SDKWORK_GITHUB_OAUTH_SCOPES")
            .unwrap_or_else(|_| DEFAULT_SCOPES.to_string());
        if client_id.trim().is_empty() || client_secret.trim().is_empty() || redirect_uri.trim().is_empty() {
            return Err(ProviderError::Configuration(
                "GitHub OAuth configuration is incomplete".to_string(),
            ));
        }
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            scopes,
        })
    }

    pub fn build_authorization_url(&self, state: &str) -> String {
        format!(
            "{GITHUB_AUTHORIZE_URL}?client_id={}&redirect_uri={}&scope={}&state={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&self.scopes),
            urlencoding::encode(state),
        )
    }
}

#[derive(Clone)]
pub struct GitHubOAuthClient {
    config: GitHubOAuthConfig,
    client: reqwest::Client,
}

impl GitHubOAuthClient {
    pub fn from_env() -> Result<Self, ProviderError> {
        Ok(Self {
            config: GitHubOAuthConfig::from_env()?,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|error| ProviderError::Request(error.to_string()))?,
        })
    }

    pub fn build_authorization_url(&self, state: &str) -> String {
        self.config.build_authorization_url(state)
    }

    pub fn configured_scopes(&self) -> &str {
        self.config.scopes.as_str()
    }

    pub async fn exchange_code(&self, code: &str) -> Result<OAuthExchangeResult, ProviderError> {
        let started = std::time::Instant::now();
        let response = self
            .client
            .post(GITHUB_TOKEN_URL)
            .header(USER_AGENT, DEFAULT_USER_AGENT)
            .header(ACCEPT, "application/json")
            .json(&serde_json::json!({
                "client_id": self.config.client_id,
                "client_secret": self.config.client_secret,
                "code": code,
                "redirect_uri": self.config.redirect_uri,
            }))
            .send()
            .await
            .map_err(|error| ProviderError::Request(error.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable body>".to_string());
            crate::metrics::record_failure();
            return Err(ProviderError::Request(format!(
                "GitHub OAuth token exchange returned {status}: {body}"
            )));
        }

        let payload: OAuthTokenResponse = response
            .json()
            .await
            .map_err(|error| ProviderError::Response(error.to_string()))?;

        if let Some(error) = payload.error {
            crate::metrics::record_failure();
            return Err(ProviderError::Request(format!(
                "GitHub OAuth token exchange failed: {error}"
            )));
        }

        let access_token = payload.access_token.ok_or_else(|| {
            ProviderError::Response("GitHub OAuth token exchange returned no access_token".to_string())
        })?;

        crate::metrics::record_success(started.elapsed());
        Ok(OAuthExchangeResult {
            access_token,
            scope: payload.scope,
        })
    }
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: Option<String>,
    scope: Option<String>,
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::DEFAULT_SCOPES;

    #[test]
    fn default_scopes_include_user_and_repo() {
        assert!(DEFAULT_SCOPES.contains("read:user"));
        assert!(DEFAULT_SCOPES.contains("repo"));
    }
}
