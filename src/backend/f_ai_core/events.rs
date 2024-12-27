use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::backend::{
    f_ai_database::database::DatabaseManager as Database,
    common::error::error::Result
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warn,
    Error,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_type: String,
    pub severity: Severity,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

pub struct EventLogger {
    db: Arc<Database>,
}

impl EventLogger {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn log_event(&self, event: SystemEvent) -> Result<()> {
        self.db.client()
            .query("CALL fn::record_system_event($type, $severity, $message, $metadata)")
            .bind(("type", &event.event_type))
            .bind(("severity", format!("{:?}", event.severity).to_lowercase()))
            .bind(("message", &event.message))
            .bind(("metadata", &event.metadata))
            .await?;
        Ok(())
    }
    pub async fn get_recent_events(&self, limit: usize) -> Result<Vec<SystemEvent>> {
        let mut response = self.db.client()
            .query("SELECT * FROM system_events ORDER BY timestamp DESC LIMIT $limit")
            .bind(("limit", limit))
            .await?;
        
        let events = response.take(0)?;
        Ok(events)
    }
} 
