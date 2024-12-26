use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::backend::common::{Result, AppError};

pub async fn require_auth<B>(
    State(state): State<Arc<AppState>>,
    req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    let api_key = req.headers()
        .get("X-API-Key")
        .ok_or(AppError::Unauthorized)?;

    state.key_service.validate_key(api_key.to_str()?).await?;
    
    Ok(next.run(req).await)
} 