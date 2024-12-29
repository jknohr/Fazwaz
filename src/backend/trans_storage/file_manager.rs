use std::sync::Arc;
use tokio::fs;
use image::{DynamicImage, imageops::FilterType};
use tracing::{info, instrument};
use anyhow::{Result, anyhow};
use webp::{Encoder, WebPMemory};

use crate::backend::trans_storage::b2_storage::B2Storage;
use crate::backend::common::error::error::AppError;

#[derive(Debug)]
pub struct ProcessedFile {
    pub filename: String,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct FileManager {
    temp_root: String,
    b2_storage: Arc<B2Storage>,
}

impl FileManager {
    pub fn new(temp_root: String, b2_storage: Arc<B2Storage>) -> Self {
        Self {
            temp_root,
            b2_storage,
        }
    }

    #[instrument(skip(self, data))]
    pub async fn store_temp_file(
        &self,
        listing_id: &str,
        image_id: &str,
        data: &[u8],
        gps: Option<(f64, f64)>
    ) -> Result<ProcessedFile> {
        // Create temp directory structure
        let temp_dir = format!("{}/temp/{}", self.temp_root, listing_id);
        fs::create_dir_all(&temp_dir).await?;

        // Process image
        let image = image::load_from_memory(data)?;
        let processed = self.process_image_dimensions(image)?;
        
        // Convert to WebP
        let filename = format!("{}_{}.webp", listing_id, image_id);
        let file_path = format!("{}/{}", temp_dir, filename);
        let webp_data = self.convert_to_webp(&processed)?;
        
        // Save file
        fs::write(&file_path, &webp_data).await?;

        Ok(ProcessedFile {
            filename,
            data: webp_data,
            width: processed.width(),
            height: processed.height(),
        })
    }

    fn process_image_dimensions(&self, image: DynamicImage) -> Result<DynamicImage> {
        const MIN_HEIGHT: u32 = 1080;
        const MAX_HEIGHT: u32 = 2160; // 4K

        let height = image.height();
        if height < MIN_HEIGHT {
            Ok(image.resize(0, MIN_HEIGHT, FilterType::Lanczos3))
        } else if height > MAX_HEIGHT {
            Ok(image.resize(0, MAX_HEIGHT, FilterType::Lanczos3))
        } else {
            Ok(image)
        }
    }

    fn convert_to_webp(&self, image: &DynamicImage) -> Result<Vec<u8>> {
        let encoder = Encoder::from_image(image)?;
        let encoded: WebPMemory = encoder.encode(90.0);
        Ok(encoded.to_vec())
    }

    #[instrument(skip(self))]
    pub async fn upload_to_b2(&self, listing_id: &str, image_id: &str) -> Result<String> {
        let temp_path = format!("{}/temp/{}/{}_{}.webp", 
            self.temp_root, listing_id, listing_id, image_id);
        
        // Read file
        let data = fs::read(&temp_path).await?;
        
        // Upload to B2
        let b2_path = format!("listings/{}/{}_{}.webp", listing_id, listing_id, image_id);
        let file_id = self.b2_storage.upload_file(&b2_path, data).await?;
        
        // Get download URL - dereference Arc with &*
        let url = (&*self.b2_storage).get_download_url(&b2_path).await?;
        
        Ok(url)
    }
}
