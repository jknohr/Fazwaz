use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::backend::common::{
    error::error::Result,
    types::health_types::ComponentStatus,
};

pub struct DatabaseManager {
    db: Arc<Surreal<Client>>,
}

impl DatabaseManager {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self { db }
    }

    pub fn client(&self) -> &Surreal<Client> {
        &self.db
    }

    pub async fn check_health(&self) -> Result<ComponentStatus> {
        match self.db.health().await {
            Ok(_) => Ok(ComponentStatus::Healthy),
            Err(e) => Ok(ComponentStatus::Unhealthy(e.to_string())),
        }
    }
} 