use std::sync::Arc;
use tokio::time::{self, Duration};
use crate::{
    state::AppState,
    error::Result,
};
use tracing::{error, info};

pub async fn start_temp_file_cleanup(state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            if let Err(e) = cleanup_expired_temp_files(&state).await {
                error!("Failed to cleanup temp files: {}", e);
                state.metrics.cleanup_failures.inc();
            }
        }
    });
}

async fn cleanup_expired_temp_files(state: &AppState) -> Result<()> {
    let timer = state.metrics.cleanup_duration.start_timer();
    let cutoff = chrono::Utc::now() - chrono::Duration::hours(1);
    
    let expired_files = state.db.get_expired_temp_files(cutoff).await?;
    let mut deleted = 0;
    let mut failed = 0;

    for file in expired_files {
        match state.temp_storage.delete(&file.path).await {
            Ok(_) => {
                deleted += 1;
                state.db.mark_temp_file_deleted(&file.id).await?;
            }
            Err(e) => {
                error!("Failed to delete temp file {}: {}", file.path, e);
                failed += 1;
            }
        }
    }

    info!("Temp file cleanup: {} deleted, {} failed", deleted, failed);
    state.metrics.files_cleaned.inc_by(deleted);
    state.metrics.cleanup_failures.inc_by(failed);
    timer.observe_duration();

    Ok(())
} 