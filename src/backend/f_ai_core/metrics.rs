use std::sync::Arc;
use crate::{
    db::Database,
    error::Result,
};
use serde_json::json;

pub struct MetricsCollector {
    db: Arc<Database>,
}

impl MetricsCollector {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn increment_counter(&self, name: &str, labels: Option<serde_json::Value>) -> Result<()> {
        self.db.query("CALL fn::record_metric($name, 'counter', 1, $labels)")
            .bind(("name", name))
            .bind(("labels", labels))
            .await?;
        Ok(())
    }

    pub async fn record_gauge(&self, name: &str, value: f64, labels: Option<serde_json::Value>) -> Result<()> {
        self.db.query("CALL fn::record_metric($name, 'gauge', $value, $labels)")
            .bind(("name", name))
            .bind(("value", value))
            .bind(("labels", labels))
            .await?;
        Ok(())
    }

    pub async fn record_histogram(&self, name: &str, value: f64, labels: Option<serde_json::Value>) -> Result<()> {
        self.db.query("CALL fn::record_metric($name, 'histogram', $value, $labels)")
            .bind(("name", name))
            .bind(("value", value))
            .bind(("labels", labels))
            .await?;
        Ok(())
    }

    pub async fn get_metric_summary(&self, name: &str, window: &str) -> Result<serde_json::Value> {
        let mut response = self.db
            .query(r#"
                SELECT 
                    metric_name,
                    metric_type,
                    count() as count,
                    math::sum(value) as sum,
                    math::avg(value) as avg,
                    math::min(value) as min,
                    math::max(value) as max
                FROM metrics 
                WHERE metric_name = $name 
                AND timestamp > time::now() - duration::from_str($window)
                GROUP BY metric_name, metric_type
            "#)
            .bind(("name", name))
            .bind(("window", window))
            .await?;

        let summary = response.take(0)?;
        Ok(summary)
    }
} 