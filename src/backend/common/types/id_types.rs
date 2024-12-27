use serde::{Deserialize, Serialize};
use uuid7;
use std::{fmt, str::FromStr};
use chrono::{DateTime, Utc};
use crate::backend::common::error::error::{Result, AppError};
use surrealdb::sql::Id;

macro_rules! impl_id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn generate() -> Self {
                Self(uuid7::uuid7().to_string())
            }

            pub fn from_string(s: String) -> Result<Self> {
                if s.is_empty() {
                    return Err(AppError::Validation(format!("Invalid {}: empty string", stringify!($name))));
                }
                
                // Parse and validate UUID
                let uuid = s.parse::<uuid7::Uuid>()
                    .map_err(|e| AppError::ParseError(e.to_string()))?;
                
                // Get timestamp from UUID7
                let timestamp = uuid.get_timestamp_ms();
                let now = chrono::Utc::now().timestamp_millis();
                
                if timestamp > now {
                    return Err(AppError::InvalidTimestamp("Timestamp is in future".into()));
                }
                
                Ok(Self(s))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn get_timestamp(&self) -> Result<DateTime<Utc>> {
                let uuid = self.0.parse::<uuid7::Uuid>()
                    .map_err(|e| AppError::ParseError(e.to_string()))?;
                
                let ts_ms = uuid.get_timestamp_ms();
                chrono::DateTime::from_timestamp_millis(ts_ms)
                    .ok_or_else(|| AppError::InvalidTimestamp("Invalid timestamp value".into()))
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

impl_id_type!(ListingId);
impl_id_type!(BatchId);
impl_id_type!(ImageId);
impl_id_type!(ObjectId);
impl_id_type!(UserId);

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
        assert_eq!(id1.as_str().len(), 36); // UUID format length
    }

    #[test]
    fn test_id_validation() {
        // Valid UUID7
        let valid_uuid = uuid7::uuid7().to_string();
        assert!(ListingId::from_string(valid_uuid).is_ok());

        // Invalid format
        let invalid_uuid = "not-a-uuid";
        assert!(matches!(
            ListingId::from_string(invalid_uuid.to_string()),
            Err(AppError::ParseError(_))
        ));

        // Future timestamp
        let future_uuid = "01900000-0000-7000-8000-000000000000";
        assert!(matches!(
            ListingId::from_string(future_uuid.to_string()),
            Err(AppError::InvalidTimestamp(_))
        ));
    }

    #[test]
    fn test_timestamp_extraction() {
        let id = ListingId::generate();
        let timestamp = id.get_timestamp().unwrap();
        let now = Utc::now();
        
        assert!(timestamp <= now);
        assert!(now.signed_duration_since(timestamp).num_seconds() < 1);
    }
} 