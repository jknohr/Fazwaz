use image::{DynamicImage, ImageFormat, GenericImageView};
use crate::backend::common::error::error::{Result, AppError, ImageError};

pub struct ImageProcessor {
    image: DynamicImage,
    format: ImageFormat,
}

impl ImageProcessor {
    pub fn new(data: &[u8]) -> Result<Self> {
        let format = image::guess_format(data)
            .map_err(|e| AppError::ImageError(ImageError::InvalidFormat(e.to_string())))?;
            
        let image = image::load_from_memory(data)
            .map_err(|e| AppError::ImageError(ImageError::LoadError(e.to_string())))?;
            
        Ok(Self { image, format })
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }
} 