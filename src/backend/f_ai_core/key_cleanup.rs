use tokio::time::{interval, Duration};
use tracing::{info, error};
use std::sync::Arc;
use crate::backend::f_ai_database::database::DatabaseManager as Database;

pub async fn start_key_cleanup(db: Arc<Database>) {
    let mut interval = interval(Duration::from_secs(3600));

    loop {
        interval.tick().await;
        info!("Starting expired key cleanup");
        
        if let Err(e) = db.client()
            .query("DELETE FROM api_keys WHERE expires_at < time::now()")
            .await 
        {
            error!("Failed to clean up expired keys: {}", e);
        }
    }
} 