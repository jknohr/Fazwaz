use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("ID error: {0}")]
    Id(#[from] IdError),

    #[error("External service error: {0}")]
    ExternalService(String),
}

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("Invalid UUID7 format: {0}")]
    InvalidFormat(String),
    #[error("Invalid UUID7 version")]
    InvalidVersion,
    #[error("UUID7 parse error: {0}")]
    ParseError(String),
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
}

impl From<StorageError> for Error {
    fn from(err: StorageError) -> Self {
        Error::ExternalService(err.to_string())
    }
} 