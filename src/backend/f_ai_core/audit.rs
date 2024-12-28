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
        let mut query = self.db.client()
            .query("CALL fn::record_audit_log($action, $user_id, $resource_type, $resource_id, $changes)");
        
        query = query
            .bind(("action", log.action))
            .bind(("user_id", log.user_id))
            .bind(("resource_type", log.resource_type))
            .bind(("resource_id", log.resource_id))
            .bind(("changes", log.changes));
        
        query.await?;
        Ok(())
    }

    pub async fn get_resource_history(&self, resource_type: String, resource_id: String) -> Result<Vec<AuditLog>> {
        let mut query = self.db.client()
            .query("SELECT * FROM audit_logs WHERE resource_type = $type AND resource_id = $id ORDER BY timestamp DESC");
        
        query = query
            .bind(("type", resource_type))
            .bind(("id", resource_id));
        
        let mut response = query.await?;
        let logs = response.take(0)?;
        Ok(logs)
    }

    pub async fn get_user_actions(&self, user_id: String, limit: usize) -> Result<Vec<AuditLog>> {
        let mut query = self.db.client()
            .query("SELECT * FROM audit_logs WHERE user_id = $user_id ORDER BY timestamp DESC LIMIT $limit");
        
        query = query
            .bind(("user_id", user_id))
            .bind(("limit", limit));
        
        let mut response = query.await?;
        let logs = response.take(0)?;
        Ok(logs)
    }
} 