use crate::backend::common::{Result, AppError, ImageError};
use image::ImageFormat;

pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const ALLOWED_FORMATS: &[ImageFormat] = &[
    ImageFormat::Jpeg,
    ImageFormat::Png,
    ImageFormat::WebP
];

pub fn validate_image_format(format: ImageFormat) -> Result<()> {
    if !ALLOWED_FORMATS.contains(&format) {
        return Err(AppError::ImageError(ImageError::InvalidFileType {
            found: format.to_string(),
            expected: ALLOWED_FORMATS.iter().map(|f| f.to_string()).collect(),
        }));
    }
    Ok(())
} 