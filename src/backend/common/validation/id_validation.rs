use crate::backend::common::error::error::{Result, AppError};
use crate::backend::common::types::id_types::*;

pub trait ValidateId {
    fn validate_id(&self) -> Result<()>;
}

macro_rules! impl_validate_id {
    ($($id_type:ty),*) => {
        $(
            impl ValidateId for $id_type {
                fn validate_id(&self) -> Result<()> {
                    let uuid_str = self.as_str();
                    let uuid = uuid_str.parse::<uuid7::Uuid>()
                        .map_err(|e| AppError::ParseError(format!("Invalid UUID format: {}", e)))?;
                    
                    let ts = uuid.get_unix_ts_ms();
                    let now = chrono::Utc::now().timestamp_millis();
                    
                    if ts > now {
                        return Err(AppError::InvalidTimestamp("Timestamp is in future".into()));
                    }

                    Ok(())
                }
            }
        )*
    };
}

impl_validate_id!(ListingId, BatchId, ImageId, ObjectId, UserId); 