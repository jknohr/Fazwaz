use surrealdb::{Surreal, engine::remote::ws::Ws, Response};
use std::sync::Arc;
use crate::backend::common::{Result, AppError};
use tracing::info;
use crate::backend::trans_storage::b2_storage::B2Storage;
use crate::backend::common::config::DatabaseConfig;

pub struct DatabaseManager {
    client: Arc<Surreal<Ws>>,
    storage: Arc<B2Storage>,
}

impl DatabaseManager {
    pub async fn new(config: &DatabaseConfig, storage: Arc<B2Storage>) -> Result<Self> {
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

    // ... rest of implementation
} 