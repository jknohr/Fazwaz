use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Storage error: {0}")]
    Generic(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("File not found: {0}")]
    NotFound(String),
    
    #[error("Upload failed: {0}")]
    UploadFailed(String),
} 