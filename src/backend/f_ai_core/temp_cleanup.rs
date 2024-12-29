use std::sync::Arc;
use tokio::time::{self, Duration};
use crate::{backend::common::error::error::Result, backend::f_ai_core::state::AppState};
use tracing::{error, info};

pub async fn start_temp_file_cleanup(state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            if let Err(e) = cleanup_expired_temp_files(&state).await {
                error!("Failed to cleanup temp files: {}", e);
                state.increment_cleanup_failures(1);
            }
        }
    });
}

async fn cleanup_expired_temp_files(state: &AppState) -> Result<()> {
    let timer = state.start_cleanup_timer();
    let cutoff = chrono::Utc::now() - chrono::Duration::hours(1);
    
    let expired_files = state.get_expired_temp_files_with_metrics(cutoff).await?;
    let mut deleted = 0;
    let mut failed = 0;

    for file in expired_files {
        match state.delete_temp_file(&file.path).await {
            Ok(_) => {
                deleted += 1;
                state.mark_temp_file_deleted_with_metrics(&file.id).await?;
            }
            Err(e) => {
                error!("Failed to delete temp file {}: {}", file.path, e);
                failed += 1;
            }
        }
    }

    info!("Temp file cleanup: {} deleted, {} failed", deleted, failed);
    state.increment_files_cleaned(deleted);
    state.increment_cleanup_failures(failed);
    timer.observe_duration();

    Ok(())
} 