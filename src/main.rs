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
        },
        f_ai_database::DatabaseManager,
        image_processor::ImageProcessor,
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
        monitoring::metrics::Metrics,
        trans_storage::b2_storage::B2Storage,
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
    
    // Initialize state
    let state = AppState::new(config).await?;
    
    // Build router with all components
    let app = Router::new()
        // API routes
        .merge(api::listings::router())
        .merge(api::images::router())
        .merge(api::keys::router())
        .merge(api::health::router())
        .merge(api::metrics::router())
        
        // Add state and middleware
        .layer(Extension(state))
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