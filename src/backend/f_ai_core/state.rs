use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use crate::backend::{
    common::{
        error::error::Result,
        types::{
            listing_types::{Listing, AgentListingRequest},
            batch_types::BatchStatus,
        },
    },
    f_ai_database::{
        database::DatabaseManager,
        listing_model::ListingService,
    },
    monitoring::{
        metrics::MetricsManager,
        events::{EventLogger, SystemEvent, Severity},
        health::ComponentStatus,
    },
    image_processor::{
        job_scheduler::ImageJobScheduler,
        processor::ImageProcessor,
    },
    llm_caller::batch_analysis_service::BatchAnalysisService,
    key_logic_auth::key_service::KeyService,
    email::email_service::EmailService,
};

pub struct AppState {
    pub db: Arc<DatabaseManager>,
    pub metrics: Arc<MetricsManager>,
    pub event_logger: Arc<EventLogger>,
    pub image_scheduler: Arc<ImageJobScheduler>,
    pub image_processor: Arc<ImageProcessor>,
    pub batch_analyzer: Arc<BatchAnalysisService>,
    pub key_service: Arc<KeyService>,
    pub email_service: Arc<EmailService>,
    pub start_time: Instant,
    pub active_jobs: Arc<RwLock<Vec<String>>>,
    pub listing_service: Arc<ListingService>,
}

impl AppState {
    pub async fn new(
        db: DatabaseManager,
        metrics: MetricsManager,
        event_logger: EventLogger,
    ) -> Self {
        let db = Arc::new(db);
        let metrics = Arc::new(metrics);
        let event_logger = Arc::new(event_logger);
        
        let image_processor = Arc::new(ImageProcessor::new());
        let batch_analyzer = Arc::new(BatchAnalysisService::new(db.clone(), event_logger.clone()));
        let image_scheduler = Arc::new(ImageJobScheduler::new(
            image_processor.clone(),
            batch_analyzer.clone(),
            metrics.clone(),
        ));
        
        let key_service = Arc::new(KeyService::new(db.clone()));
        let email_service = Arc::new(EmailService::new());

        let listing_service = Arc::new(ListingService::new(
            db.clone(),
            event_logger.clone(),
        ));

        Self {
            db,
            metrics,
            event_logger,
            image_scheduler,
            image_processor,
            batch_analyzer,
            key_service,
            email_service,
            start_time: Instant::now(),
            active_jobs: Arc::new(RwLock::new(Vec::new())),
            listing_service,
        }
    }

    pub async fn check_database_health(&self) -> Result<ComponentStatus> {
        self.db.check_health().await
    }

    pub async fn track_job(&self, job_id: String) {
        let mut jobs = self.active_jobs.write().await;
        jobs.push(job_id);
    }

    pub async fn complete_job(&self, job_id: &str) {
        let mut jobs = self.active_jobs.write().await;
        if let Some(pos) = jobs.iter().position(|id| id == job_id) {
            jobs.remove(pos);
        }
    }

    pub async fn get_active_jobs(&self) -> Vec<String> {
        self.active_jobs.read().await.clone()
    }

    #[instrument(skip(self))]
    pub async fn create_agent_listing(&self, request: AgentListingRequest) -> Result<Listing> {
        // 1. Create initial listing
        let listing = self.listing_service.create_initial_listing(request.clone()).await?;
        
        // 2. Generate API key
        let api_key = self.key_service.create_key_for_listing(&listing.id).await?;
        
        // 3. Update listing with API key
        let listing = self.listing_service.update_api_key(&listing.id, &api_key).await?;
        
        // 4. Send confirmation email
        self.email_service.send_listing_confirmation(
            &request.email,
            &request.fullname,
            &api_key,
            &listing.id.to_string()
        ).await?;
        
        // 5. Emit event
        self.event_logger.publish(SystemEvent {
            event_type: EventType::ListingCreated,
            listing_id: Some(listing.id.to_string()),
            batch_id: None,
            timestamp: chrono::Utc::now(),
            metadata: Some(serde_json::json!({
                "email": request.email,
                "status": "created"
            })),
        }).await;

        Ok(listing)
    }
} 