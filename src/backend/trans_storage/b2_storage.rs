// Move from src/services/storage/b2.rs
// This handles B2 storage operations 

use aws_sdk_s3::{
    Client,
    config::Credentials,
    primitives::ByteStream,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn, instrument};
use uuid7;
use async_trait::async_trait;
use prometheus::Registry;
use crate::backend::monitoring::metrics::StorageMetrics;
use crate::backend::{
    common::error::error::{Result, AppError, StorageError},
    common::validation::image_validation::{MAX_FILE_SIZE, ALLOWED_MIME_TYPES},
    common::config::StorageConfig,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct B2Config {
    pub key_id: String,
    pub key: String,
    pub country: String,
    pub district: String,
    pub subdistrict: String,
    pub bucket_name: String,
}

impl B2Config {
    pub fn validate(&self) -> Result<()> {
        if self.key_id.is_empty() || self.key.is_empty() {
            return Err(AppError::Validation("B2 credentials cannot be empty".into()));
        }
        if self.country.is_empty() || self.district.is_empty() || self.subdistrict.is_empty() {
            return Err(AppError::Validation("Location fields cannot be empty".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2FileInfo {
    pub file_id: String,
    pub file_name: String,
    pub content_type: String,
    pub content_length: i64,
    pub url: String,
}

#[async_trait]
pub trait StorageProvider: Send + Sync {
   async fn store_file(&self, data: Bytes, filename: &str, content_type: &str) -> Result<B2FileInfo>;
   async fn delete_file(&self, file_id: &str) -> Result<()>;
}

pub struct B2Storage {
    client: Client,
    config: StorageConfig,
    metrics: Arc<StorageMetrics>,
}

impl B2Storage {
    pub async fn new(config: StorageConfig, metrics: Arc<StorageMetrics>) -> Result<Self> {
        config.validate()?;

        let sdk_config = aws_sdk_s3::Config::builder()
            .endpoint_url(&config.endpoint)
            .credentials_provider(Credentials::new(
                &config.access_key,
                &config.secret_key,
                None,
                None,
                "B2Storage"
            ))
            .force_path_style(true)
            .build();

        Ok(Self {
            client: Client::from_conf(sdk_config),
            config,
            metrics,
        })
    }

    #[instrument(skip(self, data))]
    pub async fn store_file(&self, data: Bytes, filename: &str, content_type: &str) -> Result<B2FileInfo> {
        // Size validation
        if data.len() > MAX_FILE_SIZE {
            warn!("File size {} exceeds maximum allowed size {}", data.len(), MAX_FILE_SIZE);
            return Err(AppError::Validation(format!(
                "File size {} exceeds maximum allowed size {}", 
                data.len(), 
                MAX_FILE_SIZE
            )));
        }

        // MIME type validation
        if !ALLOWED_MIME_TYPES.contains(&content_type) {
            warn!("Invalid content type: {}", content_type);
            return Err(AppError::Validation(format!(
                "Invalid content type: {}. Expected one of: {:?}", 
                content_type,
                ALLOWED_MIME_TYPES
            )));
        }

        let timer = self.metrics.upload_duration.start_timer();
        
        let file_name = format!("images/{}/{}", uuid7::uuid7(), filename);
        info!("Uploading file to B2: {}", &file_name);

        let bucket_name = self.config.get_bucket_name();
        let data_len = data.len();

        let response = self.client
            .put_object()
            .bucket(&bucket_name)
            .key(&file_name)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| StorageError::UploadFailed(e.to_string()))?;

        timer.observe_duration();
        self.metrics.successful_uploads.inc();
        self.metrics.bytes_transferred.with_label_values(&["upload"]).inc_by(data_len as u64);

        Ok(B2FileInfo {
            file_id: response.e_tag().unwrap_or_default().to_string(),
            file_name: file_name.to_string(),
            content_type: content_type.to_string(),
            content_length: data_len as i64,
            url: format!("{}/{}/{}", 
                self.config.endpoint,
                bucket_name,
                &file_name
            ),
        })
    }

    pub async fn delete_file(&self, file_id: &str) -> Result<()> {
        info!("Deleting file from B2: {}", file_id);
        
        let timer = self.metrics.download_duration.start_timer();
        let bucket_name = self.config.get_bucket_name();
        
        self.client
            .delete_object()
            .bucket(&bucket_name)
            .key(file_id)
            .send()
            .await
            .map_err(|e| StorageError::BucketOperation(e.to_string()))?;
            
        timer.observe_duration();
        self.metrics.bucket_operations.with_label_values(&["delete"]).inc();
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_file(&self, file_id: &str) -> Result<Bytes> {
        info!("Downloading file from B2: {}", file_id);
        
        let timer = self.metrics.download_duration.start_timer();
        let bucket_name = self.config.get_bucket_name();
        
        let response = self.client
            .get_object()
            .bucket(&bucket_name)
            .key(file_id)
            .send()
            .await
            .map_err(|e| StorageError::DownloadFailed(e.to_string()))?;
            
        let data = response.body.collect().await?.into_bytes();
        timer.observe_duration();
        self.metrics.bytes_transferred.with_label_values(&["download"]).inc_by(data.len() as u64);
        
        Ok(data)
    }

    #[instrument(skip(self))]
    pub async fn get_file_info(&self, file_id: &str) -> Result<B2FileInfo> {
        info!("Getting file info from B2: {}", file_id);
        
        let bucket_name = self.config.get_bucket_name();
        
        let head = self.client
            .head_object()
            .bucket(&bucket_name)
            .key(file_id)
            .send()
            .await
            .map_err(|e| StorageError::FileNotFound(e.to_string()))?;
            
        Ok(B2FileInfo {
            file_id: head.e_tag.unwrap_or_default(),
            file_name: file_id.to_string(),
            content_type: head.content_type.unwrap_or_default(),
            content_length: head.content_length.unwrap_or(0),
            url: format!("{}/{}/{}", 
                self.config.endpoint,
                bucket_name,
                file_id
            ),
        })
    }

    #[instrument(skip(self))]
    pub async fn list_files(&self, prefix: Option<&str>) -> Result<Vec<B2FileInfo>> {
        let timer = self.metrics.collection_duration.start_timer();
        let bucket_name = self.config.get_bucket_name();
        
        let list = self.client
            .list_objects_v2()
            .bucket(&bucket_name)
            .prefix(prefix.unwrap_or(""))
            .send()
            .await
            .map_err(|e| StorageError::BucketOperation(e.to_string()))?;
            
        let files = list.contents.unwrap_or_default();
        
        // Update metrics
        self.metrics.files_stored.set(files.len() as i64);
        let total_bytes: u64 = files.iter()
            .map(|obj| obj.size.unwrap_or(0) as u64)
            .sum();
        self.metrics.total_storage_bytes.set(total_bytes as i64);
        
        timer.observe_duration();
        
        Ok(files.into_iter()
            .map(|obj| {
                let key = obj.key.unwrap_or_default();
                B2FileInfo {
                    file_id: obj.e_tag.unwrap_or_default(),
                    file_name: key.clone(),
                    content_type: "application/octet-stream".to_string(),
                    content_length: obj.size.unwrap_or(0),
                    url: format!("{}/{}/{}", 
                        self.config.endpoint,
                        bucket_name,
                        key
                    ),
                }
            })
            .collect())
    }

    pub async fn upload_file(&self, path: &str, data: &[u8], content_type: &str) -> Result<String> {
        let timer = self.metrics.upload_duration.start_timer();
        
        let bucket_name = self.config.get_bucket_name();
        let data_len = data.len();

        let response = self.client
            .put_object()
            .bucket(&bucket_name)
            .key(path)
            .body(ByteStream::from(data.to_vec()))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| StorageError::UploadFailed(e.to_string()))?;

        timer.observe_duration();
        self.metrics.successful_uploads.inc();
        self.metrics.bytes_transferred.with_label_values(&["upload"]).inc_by(data_len as u64);

        Ok(response.e_tag().unwrap_or_default().to_string())
    }

    pub async fn download_file(&self, path: &str) -> Result<Vec<u8>> {
        let timer = self.metrics.download_duration.start_timer();
        let bucket_name = self.config.get_bucket_name();
        
        let response = self.client
            .get_object()
            .bucket(&bucket_name)
            .key(path)
            .send()
            .await
            .map_err(|e| StorageError::DownloadFailed(e.to_string()))?;
            
        let data = response.body.collect().await?.into_bytes();
        timer.observe_duration();
        self.metrics.bytes_transferred.with_label_values(&["download"]).inc_by(data.len() as u64);
        
        Ok(data.to_vec())
    }
}

#[async_trait]
impl StorageProvider for B2Storage {
    #[instrument(skip(self, data))]
    async fn store_file(&self, data: Bytes, filename: &str, content_type: &str) -> Result<B2FileInfo> {
        self.store_file(data, filename, content_type).await
    }

    #[instrument(skip(self))]
    async fn delete_file(&self, file_id: &str) -> Result<()> {
        self.delete_file(file_id).await
    }
} 
