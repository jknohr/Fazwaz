use super::id_types::ImageId;
use crate::backend::f_ai_database::listing_model::Listing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContext {
    pub id: ImageId,
    pub listing: Listing,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub width: u32,
    pub height: u32,
}

impl ImageContext {
    pub fn new(id: ImageId, listing: Listing, filename: String, content_type: String, size: i64, width: u32, height: u32) -> Self {
        Self { id, listing, filename, content_type, size, width, height }
    }

    pub fn location_path(&self) -> String {
        format!("{}/{}/images/{}", 
            self.listing.country.to_lowercase(),
            self.listing.district.to_lowercase(),
            self.listing.subdistrict.to_lowercase()
        )
    }
} 