use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::id_types::BatchId;

#[derive(Debug, Serialize, Deserialize)]
pub enum BatchStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchProcessingStatus {
    pub batch_id: BatchId,
    pub status: BatchStatus,
    pub total: usize,
    pub processed: usize,
    pub failed: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} 