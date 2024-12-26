use crate::backend::common::{Result, AppError, ImageError};
use axum::extract::multipart::{Field, Multipart};

pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const ALLOWED_MIME_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

pub async fn validate_file(content_type: &str, data: &[u8]) -> Result<()> {
    // Check mime type
    if !ALLOWED_MIME_TYPES.contains(&content_type) {
        return Err(AppError::ImageError(ImageError::InvalidFileType {
            found: content_type.to_string(),
            expected: ALLOWED_MIME_TYPES.iter().map(|s| s.to_string()).collect(),
        }));
    }

    // Check file size
    if data.len() > MAX_FILE_SIZE {
        return Err(AppError::ImageError(ImageError::FileTooLarge {
            size: data.len(),
            max: MAX_FILE_SIZE,
        }));
    }

    Ok(())
}

pub async fn extract_and_validate_file(multipart: &mut Multipart) -> Result<(String, String, Vec<u8>)> {
    let field = multipart.next_field().await?
        .ok_or_else(|| AppError::Internal("No file provided".to_string()))?;
    extract_and_validate_field(field).await
}

pub async fn extract_and_validate_field(mut field: Field<'_>) -> Result<(String, String, Vec<u8>)> {
    let filename = field.file_name()
        .ok_or_else(|| AppError::Internal("No filename provided".to_string()))?
        .to_string();

    let content_type = field.content_type()
        .ok_or_else(|| AppError::Internal("No content type provided".to_string()))?
        .to_string();

    let data = field.bytes().await?.to_vec();
    
    validate_file(&content_type, &data).await?;

    Ok((filename, content_type, data))
} 