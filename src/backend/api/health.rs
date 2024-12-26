use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use serde::Serialize;
use std::sync::Arc;
use tracing::{info, warn, instrument};
use crate::{
    state::AppState,
    error::Result,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SystemStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentStatus {
    Up,
    Down,
    Degraded,
}

#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: ComponentStatus,
    pub latency_ms: u64,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: SystemStatus,
    version: String,
    uptime_seconds: u64,
    components: Vec<ComponentHealth>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[instrument(skip(state))]
pub async fn check_health(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<HealthResponse>)> {
    let timer = state.metrics.health_check_duration.start_timer();
    let mut components = Vec::new();
    let mut is_degraded = false;
    let mut is_healthy = true;

    // Get health checks from components
    let checks = vec![
        state.db.check_health().await,
        state.storage.check_health().await,
        state.cache.check_health().await,
    ];

    for check in checks {
        is_degraded |= matches!(check.status, ComponentStatus::Degraded);
        is_healthy &= matches!(check.status, ComponentStatus::Up);
        components.push(check);
    }

    let system_status = if is_healthy {
        SystemStatus::Healthy
    } else if is_degraded {
        SystemStatus::Degraded
    } else {
        SystemStatus::Unhealthy
    };

    let response = HealthResponse {
        status: system_status,
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: state.start_time.elapsed().as_secs(),
        components,
        timestamp: chrono::Utc::now(),
    };

    let status_code = match system_status {
        SystemStatus::Healthy => StatusCode::OK,
        SystemStatus::Degraded => StatusCode::OK,
        SystemStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    timer.observe_duration();
    Ok((status_code, Json(response)))
}

#[instrument(skip(state))]
pub async fn check_readiness(
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode> {
    let checks = vec![
        state.db.check_health().await,
        state.storage.check_health().await,
    ];

    let all_ready = checks.iter()
        .all(|c| matches!(c.status, ComponentStatus::Up));

    Ok(if all_ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    })
}

#[instrument(skip(state))]
pub async fn check_liveness(
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode> {
    Ok(StatusCode::OK)
} 


use axum::extract::State;
use std::sync::Arc;
use crate::backend::{
    common::error::Result,
    f_ai_core::state::AppState,
};

#[axum::debug_handler]
pub async fn check_health(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<HealthResponse>)> {
    // ...
}

#[axum::debug_handler]
pub async fn check_readiness(
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode> {
    // ...
} 