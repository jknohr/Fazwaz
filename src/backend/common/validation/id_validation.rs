use crate::backend::common::{Result, AppError};
use crate::backend::common::types::id_types::ObjectId;

pub trait ValidateId {
    fn validate_id(&self) -> Result<()>;
}

impl ValidateId for ObjectId {
    fn validate_id(&self) -> Result<()> {
        // Validate timestamp
        self.timestamp()?;
        Ok(())
    }
} 