use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use uuid7;
use chrono::{DateTime, Utc, Duration};
use crate::backend::common::{
    error::error::{Result, AppError},
    types::auth_types::*,
};

pub struct KeyService {
    db: Arc<Surreal<Client>>,
}

impl KeyService {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self { db }
    }

    pub async fn create_key(&self, request: CreateKeyRequest) -> Result<KeyResponse> {
        let key = format!("nr_{}", uuid7::uuid7());
        let expires_at = request.expires_in_days
            .map(|days| Utc::now() + Duration::days(days));

        let api_key = ApiKey {
            key: key.clone(),
            user_id: request.user_id,
            name: request.name,
            created_at: Utc::now(),
            expires_at,
            last_used_at: None,
            revoked: false,
        };

        let created: Option<ApiKey> = self.db.client()
            .query("CREATE api_keys CONTENT $key RETURN AFTER")
            .bind(("key", &api_key))
            .await?
            .take(0)?;

        created
            .map(|k| KeyResponse {
                key: k.key,
                expires_at: k.expires_at,
            })
            .ok_or_else(|| AppError::Internal("Failed to create API key".into()))
    }

    pub async fn validate_key(&self, key: &str) -> Result<bool> {
        let api_key: Option<ApiKey> = self.db.client()
            .query("SELECT * FROM api_keys WHERE key = $key AND revoked = false")
            .bind(("key", key))
            .await?
            .take(0)?;

        match api_key {
            Some(key) => {
                if let Some(expires_at) = key.expires_at {
                    if expires_at < Utc::now() {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            None => Ok(false)
        }
    }

    #[instrument(skip(self))]
    pub async fn revoke_key(&self, key_id: &str) -> Result<()> {
        let mut response = self.db.client()
            .query("UPDATE api_keys SET revoked = true WHERE key = $key")
            .bind(("key", key_id))
            .await?;
            
        let updated: Option<ApiKey> = response.take(0)?;
        if updated.is_none() {
            return Err(AppError::NotFound("Key not found".into()));
        }
        
        Ok(())
    }
} 