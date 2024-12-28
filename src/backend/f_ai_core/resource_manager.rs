use tokio::sync::Semaphore;
use std::sync::Arc;
use crate::backend::common::error::error::{Result, AppError};
use tracing::info;
use crate::backend::f_ai_core::SemaphorePermit;

#[derive(Debug)]
pub struct ResourceConfig {
    pub max_concurrent_uploads: usize,
    pub max_concurrent_processing: usize,
    pub max_concurrent_searches: usize,
    pub max_concurrent_embeddings: usize,
}

pub struct ResourceManager {
    upload_semaphore: Arc<Semaphore>,
    processing_semaphore: Arc<Semaphore>,
    search_semaphore: Arc<Semaphore>,
    embedding_semaphore: Arc<Semaphore>,
}

impl ResourceManager {
    pub fn new(config: ResourceConfig) -> Self {
        info!("Initializing resource manager with config: {:?}", config);
        
        Self {
            upload_semaphore: Arc::new(Semaphore::new(config.max_concurrent_uploads)),
            processing_semaphore: Arc::new(Semaphore::new(config.max_concurrent_processing)),
            search_semaphore: Arc::new(Semaphore::new(config.max_concurrent_searches)),
            embedding_semaphore: Arc::new(Semaphore::new(config.max_concurrent_embeddings)),
        }
    }

    pub async fn acquire_upload(&self) -> Result<SemaphorePermit> {
        let permit = self.upload_semaphore.acquire().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(SemaphorePermit::new(permit))
    }

    pub async fn acquire_processing(&self) -> Result<SemaphorePermit> {
        let permit = self.processing_semaphore.acquire().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(SemaphorePermit::new(permit))
    }

    pub async fn acquire_search(&self) -> Result<SemaphorePermit> {
        let permit = self.search_semaphore.acquire().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(SemaphorePermit::new(permit))
    }

    pub async fn acquire_embedding(&self) -> Result<SemaphorePermit> {
        let permit = self.embedding_semaphore.acquire().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(SemaphorePermit::new(permit))
    }
} 