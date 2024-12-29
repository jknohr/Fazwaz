pub mod processor;
pub mod analysis_pipeline;
pub mod job_scheduler;
pub mod image_utils;
pub mod upload_processor;
pub mod color;
pub mod histogram;
pub mod quality_report;

// Only expose what's needed
pub use processor::ImageProcessor;
pub use job_scheduler::ImageJobScheduler;
pub use upload_processor::UploadProcessor;