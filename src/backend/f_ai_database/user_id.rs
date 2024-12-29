use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::backend::{
    common::types::{
        user_id_types::*,
        id_types::*,
    },
    f_ai_database::database::DatabaseManager,
};

#[derive(Debug)]
pub struct UserGraph {
    db: Arc<DatabaseManager>,
}

impl UserGraph {
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    #[instrument(skip(self))]
    pub async fn relate_user_to_listing(&self, user_id: &UserId, listing_id: &ListingId) -> Result<()> {
        let query = r#"
            RELATE user:$user_id->owns->listing:$listing_id 
            SET created_at = time::now();
        "#;

        self.db.query(query)
            .bind(("user_id", user_id))
            .bind(("listing_id", listing_id))
            .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn relate_user_to_property(&self, user_id: &UserId, property_id: &PropertyId) -> Result<()> {
        let query = r#"
            RELATE user:$user_id->owns->property:$property_id 
            SET created_at = time::now();
        "#;

        self.db.query(query)
            .bind(("user_id", user_id))
            .bind(("property_id", property_id))
            .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn relate_user_to_profile(&self, user_id: &UserId, profile_id: &ObjectId) -> Result<()> {
        let query = r#"
            RELATE user:$user_id->has_profile->profile:$profile_id 
            SET created_at = time::now();
        "#;

        self.db.query(query)
            .bind(("user_id", user_id))
            .bind(("profile_id", profile_id))
            .await?;

        Ok(())
    }
} 