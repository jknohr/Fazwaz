use std::sync::Arc;
use tokenizers::Tokenizer;
use llama2_rust::{Model, ModelConfig};
use tracing::{info, instrument};

use crate::backend::{
    common::error::{Result, AppError},
    monitoring::metrics::LLMMetrics,
};

pub struct LocalEmbedding {
    model: Arc<Model>,
    tokenizer: Arc<Tokenizer>,
    metrics: Arc<LLMMetrics>,
}

impl LocalEmbedding {
    pub fn new(model_path: &str, tokenizer_path: &str, metrics: Arc<LLMMetrics>) -> Result<Self> {
        let config = ModelConfig::tiny();
        let model = Model::load(model_path, config)
            .map_err(|e| AppError::Internal(format!("Failed to load model: {}", e)))?;
            
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| AppError::Internal(format!("Failed to load tokenizer: {}", e)))?;

        Ok(Self {
            model: Arc::new(model),
            tokenizer: Arc::new(tokenizer),
            metrics,
        })
    }

    #[instrument(skip(self, text))]
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let timer = self.metrics.embedding_generation_duration.start_timer();
        
        let tokens = self.tokenizer.encode(text, true)
            .map_err(|e| AppError::Internal(format!("Tokenization failed: {}", e)))?;

        let embedding = self.model.generate_embedding(&tokens.get_ids())
            .map_err(|e| AppError::Internal(format!("Embedding generation failed: {}", e)))?;

        timer.observe_duration();
        self.metrics.successful_embeddings.inc();

        Ok(embedding)
    }
} 