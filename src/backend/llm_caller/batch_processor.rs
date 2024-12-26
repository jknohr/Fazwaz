use std::sync::Arc;
use async_openai::{Client, types::{CreateBatchImageAnalysisRequest, ChatCompletionRequestMessage}};
use tokio::sync::Semaphore;
use futures::future::join_all;
use tracing::{info, warn, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::backend::{
    common::{
        error::{Result, AppError},
        config::OpenAIConfig,
        types::image_types::{BatchId, ImageMetadata, BatchProcessingStatus},
    },
    monitoring::metrics::LLMMetrics,
};

pub struct BatchImageProcessor {
    client: Client,
    metrics: Arc<LLMMetrics>,
    semaphore: Arc<Semaphore>,
}

impl BatchImageProcessor {
    pub fn new(config: OpenAIConfig, metrics: Arc<LLMMetrics>) -> Self {
        Self {
            client: Client::new().with_api_key(&config.api_key),
            metrics,
            semaphore: Arc::new(Semaphore::new(5)), // Limit concurrent batches
        }
    }

    #[instrument(skip(self, images))]
    pub async fn process_batch(&self, batch_id: BatchId, images: Vec<ImageMetadata>) -> Result<BatchProcessingStatus> {
        let timer = self.metrics.batch_processing_duration.start_timer();
        self.metrics.batch_jobs_total.inc();

        info!(batch_id = %batch_id, "Starting batch image analysis");

        let _permit = self.semaphore.acquire().await.map_err(|e| {
            AppError::Internal(format!("Failed to acquire semaphore: {}", e))
        })?;

        let batch_size = images.len();
        let mut completed = 0;
        let mut failed = 0;
        let mut errors = Vec::new();

        // Prepare batch request
        let requests = images.iter().map(|img| {
            let base64_image = BASE64.encode(&img.data);
            CreateBatchImageAnalysisRequest {
                model: "gpt-4-vision-preview",
                messages: vec![ChatCompletionRequestMessage {
                    role: "user".into(),
                    content: vec![
                        MessageContent::Text { 
                            text: include_str!("../assets/analyse_image.md").to_string()
                        },
                        MessageContent::ImageUrl {
                            url: format!("data:image/webp;base64,{}", base64_image),
                            detail: Some("low".into()),
                        }
                    ],
                }],
                max_tokens: Some(1000),
            }
        }).collect::<Vec<_>>();

        // Process in batches of 10
        for chunk in requests.chunks(10) {
            match self.client.batch_chat()
                .create(chunk.to_vec())
                .await 
            {
                Ok(responses) => {
                    completed += responses.len();
                    // Process responses...
                },
                Err(e) => {
                    failed += chunk.len();
                    errors.push(e.to_string());
                }
            }
        }

        timer.observe_duration();

        if failed > 0 {
            self.metrics.batch_jobs_failed.inc();
            warn!(
                batch_id = %batch_id,
                failed = failed,
                "Batch processing completed with errors"
            );
        }

        Ok(BatchProcessingStatus {
            batch_id,
            total: batch_size,
            completed,
            failed,
            in_progress: 0,
            errors,
        })
    }
} 