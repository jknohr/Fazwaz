use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::id_types::ListingId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: ListingId,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub square_meters: u32,
    pub amenities: Vec<String>,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub bedrooms: Option<u32>,
    pub bathrooms: Option<u32>,
    pub square_meters: Option<u32>,
    pub amenities: Option<Vec<String>>,
    pub status: Option<ListingStatus>,
}

impl Listing {
    pub fn new(
        id: ListingId,
        title: String,
        description: String,
        price: f64,
        bedrooms: u32,
        bathrooms: u32,
        square_meters: u32,
        amenities: Vec<String>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            price,
            bedrooms,
            bathrooms,
            square_meters,
            amenities,
            status: ListingStatus::Draft,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
