use std::sync::Arc;
use async_openai::{
    Client,
    types::{CreateEmbeddingRequest, Embedding},
};
use serde::{Serialize, Deserialize};
use tracing::{info, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::{
    error::{Result, AppError},
    monitoring::metrics::ImageMetrics,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageEmbedding {
    pub vector: Vec<f32>,
    pub metadata: EmbeddingMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub scene_type: String,
    pub room_type: Option<String>,
    pub objects: Vec<DetectedObject>,
    pub features: RealEstateFeatures,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedObject {
    pub label: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RealEstateFeatures {
    pub lighting: f32,
    pub quality: f32,
    pub staging: f32,
    pub cleanliness: f32,
    pub maintenance: f32,
}

pub struct EmbeddingService {
    client: Client,
    metrics: Arc<ImageMetrics>,
}

impl EmbeddingService {
    #[instrument(skip(self, image_data))]
    pub async fn generate_embedding(&self, image_data: &[u8]) -> Result<ImageEmbedding> {
        let timer = self.metrics.embedding_generation_duration.start_timer();
        
        let base64_image = BASE64.encode(image_data);
        
        let request = CreateEmbeddingRequest {
            model: "text-embedding-3-large",
            input: vec![base64_image],
            encoding_format: None,
            user: None,
        };

        let response = self.client
            .embeddings()
            .create(request)
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI API error: {}", e)))?;

        timer.observe_duration();
        self.metrics.successful_embeddings.inc();

        let embedding = response.data.first().ok_or_else(|| {
            AppError::ExternalService("No embedding returned from OpenAI".to_string())
        })?;

        Ok(ImageEmbedding {
            vector: embedding.embedding.clone(),
            metadata: EmbeddingMetadata {
                created_at: chrono::Utc::now(),
                scene_type: "real_estate".to_string(),
                room_type: None,
                objects: vec![],
                features: RealEstateFeatures::default(),
                description: String::new(),
            },
        })
    }
} 