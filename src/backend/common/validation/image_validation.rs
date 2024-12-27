use image::{ImageFormat, GenericImageView};
use crate::backend::common::error::error::{Result, AppError, ImageError};
use bytes::Bytes;

pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const MIN_WIDTH: u32 = 100;
pub const MIN_HEIGHT: u32 = 100;
pub const MAX_WIDTH: u32 = 8192;
pub const MAX_HEIGHT: u32 = 8192;

pub const ALLOWED_FORMATS: &[ImageFormat] = &[
    ImageFormat::Jpeg,
    ImageFormat::Png,
    ImageFormat::WebP
];

pub const ALLOWED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/jpg",
    "image/png",
    "image/webp",
];

pub fn validate_image(data: &Bytes, filename: &str) -> Result<()> {
    // Check file size
    if data.len() > MAX_FILE_SIZE {
        return Err(AppError::ImageError(ImageError::FileTooLarge {
            size: data.len(),
            max: MAX_FILE_SIZE,
        }));
    }

    // Check format
    let format = image::guess_format(data)
        .map_err(|e| AppError::ImageError(ImageError::InvalidFormat(e.to_string())))?;

    if !ALLOWED_FORMATS.contains(&format) {
        return Err(AppError::ImageError(ImageError::InvalidFileType {
            found: format!("{:?}", format),
            expected: ALLOWED_FORMATS.iter().map(|f| format!("{:?}", f)).collect(),
        }));
    }

    // Check dimensions
    let img = image::load_from_memory(data)
        .map_err(|e| AppError::ImageError(ImageError::LoadError(e.to_string())))?;

    let (width, height) = img.dimensions();
    if width < MIN_WIDTH || height < MIN_HEIGHT {
        return Err(AppError::ImageError(ImageError::TooSmall {
            width,
            height,
            min_width: MIN_WIDTH,
            min_height: MIN_HEIGHT,
        }));
    }

    if width > MAX_WIDTH || height > MAX_HEIGHT {
        return Err(AppError::ImageError(ImageError::TooLarge {
            width,
            height,
            max_width: MAX_WIDTH,
            max_height: MAX_HEIGHT,
        }));
    }

    Ok(())
} 