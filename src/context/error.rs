use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("{0}")]
    InitializationError(String),
}
