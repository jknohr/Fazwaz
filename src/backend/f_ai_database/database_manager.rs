use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::backend::common::error::error::Result;

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
} 