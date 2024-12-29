use super::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// Core Listing Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub listing_id: ListingId,
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

// ... rest of the model types ... 