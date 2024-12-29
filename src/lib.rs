// Async & Runtime
pub use tokio;
pub use futures;
pub use async_trait;
pub use tokio_stream::Stream;

// Web Framework
pub use axum::{
    self,
    extract::{Multipart, Path, Query, State},
    routing::{get, post, delete, patch},
    Router,
    Json,
};
pub use tower_http::cors::{self, CorsLayer};

// Serialization & Data
pub use serde::{self, Serialize, Deserialize};
pub use serde_json::{self, json};
pub use bytes::Bytes;

// Database
pub use surrealdb::{self, Surreal};
pub use surrealdb::engine::remote::ws::Wss;
pub use surrealdb::opt::auth::Root;
pub use backend::f_ai_database::config::DatabaseConfig;
pub use crate::surrealdb::engine::remote::ws::Client;
use once_cell::sync::Lazy;

// AI & ML
pub use async_openai;
pub use rayon::prelude::*;
pub use imageproc::{
    gradients::sobel_gradients,
    geometric_transformations::{Projection, warp, Interpolation},
    hough,
};
pub use imageproc::filter::{gaussian_blur_f32, unsharp_mask};
pub use imageproc::histogram::{calculate_mean, calculate_std_dev};
pub use imageproc::color::{Rgb, ImageEnhancement};
pub use imageproc::edges::{detect_edges, EdgeDetectionMethod};
pub use imageproc::geometry::{rotate, scale, translate};
pub use imageproc::morphology::{dilate, erode};
pub use imageproc::segmentation::{threshold, threshold_otsu};
pub use imageproc::transform::{resize, rotate_180, rotate_90, rotate_270};

// Utilities
pub use uuid7::{self, Uuid};
pub use image;
pub use lettre;
pub use base64;
pub use mime_guess;
pub use std::sync::LazyLock;
pub use once_cell;
pub use chrono::{self, DateTime, Utc};
pub use backoff;
pub use cached;

// Error Handling
pub use anyhow;
pub use thiserror;

// Monitoring & Metrics
pub use tracing::{self, info, warn, error, instrument};
pub use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};
pub use metrics_exporter_prometheus;

// Internal modules
pub mod backend;

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_UPLOAD_SIZE: usize = 10 * 1024 * 1024; // 10MB 

// Re-export commonly used items
pub use backend::common::error::error::Result;
pub use backend::f_ai_database::database::DatabaseManager;

// Global database instance
pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

// Initialize database connection
pub async fn init_db(config: &DatabaseConfig) -> Result<()> {
    // Connect to the server
    DB.connect::<Wss>(config.url.as_str()).await?;
    
    // Sign in using root credentials
    DB.signin(config.get_credentials()).await?;

    // Select namespace and database
    DB.use_ns(&config.namespace)
        .use_db(&config.database)
        .await?;

    // Initialize schema
    crate::backend::f_ai_database::schema::initialize_schema(&DB).await?;

    Ok(())
}
