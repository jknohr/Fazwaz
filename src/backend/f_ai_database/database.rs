use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
use crate::backend::common::error::error::Result;

pub struct DatabaseManager {
    client: Arc<Surreal<Client>>,
}

impl DatabaseManager {
    pub fn new(client: Surreal<Client>) -> Self {
        Self {
            client: Arc::new(client)
        }
    }

    pub fn client(&self) -> &Surreal<Client> {
        &self.client
    }

    pub async fn connect(url: &str, namespace: &str, database: &str) -> Result<Self> {
        let client = Surreal::new::<Client>(url)
            .await?
            .use_ns(namespace)
            .use_db(database)
            .await?;
            
        Ok(Self::new(client))
    }
} 