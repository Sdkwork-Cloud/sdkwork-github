mod client;
mod credential;
mod error;
mod metrics;
mod oauth;
mod public_api;
mod types;

pub use client::{pat_fallback_allowed, GitHubRestProvider};
pub use credential::GitHubCredentialCipher;
pub use error::ProviderError;
pub use metrics::{ProviderMetricsSnapshot, snapshot as provider_metrics_snapshot};
pub use oauth::{GitHubOAuthClient, GitHubOAuthConfig, OAuthExchangeResult};
pub use public_api::GitHubPublicApiClient;
pub use types::{GitHubApiIssue, GitHubApiRepository, GitHubApiUser};
