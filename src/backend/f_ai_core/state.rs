use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use prometheus::Registry;

use crate::backend::{
    f_ai_database::DatabaseManager,
    common::config::Config,
    f_ai_database::listing_service::ListingService,
    f_ai_database::image_service::ImageService,
    key_logic_auth::key_service::KeyService,
};

pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub start_time: Instant,
    pub metrics: Registry,
    pub listing_service: Arc<ListingService>,
    pub key_service: Arc<KeyService>,
    pub image_service: Arc<ImageService>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize database
        let db = Arc::new(DatabaseManager::new(&config.database).await?);
        let listing_service = Arc::new(ListingService::new(db.clone(), config.clone()));
        let key_service = Arc::new(KeyService::new(db.clone(), config.clone()));
        let image_service = Arc::new(ImageService::new(db.clone(), config.clone()));
        
        Ok(Self {
            db,
            start_time: Instant::now(),
            metrics: Registry::new(),
            listing_service,
            key_service,
            image_service,
        })
    }
} 