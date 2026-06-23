use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("repository error: {0}")]
    Repository(String),
    #[error("validation error: {0}")]
    Validation(String),
}
