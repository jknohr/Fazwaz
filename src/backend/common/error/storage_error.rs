use thiserror::Error;

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