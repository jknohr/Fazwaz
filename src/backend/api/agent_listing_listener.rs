use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;
use crate::backend::{
    common::error::error::{Result, AppError},
    f_ai_core::state::AppState,
    common::types::listing_types::{AgentListingRequest, AgentListingResponse},
};
use tracing::{info, instrument};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    static ref PHONE_REGEX: Regex = Regex::new(
        r"^\+?[1-9]\d{1,14}$"
    ).unwrap();
}

#[instrument(skip(state))]
pub async fn create_agent_listing(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AgentListingRequest>,
) -> Result<(StatusCode, Json<AgentListingResponse>)> {
    // Validate inputs
    if !EMAIL_REGEX.is_match(&request.email) {
        return Err(AppError::Validation("Invalid email format".into()));
    }
    if !PHONE_REGEX.is_match(&request.phone_number) {
        return Err(AppError::Validation("Invalid phone number format".into()));
    }
    let name_parts: Vec<&str> = request.fullname.split_whitespace().collect();
    if name_parts.len() < 2 {
        return Err(AppError::Validation("Full name must include first and last name".into()));
    }

    // Validate country
    let country = SupportedCountry::from_str(&request.country)?;

    // Create listing through state
    let listing = state.create_agent_listing(request).await?;
    
    Ok((
        StatusCode::CREATED,
        Json(AgentListingResponse {
            listing_id: listing.id.to_string(),
            api_key: listing.api_key,
        })
    ))
} 