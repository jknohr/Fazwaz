use crate::{
    error::Result,
    common::types::id_types::{ObjectId, ListingId, ImageId, BatchId},
};

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

// Add specific validation for different ID types if needed
impl ValidateId for ListingId {
    fn validate_id(&self) -> Result<()> {
        self.validate_id()
    }
}

impl ValidateId for ImageId {
    fn validate_id(&self) -> Result<()> {
        self.validate_id()
    }
}

impl ValidateId for BatchId {
    fn validate_id(&self) -> Result<()> {
        self.validate_id()
    }
} 