use std::sync::Arc;
use async_openai::{
    Client,
    types::{CreateChatCompletionRequest, Role, MessageContent},
    config::OpenAIConfig
};
use serde_json::Value;
use tracing::{info, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::backend::{
    common::error::{Result, AppError},
    monitoring::metrics::LLMMetrics,
    llm_caller::prompts::{ImageAnalysisPrompt, AnalysisResponse}
};

pub struct AnalysisService {
    client: Client,
    metrics: Arc<LLMMetrics>,
}

impl AnalysisService {
    pub fn new(config: OpenAIConfig, metrics: Arc<LLMMetrics>) -> Self {
        Self {
            client: Client::new_with_config(config),
            metrics,
        }
    }

    #[instrument(skip(self, image_data))]
    pub async fn analyze_image(&self, image_data: &[u8]) -> Result<AnalysisResponse> {
        let timer = self.metrics.embedding_generation_duration.start_timer();
        
        let base64_image = BASE64.encode(image_data);
        
        let request = CreateChatCompletionRequest::new()
            .model("gpt-4-vision-preview")
            .messages([
                serde_json::from_str(ImageAnalysisPrompt::get_role())?,
                CreateChatCompletionRequestMessage::new()
                    .role(Role::User)
                    .content(vec![
                        MessageContent::Text(ImageAnalysisPrompt::get_prompt().to_string()),
                        MessageContent::ImageUrl { 
                            url: format!("data:image/jpeg;base64,{}", base64_image),
                            detail: Some("high".to_string())
                        }
                    ])
                    .build()?
            ])
            .max_tokens(1000);

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI API error: {}", e)))?;

        timer.observe_duration();
        self.metrics.images_analyzed_total.inc();

        let content = response.choices[0].message.content.as_ref()
            .ok_or_else(|| AppError::ParseError("Empty response from OpenAI".to_string()))?;

        serde_json::from_str(content)
            .map_err(|e| AppError::ParseError(format!("Failed to parse OpenAI response: {}", e)))
    }
} 