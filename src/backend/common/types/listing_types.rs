use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid7::uuid7;
use anyhow::Result;
use crate::common::error::AppError;
use crate::common::types::id_types::ListingId;

#[derive(Debug, Serialize, Deserialize)]
pub enum ListingStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub listing_id: ListingId,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub square_feet: u32,
    pub amenities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub bedrooms: Option<u32>,
    pub bathrooms: Option<u32>,
    pub square_feet: Option<u32>,
    pub amenities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub id: ListingId,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub square_feet: u32,
    pub amenities: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ListingStatus,
    pub country: String,
    pub district: String,
    pub subdistrict: String,
}

impl Listing {
    pub fn validate(&self) -> Result<()> {
        if self.title.is_empty() {
            return Err(AppError::Validation("Title cannot be empty".into()).into());
        }
        
        if self.country.is_empty() || self.district.is_empty() || self.subdistrict.is_empty() {
            return Err(AppError::Validation("Location fields cannot be empty".into()).into());
        }

        Ok(())
    }

    pub fn location_path(&self) -> String {
        format!("{}/{}/{}", 
            self.country.to_lowercase(),
            self.district.to_lowercase(),
            self.subdistrict.to_lowercase()
        )
    }
} 