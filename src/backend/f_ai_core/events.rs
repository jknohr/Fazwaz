use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::{
    f_ai_database::Database,
    error::Result,
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
        self.db.query("CALL fn::record_system_event($type, $severity, $message, $metadata)")
            .bind(("type", &event.event_type))
            .bind(("severity", format!("{:?}", event.severity).to_lowercase()))
            .bind(("message", &event.message))
            .bind(("metadata", &event.metadata))
            .await?;
        Ok(())
    }
} 