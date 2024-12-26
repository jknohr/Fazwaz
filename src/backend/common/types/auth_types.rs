use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: i32,
    pub burst_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub user_id: String,
    pub scopes: Vec<String>,
    pub rate_limit: Option<RateLimit>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateKeyRequest {
    pub user_id: String,
    pub scopes: Vec<String>,
    pub rate_limit: Option<RateLimit>,
    pub expires_in_days: Option<i64>,
} 