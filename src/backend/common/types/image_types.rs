use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::{
    id_types::{ListingId, ImageId, BatchId},
    listing_types::Listing,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchQuery {
    pub listing_id: Option<ListingId>,
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchResponse {
    pub image_id: ImageId,
    pub listing_id: ListingId,
    pub filename: String,
    pub similarity: f32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadOptions {
    pub batch_id: Option<BatchId>,
    pub watermark: Option<bool>,
    pub optimize: Option<bool>,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: ImageId,
    pub listing_id: ListingId,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub width: u32,
    pub height: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContext {
    pub id: ImageId,
    pub listing: Listing,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub width: u32,
    pub height: u32,
    pub created_at: DateTime<Utc>,
}

impl ImageContext {
    pub fn new(id: ImageId, listing: Listing, filename: String, content_type: String, size: i64, width: u32, height: u32) -> Self {
        Self { 
            id, 
            listing, 
            filename, 
            content_type, 
            size, 
            width, 
            height,
            created_at: Utc::now(),
        }
    }

    pub fn location_path(&self) -> String {
        format!("listings/{}/images", self.listing.id.as_str())
    }
} 