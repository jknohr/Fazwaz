use std::{sync::Arc, time::{Duration, Instant}, collections::HashMap};
use tokio::sync::RwLock;
use futures::future;
use tracing::{info, warn, error, instrument};
use uuid7;
use chrono::Utc;
use surrealdb::{Surreal, engine::remote::ws::Ws};
use crate::backend::common::{Result, AppError};

use super::listing_model::{Listing, ListingStatus, ListingId};
use crate::backend::trans_storage::b2_storage::B2Storage;

#[derive(Clone)]
pub struct ListingService {
    db: Arc<Surreal<Ws>>,
    storage: Arc<B2Storage>,
    cache: Arc<RwLock<HashMap<String, (Listing, Instant)>>>,
}

impl ListingService {
    pub fn new(db: Arc<Surreal<Ws>>, storage: Arc<B2Storage>) -> Self {
        Self { 
            db,
            storage,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[instrument(skip(self, listing))]
    pub async fn create_listing(&self, mut listing: Listing) -> Result<Listing> {
        info!(listing_id = %listing.listing_id.as_str(), "Creating new listing");
        
        if listing.title.is_empty() {
            warn!("Attempted to create listing with empty title");
            return Err(AppError::Validation("Title cannot be empty".to_string()));
        }

        listing.id = uuid7::uuid7().to_string();
        listing.created_at = Utc::now();
        listing.updated_at = Utc::now();
        
        if let Err(e) = self.validate_listing(&listing) {
            error!(error = %e, listing_id = %listing.listing_id.as_str(), "Listing validation failed");
            return Err(e);
        }

        match self.db.create::<Listing>("listing").content(&listing).await {
            Ok(_) => {
                info!(listing_id = %listing.listing_id.as_str(), "Listing created successfully");
                Ok(listing)
            }
            Err(e) => {
                error!(error = %e, listing_id = %listing.listing_id.as_str(), "Failed to create listing");
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn get_listing(&self, id: &str) -> Result<Option<Listing>> {
        if let Some((listing, timestamp)) = self.cache.read().await.get(id) {
            if timestamp.elapsed() < Duration::from_secs(60) {
                return Ok(Some(listing.clone()));
            }
        }

        let result: Option<Listing> = self.db.select(("listing", id)).await?;
        
        if let Some(listing) = &result {
            self.cache.write().await.insert(
                id.to_string(),
                (listing.clone(), Instant::now())
            );
        }

        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn get_listing_by_listing_id(&self, listing_id: &ListingId) -> Result<Option<Listing>> {
        info!(listing_id = %listing_id.as_str(), "Fetching listing by listing_id");
        let result = self.db
            .query("SELECT * FROM listing WHERE listing_id = $id")
            .bind(("id", listing_id.as_str()))
            .await?
            .take(0)?;
            
        if result.is_none() {
            warn!(listing_id = %listing_id.as_str(), "Listing not found");
        }
        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn update_status(&self, listing_id: &ListingId, status: ListingStatus) -> Result<()> {
        let db = self.db.clone();
        let listing_id = listing_id.clone();

        let result = db.transaction(|tx| {
            let listing_id = listing_id.clone();
            
            async move {
                let result: Option<Listing> = tx
                    .query("SELECT * FROM listing WHERE listing_id = $id")
                    .bind(("id", listing_id.as_str()))
                    .await?
                    .take(0)?;

                let mut listing = result.ok_or_else(|| 
                    Error::NotFound("Listing not found".to_string())
                )?;
                
                listing.update_status(status)?;
                
                tx.update(("listing", &listing.id))
                    .content(&listing)
                    .await?;

                Ok::<_, Error>(listing.id.clone())
            }
        }).await?;

        if let Ok(mut cache) = self.cache.try_write() {
            cache.remove(&result);
        }
                    
        Ok(())
    }

    pub async fn update_multiple(&self, updates: Vec<(ListingId, ListingStatus)>) -> Result<()> {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(10));
        
        let futures = updates.into_iter().map(|(id, status)| {
            let permit = semaphore.clone().acquire_owned();
            let this = self.clone();
            
            async move {
                let _permit = permit.await.map_err(|_| 
                    Error::Other(anyhow::anyhow!("Failed to acquire semaphore"))
                )?;
                this.update_status(&id, status).await
            }
        });

        future::try_join_all(futures).await?;
        Ok(())
    }

    #[instrument(skip(self, listing))]
    fn validate_listing(&self, listing: &Listing) -> Result<()> {
        if listing.title.is_empty() {
            warn!(listing_id = %listing.listing_id.as_str(), "Empty title");
            return Err(AppError::Validation("Title cannot be empty".to_string()));
        }
        if listing.price < 0.0 {
            warn!(listing_id = %listing.listing_id.as_str(), price = listing.price, "Negative price");
            return Err(AppError::Validation("Price cannot be negative".to_string()));
        }
        if listing.listing_id.as_str().is_empty() {
            warn!(listing_id = %listing.listing_id.as_str(), "Empty listing ID");
            return Err(AppError::Validation("Listing ID cannot be empty".to_string()));
        }
        Ok(())
    }
} 