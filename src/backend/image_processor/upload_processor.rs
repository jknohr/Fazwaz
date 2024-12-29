use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, error, instrument};
use uuid7;
use anyhow::Result;

use crate::backend::{
    common::types::{
        image_types::{ImageChunk, ImageUploadSession, ProcessedImage, ProcessingStatus},
        website_sections::WebsiteSections,
        listing_types::ListingStatus,
    },
    f_ai_database::image_service::ImageService,
    trans_storage::file_manager::FileManager,
};

#[derive(Debug)]
pub struct ProcessingJob {
    pub session_id: String,
    pub listing_id: String,
    pub section: WebsiteSections,
    pub chunks: Vec<ImageChunk>,
    pub gps_coordinates: Option<(f64, f64)>,
}

pub struct UploadProcessor {
    image_service: Arc<ImageService>,
    file_manager: Arc<FileManager>,
    processing_channel: mpsc::Sender<ProcessingJob>,
}

impl UploadProcessor {
    pub fn new(image_service: Arc<ImageService>, file_manager: Arc<FileManager>) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self::spawn_processor(rx);
        
        Self {
            image_service,
            file_manager,
            processing_channel: tx,
        }
    }

    pub async fn queue_upload(&self, job: ProcessingJob) -> Result<()> {
        self.processing_channel.send(job).await?;
        Ok(())
    }

    fn spawn_processor(mut rx: mpsc::Receiver<ProcessingJob>) {
        tokio::spawn(async move {
            while let Some(job) = rx.recv().await {
                if let Err(e) = Self::process_upload(job).await {
                    error!("Failed to process upload: {}", e);
                }
            }
        });
    }

    #[instrument(skip(job))]
    async fn process_upload(job: ProcessingJob) -> Result<()> {
        info!("Starting upload processing for listing {}", job.listing_id);
        
        // Assemble chunks into file data
        let mut assembled_data = Vec::new();
        for chunk in job.chunks {
            assembled_data.extend(chunk.data);
        }

        // Generate image ID and path
        let image_id = uuid7::uuid7().to_string();
        
        // Let FileManager handle the file operations
        self.file_manager.store_temp_file(
            &job.listing_id,
            &image_id,
            &assembled_data,
            job.gps_coordinates,
        ).await?;

        // Queue for analysis
        self.image_service.queue_for_analysis(job.listing_id, image_id).await?;

        Ok(())
    }
} 