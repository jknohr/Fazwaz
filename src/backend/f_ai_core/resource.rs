use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{info, warn};
use serde::{Serialize, Deserialize};

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_concurrent_llm_calls: usize,
    pub max_concurrent_uploads: usize,
    pub max_concurrent_embeddings: usize,
    pub max_batch_size: usize,
}

pub struct ResourceManager {
    llm_semaphore: Arc<Semaphore>,
    upload_semaphore: Arc<Semaphore>,
    embedding_semaphore: Arc<Semaphore>,
    limits: ResourceLimits,
}

impl ResourceManager {
    pub fn new(limits: ResourceLimits) -> Self {
        info!("Initializing resource manager with limits: {:?}", limits);
        
        Self {
            llm_semaphore: Arc::new(Semaphore::new(limits.max_concurrent_llm_calls)),
            upload_semaphore: Arc::new(Semaphore::new(limits.max_concurrent_uploads)),
            embedding_semaphore: Arc::new(Semaphore::new(limits.max_concurrent_embeddings)),
            limits,
        }
    }

    pub async fn acquire_llm(&self) -> Result<SemaphorePermit> {
        Ok(SemaphorePermit::new(self.llm_semaphore.acquire().await?))
    }

    pub async fn acquire_upload(&self) -> Result<SemaphorePermit> {
        Ok(SemaphorePermit::new(self.upload_semaphore.acquire().await?))
    }

    pub async fn acquire_embedding(&self) -> Result<SemaphorePermit> {
        Ok(SemaphorePermit::new(self.embedding_semaphore.acquire().await?))
    }

    pub fn get_batch_size_limit(&self) -> usize {
        self.limits.max_batch_size
    }
}

pub struct SemaphorePermit<'a> {
    _permit: tokio::sync::SemaphorePermit<'a>,
}

impl<'a> SemaphorePermit<'a> {
    fn new(permit: tokio::sync::SemaphorePermit<'a>) -> Self {
        Self { _permit: permit }
    }
}

impl<'a> Drop for SemaphorePermit<'a> {
    fn drop(&mut self) {
        // Permit is automatically released when dropped
    }
} 