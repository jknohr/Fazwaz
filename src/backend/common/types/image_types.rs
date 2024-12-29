use serde::{Serialize, Deserialize};
use super::website_sections::WebsiteSections;
use chrono::{DateTime, Utc};
use uuid7;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadSession {
    pub session_id: String,
    pub listing_id: String,
    pub section: WebsiteSections,
    pub status: UploadStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageChunk {
    pub session_id: String,
    pub sequence: u32,
    pub data: Vec<u8>,
    pub is_final: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UploadStatus {
    Initialized,
    Uploading { chunks_received: u32, total_chunks: u32 },
    Processing,
    Completed,
    Failed { reason: String },
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ImageTypes {
    pub image_id: ImageId,
    pub listing_id: ListingId,
    pub content_type: String,
    pub metadata: ImageMetadata,
    pub status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub size: usize,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize)]    
pub struct Metadata {
    image_id: ImageId,
    processing_status: ProcessingStatus,
    processing_info: Option<ProcessingInfo>,
    location: Option<Location>,
    listing_id: ListingId,
    extra_info: Option<ExtraInfo>,
    property_description: Option<PropertyDescription>,
    semantic_description: Option<SemanticDescription>,
    property_title: Option<PropertyTitle>,
    semantic_title: Option<SemanticTitle>,
    property_type: Option<PropertyType>,
    semantic_type: Option<SemanticType>,
    property_status: Option<PropertyTags>,
    semantic_status: Option<SemanticStatus>,
    property_id: Option<PropertyId>,
    semantic_id: Option<SemanticId>,
    agentic_info: Option<AgenticInfo>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessingStatus {
    Queued,
    Analyzing,
    Optimizing,
    GeneratingThumbnails,
    Complete,
    Failed(String),
}



pub struct ProcessingInfo {
    pub processing_status: ProcessingStatus,
    pub processing_info: Option<ProcessingInfo>,
    pub semantic_results: Option<Vec<SemanticResults>>,
    pub image_processing_results: Option<Vec<ImageProcessingResults>>,
    pub image_processing_alterations: Option<Vec<ImageProcessingAlterations>>,
    pub image_processing_enhancements: Option<Vec<ImageProcessingEnhancements>>,

}