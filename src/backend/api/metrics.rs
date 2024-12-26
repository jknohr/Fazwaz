use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use crate::{
    state::AppState,
    error::Result,
};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn serve_metrics(
    State(state): State<Arc<AppState>>,
) -> Result<Response> {
    let metrics = state.metrics.gather();
    let encoded = prometheus::TextEncoder::new()
        .encode_to_string(&metrics)?;
    
    Ok(encoded.into_response())
} 