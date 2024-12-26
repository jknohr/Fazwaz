use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::{
    error::Result,
    monitoring::metrics::HealthMetrics,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: SystemStatus,
    pub components: Vec<ComponentHealth>,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: ComponentStatus,
    pub message: Option<String>,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentStatus {
    Ok,
    Warning,
    Error,
}

pub struct HealthCheck {
    status: Arc<RwLock<HealthStatus>>,
    metrics: Arc<HealthMetrics>,
}

impl HealthCheck {
    pub fn new(metrics: Arc<HealthMetrics>) -> Self {
        Self {
            status: Arc::new(RwLock::new(HealthStatus {
                status: SystemStatus::Healthy,
                components: Vec::new(),
                last_check: Utc::now(),
            })),
            metrics,
        }
    }

    pub async fn check_health(&self) -> Result<HealthStatus> {
        let mut status = self.status.write().await;
        status.last_check = Utc::now();
        
        // Check each component
        let mut components = Vec::new();
        
        // Check LLM service
        components.push(self.check_llm_service().await);
        
        // Check storage service
        components.push(self.check_storage_service().await);
        
        // Update overall status
        status.components = components;
        status.status = self.determine_system_status(&status.components);
        
        self.metrics.health_checks.inc();
        if status.status != SystemStatus::Healthy {
            self.metrics.health_check_failures.inc();
        }
        
        Ok(status.clone())
    }

    async fn check_llm_service(&self) -> ComponentHealth {
        ComponentHealth {
            name: "llm_service".to_string(),
            status: ComponentStatus::Ok,
            message: None,
            last_check: Utc::now(),
        }
    }

    async fn check_storage_service(&self) -> ComponentHealth {
        ComponentHealth {
            name: "storage_service".to_string(),
            status: ComponentStatus::Ok,
            message: None,
            last_check: Utc::now(),
        }
    }

    fn determine_system_status(&self, components: &[ComponentHealth]) -> SystemStatus {
        let error_count = components.iter()
            .filter(|c| c.status == ComponentStatus::Error)
            .count();
            
        let warning_count = components.iter()
            .filter(|c| c.status == ComponentStatus::Warning)
            .count();

        if error_count > 0 {
            SystemStatus::Unhealthy
        } else if warning_count > 0 {
            SystemStatus::Degraded
        } else {
            SystemStatus::Healthy
        }
    }
} 