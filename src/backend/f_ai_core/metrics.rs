use std::sync::Arc;
use crate::backend::common::error::error::Result;
use crate::backend::f_ai_database::database::DatabaseManager as Database;
use serde_json::{json, Value as JsonValue};
use prometheus::{IntCounter, Registry, Histogram, HistogramOpts};
use serde::{Serialize, Deserialize};
use surrealdb::Response;
use surrealdb::sql::Value;

pub struct MetricsCollector {
    pub registry: Registry,
    pub health_metrics: Arc<HealthMetrics>,
    pub batch_metrics: Arc<BatchMetrics>,
    db: Arc<Database>,
}

impl MetricsCollector {
    pub fn new(db: Arc<Database>) -> Self {
        let registry = Registry::new();
        let health_metrics = Arc::new(HealthMetrics::new());
        let batch_metrics = Arc::new(BatchMetrics::new(&registry));

        Self {
            registry,
            health_metrics,
            batch_metrics,
            db,
        }
    }

    pub async fn increment_counter(&self, name: String, labels: Option<serde_json::Value>) -> Result<()> {
        let response = self.db.client()
            .query("CALL fn::record_metric($name, 'counter', 1, $labels)")
            .bind(("name", name))
            .bind(("labels", labels))
            .await?;
        response.check()?;
        Ok(())
    }

    pub async fn record_gauge(&self, name: String, value: f64, labels: Option<serde_json::Value>) -> Result<()> {
        let response = self.db.client()
            .query("CALL fn::record_metric($name, 'gauge', $value, $labels)")
            .bind(("name", name))
            .bind(("value", value))
            .bind(("labels", labels))
            .await?;
        response.check()?;
        Ok(())
    }

    pub async fn record_histogram(&self, name: String, value: f64, labels: Option<serde_json::Value>) -> Result<()> {
        let response = self.db.client()
            .query("CALL fn::record_metric($name, 'histogram', $value, $labels)")
            .bind(("name", name))
            .bind(("value", value))
            .bind(("labels", labels))
            .await?;
        response.check()?;
        Ok(())
    }

    pub async fn get_metric_summary(&self, name: String, window: String) -> Result<JsonValue> {
        let mut response = self.db.client()
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

        let result: Vec<Value> = response.take(0)?;
        Ok(serde_json::to_value(&result)?)
    }
}

#[derive(Debug, Clone)]
pub struct BatchMetrics {
    pub batches_processed: IntCounter,
    pub jobs_completed: IntCounter,
    pub jobs_failed: IntCounter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMetricsSnapshot {
    pub batches_processed: u64,
    pub jobs_completed: u64,
    pub jobs_failed: u64,
}

impl BatchMetrics {
    pub fn new(registry: &Registry) -> Self {
        let batches_processed = IntCounter::new("batches_processed_total", "Total batches processed").unwrap();
        let jobs_completed = IntCounter::new("jobs_completed_total", "Total jobs completed").unwrap();
        let jobs_failed = IntCounter::new("jobs_failed_total", "Total jobs failed").unwrap();

        registry.register(Box::new(batches_processed.clone())).unwrap();
        registry.register(Box::new(jobs_completed.clone())).unwrap();
        registry.register(Box::new(jobs_failed.clone())).unwrap();

        Self {
            batches_processed,
            jobs_completed,
            jobs_failed,
        }
    }

    pub fn snapshot(&self) -> BatchMetricsSnapshot {
        BatchMetricsSnapshot {
            batches_processed: self.batches_processed.get(),
            jobs_completed: self.jobs_completed.get(),
            jobs_failed: self.jobs_failed.get(),
        }
    }
}

pub struct HealthMetrics {
    pub registry: Registry,
    pub health_check_duration: Histogram,
}

impl HealthMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        let health_check_duration = Histogram::with_opts(HistogramOpts::new(
            "health_check_duration_seconds",
            "Time spent performing health checks"
        )).unwrap();
        
        registry.register(Box::new(health_check_duration.clone())).unwrap();
        
        Self {
            registry,
            health_check_duration,
        }
    }
} 