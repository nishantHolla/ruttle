use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgsError {
    #[error("{0}")]
    InvalidArgument(String),
}
