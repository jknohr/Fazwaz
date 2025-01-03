use std::sync::Arc;
use tokio::sync::mpsc;
use uuid7;
use tracing::{info, instrument};

use crate::backend::common::error::error::Result;
use crate::backend::f_ai_core::metrics::BatchMetrics;

#[derive(Debug, Clone)]
pub struct BatchProcessor<T> 
where 
    T: Clone + Send + 'static 
{
    metrics: Arc<BatchMetrics>,
    max_attempts: u32,
    job_tx: mpsc::Sender<T>,
}

impl<T> BatchProcessor<T>
where 
    T: Clone + Send + 'static
{
    pub fn new(
        metrics: Arc<BatchMetrics>,
        max_attempts: u32,
        job_tx: mpsc::Sender<T>,
    ) -> Self {
        Self {
            metrics,
            max_attempts,
            job_tx,
        }
    }

    #[instrument(skip(self, jobs))]
    pub async fn process_batch(&self, jobs: Vec<T>) -> Result<String> {
        let batch_id = uuid7::uuid7().to_string();
        info!("Processing batch {}", batch_id);

        for job in jobs {
            self.job_tx.send(job).await?;
        }

        self.metrics.batches_processed.inc();
        Ok(batch_id)
    }
} 