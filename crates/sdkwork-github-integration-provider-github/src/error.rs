use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("provider request failed: {0}")]
    Request(String),
    #[error("provider response invalid: {0}")]
    Response(String),
}
