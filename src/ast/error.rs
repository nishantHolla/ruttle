use thiserror::Error;

#[derive(Debug, Error)]
pub enum AstError {
    #[error("{0}")]
    FileNotFound(String),

    #[error("{0}")]
    ConstructionFailed(String),

    #[error("{0}")]
    UnclosedDirective(String),

    #[error("{0}")]
    UnknownDirective(String),

    #[error("{0}")]
    InvalidSyntax(String),

    #[error("{0}")]
    EvaluationFailed(String),
}
