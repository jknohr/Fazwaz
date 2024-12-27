use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::backend::{
    common::error::error::{Result, AppError},
    common::types::{
        listing_types::{Listing, ListingStatus, UpdateListingRequest},
        id_types::ListingId,
    },
    f_ai_core::state::AppState,
};
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct CreateListingRequest {
    listing_id: String,
    title: String,
    description: String,
    price: f64,
    bedrooms: u32,
    bathrooms: u32,
    square_meters: u32,
    amenities: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    status: ListingStatus,
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn create_listing(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateListingRequest>,
) -> Result<(StatusCode, Json<Listing>)> {
    let listing_id = ListingId::from_string(req.listing_id)?;
    
    let listing = Listing::new(
        listing_id,
        req.title,
        req.description,
        req.price,
        req.bedrooms,
        req.bathrooms,
        req.square_meters,
        req.amenities,
    );

    let created = state.listing_service.create_listing(listing).await?;
    
    Ok((StatusCode::CREATED, Json(created)))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn get_listing(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Listing>> {
    let listing_id = ListingId::from_string(id)?;
    let listing = state.listing_service.get_listing_by_listing_id(&listing_id).await?
        .ok_or_else(|| AppError::NotFound("Listing not found".into()))?;
    
    Ok(Json(listing))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn update_listing(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(updates): Json<UpdateListingRequest>,
) -> Result<Json<Listing>> {
    let listing_id = ListingId::from_string(id)?;
    let updated = state.listing_service.update_listing(&listing_id, updates).await?;
    Ok(Json(updated))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn update_listing_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(status_update): Json<UpdateStatusRequest>,
) -> Result<StatusCode> {
    let listing_id = ListingId::from_string(id)?;
    state.listing_service.update_status(&listing_id, status_update.status).await?;
    Ok(StatusCode::OK)
} 