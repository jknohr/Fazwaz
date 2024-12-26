use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ListingStatus {
    Draft,
    Active,
    Inactive,
    Archived,
} 