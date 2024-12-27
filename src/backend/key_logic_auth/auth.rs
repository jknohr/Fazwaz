use axum::{
    extract::State,
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tower_layer::Layer;
use tower_service::Service;
use std::task::{Context, Poll};
use tracing::{info, warn};

use crate::backend::{
    f_ai_core::state::AppState,
    common::error::error::{Result, AppError},
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type BoxFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>;

#[derive(Clone)]
pub struct RequireAuth;

impl RequireAuth {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequireAuth {
    type Service = RequireAuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        RequireAuthMiddleware { service }
    }
}

#[derive(Clone)]
pub struct RequireAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<Request<B>> for RequireAuthMiddleware<S>
where
    S: Service<Request<B>, Response = Response, Error = BoxError> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = BoxError;
    type Future = BoxFuture<std::result::Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        let service = self.service.clone();
        let state = request.extensions().get::<Arc<AppState>>().cloned();

        Box::pin(async move {
            // Extract API key from Authorization header
            let api_key = request
                .headers()
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .ok_or_else(|| BoxError::from(AppError::Unauthorized))?;

            // Validate key using KeyService
            if let Some(state) = state {
                let is_valid = state.key_service.validate_key(api_key).await
                    .map_err(BoxError::from)?;

                if !is_valid {
                    warn!("Invalid API key attempt");
                    return Err(BoxError::from(AppError::Unauthorized));
                }

                info!("Authenticated request with valid API key");
                Ok(service.call(request).await?)
            } else {
                Err(BoxError::from(AppError::Internal("Missing app state".into())))
            }
        })
    }
} 