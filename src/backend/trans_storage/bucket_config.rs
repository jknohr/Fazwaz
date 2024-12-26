use serde::{Deserialize, Serialize};
use crate::backend::trans_storage::storage_error::StorageError;
use crate::backend::common::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketConfig {
    pub endpoint: String,
    pub region: String,
    pub bucket_prefix: String,
    pub access_key: String,
    pub secret_key: String,
}

impl BucketConfig {
    pub fn get_bucket_name(&self, country: &str, district: &str, subdistrict: &str) -> String {
        format!("{}-{}-{}-{}", 
            self.bucket_prefix,
            country.to_lowercase(),
            district.to_lowercase(),
            subdistrict.to_lowercase()
        )
    }

    pub fn validate(&self) -> Result<()> {
        if self.endpoint.is_empty() {
            return Err(StorageError::Configuration("Endpoint cannot be empty".into()).into());
        }
        if self.access_key.is_empty() || self.secret_key.is_empty() {
            return Err(StorageError::Configuration("Credentials cannot be empty".into()).into());
        }
        Ok(())
    }
} 