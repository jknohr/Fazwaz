use std::sync::Arc;
use image::{DynamicImage, ImageFormat};
use webp::Encoder;
use tracing::{info, warn, instrument};
use crate::backend::common::types::id_types::{ListingId, ImageId};
use crate::backend::common::{Result, AppError, ImageError};

use crate::{
    error::Result,
    trans_storage::file_manager::FileManager,
    monitoring::metrics::ImageMetrics,
};

pub struct ImageProcessor {
    file_manager: Arc<FileManager>,
    metrics: Arc<ImageMetrics>,
    max_size: usize,
    supported_formats: Vec<ImageFormat>,
}

impl ImageProcessor {
    #[instrument(skip(self, data))]
    pub async fn process_image(
        &self,
        listing_id: &ListingId,
        image_data: Vec<u8>,
        content_type: &str
    ) -> Result<ProcessedImage> {
        let image_id = ImageId::generate();
        let filename = format!("{}-{}", listing_id.as_str(), image_id.as_str());
        
        let timer = self.metrics.image_processing_duration.start_timer();
        
        // Validate format and size
        self.validate_image(&image_data)?;
        
        // Load and process image
        let img = image::load_from_memory(&image_data)?;
        let processed = self.optimize_image(img)?;
        
        // Convert to WebP
        let webp_data = self.convert_to_webp(&processed)?;
        
        timer.observe_duration();
        Ok(ProcessedImage {
            id: image_id,
            listing_id: listing_id.clone(),
            filename,
            content_type: content_type.to_string(),
            size: webp_data.len() as i64,
            data: webp_data,
            width: processed.width(),
            height: processed.height(),
        })
    }

    // Rest of implementation...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedImage {
    pub id: ImageId,
    pub listing_id: ListingId,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
} 