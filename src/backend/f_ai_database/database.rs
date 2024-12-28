use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Wss};
use std::sync::Arc;
use crate::backend::common::error::error::Result;
use crate::backend::f_ai_database::config::DatabaseConfig;
use crate::backend::f_ai_database::schema;

pub struct DatabaseManager {
    client: Arc<Surreal<Client>>,
}

impl DatabaseManager {
    pub fn new(client: Arc<Surreal<Client>>) -> Self {
        Self { client }
    }

    pub async fn connection(config: &DatabaseConfig) -> Result<Self> {
        let client = Surreal::new::<Wss>(config.url.as_str()).await?;
        
        client.signin(config.get_credentials()).await?;
        client.use_ns(&config.namespace)
            .use_db(&config.database)
            .await?;
            
        let db = Self::new(Arc::new(client));
        schema::initialize_schema(db.client()).await?;
        Ok(db)
    }

    pub fn client(&self) -> &Surreal<Client> {
        &self.client
    }
} 