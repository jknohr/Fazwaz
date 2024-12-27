use axum::{Router, routing::{get, post, delete, patch}};
use std::sync::Arc;
use crate::backend::f_ai_core::state::AppState;
use crate::backend::key_logic_auth::{auth::RequireAuth, rate_limit::RateLimit};

use super::{health, image, key, listing, metrics, search};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health::check_health))
        .route("/ready", get(health::check_readiness))
        .route("/live", get(health::check_liveness))
        .route("/listings", post(listing::create_listing))
        .route("/listings/:id", get(listing::get_listing))
        .route("/listings/:id", patch(listing::update_listing))
        .route("/listings/:id/status", patch(listing::update_listing_status))
        .nest("/images", image::image_routes())
        .route("/keys", post(key::create_key))
        .route("/keys/:id", delete(key::revoke_key))
        .route("/keys/:id/validate", get(key::validate_key))
        .route("/search/images", get(search::search_images))
        .route("/search/embedding", post(search::search_by_embedding))
        .route("/metrics", get(metrics::serve_metrics))
        .layer(RequireAuth::new())
        .layer(RateLimit::new("api", 100, 60))
        .with_state(state)
} 