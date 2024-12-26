use std::sync::Arc;
use anyhow::Result;
use std::time::Instant;

use crate::backend::{
    f_ai_database::DatabaseManager,
    common::config::Config,
};

pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub start_time: Instant,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize database
        let db = Arc::new(DatabaseManager::new(&config.database).await?);
        
        Ok(Self {
            db,
            start_time: Instant::now(),
        })
    }
} 