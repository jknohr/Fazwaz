use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use prometheus::Registry;
use chrono::{DateTime, Utc};

use crate::backend::{
    f_ai_database::DatabaseManager,
    common::config::Config,
    f_ai_database::listing_service::ListingService,
    f_ai_database::image_service::ImageService,
    key_logic_auth::key_service::KeyService,
    health::metrics::{MetricsCollector, HealthMetrics},
    trans_storage::b2_storage::B2Storage,
};

pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub start_time: Instant,
    pub metrics: Arc<MetricsCollector>,
    pub health_metrics: Arc<HealthMetrics>,
    pub listing_service: Arc<ListingService>,
    pub key_service: Arc<KeyService>,
    pub image_service: Arc<ImageService>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize database
        let db = Arc::new(DatabaseManager::new(&config.database).await?);
        let metrics = Arc::new(MetricsCollector::new());
        let health_metrics = Arc::new(HealthMetrics::new());
        let storage = Arc::new(B2Storage::new(&config.storage)?);
        
        let listing_service = Arc::new(ListingService::new(db.client()));
        let key_service = Arc::new(KeyService::new(db.client()));
        let image_service = Arc::new(ImageService::new(
            db.client(),
            storage,
            health_metrics.clone(),
        ));
        
        Ok(Self {
            db,
            start_time: Instant::now(),
            metrics,
            health_metrics,
            listing_service,
            key_service,
            image_service,
        })
    }
} 