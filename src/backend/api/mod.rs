use axum::{
    Router,
    routing::{get, post, delete, patch},
};
use std::sync::Arc;
use crate::backend::{
    f_ai_core::state::AppState,
    common::error::Result,
    key_logic_auth::{auth::RequireAuth, rate_limit::RateLimit},
};

pub mod health;
pub mod image;
pub mod key;
pub mod listing;
pub mod metrics;
pub mod search;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health::check_health))
        .route("/ready", get(health::check_readiness))
        .route("/live", get(health::check_liveness))
        
        // Listing routes
        .route("/listings", post(listing::create_listing))
        .route("/listings/:id", get(listing::get_listing))
        .route("/listings/:id", patch(listing::update_listing))
        .route("/listings/:id/status", patch(listing::update_listing_status))
        
        // Image routes
        .nest("/images", image::image_routes())
        
        // Key management
        .route("/keys", post(key::create_key))
        .route("/keys/:id", delete(key::revoke_key))
        .route("/keys/:id/validate", get(key::validate_key))
        
        // Search routes
        .route("/search/images", get(search::search_images))
        .route("/search/embedding", post(search::search_by_embedding))
        
        // Metrics
        .route("/metrics", get(metrics::serve_metrics))
        
        .layer(RequireAuth::new())
        .layer(RateLimit::new("api", 100, 60))
}

// Common response types
pub mod response {
    use serde::Serialize;
    use axum::Json;
    use crate::error::Result;

    #[derive(Serialize)]
    pub struct MessageResponse {
        pub message: String,
    }

    #[derive(Serialize)]
    pub struct IdResponse {
        pub id: String,
    }

    pub type JsonResult<T> = Result<Json<T>>;
} 