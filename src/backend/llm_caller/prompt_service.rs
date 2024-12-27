use super::prompts::ImageAnalysisPrompt;
use crate::backend::{
    common::error::error::Result,
    monitoring::metrics::LLMMetrics,
};
use std::sync::Arc;

pub struct PromptService {
    metrics: Arc<LLMMetrics>,
    image_prompt: ImageAnalysisPrompt,
}

impl PromptService {
    pub fn new(metrics: Arc<LLMMetrics>) -> Self {
        Self {
            metrics,
            image_prompt: ImageAnalysisPrompt::default(),
        }
    }

    pub fn get_image_analysis_prompt(&self) -> String {
        self.image_prompt.get_system_content()
    }
} 