use serde::{Deserialize, Serialize};
use uuid7;
use std::{fmt, str::FromStr};
use chrono::{DateTime, Utc};
use crate::backend::common::error::error::{Result, AppError};
use surrealdb::sql::Id;
use uuid7::Uuid as Uuid7;

macro_rules! impl_id_type {
    ($name:ident, $prefix:expr) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl FromStr for $name {
            type Err = AppError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::from_string(s.to_string())
            }
        }

        impl $name {
            pub fn generate() -> Self {
                let uuid = uuid7::uuid7().to_string();
                Self(format!("{}_{}", $prefix, uuid))
            }

            pub fn from_string(s: String) -> Result<Self> {
                if s.is_empty() {
                    return Err(AppError::Validation(format!("Invalid {}: empty string", stringify!($name))));
                }
                
                let parts: Vec<&str> = s.split('_').collect();
                if parts.len() != 2 || parts[0] != $prefix {
                    return Err(AppError::Validation(format!("Invalid {}: must start with {}_", stringify!($name), $prefix)));
                }
                
                // Parse and validate UUID part
                parts[1].parse::<uuid7::Uuid>()
                    .map_err(|e| AppError::ParseError(e.to_string()))?;
                
                Ok(Self(s))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$name> for Id {
            fn from(id: $name) -> Self {
                Id::from(id.0)
            }
        }

        impl From<&$name> for Id {
            fn from(id: &$name) -> Self {
                Id::from(id.0.as_str())
            }
        }
    }
}

impl_id_type!(ListingId, "FL");
impl_id_type!(BatchId, "FB");
impl_id_type!(ImageId, "FI");
impl_id_type!(ObjectId, "FO");
impl_id_type!(UserId, "FU");

impl ImageId {
    pub fn to_uuid7(&self) -> Result<Uuid7> {
        self.0.parse::<Uuid7>()
            .map_err(|e| AppError::ParseError(e.to_string()))
    }

    pub fn from_batch(batch_id: &BatchId, index: u32) -> Self {
        let uuid = uuid7::uuid7();
        Self(format!("FI_{}_{}_{}", batch_id.as_str(), index, uuid))
    }
}

impl BatchId {
    pub fn create_for_listing(listing_id: &ListingId) -> Self {
        let uuid = uuid7::uuid7();
        Self(format!("FB_{}_{}", listing_id.as_str(), uuid))
    }
}

impl ListingId {
    pub fn validate_batch(&self, batch_id: &BatchId) -> bool {
        batch_id.as_str().contains(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_id_generation() {
        let id1 = ListingId::generate();
        thread::sleep(Duration::from_millis(1));
        let id2 = ListingId::generate();
        
        assert_ne!(id1, id2);
        assert!(id1.as_str().starts_with("FL_"));
        assert!(id2.as_str().starts_with("FL_"));
        assert_eq!(id1.as_str().len(), 39); // prefix(2) + underscore(1) + UUID(36)
    }

    #[test]
    fn test_id_validation() {
        // Valid prefixed UUID7
        let valid_uuid = format!("FL_{}", uuid7::uuid7());
        assert!(ListingId::from_string(valid_uuid).is_ok());

        // Invalid prefix
        let invalid_prefix = format!("XX_{}", uuid7::uuid7());
        assert!(matches!(
            ListingId::from_string(invalid_prefix),
            Err(AppError::Validation(_))
        ));

        // Invalid format
        let invalid_uuid = "FL_not-a-uuid";
        assert!(matches!(
            ListingId::from_string(invalid_uuid.to_string()),
            Err(AppError::ParseError(_))
        ));
    }

    #[test]
    fn test_from_str() {
        let valid_id = format!("FL_{}", uuid7::uuid7());
        let id = ListingId::from_str(&valid_id).unwrap();
        assert_eq!(id.as_str(), valid_id);

        let invalid = "not-a-uuid";
        assert!(ListingId::from_str(invalid).is_err());
    }

    #[test]
    fn test_uuid7_monotonic() {
        let mut ids = Vec::new();
        for _ in 0..1000 {
            ids.push(ListingId::generate());
        }
        
        // Check that IDs are strictly increasing
        for i in 1..ids.len() {
            assert!(ids[i].as_str() > ids[i-1].as_str());
        }
    }

    #[test]
    fn test_different_id_types() {
        let listing = ListingId::generate();
        let batch = BatchId::generate();
        let image = ImageId::generate();
        let object = ObjectId::generate();
        let user = UserId::generate();

        assert!(listing.as_str().starts_with("FL_"));
        assert!(batch.as_str().starts_with("FB_"));
        assert!(image.as_str().starts_with("FI_"));
        assert!(object.as_str().starts_with("FO_"));
        assert!(user.as_str().starts_with("FU_"));
    }

    #[test]
    fn test_id_prefixes() {
        let listing = ListingId::generate();
        let batch = BatchId::generate();
        let image = ImageId::generate();

        assert!(listing.as_str().starts_with("FL_"));
        assert!(batch.as_str().starts_with("FB_"));
        assert!(image.as_str().starts_with("FI_"));
    }

    #[test]
    fn test_batch_from_listing() {
        let listing = ListingId::generate();
        let batch = BatchId::create_for_listing(&listing);
        assert!(listing.validate_batch(&batch));
    }

    #[test]
    fn test_image_from_batch() {
        let batch = BatchId::generate();
        let image = ImageId::from_batch(&batch, 1);
        assert!(image.as_str().contains(batch.as_str()));
    }
} 