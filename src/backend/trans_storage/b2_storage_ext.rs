use std::sync::Arc;
use bytes::{Bytes, ByteStream};
use anyhow::Result;
use tracing::{info, warn};

use crate::backend::{
    common::error::error::StorageError,
    monitoring::metrics::HealthMetrics,
};

// Extension traits for B2Storage
impl B2Storage {
    pub async fn upload_file_with_metrics(
        &self, 
        path: &str, 
        data: &[u8], 
        content_type: &str,
        metrics: &HealthMetrics
    ) -> Result<B2FileInfo> {
        let timer = metrics.upload_duration.start_timer();
        
        let bucket_name = self.bucket_name.clone();
        let data_len = data.len();

        let response = self.client.lock().await
            .upload_file()
            .url_auth(&self.get_upload_auth().await?)
            .file_name(path)
            .content_type(content_type)
            .content_length(data_len as u64)
            .body(data.to_vec())
            .send()
            .await?;

        timer.observe_duration();
        metrics.successful_uploads.inc();
        metrics.bytes_transferred.with_label_values(&["upload"]).inc_by(data_len as u64);

        Ok(B2FileInfo {
            file_id: response.file_id().to_string(),
            file_name: path.to_string(),
            content_type: content_type.to_string(),
            content_length: data_len as i64,
            url: format!("{}/file/{}/{}", 
                self.client.lock().await.authorization().download_url(),
                bucket_name,
                path
            ),
        })
    }

    // Add other extended functionality here...
}

#[derive(Debug, Clone)]
pub struct B2FileInfo {
    pub file_id: String,
    pub file_name: String,
    pub content_type: String,
    pub content_length: i64,
    pub url: String,
} 