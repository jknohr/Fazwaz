use std::sync::Arc;
use surrealdb::{Surreal, engine::remote::ws::Client};
use bytes::Bytes;
use tracing::{info, error, instrument};
use uuid7;
use image;

use crate::backend::{
    common::{
        error::error::{Result, AppError},
        types::{
            id_types::{ImageId, BatchId, ListingId},
            image_types::{Image, ImageSearchQuery, ImageSearchResponse, ImageUploadOptions},
            batch_types::{BatchProcessingStatus, BatchStatus},
            image_context::ImageContext,
        },
        metrics::HealthMetrics,
    },
    trans_storage::b2_storage::B2Storage,
    f_ai_database::image_model::ImageModel,
};

pub struct ImageService {
    model: Arc<ImageModel>,
    storage: Arc<B2Storage>,
    metrics: Arc<HealthMetrics>,
}

impl ImageService {
    pub fn new(
        db: Arc<Surreal<Client>>,
        storage: Arc<B2Storage>,
        metrics: Arc<HealthMetrics>,
    ) -> Self {
        Self { 
            model: Arc::new(ImageModel::new(db)),
            storage,
            metrics,
        }
    }

    pub async fn upload_image(&self, context: ImageContext, data: Vec<u8>) -> Result<()> {
        info!("Uploading image: {}", context.filename);

        // Upload to B2 storage
        let path = format!("{}/{}", context.location_path(), context.filename);
        self.storage.upload_file(&path, &data, &context.content_type).await?;

        // Store metadata in database
        let created: Option<ImageContext> = self.model.create(context).await?;

        if created.is_none() {
            error!("Failed to store image metadata");
            return Err(AppError::Internal("Failed to store image metadata".into()));
        }

        Ok(())
    }

    pub async fn get_image(&self, id: &ImageId) -> Result<(ImageContext, Vec<u8>)> {
        // Get metadata from database
        let context: Option<ImageContext> = self.model.get(id).await?;

        let context = context.ok_or_else(|| AppError::Internal("Image not found".into()))?;

        // Get image data from storage
        let path = format!("{}/{}", context.location_path(), context.filename);
        let data = self.storage.download_file(&path).await?;

        Ok((context, data))
    }

    pub async fn delete_image(&self, id: &ImageId) -> Result<()> {
        // Get metadata first to know the path
        let context: Option<ImageContext> = self.model.get(id).await?;

        if let Some(context) = context {
            // Delete from storage
            let path = format!("{}/{}", context.location_path(), context.filename);
            self.storage.delete_file(&path).await?;

            // Delete metadata
            let _: Option<ImageContext> = self.model.delete(id).await?;
        }

        Ok(())
    }

    pub async fn get_listing_images(&self, listing_id: &str) -> Result<Vec<ImageContext>> {
        let images: Vec<ImageContext> = self.model.query("SELECT * FROM image WHERE listing_id = $id")
            .bind(("id", listing_id))
            .await?
            .take(0)?;

        Ok(images)
    }

    pub async fn process_upload(&self, data: Bytes, filename: &str, listing_id: &str) -> Result<Image> {
        let timer = self.metrics.image_processing_duration.start_timer();
        
        // Process image with metadata
        let processed = self.processor.process_image(
            &ListingId::from_string(listing_id.to_string())?,
            data.to_vec(),
            mime_guess::from_path(filename)
                .first_or_octet_stream()
                .to_string().as_str()
        ).await?;
        
        // Store processed image with metadata intact
        let path = format!("{}/{}", listing_id, filename);
        self.storage.upload_file(&path, &processed.data, "image/webp").await?;
        
        // Store metadata in database
        let image = Image {
            id: processed.id,
            listing_id: processed.listing_id,
            filename: processed.filename,
            content_type: "image/webp".to_string(),
            size: processed.size,
            width: processed.width,
            height: processed.height,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.model.create_image(&image).await?;
        
        timer.observe_duration();
        Ok(image)
    }

    pub async fn search_images(&self, query: ImageSearchQuery) -> Result<Vec<ImageSearchResponse>> {
        // TODO: Implement actual search logic using SurrealDB
        Ok(Vec::new())  // Return empty results for now
    }
    
    pub async fn search_by_embedding(&self, embedding: Vec<f32>) -> Result<Vec<ImageSearchResponse>> {
        // TODO: Implement vector similarity search
        Ok(Vec::new())
    }

    #[instrument(skip(self, files))]
    pub async fn process_batch_upload(
        &self,
        listing_id: String,
        files: Vec<(String, Bytes)>,
        options: ImageUploadOptions,
    ) -> Result<BatchId> {
        let batch_id = BatchId::generate();
        info!(batch_id = %batch_id, "Starting batch upload");

        for (filename, data) in files {
            self.process_upload(data, &filename, &listing_id).await?;
        }

        Ok(batch_id)
    }

    #[instrument(skip(self))]
    pub async fn transform_image(&self, listing_id: &str, image_id: &str) -> Result<Image> {
        // TODO: Implement actual image transformation
        let image = self.get_image(&ImageId::from_string(image_id.to_string())?).await?;
        Ok(Image {
            id: ImageId::from_string(image_id.to_string())?,
            listing_id: ListingId::from_string(listing_id.to_string())?,
            filename: image.0.filename,
            content_type: image.0.content_type,
            size: image.0.size,
            width: image.0.width,
            height: image.0.height,
            created_at: image.0.created_at,
            updated_at: chrono::Utc::now(),
        })
    }

    #[instrument(skip(self))]
    pub async fn get_batch_status(&self, batch_id: &BatchId) -> Result<BatchProcessingStatus> {
        // Get batch status from database
        let status = self.model.get_batch_status(batch_id).await?
            .ok_or_else(|| AppError::NotFound("Batch not found".into()))?;
            
        Ok(status)
    }

    #[instrument(skip(self))]
    pub async fn cancel_batch(&self, batch_id: &str) -> Result<()> {
        let batch_id = BatchId::from_string(batch_id.to_string())?;
        
        // Update batch status to cancelled
        let mut response = self.model.db
            .query("UPDATE batch SET status = 'cancelled' WHERE batch_id = $id")
            .bind(("id", &batch_id))
            .await?;
            
        let updated: Option<BatchProcessingStatus> = response.take(0)?;
        if updated.is_none() {
            return Err(AppError::NotFound("Batch not found".into()));
        }
        
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn optimize_image(&self, listing_id: &str, image_id: &str) -> Result<Image> {
        let image_id = ImageId::from_string(image_id.to_string())?;
        let listing_id = ListingId::from_string(listing_id.to_string())?;
        
        // Get original image
        let (context, data) = self.get_image(&image_id).await?;
        
        // TODO: Implement actual image optimization
        // For now, just return the original image metadata
        Ok(Image {
            id: image_id,
            listing_id,
            filename: context.filename,
            content_type: context.content_type,
            size: context.size as i64,
            width: context.width as u32,
            height: context.height as u32,
            created_at: context.created_at,
            updated_at: chrono::Utc::now(),
        })
    }

    #[instrument(skip(self))]
    pub async fn update_metadata(&self, listing_id: &str, image_id: &str) -> Result<Image> {
        let image_id = ImageId::from_string(image_id.to_string())?;
        let listing_id = ListingId::from_string(listing_id.to_string())?;
        
        // Get current metadata
        let (context, _) = self.get_image(&image_id).await?;
        
        // TODO: Implement actual metadata update logic
        // For now, just return current metadata
        Ok(Image {
            id: image_id,
            listing_id,
            filename: context.filename,
            content_type: context.content_type,
            size: context.size as i64,
            width: context.width as u32,
            height: context.height as u32,
            created_at: context.created_at,
            updated_at: chrono::Utc::now(),
        })
    }

    #[instrument(skip(self))]
    pub async fn update_batch_status(&self, batch_id: &str, status: BatchStatus) -> Result<()> {
        let batch_id = BatchId::from_string(batch_id.to_string())?;
        
        // Update batch status in database
        let mut response = self.model.db
            .query("UPDATE batch SET status = $status WHERE batch_id = $id")
            .bind(("id", &batch_id))
            .bind(("status", &status))
            .await?;
            
        let updated: Option<BatchProcessingStatus> = response.take(0)?;
        if updated.is_none() {
            return Err(AppError::NotFound("Batch not found".into()));
        }
        
        Ok(())
    }
} 