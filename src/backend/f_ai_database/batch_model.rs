use std::sync::Arc;
use surrealdb::{Surreal, engine::remote::ws::Client};
use crate::backend::common::error::Result;
use tracing::instrument;
pub struct BatchService {
    db: Arc<Surreal<Client>>,
}

impl BatchService {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self { db }
    }
} 