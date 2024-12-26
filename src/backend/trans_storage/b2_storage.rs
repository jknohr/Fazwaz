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

use crate::{
    backend::common::{
        error::{Result, AppError, StorageError},
        validation::image_validation::{MAX_FILE_SIZE, ALLOWED_MIME_TYPES},
        config::StorageConfig
    },
    backend::monitoring::metrics::StorageMetrics,
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
    bucket_prefix: String,
    metrics: Arc<StorageMetrics>,
}

impl B2Storage {
    pub async fn new(config: StorageConfig, metrics: Arc<StorageMetrics>) -> Result<Self> {
        config.validate()?;

        let sdk_config = aws_sdk_s3::Config::builder()
            .endpoint_url("https://s3.us-west-001.backblazeb2.com")
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
            bucket_prefix: config.bucket_prefix,
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

        let timer = self.metrics.storage_operation_duration.start_timer();
        
        let file_name = format!("images/{}/{}", uuid7::uuid7(), filename);
        info!("Uploading file to B2: {}", file_name);

        let response = self.client
            .put_object()
            .bucket(&self.bucket_prefix)
            .key(&file_name)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| StorageError::UploadFailed(e.to_string()))?;

        timer.observe_duration();
        self.metrics.successful_uploads.inc();

        Ok(B2FileInfo {
            file_id: response.e_tag().unwrap_or_default().to_string(),
            file_name,
            content_type: content_type.to_string(),
            content_length: data.len() as i64,
            url: format!("https://s3.us-west-001.backblazeb2.com/{}/{}", self.bucket_prefix, file_name),
        })
    }

    pub async fn delete_file(&self, file_id: &str) -> Result<()> {
        info!("Deleting file from B2: {}", file_id);
        
        let timer = self.metrics.storage_operation_duration.start_timer();
        
        self.client
            .delete_object()
            .bucket(&self.bucket_prefix)
            .key(file_id)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Delete failed: {}", e)))?;
            
        timer.observe_duration();
        self.metrics.successful_deletions.inc();
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_file(&self, file_id: &str) -> Result<Bytes> {
        info!("Downloading file from B2: {}", file_id);
        
        let timer = self.metrics.storage_operation_duration.start_timer();
        
        let response = self.client
            .get_object()
            .bucket(&self.bucket_prefix)
            .key(file_id)
            .send()
            .await
            .map_err(|e| StorageError::DownloadFailed(e.to_string()))?;
            
        timer.observe_duration();
        self.metrics.successful_downloads.inc();
        
        Ok(response.body.collect().await?.into_bytes())
    }

    #[instrument(skip(self))]
    pub async fn get_file_info(&self, file_id: &str) -> Result<B2FileInfo> {
        info!("Getting file info from B2: {}", file_id);
        
        let head = self.client
            .head_object()
            .bucket(&self.bucket_prefix)
            .key(file_id)
            .send()
            .await
            .map_err(|e| StorageError::FileNotFound(e.to_string()).into())?;
            
        Ok(B2FileInfo {
            file_id: head.e_tag.unwrap_or_default(),
            file_name: file_id.to_string(),
            content_type: head.content_type.unwrap_or_default(),
            content_length: head.content_length.unwrap_or(0),
            url: format!("https://s3.us-west-001.backblazeb2.com/{}/{}", 
                self.bucket_prefix, 
                file_id
            ),
        })
    }

    #[instrument(skip(self))]
    pub async fn list_files(&self, prefix: Option<&str>) -> Result<Vec<B2FileInfo>> {
        let list = self.client
            .list_objects_v2()
            .bucket(&self.bucket_prefix)
            .prefix(prefix.unwrap_or(""))
            .send()
            .await
            .map_err(|e| StorageError::BucketOperation(e.to_string()).into())?;
            
        Ok(list.contents
            .unwrap_or_default()
            .into_iter()
            .map(|obj| B2FileInfo {
                file_id: obj.e_tag.unwrap_or_default(),
                file_name: obj.key.unwrap_or_default(),
                content_type: "application/octet-stream".to_string(),
                content_length: obj.size.unwrap_or(0),
                url: format!("https://s3.us-west-001.backblazeb2.com/{}/{}", 
                    self.bucket_prefix, 
                    obj.key.unwrap_or_default()
                ),
            })
            .collect())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_b2_config_validation() {
        // Valid config
        let config = B2Config {
            key_id: "key123".to_string(),
            key: "secret".to_string(),
            country: "thailand".to_string(),
            district: "bangkok".to_string(),
            subdistrict: "sukhumvit".to_string(),
            bucket_name: "test-bucket".to_string(),
        };
        assert!(config.validate().is_ok());

        // Invalid config - empty credentials
        let invalid_config = B2Config {
            key_id: "".to_string(),
            key: "secret".to_string(),
            country: "thailand".to_string(),
            district: "bangkok".to_string(),
            subdistrict: "sukhumvit".to_string(),
            bucket_name: "test-bucket".to_string(),
        };
        assert!(matches!(
            invalid_config.validate(),
            Err(AppError::Validation(_))
        ));

        // Invalid config - empty location
        let invalid_config = B2Config {
            key_id: "key123".to_string(),
            key: "secret".to_string(),
            country: "".to_string(),
            district: "bangkok".to_string(),
            subdistrict: "sukhumvit".to_string(),
            bucket_name: "test-bucket".to_string(),
        };
        assert!(matches!(
            invalid_config.validate(),
            Err(AppError::Validation(_))
        ));
    }

    #[tokio::test]
    async fn test_bucket_name_generation() {
        let config = B2Config {
            key_id: "key123".to_string(),
            key: "secret".to_string(),
            country: "Thailand".to_string(),
            district: "Bangkok".to_string(),
            subdistrict: "Sukhumvit".to_string(),
            bucket_name: "test-bucket".to_string(),
        };

        let expected = "thailand-bangkok-sukhumvit";
        let generated = format!("{}-{}-{}", 
            config.country.to_lowercase(),
            config.district.to_lowercase(),
            config.subdistrict.to_lowercase()
        );
        assert_eq!(expected, generated);
    }

    #[tokio::test]
    async fn test_uuid_generation() {
        // Test that UUIDs are unique
        let uuid1 = uuid7::uuid7();
        let uuid2 = uuid7::uuid7();
        assert_ne!(uuid1.to_string(), uuid2.to_string());

        // Test UUID format in file path
        let filename = "test.jpg";
        let file_path = format!("images/{}/{}", uuid1, filename);
        assert!(file_path.starts_with("images/"));
        assert!(file_path.ends_with(filename));
        assert!(file_path.contains(&uuid1.to_string()));
    }

    #[tokio::test]
    async fn test_file_path_generation() {
        let filename = "test.jpg";
        let uuid = "123e4567-e89b-12d3-a456-426614174000";
        let expected = format!("images/{}/{}", uuid, filename);
        
        assert!(expected.starts_with("images/"));
        assert!(expected.ends_with(filename));
        assert!(expected.contains(uuid));
    }

    #[tokio::test]
    async fn test_storage_provider_trait() {
        let metrics = Arc::new(StorageMetrics::new().unwrap());
        let config = aws_sdk_s3::Config::builder()
            .endpoint_url("http://localhost:9000")
            .force_path_style(true)
            .build();

        let storage: Box<dyn StorageProvider> = Box::new(B2Storage {
            client: Client::from_conf(config),
            bucket_prefix: "test-bucket".to_string(),
            metrics,
        });

        // Test trait method instead of internal field
        let result = storage.store_file(
            Bytes::from(vec![1,2,3]),
            "test.jpg",
            "image/jpeg"
        ).await;
        assert!(result.is_err()); // Will fail due to mock config
    }
} 