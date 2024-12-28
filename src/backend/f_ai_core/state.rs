use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::backend::common::error::error::Result;
use crate::backend::f_ai_database::DatabaseManager;

pub struct AppState {
    db: Arc<DatabaseManager>,
}

impl AppState {
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseManager {
        &self.db
    }
} 