use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use prometheus::Registry;
use chrono::{DateTime, Utc};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

use crate::backend::{
    f_ai_database::{
        database::DatabaseManager,
        listing_service::ListingService,
        image_service::ImageService,
    },
    f_ai_core::health::{MetricsCollector, HealthMetrics, HealthCheck},
    common::config::Config,
    key_logic_auth::key_service::KeyService,
    trans_storage::b2_storage::B2Storage,
    f_ai_core::resource_manager::{ResourceManager, ResourceConfig},
};

pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub start_time: Instant,
    pub metrics: Arc<MetricsCollector>,
    pub health_metrics: Arc<HealthMetrics>,
    pub listing_service: Arc<ListingService>,
    pub key_service: Arc<KeyService>,
    pub image_service: Arc<ImageService>,
    pub resource_manager: Arc<ResourceManager>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize database with proper connection
        let db = Arc::new(DatabaseManager::connect(
            &config.database.url,
            &config.database.namespace,
            &config.database.database
        ).await?);

        // Initialize services with Arc-wrapped client
        let db_client = Arc::new(db.client().clone());
        let metrics = Arc::new(MetricsCollector::new(db.clone()));
        let health_metrics = Arc::new(HealthMetrics::new());
        let storage = Arc::new(B2Storage::new(&config.storage)?);
        
        let listing_service = Arc::new(ListingService::new(db_client.clone()));
        let key_service = Arc::new(KeyService::new(db_client.clone()));
        let image_service = Arc::new(ImageService::new(
            db_client,
            storage,
            health_metrics.clone(),
        ));
        
        // Initialize resource manager
        let resource_config = ResourceConfig {
            max_concurrent_uploads: config.resources.max_concurrent_uploads,
            max_concurrent_processing: config.resources.max_concurrent_processing,
            max_concurrent_searches: config.resources.max_concurrent_searches,
            max_concurrent_embeddings: config.resources.max_concurrent_embeddings,
        };
        let resource_manager = Arc::new(ResourceManager::new(resource_config));

        Ok(Self {
            db,
            start_time: Instant::now(),
            metrics,
            health_metrics,
            listing_service,
            key_service,
            image_service,
            resource_manager,
        })
    }
} 