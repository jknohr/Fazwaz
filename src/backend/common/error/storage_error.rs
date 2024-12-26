#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Bucket operation failed: {0}")]
    BucketOperation(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Upload failed: {0}")]
    UploadFailed(String),
    
    #[error("Download failed: {0}")]
    DownloadFailed(String),
} 