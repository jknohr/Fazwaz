use std::sync::Arc;
use serde::{Serialize, Deserialize};
use surrealdb::{Surreal, engine::remote::ws::Client};
use rexiv2::Metadata as XmpMetadata;
use tracing::{info, warn, instrument};
use chrono::{DateTime, Utc};
use crate::backend::{
    common::{
        error::error::{Result, AppError},
        types::{
            id_types::{BatchId, ImageId, ListingId},
            image_types::{ImageMetadata, ImageContext},
            batch_types::{BatchProcessingStatus, BatchStatus},
        },
    },
    trans_storage::b2_storage::B2Storage,
};
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchImageGroup {
    pub content_type: String,
    pub images: Vec<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ImageModel {
    db: Arc<Surreal<Client>>,
    storage: Arc<B2Storage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchImageMetadata {
    pub content_type: String,
    pub processing_status: String,
    pub analysis_complete: bool,
    pub quality_score: f32,
    pub semantic_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub image_id: String,
    pub score: f64,
    pub metadata: ImageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadMetadata {
    pub listing_id: String,
    pub filename: String,
    pub storage_path: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub b2_url: String,
    pub content_type: Option<String>,
    pub gps_coordinates: Option<String>,
    pub processing_version: i32,
    pub enhancement_preset: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ImageModel {
    pub fn new(db: Arc<Surreal<Client>>, storage: Arc<B2Storage>) -> Self {
        Self { db, storage }
    }

    #[instrument(skip(self, metadata))]
    pub async fn create(&self, metadata: ImageMetadata) -> Result<String> {
        info!("Creating new image record");
        let mut response = self.db
            .query("INSERT INTO images $metadata")
            .bind(("metadata", metadata))
            .await
            .map_err(|e| AppError::Database(e.into()))?;
            
        let created: Option<ImageContext> = response
            .take(0)
            .map_err(|e| AppError::Database(e.into()))?;
            
        match created {
            Some(ctx) => Ok(ctx.id),
            None => Err(AppError::Database("Failed to create image record".into()))
        }
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
        info!(listing_id, filename, "Recording image upload");
        let query = "fn::record_image_upload";
        let result: Option<ImageMetadata> = self.db
            .query(query)
            .bind(("listing_id", listing_id))
            .bind(("filename", filename))
            .bind(("original_filename", original_filename))
            .bind(("b2_file_id", b2_file_id))
            .bind(("b2_bucket_id", b2_bucket_id))
            .bind(("b2_url", b2_url))
            .bind(("size", size))
            .bind(("mime_type", mime_type))
            .bind(("width", width))
            .bind(("height", height))
            .await?
            .take(0)?;

        match result {
            Some(img) => {
                info!(image_id = %img.id, "Image upload recorded successfully");
                Ok(img.id.to_string())
            }
            None => {
                warn!(listing_id, filename, "Failed to record image upload");
                Ok(String::new())
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn update_embedding(&self, id: String, text: String) -> Result<ImageMetadata> {
        info!(id, "Updating image embedding");
        let query = "fn::update_image_embedding";
        let mut response = self.db
            .query(query)
            .bind(("id", id))
            .bind(("text", text))
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
            
        let result: Option<ImageMetadata> = response
            .take(0)
            .map_err(|e| AppError::Database(e.to_string()))?;

        result.ok_or_else(|| {
            warn!(id, "Failed to update image embedding");
            AppError::Internal("Failed to update embedding".into())
        })
    }

    #[instrument(skip(self))]
    pub async fn search_similar(&self, query_text: String, limit: usize) -> Result<Vec<(ImageMetadata, f32)>> {
        if query_text.is_empty() {
            warn!("Search attempted with empty query text");
        }
        if limit == 0 {
            warn!("Search attempted with zero limit");
        }

        info!(limit, "Searching for similar images");
        let mut response = self.db
            .query("fn::find_similar_images")
            .bind(("text", query_text))
            .bind(("limit", limit))
            .await
            .map_err(|e| AppError::Database(e.into()))?;
            
        let results = response
            .take::<Vec<SearchResult>>(0)
            .map_err(|e| AppError::Database(e.into()))?;
            
        Ok(results.into_iter().map(|r| (r.metadata, r.score as f32)).collect())
    }

    #[instrument(skip(self))]
    pub async fn get_metadata(&self, id: &str) -> Result<Option<ImageMetadata>> {
        info!(id, "Fetching image metadata from database");
        Ok(self.db.select(("images", id)).await?)
    }

    #[instrument(skip(self))]
    pub async fn get_batch_status(&self, batch_id: &BatchId) -> Result<Option<BatchProcessingStatus>> {
        info!(batch_id = %batch_id, "Fetching batch status");
        let mut response = self.db
            .query("SELECT * FROM batches WHERE id = $id")
            .bind(("id", batch_id.to_string()))
            .await
            .map_err(|e| AppError::Database(e.into()))?;
            
        response.take(0)
            .map_err(|e| AppError::Database(e.into()))
    }

    pub async fn extract_metadata(&self, image_id: &ImageId) -> Result<JsonValue> {
        let image = self.get(image_id).await?
            .ok_or_else(|| AppError::NotFound("Image not found".into()))?;
            
        // Get image data from storage
        let data = self.storage.download_file(&image.location_path()).await?;
        
        // Extract XMP metadata
        let xmp = XmpMetadata::new_from_buffer(&data)?;
        let mut metadata = serde_json::Map::new();
        
        // Extract standard fields
        if let Ok(identifier) = xmp.get_tag_string("Xmp.dc.identifier") {
            metadata.insert("identifier".to_string(), JsonValue::String(identifier));
        }
        
        // Extract custom fields
        if let Ok(processing_version) = xmp.get_tag_string("Xmp.neural-reef.processingVersion") {
            metadata.insert("processingVersion".to_string(), JsonValue::String(processing_version));
        }
        
        Ok(JsonValue::Object(metadata))
    }

    pub async fn update_batch_status(&self, batch_id: &BatchId, status: BatchStatus) -> Result<Option<BatchProcessingStatus>> {
        let mut response = self.db
            .query("UPDATE batch SET status = $status WHERE batch_id = $id")
            .bind(("id", batch_id))
            .bind(("status", status))
            .await?;
        response.take(0)
    }

    pub async fn cancel_batch(&self, batch_id: &BatchId) -> Result<Option<BatchProcessingStatus>> {
        let mut response = self.db
            .query("UPDATE batch SET status = 'cancelled' WHERE batch_id = $id")
            .bind(("id", batch_id))
            .await?;
        response.take(0)
    }

    #[instrument(skip(self))]
    pub async fn get(&self, image_id: &ImageId) -> Result<Option<ImageContext>> {
        info!(image_id = %image_id, "Fetching image");
        let mut response = self.db
            .query("SELECT * FROM images WHERE id = $id")
            .bind(("id", image_id))
            .await?;
            
        let image: Option<ImageContext> = response.take(0)?;
        
        if image.is_none() {
            warn!(image_id = %image_id, "Image not found");
        }
        
        Ok(image)
    }

    #[instrument(skip(self))]
    pub async fn get_listing_images(&self, listing_id: &ListingId) -> Result<Vec<ImageContext>> {
        info!(listing_id = %listing_id, "Fetching listing images");
        let mut response = self.db
            .query("SELECT * FROM images 
                   WHERE listing_id = $id 
                   ORDER BY metadata.content_type, created_at")
            .bind(("id", listing_id.to_string()))
            .await
            .map_err(|e| AppError::Database(e.into()))?;
            
        let images = response
            .take(0)
            .map_err(|e| AppError::Database(e.into()))?;

        info!(count = images.len(), "Found images for listing");
        Ok(images)
    }

    #[instrument(skip(self, embedding))]
    pub async fn update_image_analysis(
        &self,
        image_id: &ImageId,
        content_type: &str,
        semantic_tags: Vec<String>,
        quality_score: f32,
        embedding: Vec<f32>,
    ) -> Result<()> {
        info!(image_id = %image_id, "Updating image analysis");
        
        let metadata = BatchImageMetadata {
            content_type: content_type.to_string(),
            processing_status: "analyzed".to_string(),
            analysis_complete: true,
            quality_score,
            semantic_tags,
        };

        let mut response = self.db
            .query("UPDATE images 
                   SET metadata = $metadata, 
                       embedding = <vector>$embedding,
                       updated_at = time::now()
                   WHERE id = $id")
            .bind(("id", image_id))
            .bind(("metadata", metadata))
            .bind(("embedding", embedding))
            .await?;

        let updated: Option<ImageContext> = response.take(0)?;
        if updated.is_none() {
            warn!(image_id = %image_id, "Failed to update image analysis");
            return Err(AppError::NotFound("Image not found".into()));
        }

        info!(image_id = %image_id, "Image analysis updated successfully");
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn search_by_features(&self, features: Vec<String>) -> Result<Vec<ImageContext>> {
        info!(features = ?features, "Searching images by features");
        let mut response = self.db
            .query("SELECT * FROM images 
                   WHERE metadata.semantic_tags CONTAINS ANY $features 
                   ORDER BY metadata.quality_score DESC 
                   LIMIT 20")
            .bind(("features", features))
            .await?;
            
        let images = response.take(0)?;
        Ok(images)
    }

    #[instrument(skip(self))]
    pub async fn search_by_embedding(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<ImageContext>> {
        info!("Searching images by embedding vector");
        let mut response = self.db
            .query("SELECT *, 
                   vector::similarity(embedding, $query) as score 
                   FROM images 
                   WHERE vector::similarity(embedding, $query) > 0.7 
                   ORDER BY score DESC 
                   LIMIT $limit")
            .bind(("query", embedding))
            .bind(("limit", limit))
            .await
            .map_err(|e| AppError::Database(e.into()))?;
            
        let images = response
            .take(0)
            .map_err(|e| AppError::Database(e.into()))?;

        Ok(images)
    }

    #[instrument(skip(self, data))]
    pub async fn create_from_upload(
        &self,
        listing_id: ListingId,
        filename: String,
        content_type: String,
        data: Vec<u8>,
        content_type: String,
        width: u32,
        height: u32,
    ) -> Result<ImageId> {
        info!(listing_id = %listing_id, filename, "Creating new image from upload");

        // Generate unique image ID
        let image_id = ImageId::generate();
        let storage_path = format!("{}/{}.webp", listing_id, image_id);

        // Upload to B2
        let b2_url = self.storage.upload_file(&storage_path, &data, "image/webp").await?;

        // Create metadata record
        let metadata = ImageUploadMetadata {
            listing_id: listing_id.to_string(),
            filename,
            storage_path,
            content_type,
            size_bytes: data.len() as u64,
            width,
            height,
            b2_url,
            content_type: Some(content_type),
            gps_coordinates: None,
            processing_version: 1,
            enhancement_preset: "standard".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut response = self.db
            .query("INSERT INTO images $metadata RETURN id")
            .bind(("metadata", metadata))
            .await
            .map_err(|e| AppError::Database(e.into()))?;

        let created_id: Option<String> = response
            .take(0)
            .map_err(|e| AppError::Database(e.into()))?;

        match created_id {
            Some(id) => Ok(ImageId::from_string(&id)?),
            None => Err(AppError::Database("Failed to create image record".into()))
        }
    }

    #[instrument(skip(self))]
    pub async fn update_batch_group(
        &self,
        batch_id: &BatchId,
        content_type: &str,
        image_ids: Vec<String>,
    ) -> Result<()> {
        info!(batch_id = %batch_id, content_type, "Updating batch group");
        
        let group = BatchImageGroup {
            content_type: content_type.to_string(),
            images: image_ids,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut response = self.db
            .query("UPDATE batches 
                   MERGE { groups: array::add(groups, $group) }
                   WHERE id = $id")
            .bind(("id", batch_id))
            .bind(("group", group))
            .await?;

        let updated: Option<BatchProcessingStatus> = response.take(0)?;
        if updated.is_none() {
            warn!(batch_id = %batch_id, "Failed to update batch group");
            return Err(AppError::NotFound("Batch not found".into()));
        }

        Ok(())
    }

    pub async fn search_by_vector(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<ImageContext>> {
        self.search_by_embedding(embedding, limit).await
    }
} 