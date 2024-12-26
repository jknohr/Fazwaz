use std::sync::Arc;
use async_openai::{Client, types::CreateChatCompletionRequest};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, instrument};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use tokio::sync::Semaphore;
use futures::future;

use crate::backend::common::{
    error::{Result, AppError},
    config::OpenAIConfig,
    types::{ListingId, BatchId},
};

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
    client: Client,
    metrics: Arc<LLMMetrics>,
    prompt: String,
    semaphore: Arc<Semaphore>,
}

impl BatchAnalysisService {
    pub fn new(config: OpenAIConfig, metrics: Arc<LLMMetrics>) -> Self {
        let client = Client::new().with_api_key(&config.api_key);
        let prompt = include_str!("../assets/analyse_image.md").to_string();
        Self { 
            client,
            metrics,
            prompt,
            semaphore: Arc::new(Semaphore::new(5)), // Limit concurrent batches
        }
    }

    #[instrument(skip(self, request))]
    pub async fn process_batch(&self, request: BatchImageRequest) -> Result<BatchResult> {
        let timer = self.metrics.batch_processing_duration.start_timer();
        let batch_id = BatchId::generate();

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
            let prompt = self.prompt.clone();
            
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

        for (filename, result) in request.images.iter().map(|(f, _)| f).zip(results) {
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
            model: "1o-mini".into(),
            messages: vec![serde_json::json!({
                "role": "user",
                "content": [
                    { "type": "text", "text": self.prompt },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/webp;base64,{}", base64_image),
                            "detail": "high"
                        }
                    }
                ]
            })],
            max_tokens: Some(1000),
            ..Default::default()
        };

        let response = self.client
            .chat()
            .create(request)
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI error: {}", e)))?;

        serde_json::from_str(&response.choices[0].message.content)
            .map_err(|e| AppError::ParseError(format!("Failed to parse response: {}", e)))
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDetails {
    pub area_type: String,
    pub size_category: SizeCategory,
    pub notable_features: Vec<String>,
    pub condition: AreaCondition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SizeCategory {
    Small,
    Medium,
    Large,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCondition {
    pub cleanliness: u8,  // 1-5
    pub damage: String,
    pub renovation_status: RenovationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenovationStatus {
    Modern,
    Dated,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightingAtmosphere {
    pub natural_light_level: u8,  // 1-5
    pub artificial_light_level: u8,  // 1-5
    pub ambiance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FurnitureFixtures {
    pub furniture_present: bool,
    pub furniture_type: Vec<String>,
    pub furniture_condition: FurnitureCondition,
    pub built_in_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FurnitureCondition {
    New,
    Worn,
    Broken,
    Indeterminate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutdoorFeatures {
    pub outdoor_type: String,
    pub condition: u8,  // 1-5
    pub special_features: Vec<String>,
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct View {
    #[serde(rename = "type")]
    pub view_type: ViewType,
    pub quality: u8,  // 1-5
    pub obstructions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ViewType {
    Nature,
    Urban,
    Mixed,
    Obstructed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmenitiesSellingPoints {
    pub visible_amenities: Vec<String>,
    pub decorative_elements: Vec<String>,
    pub standout_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationsIssues {
    pub property_issues: String,
    pub potential_selling_points: Vec<String>,
    pub additional_notes: String,
} 