use super::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// Core Listing Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub listing_id: ListingId,
    pub owner_id: UserId,
    pub title: String,
    pub description: String,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Property Details Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDetails {
    pub property_type: PropertyType,
    pub furnishing: PropertyFurnishing,
    pub condition: PropertyCondition,
    pub views: Vec<PropertyView>,
    pub parking: ParkingSpots,
    pub created_at: DateTime<Utc>,
}

// Price Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub amount: f64,
    pub currency: String,
    pub price_type: PriceType,
    pub created_at: DateTime<Utc>,
}

// Dimension Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDimensions {
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub indoor_area: u32,
    pub outdoor_area: u32,
    pub plot_size: u32,
    pub created_at: DateTime<Utc>,
}

// Location Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDetails {
    pub country: String,
    pub province: String,
    pub district: String,
    pub coordinates: GpsCoordinates,
    pub created_at: DateTime<Utc>,
}

// Image Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingImage {
    pub image_id: ImageId,
    pub filename: String,
    pub status: ImageStatus,
    pub analysis_result: Option<ImageAnalysis>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Batch Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingBatch {
    pub batch_id: String,
    pub status: BatchStatus,
    pub total_images: i32,
    pub processed_images: i32,
    pub failed_images: i32,
    pub images: Vec<String>, // Keep track of image IDs in batch
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// GPS Coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

// Image Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub quality_score: f32,
    pub detected_objects: Vec<DetectedObject>,
    pub metadata: ImageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
}

// Add these node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amenity {
    pub name: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    pub view_type: PropertyView,
    pub description: String,
    pub created_at: DateTime<Utc>,
} 