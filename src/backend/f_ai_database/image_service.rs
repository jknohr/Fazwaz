use crate::backend::common::types::image_types::*;
use surrealdb::{Surreal, engine::remote::ws::Client};
use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn, instrument};
use super::image_model::ImageModel;
use bytes::Bytes;
use futures::future::{self, TryFutureExt};
use tokio::task::JoinError;

pub struct ImageService {
    model: ImageModel,
    storage: Arc<B2Storage>,
}

impl ImageService {
    pub fn new(db: Arc<Surreal<Client>>, storage: Arc<B2Storage>) -> Self {
        Self {
            model: ImageModel::new(db),
            storage,
        }
    }

    #[instrument(skip(self, metadata))]
    pub async fn store_image_metadata(&self, metadata: ImageMetadata) -> Result<String> {
        if metadata.id.is_empty() {
            warn!("Attempted to store image metadata with empty ID");
        }
        
        info!("Storing image metadata");
        let id = self.model.create(metadata).await?;
        
        if id.is_empty() {
            warn!("Created image record but received empty ID");
        } else {
            info!(image_id = %id, "Image metadata stored successfully");
        }
        Ok(id)
    }

    #[instrument(skip(self))]
    pub async fn process_text(&self, id: &str, text: &str) -> Result<ImageMetadata> {
        if text.is_empty() {
            warn!("Processing attempted with empty text");
        }
        info!(id, "Processing text for image");
        self.model.update_embedding(id, text).await
    }

    #[instrument(skip(self))]
    pub async fn search_similar(&self, query_text: &str, limit: usize) -> Result<Vec<ImageMetadata>> {
        if query_text.is_empty() {
            warn!("Service: Search attempted with empty query text");
        }
        if limit == 0 {
            warn!("Service: Search attempted with zero limit");
            return Ok(Vec::new());
        }

        info!(limit, "Searching for similar images via service");
        let results = self.model.search_similar(query_text, limit).await?;
        
        if results.is_empty() {
            warn!(query_text, "Service: No similar images found");
        } else {
            info!(found = results.len(), "Found similar images");
        }
        Ok(results.into_iter().map(|(metadata, _)| metadata).collect())
    }

    #[instrument(skip(self))]
    pub async fn record_upload(&self,
        listing_id: &str,
        filename: &str,
        original_filename: &str,
        b2_file_id: &str,
        b2_bucket_id: &str,
        b2_url: &str,
        size: i32,
        mime_type: &str,
        width: i32,
        height: i32,
    ) -> Result<String> {
        if listing_id.is_empty() || filename.is_empty() {
            warn!("Required upload fields missing");
            return Err(anyhow::anyhow!("Missing required fields").into());
        }

        info!(listing_id, filename, "Recording image upload via service");
        self.model.record_upload(
            listing_id, filename, original_filename,
            b2_file_id, b2_bucket_id, b2_url,
            size, mime_type, width, height
        ).await
    }

    #[instrument(skip(self))]
    pub async fn get_metadata(&self, id: &str) -> Result<Option<ImageMetadata>> {
        if id.is_empty() {
            warn!("Attempted to get metadata with empty ID");
            return Ok(None);
        }

        info!(id, "Fetching image metadata");
        let result = self.model.get_metadata(id).await?;
        
        if result.is_none() {
            warn!(id, "Image metadata not found");
        }
        Ok(result)
    }

    #[instrument(skip(self, data))]
    pub async fn upload_image(&self, data: Bytes, filename: &str, mime_type: &str) -> Result<String> {
        // Clone Arc before moving into async block
        let storage = self.storage.clone();
        
        // Use spawn instead of spawn_blocking since store_file is already async
        let upload_result = tokio::spawn(async move {
            storage.store_file(data, filename, mime_type).await
        })
        .await
        .map_err(|e: JoinError| Error::Other(e.into()))?
        .map_err(|e| Error::Storage(e))?;

        // Record metadata after successful upload
        self.record_upload(
            upload_result.file_id,
            upload_result.file_name,
            upload_result.original_filename,
            upload_result.b2_file_id,
            upload_result.b2_bucket_id, 
            upload_result.url,
            upload_result.content_length as i32,
            mime_type,
            upload_result.width,
            upload_result.height
        ).await
    }

    #[instrument(skip(self))]
    pub async fn delete_image(&self, id: &str) -> Result<()> {
        // Get metadata and prepare storage deletion in parallel
        let metadata = self.get_metadata(id).await?;
        let storage = self.storage.clone();

        if let Some(metadata) = metadata {
            // Run storage deletion and DB deletion concurrently
            let (storage_result, db_result) = tokio::join!(
                tokio::spawn(async move {
                    storage.delete_file(&metadata.b2_file_id).await
                }),
                self.model.delete(id)
            );

            // Handle storage deletion result
            storage_result
                .map_err(|e: JoinError| Error::Other(e.into()))?
                .map_err(|e| Error::Storage(e))?;

            // Handle DB deletion result
            db_result?;
            
            Ok(())
        } else {
            Err(Error::NotFound("Image not found".into()))
        }
    }
} 