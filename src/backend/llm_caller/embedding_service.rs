use std::sync::Arc;
use async_openai::{
    Client as OpenAIClient,
    types::{CreateImageAnalysisRequest, ImageAnalysis, ChatCompletionRequestMessage, ChatCompletionRequestMessageContent},
};
use tracing::{info, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Serialize, Deserialize};

use crate::{
    error::{Result, AppError},
    monitoring::metrics::ImageMetrics,
    config::OpenAIConfig,
};

pub struct EmbeddingService {
    openai: OpenAIClient,
    metrics: Arc<ImageMetrics>,
}

impl EmbeddingService {
    pub fn new(config: OpenAIConfig, metrics: Arc<ImageMetrics>) -> Self {
        let openai = OpenAIClient::new().with_api_key(&config.api_key);
        Self { openai, metrics }
    }

    #[instrument(skip(self, image_data))]
    pub async fn analyze_image(&self, image_data: &[u8]) -> Result<ImageAnalysis> {
        let timer = self.metrics.embedding_generation_duration.start_timer();
        
        let base64_image = BASE64.encode(image_data);
        
        // Call OpenAI Vision API with our real estate analysis prompt
        let request = CreateImageAnalysisRequest {
            model: "gpt-4-vision-preview",
            messages: vec![
                ChatMessage {
                    role: "user",
                    content: vec![
                        MessageContent::Text(self.analysis_prompt()),
                        MessageContent::ImageUrl { 
                            url: format!("data:image/jpeg;base64,{}", base64_image),
                            detail: "high",
                        },
                    ],
                },
            ],
            max_tokens: Some(1000),
        };

        let response = self.openai
            .chat()
            .create(request)
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI API error: {}", e)))?;

        timer.observe_duration();
        self.metrics.images_analyzed_total.inc();

        self.parse_openai_response(&response.choices[0].message.content)
    }

    fn analysis_prompt(&self) -> String {
        include_str!("../prompts/real_estate_analysis.txt").to_string()
    }

    fn parse_openai_response(&self, content: &str) -> Result<ImageAnalysis> {
        serde_json::from_str(content)
            .map_err(|e| AppError::ParseError(format!("Failed to parse OpenAI response: {}", e)))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub description: String,
    pub features: Vec<String>,
    pub quality_score: f32,
    pub is_interior: bool,
    pub room_type: Option<String>,
} 