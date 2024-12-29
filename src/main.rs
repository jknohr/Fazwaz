use std::sync::Arc;
use std::time::Duration;
use axum::{
    Router,
    Extension,
};
use tower_http::cors::CorsLayer;
use tracing::info;
use anyhow::Result;

use crate::{
    backend::{
        api,
        common::config::Config,
        f_ai_core::{
            AppState,
            events::EventBus,
            resource_manager::ResourceManager,
            cache::Cache,
            voice_agent::VoiceAgentManager,
            listing_manager::ListingManager,
        },
        f_ai_database::{
            DatabaseManager,
            listing_model::ListingService,
            image_service::ImageService,
            image_model::ImageModel,
        },
        image_processor::{
            ImageProcessor,
            batch_processor::BatchProcessor,
        },
        key_logic_auth::{
            KeyService,
            email_service::EmailService,
            middleware::auth::key_auth,
            middleware::rate_limit::RateLimitLayer,
        },
        llm_caller::{
            LLMClient,
            EmbeddingService,
        },
        monitoring::{
            metrics::Metrics,
            audit::AuditLogger,
        },
        trans_storage::{
            b2_storage::B2Storage,
            metadata::XmpProcessor,
        },
        voice_agent::{
            AgentInterviewService,
            QuestionnaireManager,
            InterviewState,
        },
    },
    Config,
    create_router,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging and metrics
    f_ai_database::config::init_logging()?;
    init_metrics()?;
    
    // Load configuration
    let config = Config::new()?;
    
    // Initialize core services
    let db_manager = DatabaseManager::new(&config).await?;
    
    // Initialize storage and metrics
    let storage = Arc::new(B2Storage::new(&config)?);
    let metrics = Arc::new(Metrics::new());

    // Initialize job scheduler
    
    let job_scheduler = ImageJobScheduler::new(storage.clone(), metrics.clone());
    job_scheduler.start().await?;

    // Pipeline 1: Voice Agent and API Key Generation
    let listing_service = ListingService::new(db_manager.clone());
    let email_service = EmailService::new(&config)?;
    let key_service = KeyService::new(db_manager.clone(), email_service.clone());
    
    // Pipeline 2: Image Processing
    let image_model = Arc::new(ImageModel::new(db_manager.clone(), storage.clone()));
    let image_service = Arc::new(ImageService::new(image_model));
    let image_processor = ImageProcessor::new(&config);
    let batch_processor = BatchProcessor::new(
        image_processor,
        image_service.clone(),
        storage.clone(),
    );

    // Initialize AI services
    let llm_client = LLMClient::new(&config);
    let embedding_service = EmbeddingService::new(llm_client.clone());
    let voice_agent = VoiceAgentManager::new(
        llm_client.clone(),
        listing_service.clone(),
        key_service.clone(),
    );
    
    // Initialize state management
    let event_bus = EventBus::new();
    let audit_logger = AuditLogger::new(db_manager.clone());
    
    // Initialize voice agent interview components
    let questionnaire_manager = QuestionnaireManager::new(db_manager.clone())?;
    let interview_service = AgentInterviewService::new(
        questionnaire_manager,
        listing_service.clone(),
        key_service.clone(),
        email_service.clone(),
    );
    
    // Build AppState with all components
    let state = AppState::new(
        db_manager,
        storage,
        metrics,
        image_service,
        key_service,
        email_service,
        batch_processor,
    )?;
    
    // Build router with all components
    let app = Router::new()
        // Pipeline 1: Voice Agent Interview Flow
        .merge(api::voice::router())
        .nest("/interview", api::voice::interview::router())
        .merge(api::listings::router())
        .merge(api::keys::router())
        
        // Pipeline 2: Image Upload and Processing
        .merge(api::images::router())
        .merge(api::batch::router())
        .merge(api::location::router())
        
        // Utility routes
        .merge(api::health::router())
        .merge(api::metrics::router())
        
        // Add state and middleware
        .layer(Extension(Arc::new(state)))
        .layer(key_auth)
        .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
        .layer(CorsLayer::permissive());

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server on {}", addr);
    
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn init_metrics() -> anyhow::Result<()> {
    metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_endpoint("/metrics")
        .with_prefix("f_ai_backend")
        .install()?;
    
    info!("Metrics initialized");
    Ok(())
} 