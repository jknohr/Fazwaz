use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use crate::backend::{
    f_ai_core::state::AppState,
    common::error::error::{Result, AppError},
};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn serve_metrics(
    State(state): State<Arc<AppState>>,
) -> Result<Response> {
    let metrics = state.metrics.gather();
    let encoded = prometheus::TextEncoder::new()
        .encode_to_string(&metrics)
        .map_err(|e| AppError::Internal(format!("Prometheus encoding error: {}", e)))?;
    
    Ok(encoded.into_response())
} 