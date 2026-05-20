use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileStoreError {
    #[error("{0}")]
    InvalidPath(String),

    #[error("{0}")]
    DuplicatePath(String),
}
