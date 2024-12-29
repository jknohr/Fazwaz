use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::{
    id_types::{ListingId, ImageId},
    user_id_types::UserId,
    countries::geograficalinfo::Country,
    batch_types::BatchStatus,
};

// Core Listing Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    // Core identification
    pub listing_id: ListingId,
    pub owner_id: UserId,
    
    // Basic info
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
    pub currency: Currency,
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

// Batch Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingBatch {
    pub batch_id: String,
    pub status: BatchStatus,
    pub total_images: i32,
    pub processed_images: i32,
    pub failed_images: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Edge Relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingRelationships {
    pub has_details: PropertyDetails,
    pub has_price: Vec<Price>,
    pub has_dimension: PropertyDimensions,
    pub has_location: LocationDetails,
    pub has_batch: Vec<ListingBatch>,
    pub has_api_key: Vec<ListingApiKey>,
}

// API Key Relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingApiKey {
    pub api_key: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked: bool,
}

// Enums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingStatus {
    Open,
    Uploading,
    Preprocessing,
    Enrichment,
    PostProcessing,
    Storing,
    Completed,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    Apartment,
    Condominium,
    Townhouse,
    Villa,
    Land,
    Commercial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyFurnishing {
    FullyFurnished,
    PartiallyFurnished,
    Unfurnished,
    Negotiable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyCondition {
    New,
    AsNew,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyView {
    SeaView,
    MountainView,
    PoolView,
    GardenView,
    CityView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParkingSpots {
    Covered(u32),
    Open(u32),
    Valet(u32),
    Negotiable(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceType {
    AskingPrice,
    LongTermRental,
    ShortTermRental,
}

// Request/Response Types
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerListingRequest {
    pub fullname: String,
    pub email: String,
    pub phone_number: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerListingResponse {
    pub listing_id: String,
    pub api_key: String,
}

