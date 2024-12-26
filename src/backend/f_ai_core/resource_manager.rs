use tokio::sync::Semaphore;
use std::sync::Arc;
use anyhow::Result;
use tracing::info;

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
        Ok(self.upload_semaphore.acquire().await?)
    }

    pub async fn acquire_processing(&self) -> Result<SemaphorePermit> {
        Ok(self.processing_semaphore.acquire().await?)
    }

    pub async fn acquire_search(&self) -> Result<SemaphorePermit> {
        Ok(self.search_semaphore.acquire().await?)
    }

    pub async fn acquire_embedding(&self) -> Result<SemaphorePermit> {
        Ok(self.embedding_semaphore.acquire().await?)
    }
}

pub struct SemaphorePermit<'a> {
    _permit: tokio::sync::SemaphorePermit<'a>,
}

impl<'a> Drop for SemaphorePermit<'a> {
    fn drop(&mut self) {
        // Permit is automatically released when dropped
    }
} 