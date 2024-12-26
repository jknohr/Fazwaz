use tokio::time::{interval, Duration};
use tracing::{info, error};
use crate::f_ai_database::Database;

pub async fn start_key_cleanup(db: Database) {
    let mut interval = interval(Duration::from_secs(3600)); // Run every hour

    loop {
        interval.tick().await;
        info!("Starting expired key cleanup");
        
        if let Err(e) = db.delete_expired_keys().await {
            error!("Failed to clean up expired keys: {}", e);
        }
    }
} 