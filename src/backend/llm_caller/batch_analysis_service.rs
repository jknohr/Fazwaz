use std::sync::Arc;
use async_openai::{
    Client,
    types::{
        CreateChatCompletionRequest,
        ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessage,
        ChatCompletionRequestSystemMessageContent,
        ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent,
    },
    config::Config,
};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use tokio::sync::Semaphore;
use futures::future;

use crate::backend::common::{
    error::error::{Result, AppError},
    config::OpenAIConfig,
    types::id_types::{ListingId, BatchId}
};
use crate::backend::monitoring::metrics::LLMMetrics;
use super::types::ImageAnalysis;
use super::prompts::ImageAnalysisPrompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchImageRequest {
    pub listing_id: ListingId,
    pub images: Vec<(String, Vec<u8>)>, // (filename, webp_data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResult {
    pub listing_id: ListingId,
    pub analyses: Vec<ImageAnalysis>,
    pub failed: Vec<String>, // Failed filenames
}

pub struct BatchAnalysisService {
    client: Client<async_openai::config::OpenAIConfig>,
    metrics: Arc<LLMMetrics>,
    prompt: String,
    semaphore: Arc<Semaphore>,
}

impl BatchAnalysisService {
    pub fn new(config: OpenAIConfig, metrics: Arc<LLMMetrics>) -> Self {
        let client = Client::with_config(config.clone().into_client_config());
        let prompt = include_str!("../assets/analyse_image.md").to_string();
        Self { 
            client,
            metrics,
            prompt,
            semaphore: Arc::new(Semaphore::new(5)),
        }
    }

    #[instrument(skip(self, request))]
    pub async fn process_batch(&self, request: BatchImageRequest) -> Result<BatchResult> {
        let timer = self.metrics.batch_processing_duration.start_timer();
        let batch_id = BatchId::generate();

        let filenames: Vec<_> = request.images.iter().map(|(f, _)| f.clone()).collect();
        
        info!(
            listing_id = %request.listing_id,
            batch_id = %batch_id,
            image_count = request.images.len(),
            "Starting batch analysis for listing"
        );

        // Process all images for this listing concurrently with semaphore control
        let futures = request.images.into_iter().map(|(filename, data)| {
            let permit = self.semaphore.clone().acquire_owned();
            let client = self.client.clone();
            let prompt = ImageAnalysisPrompt::default();
            
            async move {
                let _permit = permit.await.map_err(|e| 
                    AppError::Internal(format!("Semaphore error: {}", e))
                )?;

                self.analyze_single(filename, data).await
            }
        });

        let results = future::join_all(futures).await;
        
        let mut analyses = Vec::new();
        let mut failed = Vec::new();

        for (filename, result) in filenames.iter().zip(results) {
            match result {
                Ok(analysis) => analyses.push(analysis),
                Err(_) => failed.push(filename.clone()),
            }
        }

        timer.observe_duration();
        self.metrics.batch_jobs_total.inc();

        if !failed.is_empty() {
            self.metrics.batch_jobs_failed.inc();
            warn!(
                listing_id = %request.listing_id,
                failed_count = failed.len(),
                "Batch completed with failures"
            );
        }

        Ok(BatchResult {
            listing_id: request.listing_id,
            analyses,
            failed,
        })
    }

    async fn analyze_single(&self, filename: String, data: Vec<u8>) -> Result<ImageAnalysis> {
        let base64_image = BASE64.encode(&data);
        
        let request = CreateChatCompletionRequest {
            model: "gpt-1o-mini".into(),
            messages: vec![
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                        role: async_openai::types::Role::System,
                        content: ChatCompletionRequestSystemMessageContent::Text(self.prompt.clone()),
                        name: None,
                    }
                ),
                ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessage {
                        role: async_openai::types::Role::User,
                        content: Some(ChatCompletionRequestUserMessageContent::Text(
                            format!("Analyze this image: {}", filename)
                        )),
                        name: None,
                    }
                ),
            ],
            max_tokens: Some(1000),
            ..Default::default()
        };

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI error: {}", e)))?;

        let content = response.choices[0].message.content
            .as_ref()
            .ok_or_else(|| AppError::ParseError("Empty response from OpenAI".to_string()))?;

        serde_json::from_str(content)
            .map_err(|e| AppError::ParseError(format!("Failed to parse response: {}", e)))
    }
} 