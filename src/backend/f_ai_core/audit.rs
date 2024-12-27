use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::backend::{
    common::error::error::Result,
    f_ai_database::database::DatabaseManager
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub action: String,
    user_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub changes: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

pub struct AuditLogger {
    db: Arc<DatabaseManager>,
}

impl AuditLogger {
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    pub async fn log_action(&self, log: AuditLog) -> Result<()> {
        self.db.client()
            .query("CALL fn::record_audit_log($action, $user_id, $resource_type, $resource_id, $changes)")
            .bind(("action", &log.action))
            .bind(("user_id", &log.user_id))
            .bind(("resource_type", &log.resource_type))
            .bind(("resource_id", &log.resource_id))
            .bind(("changes", &log.changes))
            .await?;
        Ok(())
    }

    pub async fn get_resource_history(&self, resource_type: &str, resource_id: &str) -> Result<Vec<AuditLog>> {
        let mut response = self.db.client()
            .query("SELECT * FROM audit_logs WHERE resource_type = $type AND resource_id = $id ORDER BY timestamp DESC")
            .bind(("type", resource_type))
            .bind(("id", resource_id))
            .await?;
        
        let logs = response.take(0)?;
        Ok(logs)
    }

    pub async fn get_user_actions(&self, user_id: &str, limit: usize) -> Result<Vec<AuditLog>> {
        let mut response = self.db.client()
            .query("SELECT * FROM audit_logs WHERE user_id = $user_id ORDER BY timestamp DESC LIMIT $limit")
            .bind(("user_id", user_id))
            .bind(("limit", limit))
            .await?;
        
        let logs = response.take(0)?;
        Ok(logs)
    }
} 