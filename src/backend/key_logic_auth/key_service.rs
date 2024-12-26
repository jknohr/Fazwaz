use uuid7::uuid7;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::backend::common::{Result, AppError};
use crate::{
    f_ai_database::Database,
    key_logic_auth::email_service::EmailService,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub rate_limit: RateLimit,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests_per_day: u32,
}

pub struct KeyService {
    db: Database,
    email: EmailService,
}

impl KeyService {
    pub fn new(db: Database, email: EmailService) -> Self {
        Self { db, email }
    }

    #[instrument(skip(self))]
    pub async fn generate_key(&self, email: &str, username: &str) -> Result<String> {
        let key = uuid7().to_string();
        let now = Utc::now();
        
        let metadata = KeyMetadata {
            username: username.to_string(),
            email: email.to_string(),
            created_at: now,
            expires_at: now + Duration::days(7),
            rate_limit: RateLimit {
                requests_per_minute: 60,
                requests_per_day: 1000,
            },
            permissions: vec!["basic".to_string()],
        };

        self.db.create_key(&key, &metadata).await?;
        
        // Send welcome email with key
        self.email.send_key_email(email, username, &key).await?;
        
        info!("Generated new API key for user: {}", username);
        Ok(key)
    }

    pub async fn verify_key(&self, key: &str) -> Result<KeyMetadata> {
        let metadata = self.db.get_key_metadata(key).await?;
        
        if metadata.expires_at < Utc::now() {
            return Err(AppError::Validation("API key has expired".into()));
        }
        
        Ok(metadata)
    }

    pub async fn revoke_key(&self, key: &str) -> Result<()> {
        self.db.delete_key(key).await?;
        info!("Revoked API key: {}", key);
        Ok(())
    }

    pub async fn validate_key(&self, key: &str) -> Result<()> {
        if key.is_empty() {
            return Err(AppError::Validation("API key cannot be empty".into()));
        }
        // Rest of validation...
        Ok(())
    }
} 