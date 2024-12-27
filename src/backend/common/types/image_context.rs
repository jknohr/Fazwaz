use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::id_types::ImageId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageContext {
    pub id: ImageId,
    pub listing_id: Option<String>,
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub width: u32,
    pub height: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ImageContext {
    pub fn new(
        id: ImageId,
        listing_id: Option<String>,
        filename: String,
        content_type: String,
        size: usize,
        width: u32,
        height: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            listing_id,
            filename,
            content_type,
            size,
            width,
            height,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn location_path(&self) -> String {
        match &self.listing_id {
            Some(listing_id) => format!("listings/{}/images", listing_id),
            None => "images".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: ImageId,
    pub context: ImageContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadOptions {
    pub optimize: bool,
    pub max_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchQuery {
    pub listing_id: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchResponse {
    pub image: Image,
    pub metadata: serde_json::Value,
} 