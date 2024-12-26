use axum::extract::multipart::{Field, Multipart};
use crate::error::{Result, AppError, ImageValidationError};
use tracing::{info, warn, instrument};

pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

#[derive(Debug)]
pub struct ValidatedImageFile {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub size: usize,
}

#[instrument(skip(field))]
pub async fn validate_image_file(field: &mut Field<'_>) -> Result<()> {
    // Check file size
    if let Some(size) = field.size_hint().1 {
        if size > MAX_FILE_SIZE {
            warn!("File size {} exceeds maximum allowed size {}", size, MAX_FILE_SIZE);
            return Err(ImageValidationError::FileTooLarge {
                size,
                max: MAX_FILE_SIZE,
            }.into());
        }
    }

    // Check mime type
    let content_type = field.content_type()
        .ok_or_else(|| ImageValidationError::NoContentType)?;

    if !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
        warn!("Invalid content type: {}", content_type);
        return Err(ImageValidationError::InvalidFileType {
            found: content_type.to_string(),
            expected: ALLOWED_MIME_TYPES.iter().map(|s| s.to_string()).collect(),
        }.into());
    }

    Ok(())
}

#[instrument(skip(multipart))]
pub async fn extract_and_validate_image(multipart: &mut Multipart) -> Result<ValidatedImageFile> {
    let field = multipart.next_field().await?
        .ok_or_else(|| ImageValidationError::NoFile)?;
    
    validate_image_file(&mut field).await?;
    
    let filename = field.file_name()
        .ok_or_else(|| ImageValidationError::NoFilename)?
        .to_string();

    let content_type = field.content_type()
        .ok_or_else(|| ImageValidationError::NoContentType)?
        .to_string();

    let data = field.bytes().await?;
    let size = data.len();

    Ok(ValidatedImageFile {
        filename,
        content_type,
        data: data.to_vec(),
        size,
    })
} 