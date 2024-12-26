use serde::{Deserialize, Serialize};
use uuid7;
use chrono::{DateTime, Utc};
use crate::common::types::id_types::{ImageId, ListingId, BatchId, JobId};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadOptions {
    pub priority: Option<JobPriority>,
    pub batch_id: Option<BatchId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageTransformOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,
    pub quality: Option<u8>,
    pub watermark: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchProcessingStatus {
    pub batch_id: BatchId,
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchQuery {
    pub query: Option<String>,
    pub scene_type: Option<String>,
    pub objects: Option<Vec<String>>,
    pub colors: Option<Vec<[u8; 3]>>,
    pub min_confidence: Option<f32>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub id: ImageId,
    pub listing_id: ListingId,
    pub filename: String,
    pub original_filename: String,
    pub b2_file_id: String,
    pub b2_bucket_id: String,
    pub b2_url: String,
    pub size: i32,
    pub mime_type: String,
    pub width: i32,
    pub height: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub processed: bool,
    pub watermarked: bool,
    pub text_content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: ImageMetadataAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadataAnalysis {
    pub tags: Vec<String>,
    pub objects: Vec<String>,
    pub scene_type: String,
    pub color_palette: Vec<String>,
    pub text_features: TextFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFeatures {
    pub keywords: Vec<String>,
    pub sentiment: String,
    pub property_features: PropertyFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyFeatures {
    pub bedrooms: Option<f32>,
    pub bathrooms: Option<f32>,
    pub square_feet: Option<f32>,
    pub amenities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisJob {
    pub job_id: JobId,
    pub listing_id: ListingId,
    pub image_id: ImageId,
    pub b2_file_id: String,
    pub created_at: DateTime<Utc>,
    pub status: JobStatus,
    pub attempts: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobPriority {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
} 