use axum::{
    extract::{State, Path, Query, Multipart},
    Json,
    http::StatusCode,
    Router,
    routing::{get, post, delete, patch},
};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid7;
use serde_json::json;
use crate::backend::{
    common::error::Result,
    f_ai_core::state::AppState,
    image_processor::image_model::*,
};

pub fn image_routes() -> Router {
    Router::new()
        .route("/upload/:listing_id", post(process_image_upload))
        .route("/search", get(search_images_by_criteria))
        .route("/batch/:listing_id", post(process_batch_upload))
        .route("/batch/:batch_id", get(get_batch_processing_status))
        .route("/batch/:batch_id/cancel", post(cancel_batch_processing))
        .route("/transform/:listing_id/:image_id", get(transform_image_with_options))
        .route("/optimize/:listing_id/:image_id", post(optimize_image_with_options))
        .route("/metadata/:listing_id/:image_id", patch(update_image_metadata))
        .route("/:listing_id/:image_id", delete(delete_image_record))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn process_image_upload(
    State(state): State<Arc<AppState>>,
    Path(listing_id): Path<String>,
    Query(options): Query<ImageUploadOptions>,
    mut multipart: Multipart,
) -> Result<Json<Image>> {
    let trace_id = Uuid7::new();
    info!(trace_id = %trace_id, listing_id = %listing_id, "Starting image upload");

    let validated_file = extract_and_validate_image(&mut multipart).await?;
    
    let image = state.image_service
        .process_upload(validated_file.data, &validated_file.filename, &listing_id)
        .await?;

    Ok(Json(image))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn search_images_by_criteria(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ImageSearchQuery>,
) -> Result<Json<Vec<ImageSearchResponse>>> {
    let results = state.image_service.search_images(query).await?;
    Ok(Json(results))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn process_batch_upload(
    State(state): State<Arc<AppState>>,
    Path(listing_id): Path<String>,
    Json(batch): Json<BatchUploadOptions>,
) -> Result<Json<BatchProcessingStatus>> {
    let batch_id = state.image_service.start_batch_processing(listing_id, batch).await?;
    let status = state.image_service.get_batch_status(&batch_id).await?;
    Ok(Json(status))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn get_batch_processing_status(
    State(state): State<Arc<AppState>>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchProcessingStatus>> {
    let status = state.image_service.get_batch_status(&batch_id).await?;
    Ok(Json(status))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn cancel_batch_processing(
    State(state): State<Arc<AppState>>,
    Path(batch_id): Path<String>,
) -> Result<StatusCode> {
    state.image_service.cancel_batch_processing(&batch_id).await?;
    Ok(StatusCode::OK)
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn transform_image_with_options(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
    Query(options): Query<ImageTransformOptions>,
) -> Result<Json<Image>> {
    let transformed = state.image_service
        .transform_image(&listing_id, &image_id, options)
        .await?;
    Ok(Json(transformed))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn optimize_image_with_options(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
    Json(options): Json<ImageOptimizationOptions>,
) -> Result<Json<Image>> {
    let optimized = state.image_service
        .optimize_image(&listing_id, &image_id, options)
        .await?;
    Ok(Json(optimized))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn update_image_metadata(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
    Json(metadata): Json<serde_json::Value>,
) -> Result<StatusCode> {
    state.image_service
        .update_metadata(&listing_id, &image_id, metadata)
        .await?;
    Ok(StatusCode::OK)
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_image_record(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
) -> Result<StatusCode> {
    state.image_service
        .delete_image(&listing_id, &image_id)
        .await?;
    Ok(StatusCode::OK)
} 