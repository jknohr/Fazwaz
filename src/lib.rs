// External crates
pub use uuid7;
pub use axum::{
    self,
    extract::{Multipart, Path, Query, State},
    routing::{get, post, delete, patch},
    Router,
    Json,
};
pub use tokio;
pub use serde::{self, Serialize, Deserialize};
pub use serde_json::{self, json};
pub use tracing::{self, info, warn, error, instrument};
pub use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};
pub use uuid7::Uuid;
pub use tower_http::cors::{self, CorsLayer};
pub use surrealdb::{self, Surreal};
pub use async_openai;
pub use bytes::Bytes;
pub use image;
pub use lettre;
pub use anyhow;
pub use thiserror;
pub use futures;
pub use async_trait;
pub use once_cell;
pub use chrono::{self, DateTime, Utc};
pub use backoff;
pub use cached;
pub use base64;
pub use tokio_stream::Stream;
pub use metrics_exporter_prometheus;

// Internal modules
pub mod backend;

// Re-exports from internal modules
pub use backend::api;
pub use backend::common;
pub use backend::f_ai_core;
pub use backend::f_ai_database;
pub use backend::key_logic_auth;
pub use backend::image_processor;

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_UPLOAD_SIZE: usize = 10 * 1024 * 1024; // 10MB 