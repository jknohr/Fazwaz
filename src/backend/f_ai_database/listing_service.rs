use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::backend::common::{
    error::error::{Result, AppError},
    types::{
        listing_types::{Listing, UpdateListingRequest, ListingStatus},
        id_types::ListingId,
    },
};
use tracing::{info, instrument};

pub struct ListingService {
    client: Arc<Surreal<Client>>,
}

impl ListingService {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self { client: db }
    }

    #[instrument(skip(self))]
    pub async fn create_listing(&self, listing: Listing) -> Result<Listing> {
        let mut response = self.client
            .query("CREATE listing CONTENT $listing RETURN AFTER")
            .bind(("listing", &listing))
            .await?;
            
        let created: Option<Listing> = response.take(0)?;
        created.ok_or_else(|| AppError::Internal("Failed to create listing".into()))
    }

    #[instrument(skip(self))]
    pub async fn get_listing_by_listing_id(&self, id: &ListingId) -> Result<Option<Listing>> {
        let mut response = self.client
            .query("SELECT * FROM listing WHERE listing_id = $id")
            .bind(("id", id))
            .await?;
            
        Ok(response.take(0)?)
    }

    #[instrument(skip(self))]
    pub async fn update_listing(&self, id: &ListingId, updates: UpdateListingRequest) -> Result<Listing> {
        let mut response = self.client
            .query("UPDATE listing SET * = $updates WHERE listing_id = $id RETURN AFTER")
            .bind(("id", id))
            .bind(("updates", updates))
            .await?;
            
        let updated: Option<Listing> = response.take(0)?;
        updated.ok_or_else(|| AppError::NotFound("Listing not found".into()))
    }

    #[instrument(skip(self))]
    pub async fn update_status(&self, id: &ListingId, status: ListingStatus) -> Result<()> {
        let mut response = self.client
            .query("UPDATE listing SET status = $status WHERE listing_id = $id")
            .bind(("id", id))
            .bind(("status", status))
            .await?;
            
        let updated: Option<Listing> = response.take(0)?;
        if updated.is_none() {
            return Err(AppError::NotFound("Listing not found".into()));
        }
        
        Ok(())
    }
} 