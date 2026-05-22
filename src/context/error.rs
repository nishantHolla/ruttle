use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("{0}")]
    InitializationError(String),

    #[error("{0}")]
    GenerationError(String),

    #[error("{0}")]
    FinalizationError(String),

    #[error("{0}")]
    DuplicatePush(String),
}

#[derive(Debug, Error)]
pub enum OutMapError {
    #[error("{0}")]
    FileMissingError(String),

    #[error("{0}")]
    WriteError(String),
}
