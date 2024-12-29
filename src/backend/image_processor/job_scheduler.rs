use std::sync::Arc;
use tokio::time::{Duration, interval};
use tracing::{info, warn};
use std::collections::HashMap;

use crate::backend::common::error::error::{Result, AppError};
use crate::backend::{
    trans_storage::file_manager::FileManager,
    llm_caller::embedding::OpenAIEmbedding,
    image_processor::{
        analysis_pipeline::ImageAnalysisPipeline,
        processor::ImageProcessor,
    }
};
use super::image_utils::QualityAnalysis;

pub struct ImageJobScheduler {
    file_manager: Arc<FileManager>,
    embedding_service: Arc<OpenAIEmbedding>,
    batch_size: usize,
}

impl ImageJobScheduler {
    pub async fn start(&self) {
        let mut interval = interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            if let Err(e) = self.process_pending_jobs().await {
                warn!("Job processing failed: {}", e);
            }
        }
    }

    async fn process_pending_jobs(&self) -> Result<()> {
        // Get pending jobs and coordinate between storage and LLM
        Ok(())
    }

    pub async fn process_batch(&self, config: BatchProcessingConfig) -> Result<BatchProcessingResult> {
        let mut metrics = BatchQualityMetrics::new();
        
        for group in &config.room_groups {
            for image_id in &group.images {
                let result = self.process_single_image(image_id, &config).await?;
                metrics.add_analysis(&result.quality_analysis);
            }
        }
        
        Ok(BatchProcessingResult {
            batch_id: config.batch_id,
            listing_id: config.listing_id,
            quality_metrics: metrics,
            // ... other fields
        })
    }

    async fn process_single_image(&self, image_id: &str, config: &BatchProcessingConfig) -> Result<ImageProcessingResult> {
        // Clone the batch_id since we need to use it across await points
        let batch_id = config.batch_id.clone();
        
        let file = self.file_manager.get_file(image_id).await?;
        let processor = ImageProcessor::new();
        let analysis = processor.analyze_image(&file).await?;
        
        let embedding = self.embedding_service
            .generate_embedding(&analysis.description)
            .await?;
            
        Ok(ImageProcessingResult {
            image_id: image_id.to_string(),
            quality_analysis: analysis,
            embedding,
        })
    }
}

#[derive(Debug)]
pub struct BatchQualityMetrics {
    pub total_images: usize,
    pub blurry_images: usize,
    pub perspective_issues: usize,
    pub lighting_issues: usize,
    pub window_issues: usize,
    pub avg_noise_level: f32,
    pub avg_composition_score: f32,
    pub quality_distribution: HashMap<QualityLevel, usize>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum QualityLevel {
    Excellent,  // > 0.8
    Good,       // 0.6-0.8
    Fair,       // 0.4-0.6
    Poor,       // < 0.4
}

impl BatchQualityMetrics {
    pub fn new() -> Self {
        Self {
            total_images: 0,
            blurry_images: 0,
            perspective_issues: 0,
            lighting_issues: 0,
            window_issues: 0,
            avg_noise_level: 0.0,
            avg_composition_score: 0.0,
            quality_distribution: HashMap::new(),
        }
    }

    pub fn add_analysis(&mut self, analysis: &QualityAnalysis) {
        self.total_images += 1;
        if analysis.is_blurry { self.blurry_images += 1; }
        if analysis.has_perspective_issues { self.perspective_issues += 1; }
        if analysis.has_poor_lighting { self.lighting_issues += 1; }
        if analysis.window_overexposure { self.window_issues += 1; }
        
        // Update running averages
        self.avg_noise_level = (self.avg_noise_level * (self.total_images - 1) as f32 
            + analysis.noise_level) / self.total_images as f32;
        self.avg_composition_score = (self.avg_composition_score * (self.total_images - 1) as f32 
            + analysis.composition_score) / self.total_images as f32;
            
        // Update quality distribution
        let level = match analysis.composition_score {
            score if score > 0.8 => QualityLevel::Excellent,
            score if score > 0.6 => QualityLevel::Good,
            score if score > 0.4 => QualityLevel::Fair,
            _ => QualityLevel::Poor,
        };
        *self.quality_distribution.entry(level).or_insert(0) += 1;
    }
}

#[derive(Debug)]
pub struct BatchProcessingConfig {
    pub batch_id: String,
    pub listing_id: String,
    pub room_groups: Vec<RoomGroup>,
}

#[derive(Debug)]
pub struct RoomGroup {
    pub content_type: String,
    pub images: Vec<String>,
}

#[derive(Debug)]
pub struct BatchProcessingResult {
    pub batch_id: String,
    pub listing_id: String,
    pub quality_metrics: BatchQualityMetrics,
}

#[derive(Debug)]
pub struct ImageProcessingResult {
    pub image_id: String,
    pub quality_analysis: QualityAnalysis,
    pub embedding: Vec<f32>,
} 