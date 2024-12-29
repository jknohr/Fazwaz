use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid7;
use chrono::{DateTime, Utc};
use tracing::{info, warn, instrument};
use crate::backend::{
    common::{
        error::error::{Result, AppError},
        types::{
            listing_types::{Listing, AgentListingRequest, ListingStatus, GpsCoordinates, LocationDetails},
            batch_types::BatchStatus,
            id_types::ListingId,
        },
    },
    f_ai_database::database::DatabaseManager,
    monitoring::events::{EventLogger, SystemEvent, Severity},
    f_ai_database::location_schema::{Location, LocationProperties},
};

// Add new ListingService for handling agent requests
pub struct ListingService {
    db: Arc<DatabaseManager>,
    event_logger: Arc<EventLogger>,
}

impl ListingService {
    pub fn new(db: Arc<DatabaseManager>, event_logger: Arc<EventLogger>) -> Self {
        Self { db, event_logger }
    }

    #[instrument(skip(self))]
    pub async fn create_initial_listing(&self, request: AgentListingRequest) -> Result<Listing> {
        let listing = Listing {
            // Core identification
            id: uuid7::uuid7().to_string(),
            listing_id: ListingId::new(uuid7::uuid7().to_string())?,
            api_key: String::new(), // Will be set by key service

            // Agent/Contact information
            email: request.email,
            fullname: request.fullname,
            phone: request.phone_number,

            // Property details (initialized as empty/None)
            title: String::new(),
            description: String::new(),
            property_type: None,
            country_details: None,
            gps_pin: None,
            prices: None,
            dimensions: None,
            amenities: Vec::new(),

            // Processing status
            status: ListingStatus::Open,
            batch_status: None,
            image_batch_ids: Vec::new(),

            // Timestamps
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store in database with all fields
        self.db.client().query(
            "CREATE listings CONTENT $listing RETURN AFTER"
        )
        .bind(("listing", &listing))
        .await?
        .take(0)?
        .ok_or_else(|| AppError::Database("Failed to create listing".into()))
    }

    #[instrument(skip(self))]
    pub async fn get_listing(&self, listing_id: &ListingId) -> Result<Option<Listing>> {
        let result = self.db.client().query(
            "SELECT * FROM listings WHERE listing_id = $listing_id"
        )
        .bind(("listing_id", listing_id.as_str()))
        .await?;

        Ok(result.take(0)?)
    }

    #[instrument(skip(self))]
    pub async fn update_status(&self, listing_id: &ListingId, new_status: ListingStatus) -> Result<()> {
        let now = Utc::now();
        
        self.db.client().query(
            "UPDATE listings 
            SET status = $status, updated_at = $updated_at 
            WHERE listing_id = $listing_id"
        )
        .bind(("status", new_status))
        .bind(("updated_at", now))
        .bind(("listing_id", listing_id.as_str()))
        .await?;

        self.event_logger.log_event(SystemEvent {
            event_type: "listing.status_updated".to_string(),
            severity: Severity::Info,
            message: format!("Updated listing {} status to {:?}", listing_id.as_str(), new_status),
            metadata: None,
            timestamp: now,
        }).await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn update_batch_status(&self, listing_id: &ListingId, batch_status: BatchStatus) -> Result<()> {
        let now = Utc::now();

        self.db.client().query(
            "UPDATE listings 
            SET batch_status = $batch_status, updated_at = $updated_at 
            WHERE listing_id = $listing_id"
        )
        .bind(("batch_status", batch_status))
        .bind(("updated_at", now))
        .bind(("listing_id", listing_id.as_str()))
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn add_batch_id(&self, listing_id: &ListingId, batch_id: String) -> Result<()> {
        self.db.client().query(
            "UPDATE listings 
            SET image_batch_ids += $batch_id 
            WHERE listing_id = $listing_id"
        )
        .bind(("batch_id", batch_id))
        .bind(("listing_id", listing_id.as_str()))
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn validate_listing_exists(&self, listing_id: &ListingId) -> Result<bool> {
        let result = self.db.client().query(
            "SELECT count() FROM listings WHERE listing_id = $listing_id"
        )
        .bind(("listing_id", listing_id.as_str()))
        .await?;

        Ok(result.take::<i64>(0)? > 0)
    }

    #[instrument(skip(self))]
    pub async fn validate_api_key(&self, listing_id: &ListingId, api_key: &str) -> Result<bool> {
        let result = self.db.client().query(
            "SELECT count() FROM listings 
             WHERE listing_id = $listing_id 
             AND api_key = $api_key"
        )
        .bind(("listing_id", listing_id.as_str()))
        .bind(("api_key", api_key))
        .await?;

        Ok(result.take::<i64>(0)? > 0)
    }

    #[instrument(skip(self))]
    pub async fn get_listing_by_api_key(&self, api_key: &str) -> Result<Option<Listing>> {
        let result = self.db.client().query(
            "SELECT * FROM listings WHERE api_key = $api_key"
        )
        .bind(("api_key", api_key))
        .await?;

        Ok(result.take(0)?)
    }

    #[instrument(skip(self))]
    pub async fn update_gps_coordinates(&self, listing_id: &ListingId, gps: GpsCoordinates) -> Result<()> {
        let now = Utc::now();

        self.db.client().query(
            "UPDATE listings 
             SET gps_pin = $gps, updated_at = $updated_at 
             WHERE listing_id = $listing_id"
        )
        .bind(("gps", gps))
        .bind(("updated_at", now))
        .bind(("listing_id", listing_id.as_str()))
        .await?;

        Ok(())
    }

    pub async fn update_api_key(&self, listing_id: &str, api_key: &str) -> Result<Listing> {
        let now = Utc::now();
        
        let listing: Option<Listing> = self.db.client().query(
            "UPDATE listings 
            SET api_key = $api_key, updated_at = $now 
            WHERE id = $id 
            RETURN AFTER"
        )
        .bind(("api_key", api_key))
        .bind(("now", now))
        .bind(("id", listing_id))
        .await?
        .take(0)?;

        listing.ok_or_else(|| AppError::NotFound("Listing not found".into()))
    }

    #[instrument(skip(self))]
    pub async fn update_location(&self, listing_id: &ListingId, location: &Location) -> Result<()> {
        let now = Utc::now();
        
        // Create the relationship
        self.db.client().query(
            "RELATE $listing_id->listing_location->$location_id 
             SET created_at = $now"
        )
        .bind(("listing_id", listing_id.as_str()))
        .bind(("location_id", location.id.to_string()))
        .bind(("now", now))
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_nearby_listings(&self, coords: &GpsCoordinates, radius_km: f64) -> Result<Vec<Listing>> {
        let listings: Vec<Listing> = self.db.client().query(
            "SELECT listings.* FROM listings, location, listing_location 
             WHERE (listing_location.listing = listings.id 
             AND listing_location.location = location.id)
             AND geo::distance(location.coordinates, $coords) <= $radius"
        )
        .bind(("coords", [coords.longitude, coords.latitude]))
        .bind(("radius", radius_km * 1000.0)) // Convert to meters
        .await?
        .take(0)?;

        Ok(listings)
    }

    #[instrument(skip(self))]
    pub async fn get_listings_by_batch_status(&self, status: BatchStatus) -> Result<Vec<Listing>> {
        let listings: Vec<Listing> = self.db.client().query(
            "SELECT * FROM listings 
             WHERE batch_status = $status 
             ORDER BY created_at DESC"
        )
        .bind(("status", status))
        .await?
        .take(0)?;

        Ok(listings)
    }

    #[instrument(skip(self))]
    pub async fn get_listings_with_pending_batches(&self) -> Result<Vec<Listing>> {
        let listings: Vec<Listing> = self.db.client().query(
            "SELECT listing.* FROM listing 
             WHERE ->has_batch->batch.status IN ['Pending', 'Processing']
             ORDER BY listing.updated_at ASC"
        )
        .await?
        .take(0)?;

        Ok(listings)
    }

    #[instrument(skip(self))]
    pub async fn get_batch_summary(&self, listing_id: &ListingId) -> Result<BatchSummary> {
        let summary = self.db.client().query(
            "SELECT {
                total: count(->has_batch->batch->contains_image->image),
                processed: count(->has_batch->batch->contains_image->image WHERE status = 'Processed'),
                failed: count(->has_batch->batch->contains_image->image WHERE status = 'Failed')
             } FROM listing 
             WHERE listing_id = $listing_id"
        )
        .bind(("listing_id", listing_id.as_str()))
        .await?
        .take::<BatchSummary>(0)?
        .ok_or_else(|| AppError::NotFound("Listing not found".into()))?;

        Ok(summary)
    }

    #[instrument(skip(self))]
    pub async fn initialize_schema(&self) -> Result<()> {
        let schema = r#"
            -- Node Tables
            DEFINE TABLE listing SCHEMAFULL;
            DEFINE FIELD listing_id ON listing TYPE string ASSERT $value != NONE;
            DEFINE FIELD title ON listing TYPE string;
            DEFINE FIELD description ON listing TYPE string;
            DEFINE FIELD status ON listing TYPE string ASSERT $value != NONE;
            DEFINE FIELD created_at ON listing TYPE datetime ASSERT $value != NONE;
            DEFINE FIELD updated_at ON listing TYPE datetime ASSERT $value != NONE;

            -- Edge Tables (using RELATE syntax)
            DEFINE TABLE has_details SCHEMAFULL;
            DEFINE FIELD in ON has_details TYPE record<listing>;
            DEFINE FIELD out ON has_details TYPE record<property_details>;
            DEFINE FIELD created_at ON has_details TYPE datetime;

            DEFINE TABLE has_batch SCHEMAFULL;
            DEFINE FIELD in ON has_batch TYPE record<listing>;
            DEFINE FIELD out ON has_batch TYPE record<batch>;
            DEFINE FIELD created_at ON has_batch TYPE datetime;

            DEFINE TABLE contains_image SCHEMAFULL;
            DEFINE FIELD in ON contains_image TYPE record<batch>;
            DEFINE FIELD out ON contains_image TYPE record<image>;
            DEFINE FIELD created_at ON contains_image TYPE datetime;

            -- Indexes for graph traversal
            DEFINE INDEX listing_id_idx ON listing FIELDS listing_id UNIQUE;
            DEFINE INDEX has_details_idx ON has_details COLUMNS in, out UNIQUE;
            DEFINE INDEX has_batch_idx ON has_batch COLUMNS in, out UNIQUE;
            DEFINE INDEX contains_image_idx ON contains_image COLUMNS in, out UNIQUE;
        "#;

        self.db.execute(schema).await?;
        Ok(())
    }

    pub async fn create_listing(&self, listing: &Listing) -> Result<()> {
        // Begin transaction
        self.db.query("BEGIN TRANSACTION").await?;

        // 1. Create the listing node
        let create_listing = r#"
            LET $listing = CREATE listing SET 
                listing_id = $listing_id,
                title = $title,
                description = $description,
                status = $status,
                created_at = time::now(),
                updated_at = time::now()
            RETURN $listing.id;
        "#;

        let listing_id = self.db.execute(create_listing, listing).await?;

        // 2. Create and RELATE property details
        let relate_details = r#"
            LET $details = CREATE property_details SET 
                property_type = $property_type,
                furnishing = $furnishing,
                condition = $condition,
                created_at = time::now();
            RELATE $listing_id->has_details->$details 
            SET created_at = time::now();
        "#;

        self.db.execute(relate_details, listing.details).await?;

        // 3. Create and RELATE price nodes
        for price in &listing.prices {
            let relate_price = r#"
                LET $price = CREATE price SET
                    amount = $amount,
                    currency = $currency,
                    price_type = $price_type,
                    created_at = time::now();
                RELATE $listing_id->has_price->$price
                SET created_at = time::now();
            "#;
            self.db.execute(relate_price, price).await?;
        }

        // 4. Create and RELATE location
        let relate_location = r#"
            LET $location = CREATE location SET
                country = $country,
                province = $province,
                district = $district,
                coordinates = $coordinates,
                created_at = time::now();
            RELATE $listing_id->has_location->$location
            SET created_at = time::now();
        "#;

        self.db.execute(relate_location, listing.location).await?;

        // 5. Create and RELATE images through batch
        if let Some(batch) = &listing.batch {
            let relate_batch = r#"
                LET $batch = CREATE batch SET
                    batch_id = $batch_id,
                    status = $status,
                    total_images = $total_images,
                    created_at = time::now();
                RELATE $listing_id->has_batch->$batch
                SET created_at = time::now();
            "#;

            let batch_id = self.db.execute(relate_batch, batch).await?;

            // Relate each image to the batch
            for image in &batch.images {
                let relate_image = r#"
                    LET $image = CREATE image SET
                        image_id = $image_id,
                        filename = $filename,
                        status = $status,
                        created_at = time::now();
                    RELATE $batch_id->contains_image->$image
                    SET created_at = time::now();
                "#;
                self.db.execute(relate_image, image).await?;
            }
        }

        // Commit transaction
        self.db.query("COMMIT TRANSACTION").await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSummary {
    pub total: i64,
    pub processed: i64,
    pub failed: i64,
} 