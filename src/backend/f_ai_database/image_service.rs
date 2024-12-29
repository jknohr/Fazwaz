use std::sync::Arc;
use tracing::{info, instrument};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::backend::{
    common::types::{
        image_types::{ProcessedImage, ProcessingStatus},
        listing_types::ListingStatus,
    },
    f_ai_database::database::DatabaseManager,
};

pub struct ImageService {
    db: Arc<DatabaseManager>,
}

impl ImageService {
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    #[instrument(skip(self))]
    pub async fn update_listing_status(&self, listing_id: &str, status: ListingStatus) -> Result<()> {
        self.db.client()
            .query("UPDATE listings SET status = $status WHERE listing_id = $listing_id")
            .bind(("status", status))
            .bind(("listing_id", listing_id))
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn queue_for_analysis(&self, listing_id: String, image_id: String) -> Result<()> {
        self.db.client()
            .query("CREATE analysis_queue CONTENT { 
                listing_id: $listing_id, 
                image_id: $image_id,
                status: 'queued',
                created_at: time::now()
            }")
            .bind(("listing_id", listing_id))
            .bind(("image_id", image_id))
            .await?;
        Ok(())
    }
} 