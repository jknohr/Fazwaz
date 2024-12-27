use axum::{
    extract::{State, Query},
    Json,
};
use std::sync::Arc;
use crate::backend::{
    f_ai_core::state::AppState,
    common::{
        error::error::Result,
        types::image_types::{ImageSearchQuery, ImageSearchResponse},
    }
};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn search_images(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ImageSearchQuery>,
) -> Result<Json<Vec<ImageSearchResponse>>> {
    let results = state.image_service
        .search_images(query)
        .await?;
    
    Ok(Json(results))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn search_by_embedding(
    State(state): State<Arc<AppState>>,
    Json(embedding): Json<Vec<f32>>,
) -> Result<Json<Vec<ImageSearchResponse>>> {
    let results = state.image_service
        .search_by_embedding(embedding)
        .await?;
    
    Ok(Json(results))
} 