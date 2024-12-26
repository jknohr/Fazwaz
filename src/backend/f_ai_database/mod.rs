pub mod batch_model;
pub mod config;
pub mod image_model;
pub mod image_service;
pub mod listing_model;
pub mod listing_service;
pub mod error;

pub use {
    config::DatabaseConfig,
    image_model::ImageModel,
    image_service::ImageService,
    listing_model::Listing,
    listing_service::ListingService,
};

use surrealdb::{Surreal, engine::remote::ws::Client, Response};
use std::sync::Arc;
use anyhow::Result;
use tracing::info;
use crate::backend::trans_storage::b2_storage::B2Storage;

pub struct Database {
    client: Arc<Surreal<Client>>,
    storage: Arc<B2Storage>,
}

impl Database {
    pub async fn new(config: &DatabaseConfig, storage: Arc<B2Storage>) -> Result<Self> {
        info!("Initializing database connection");
        let client = Arc::new(Surreal::new::<Client>(config.url.clone()).await?);
        client.signin(config).await?;
        client.use_ns(config.namespace.clone()).use_db(config.database.clone()).await?;
        
        let db = Self { client, storage };
        db.init_schema().await?;
        info!("Database initialized successfully");
        Ok(db)
    }

    pub async fn init_schema(&self) -> Result<()> {
        info!("Initializing database schema");
        let _: Response = self.client.query(include_str!("schema.surql")).await?;
        Ok(())
    }

    pub fn image_service(&self) -> ImageService {
        ImageService::new(self.client.clone(), self.storage.clone())
    }

    pub fn listing_service(&self) -> ListingService {
        ListingService::new(self.client.clone(), self.storage.clone())
    }

    pub fn client(&self) -> Arc<Surreal<Client>> {
        self.client.clone()
    }
} 