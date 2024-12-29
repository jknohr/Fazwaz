use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::{
    id_types::{ListingId, ImageId, BatchId},
    image_types::{ImageAnalysis, ImageMetadata},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    // Interior spaces
    MasterBedroom,
    SecondaryBedroom,
    Kitchen,
    DiningRoom,
    LivingRoom,
    Bathroom,
    Study,
    
    // Exterior & Views
    Exterior,
    Garden,
    Pool,
    Balcony,
    View,
    
    // Documentation
    FloorPlan,
    TitleDeed,
    
    // Other
    OtherInterior,
    OtherExterior,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BatchStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchProcessingStatus {
    pub batch_id: BatchId,
    pub status: BatchStatus,
    pub total: usize,
    pub processed: usize,
    pub failed: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageBatch {
    pub batch_id: BatchId,
    pub listing_id: ListingId,
    pub content_type: ContentType,
    pub images: Vec<BatchImage>,
    pub status: BatchStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchImage {
    pub image_id: ImageId,
    pub filename: String,
    pub processing_status: ProcessingStatus,
    pub analysis_result: Option<ImageAnalysis>,
    pub metadata: Option<ImageMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
} 