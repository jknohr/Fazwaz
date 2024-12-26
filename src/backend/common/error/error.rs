use thiserror::Error;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::Error as SmtpError;
use handlebars::RenderError;
use surrealdb::Error as SurrealError;
use axum::extract::multipart::MultipartError;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Email error: {0}")]
    Email(#[from] LettreError),
    #[error("SMTP error: {0}")]
    Smtp(#[from] SmtpError),
    #[error("Template error: {0}")]
    Template(#[from] RenderError),
    #[error("Database error: {0}")]
    Database(#[from] SurrealError),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge {
        size: usize,
        max: usize,
    },
    #[error("Invalid file type: found {found}, expected one of {expected:?}")]
    InvalidFileType {
        found: String,
        expected: Vec<String>,
    },
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Upload failed: {0}")]
    UploadFailed(String),
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Bucket operation failed: {0}")]
    BucketOperation(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::Internal(err.to_string())
    }
} 