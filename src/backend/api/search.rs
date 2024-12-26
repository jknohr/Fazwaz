use axum::{
    extract::{State, Query},
    Json,
};
use std::sync::Arc;
use crate::{
    state::AppState,
    error::Result,
    common::types::image_types::ImageSearchQuery,
};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn search_images(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ImageSearchQuery>,
) -> Result<Json<Vec<ImageSearchResponse>>> {
    let results = state.search_service
        .search_images(query)
        .await?;
    
    Ok(Json(results))
}

#[instrument(skip(state))]
pub async fn search_by_embedding(
    State(state): State<Arc<AppState>>,
    Json(embedding): Json<Vec<f32>>,
) -> Result<Json<Vec<ImageSearchResponse>>> {
    let results = state.search_service
        .search_by_embedding(embedding)
        .await?;
    
    Ok(Json(results))
} 

#[axum::debug_handler]
pub async fn search_by_embedding(
    State(state): State<Arc<AppState>>,
    Json(embedding): Json<Vec<f32>>,
) -> Result<Json<Vec<ImageSearchResponse>>> {
    // ...
} 