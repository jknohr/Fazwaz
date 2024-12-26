use serde::{Deserialize, Serialize};
use uuid7;
use std::fmt;
use chrono::{DateTime, Utc};
use crate::backend::common::{Result, AppError};
use crate::backend::f_ai_database::listing_model::Listing;
use surrealdb::sql::Id;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ObjectId(String);

impl ObjectId {
    pub fn new() -> Self {
        Self(uuid7::uuid7().to_string())
    }

    pub fn from_string(id: String) -> Result<Self> {
        let uuid = id.parse::<uuid7::Uuid>()
            .map_err(|e| AppError::ParseError(e.to_string()))?;

        // Validate timestamp is not in future
        let timestamp = uuid.timestamp_ms();
        let now = Utc::now().timestamp_millis() as u64;
        
        if timestamp > now {
            return Err(AppError::InvalidTimestamp("Timestamp is in future".into()).into());
        }

        Ok(Self(id))
    }

    pub fn generate() -> Self {
        Self::new()
    }

    pub fn timestamp(&self) -> Result<DateTime<Utc>> {
        let uuid = self.0.parse::<uuid7::Uuid>()
            .map_err(|e| AppError::ParseError(e.to_string()))?;
        
        let ts = chrono::DateTime::from_timestamp_millis(uuid.timestamp_ms() as i64)
            .ok_or_else(|| AppError::InvalidTimestamp("Invalid timestamp value".into()))?;
            
        Ok(ts)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Type aliases for specific IDs
pub type ListingId = ObjectId;
pub type ImageId = ObjectId;
pub type BatchId = ObjectId;
pub type JobId = ObjectId; 

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_id_generation() {
        let id1 = ObjectId::new();
        thread::sleep(Duration::from_millis(1));
        let id2 = ObjectId::new();
        
        assert_ne!(id1, id2);
        assert_eq!(id1.to_string().len(), 36); // UUID format length
    }

    #[test]
    fn test_from_string_validation() {
        // Valid UUID7
        let valid_uuid = uuid7::uuid7().to_string();
        assert!(ObjectId::from_string(valid_uuid).is_ok());

        // Invalid format
        let invalid_uuid = "not-a-uuid";
        assert!(matches!(
            ObjectId::from_string(invalid_uuid.to_string()),
            Err(AppError::ParseError(_))
        ));

        // Future timestamp
        let future_uuid = "01900000-0000-7000-8000-000000000000";
        assert!(matches!(
            ObjectId::from_string(future_uuid.to_string()),
            Err(AppError::InvalidTimestamp(_))
        ));
    }

    #[test]
    fn test_timestamp_extraction() {
        let id = ObjectId::new();
        let timestamp = id.timestamp().unwrap();
        let now = Utc::now();
        
        assert!(timestamp <= now);
        assert!(now.signed_duration_since(timestamp).num_seconds() < 1);
    }
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContext {
    pub id: ImageId,
    pub listing: Listing,  // Full listing context
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub width: u32,
    pub height: u32,
}

impl ImageContext {
    pub fn new(id: ImageId, listing: Listing, filename: String, content_type: String, size: i64, width: u32, height: u32) -> Self {
        Self {
            id,
            listing,
            filename,
            content_type,
            size,
            width,
            height,
        }
    }

    pub fn location_path(&self) -> String {
        format!("{}/{}/images/{}", 
            self.listing.country.to_lowercase(),
            self.listing.district.to_lowercase(),
            self.listing.subdistrict.to_lowercase()
        )
    }
} 

impl From<ObjectId> for Id {
    fn from(id: ObjectId) -> Self {
        Id::from(id.0)
    }
}

impl From<&ObjectId> for Id {
    fn from(id: &ObjectId) -> Self {
        Id::from(id.0.as_str())
    }
} 