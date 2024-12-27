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
    common::error::error::{Result, AppError},
    f_ai_core::state::AppState,
    common::{
        types::{
            image_types::{
                Image,
                ImageUploadOptions,
                ImageSearchQuery, 
                ImageSearchResponse
            },
            batch_types::{BatchProcessingStatus, BatchStatus},
            id_types::{BatchId, ImageId},
        },
        validation::image_validation::validate_image,
    },
};
use bytes::Bytes;

struct ValidatedFile {
    filename: String,
    data: Bytes,
}

async fn extract_and_validate_image(multipart: &mut Multipart) -> Result<ValidatedFile> {
    let field = multipart.next_field().await?
        .ok_or_else(|| AppError::Validation("No file provided".into()))?;
    
    let filename = field.file_name()
        .ok_or_else(|| AppError::Validation("No filename provided".into()))?
        .to_string();
        
    let data = field.bytes().await?;

    // Validate the image
    validate_image(&data, &filename)?;

    Ok(ValidatedFile { filename, data })
}

pub fn image_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Main image operations
        .route("/upload/:listing_id", post(process_image_upload))
        .route("/search", get(search_images_by_criteria))
        .route("/:listing_id/:image_id", delete(delete_image_record))
        
        // Batch status operations
        .route("/batch/:batch_id", get(get_batch_processing_status))
        .route("/batch/:batch_id/cancel", post(cancel_batch_processing))
        .route("/batch/:batch_id/status", patch(update_batch_status))
        
        // Image processing operations
        .route("/transform/:listing_id/:image_id", get(transform_image_with_options))
        .route("/optimize/:listing_id/:image_id", post(optimize_image_with_options))
        .route("/metadata/:listing_id/:image_id", patch(update_image_metadata))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn process_image_upload(
    State(state): State<Arc<AppState>>,
    Path(listing_id): Path<String>,
    Query(options): Query<ImageUploadOptions>,
    mut multipart: Multipart,
) -> Result<Json<BatchProcessingStatus>> {
    let trace_id = uuid7::uuid7();
    info!(trace_id = %trace_id, listing_id = %listing_id, "Starting image upload");

    let mut files = Vec::new();
    while let Some(validated_file) = extract_and_validate_image(&mut multipart).await.ok() {
        files.push((validated_file.filename, validated_file.data));
    }

    if files.is_empty() {
        return Err(AppError::Validation("No valid files provided".into()));
    }

    let batch_id = state.image_service
        .process_batch_upload(listing_id, files, options)
        .await?;

    let status = state.image_service.get_batch_status(&batch_id).await?;
    Ok(Json(status))
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
pub async fn get_batch_processing_status(
    State(state): State<Arc<AppState>>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchProcessingStatus>> {
    let batch_id = BatchId::from_string(batch_id)?;
    let status = state.image_service.get_batch_status(&batch_id).await?;
    Ok(Json(status))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn cancel_batch_processing(
    State(state): State<Arc<AppState>>,
    Path(batch_id): Path<String>,
) -> Result<StatusCode> {
    state.image_service.cancel_batch(&batch_id).await?;
    Ok(StatusCode::OK)
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn transform_image_with_options(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
) -> Result<Json<Image>> {
    let image = state.image_service.transform_image(&listing_id, &image_id).await?;
    Ok(Json(image))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn optimize_image_with_options(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
) -> Result<Json<Image>> {
    let image = state.image_service.optimize_image(&listing_id, &image_id).await?;
    Ok(Json(image))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn update_image_metadata(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
) -> Result<Json<Image>> {
    let image = state.image_service.update_metadata(&listing_id, &image_id).await?;
    Ok(Json(image))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_image_record(
    State(state): State<Arc<AppState>>,
    Path((listing_id, image_id)): Path<(String, String)>,
) -> Result<StatusCode> {
    let image_id = ImageId::from_string(image_id)?;
    state.image_service.delete_image(&image_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn update_batch_status(
    State(state): State<Arc<AppState>>,
    Path(batch_id): Path<String>,
    Json(status): Json<BatchStatus>,
) -> Result<StatusCode> {
    state.image_service.update_batch_status(&batch_id, status).await?;
    Ok(StatusCode::OK)
} 