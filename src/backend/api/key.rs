use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::backend::{
    f_ai_core::state::AppState,
    common::error::error::Result,
    common::types::auth_types::{ApiKey, CreateKeyRequest, MessageResponse, KeyResponse},
};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn create_key(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateKeyRequest>,
) -> Result<Json<KeyResponse>> {
    let key = state.key_service.create_key(req).await?;
    Ok(Json(key))
}

#[instrument(skip(state))]
pub async fn revoke_key(
    State(state): State<Arc<AppState>>,
    Path(key_id): Path<String>,
) -> Result<StatusCode> {
    state.key_service.revoke_key(&key_id).await?;
    Ok(StatusCode::OK)
}

#[instrument(skip(state))]
pub async fn validate_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<MessageResponse>> {
    state.key_service.validate_key(&key).await?;
    Ok(Json(MessageResponse {
        message: "Key is valid".to_string(),
    }))
} 