use crate::error::{Result, AppError, ImageError};
use axum::extract::multipart::{Field, Multipart};

pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

pub async fn validate_file(field: &mut Field<'_>) -> Result<()> {
    // Check file size
    if let Some(size) = field.size_hint().1 {
        if size > MAX_FILE_SIZE {
            return Err(AppError::ImageError(ImageError::FileTooLarge {
                size,
                max: MAX_FILE_SIZE,
            }));
        }
    }

    // Check mime type
    let content_type = field.content_type()
        .ok_or_else(|| AppError::Internal("No content type provided".to_string()))?;

    if !ALLOWED_MIME_TYPES.contains(&content_type) {
        return Err(AppError::ImageError(ImageError::InvalidFileType {
            found: content_type.to_string(),
            expected: ALLOWED_MIME_TYPES.iter().map(|s| s.to_string()).collect(),
        }));
    }

    Ok(())
}

pub async fn extract_and_validate_file(multipart: &mut Multipart) -> Result<(String, String, Vec<u8>)> {
    let field = multipart.next_field().await?;
    extract_and_validate_field(field).await
}

pub async fn extract_and_validate_field(mut field: Field<'_>) -> Result<(String, String, Vec<u8>)> {
    validate_file(field)?;
    
    let filename = field.file_name()
        .ok_or_else(|| AppError::Internal("No filename provided".to_string()))?
        .to_string();

    let content_type = field.content_type()
        .ok_or_else(|| AppError::Internal("No content type provided".to_string()))?
        .to_string();

    let data = field.bytes().await?;

    Ok((filename, content_type, data.to_vec()))
} 