use surrealdb::{Surreal, engine::remote::ws::Client};
use std::sync::Arc;
use crate::backend::common::error::error::Result;

pub struct DatabaseManager {
    client: Arc<Surreal<Client>>,
}

impl DatabaseManager {
    pub fn new(client: Arc<Surreal<Client>>) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &Surreal<Client> {
        &self.client
    }
} 