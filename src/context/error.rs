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

    #[error("{0}")]
    NoFrameError(String),

    #[error("{0}")]
    NoScopeError(String),

    #[error("{0}")]
    ScopeError(String),
}

#[derive(Debug, Error)]
pub enum OutMapError {
    #[error("{0}")]
    FileMissingError(String),

    #[error("{0}")]
    WriteError(String),
}

#[derive(Debug, Error)]
pub enum OpenFilesError {
    #[error("{0}")]
    FileOpenFailed(String),
}
