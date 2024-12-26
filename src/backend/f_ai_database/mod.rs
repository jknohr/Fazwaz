mod database;
pub use database::DatabaseManager;

pub mod listing_model;
pub mod listing_service;
pub mod image_model;
pub mod image_service;
pub mod batch_model;
pub mod config;
pub mod error;
pub mod storage;

pub use {
    config::DatabaseConfig,
    image_model::ImageModel,
    image_service::ImageService,
    listing_model::Listing,
    listing_service::ListingService,
};

use surrealdb::{Surreal, engine::remote::ws::Ws, Response};
use std::sync::Arc;
use crate::backend::common::{Result, AppError};
use tracing::info;
use crate::backend::trans_storage::b2_storage::B2Storage;

pub struct Database {
    client: Arc<Surreal<Ws>>,
    storage: Arc<B2Storage>,
}

impl Database {
    pub async fn new(config: &DatabaseConfig, storage: Arc<B2Storage>) -> Result<Database> {
        info!("Initializing database connection");
        let client = Arc::new(Surreal::new::<Ws>(config.url.clone()).await?);
        client.signin(Root {
            username: &config.username,
            password: &config.password,
        }).await?;
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

    pub fn client(&self) -> Arc<Surreal<Ws>> {
        self.client.clone()
    }
} 