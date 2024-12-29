use thiserror::Error;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::Error as SmtpError;
use handlebars::RenderError;
use handlebars::TemplateError;
use surrealdb::Error as SurrealError;
use axum::extract::multipart::MultipartError;
use crate::backend::f_ai_database::LogFormat::Json;
use aws_sdk_s3::{
    primitives::ByteStreamError,
    error::SdkError,
    operation::{
        get_object::GetObjectError,
        put_object::PutObjectError,
        delete_object::DeleteObjectError,
    },
};
use tokio::sync::mpsc::error::SendError;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use lettre::address::AddressError;
use config::ConfigError;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Email error: {0}")]
    Email(String),
    
    #[error("SMTP error: {0}")]
    Smtp(#[from] SmtpError),
    
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Image validation error: {0}")]
    ImageValidation(#[from] ImageValidationError),

    #[error("Image processing error: {0}")]
    ImageProcessing(String),
}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: usize, max: usize },

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid file type: found {found}, expected one of [{expected}]")]
    InvalidFileType { found: String, expected: String },

    #[error("Failed to load image: {0}")]
    LoadError(String),

    #[error("Image too small: {width}x{height} (min: {min_width}x{min_height})")]
    TooSmall { width: u32, height: u32, min_width: u32, min_height: u32 },

    #[error("Image too large: {width}x{height} (max: {max_width}x{max_height})")]
    TooLarge { width: u32, height: u32, max_width: u32, max_height: u32 },

    #[error("Failed to convert image: {0}")]
    ConversionError(String),
}

#[derive(Debug, Error)]
pub enum ImageValidationError {
    #[error("Image dimensions must be between 1080p and 4K, got {width}x{height}")]
    InvalidDimensions {
        width: u32,
        height: u32,
    },

    #[error("Invalid image format: {0}")]
    InvalidFormat(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),
}

// AWS S3 error conversions
impl From<ByteStreamError> for AppError {
    fn from(err: ByteStreamError) -> Self {
        AppError::Storage(StorageError::DownloadFailed(err.to_string()))
    }
}

impl From<SdkError<GetObjectError>> for AppError {
    fn from(err: SdkError<GetObjectError>) -> Self {
        AppError::Storage(StorageError::DownloadFailed(err.to_string()))
    }
}

impl From<SdkError<PutObjectError>> for AppError {
    fn from(err: SdkError<PutObjectError>) -> Self {
        AppError::Storage(StorageError::UploadFailed(err.to_string()))
    }
}

impl From<SdkError<DeleteObjectError>> for AppError {
    fn from(err: SdkError<DeleteObjectError>) -> Self {
        AppError::Storage(StorageError::FileNotFound(err.to_string()))
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl<T> From<SendError<T>> for AppError {
    fn from(err: SendError<T>) -> Self {
        AppError::Internal(format!("Channel send error: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<prometheus::Error> for AppError {
    fn from(err: prometheus::Error) -> Self {
        AppError::Internal(format!("Prometheus error: {}", err))
    }
}

impl From<tokio::sync::AcquireError> for AppError {
    fn from(err: tokio::sync::AcquireError) -> Self {
        AppError::Internal(format!("Failed to acquire semaphore: {}", err))
    }
}

impl From<AddressError> for AppError {
    fn from(err: AddressError) -> Self {
        AppError::Email(format!("Invalid email address: {}", err))
    }
}

impl From<TemplateError> for AppError {
    fn from(err: TemplateError) -> Self {
        AppError::Template(format!("Template registration error: {}", err))
    }
}

impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::Configuration(err.to_string())
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::ImageError(ImageError::LoadError(err.to_string()))
    }
}

impl From<rexiv2::Error> for AppError {
    fn from(err: rexiv2::Error) -> Self {
        AppError::ImageError(ImageError::ConversionError(err.to_string()))
    }
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Storage(_) => StatusCode::BAD_REQUEST,
            AppError::ImageError(_) => StatusCode::BAD_REQUEST,
            AppError::ParseError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidTimestamp(_) => StatusCode::BAD_REQUEST,
            AppError::ExternalService(_) => StatusCode::BAD_GATEWAY,
            // All other errors are treated as internal server errors
            AppError::Internal(_) |
            AppError::Database(_) |
            AppError::Email(_) |
            AppError::Smtp(_) |
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::ImageValidation(_) => StatusCode::BAD_REQUEST,
            AppError::ImageProcessing(_) => StatusCode::BAD_REQUEST,
        };

        (status, self.to_string()).into_response()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
} 