use crate::backend::common::{Result, AppError, StorageError};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Storage error: {0}")]
    Generic(String),
    
    #[error("File not found: {0}")]
    NotFound(String),
    
    #[error("Upload failed: {0}")]
    UploadFailed(String),
} 