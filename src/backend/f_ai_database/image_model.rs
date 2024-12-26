use crate::backend::common::types::image_types::*;
use surrealdb::{Surreal, engine::remote::ws::Client};
use std::sync::Arc;
use anyhow::Result;
use tracing::{info, warn, instrument};
use serde::Deserialize;

pub struct ImageModel {
    db: Arc<Surreal<Client>>,
}

impl ImageModel {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self { db }
    }

    #[instrument(skip(self, metadata))]
    pub async fn create(&self, metadata: ImageMetadata) -> Result<String> {
        info!("Creating new image record");
        let result = self.db
            .create(("images", metadata.id.clone()))
            .content(metadata)
            .await?;
        info!("Image record created successfully");
        Ok(result.id.to_string())
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
                Ok(img.id)
            }
            None => {
                warn!(listing_id, filename, "Failed to record image upload");
                Ok(String::new())
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn update_embedding(&self, id: &str, text: &str) -> Result<ImageMetadata> {
        info!(id, "Updating image embedding");
        let query = "fn::update_image_embedding";
        let result: Option<ImageMetadata> = self.db
            .query(query)
            .bind(("id", id))
            .bind(("text", text))
            .await?
            .take(0)?;
            
        if text.is_empty() {
            warn!(id, "Attempted to update embedding with empty text");
        }

        result.ok_or_else(|| {
            warn!(id, "Failed to update image embedding");
            anyhow::anyhow!("Failed to update embedding")
        })
    }

    #[instrument(skip(self))]
    pub async fn search_similar(&self, query_text: &str, limit: usize) -> Result<Vec<(ImageMetadata, f32)>> {
        if query_text.is_empty() {
            warn!("Search attempted with empty query text");
        }
        if limit == 0 {
            warn!("Search attempted with zero limit");
        }

        info!(limit, "Searching for similar images");
        let query = "fn::find_similar_images";
        #[derive(Deserialize)]
        struct SearchResult {
            #[serde(flatten)]
            metadata: ImageMetadata,
            similarity: f32,
        }

        let results: Vec<SearchResult> = self.db
            .query(query)
            .bind(("text", query_text))
            .bind(("limit", limit))
            .await?
            .take(0)?;
            
        if results.is_empty() {
            warn!(query_text, "No similar images found");
        }
            
        Ok(results.into_iter().map(|r| (r.metadata, r.similarity)).collect())
    }

    #[instrument(skip(self))]
    pub async fn get_metadata(&self, id: &str) -> Result<Option<ImageMetadata>> {
        info!(id, "Fetching image metadata from database");
        Ok(self.db.select(("images", id)).await?)
    }
} 