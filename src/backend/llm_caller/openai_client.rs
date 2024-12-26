use std::sync::Arc;
use async_openai::{Client, types::CreateChatCompletionRequest};
use serde::{Serialize, Deserialize};
use tracing::{info, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use uuid7::Uuid7;

use crate::{
    backend::common::{
        error::{Result, AppError},
        config::OpenAIConfig,
        types::listing_types::ListingId,
    },
    backend::monitoring::metrics::LLMMetrics,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub photo_id: String,
    pub timestamp: String,
    pub location_context: LocationContext,
    pub primary_focus: String,
    pub area_details: AreaDetails,
    pub lighting_and_atmosphere: LightingAtmosphere,
    pub furniture_and_fixtures: FurnitureFixtures,
    pub outdoor_features: Option<OutdoorFeatures>,
    pub amenities_and_selling_points: AmenitiesSellingPoints,
    pub observations_and_issues: ObservationsIssues,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationContext {
    Indoor,
    Outdoor,
    Mixed,
}

// ... other type definitions following analyse_image.md schema ...

pub struct OpenAIClient {
    client: Client,
    metrics: Arc<LLMMetrics>,
    prompt: String,
}

impl OpenAIClient {
    pub fn new(config: OpenAIConfig, metrics: Arc<LLMMetrics>) -> Self {
        let client = Client::new().with_api_key(&config.api_key);
        let prompt = include_str!("../../assets/analyse_image.md").to_string();
        Self { client, metrics, prompt }
    }

    #[instrument(skip(self, images))]
    pub async fn analyze_batch(
        &self, 
        listing_id: &ListingId,
        images: Vec<Vec<u8>>
    ) -> Result<Vec<ImageAnalysis>> {
        let timer = self.metrics.batch_processing_duration.start_timer();
        let mut analyses = Vec::new();

        for image in images {
            let base64_image = BASE64.encode(&image);
            
            let request = CreateChatCompletionRequest {
                model: "1o-mini".into(),
                messages: vec![ChatMessage {
                    role: "user".into(),
                    content: vec![
                        MessageContent::Text { text: self.prompt.clone() },
                        MessageContent::ImageUrl {
                            image_url: ImageUrl {
                                url: format!("data:image/webp;base64,{}", base64_image),
                                detail: Some("high".into()),
                            }
                        }
                    ],
                }],
                max_tokens: Some(2000),
                ..Default::default()
            };

            let response = self.client
                .chat()
                .create(request)
                .await
                .map_err(|e| AppError::ExternalService(format!("OpenAI API error: {}", e)))?;

            let analysis: ImageAnalysis = serde_json::from_str(&response.choices[0].message.content)
                .map_err(|e| AppError::ParseError(format!("Failed to parse OpenAI response: {}", e)))?;

            analyses.push(analysis);
        }

        timer.observe_duration();
        self.metrics.batch_jobs_total.inc();
        
        Ok(analyses)
    }
} 