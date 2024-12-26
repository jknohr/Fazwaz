use std::sync::Arc;
use tracing::{info, warn};
use crate::backend::common::{Result, AppError, ImageError};
use crate::{
    llm_caller::batch_processor::OpenAIProcessor,
    trans_storage::file_manager::FileManager,
};

pub struct ImageAnalysisPipeline {
    file_manager: Arc<FileManager>,
    openai: Arc<OpenAIProcessor>,
    max_size: usize,
    supported_formats: Vec<String>,
}

impl ImageAnalysisPipeline {
    pub async fn process_image(&self, image_data: Vec<u8>) -> Result<ProcessedImage> {
        let validated = self.validate_and_optimize(image_data).await?;
        let stored = self.file_manager.store_temp(&validated).await?;
        let analyzed = self.openai.analyze_single(stored).await?;
        
        Ok(analyzed)
    }
} 