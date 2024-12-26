use prometheus::{Registry, IntCounter, Histogram, opts, histogram_opts};

pub struct StorageMetrics {
    pub storage_operation_duration: Histogram,
    pub successful_uploads: IntCounter,
    pub successful_downloads: IntCounter,
    pub successful_deletions: IntCounter,
}

impl StorageMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        let storage_operation_duration = Histogram::with_opts(
            histogram_opts!("storage_operation_duration_seconds", "Duration of storage operations")
        )?;

        let successful_uploads = IntCounter::with_opts(
            opts!("successful_uploads_total", "Total number of successful uploads")
        )?;

        let successful_downloads = IntCounter::with_opts(
            opts!("successful_downloads_total", "Total number of successful downloads")
        )?;

        let successful_deletions = IntCounter::with_opts(
            opts!("successful_deletions_total", "Total number of successful deletions")
        )?;

        registry.register(Box::new(storage_operation_duration.clone()))?;
        registry.register(Box::new(successful_uploads.clone()))?;
        registry.register(Box::new(successful_downloads.clone()))?;
        registry.register(Box::new(successful_deletions.clone()))?;

        Ok(Self {
            storage_operation_duration,
            successful_uploads,
            successful_downloads,
            successful_deletions,
        })
    }
}

pub struct LLMMetrics {
    pub embedding_generation_duration: Histogram,
    pub images_analyzed_total: IntCounter,
    pub openai_request_duration: Histogram,
    pub successful_openai_requests: IntCounter,
    pub successful_embeddings: IntCounter,
    pub batch_processing_duration: Histogram,
    pub batch_jobs_total: IntCounter,
    pub batch_jobs_failed: IntCounter,
}

impl LLMMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        let embedding_generation_duration = Histogram::with_opts(
            histogram_opts!("embedding_generation_duration_seconds", "Duration of embedding generation")
        )?;
        let images_analyzed_total = IntCounter::with_opts(
            opts!("images_analyzed_total", "Total number of images analyzed")
        )?;
        let openai_request_duration = Histogram::with_opts(
            histogram_opts!("openai_request_duration_seconds", "Duration of OpenAI requests")
        )?;
        let successful_openai_requests = IntCounter::with_opts(
            opts!("successful_openai_requests", "Number of successful OpenAI requests")
        )?;
        let successful_embeddings = IntCounter::with_opts(
            opts!("successful_embeddings", "Number of successful embeddings generated")
        )?;

        let batch_processing_duration = Histogram::with_opts(
            histogram_opts!("batch_processing_duration_seconds", "Duration of batch processing")
        )?;

        let batch_jobs_total = IntCounter::with_opts(
            opts!("batch_jobs_total", "Total number of batch jobs")
        )?;

        let batch_jobs_failed = IntCounter::with_opts(
            opts!("batch_jobs_failed", "Number of failed batch jobs")
        )?;

        registry.register(Box::new(embedding_generation_duration.clone()))?;
        registry.register(Box::new(images_analyzed_total.clone()))?;
        registry.register(Box::new(openai_request_duration.clone()))?;
        registry.register(Box::new(successful_openai_requests.clone()))?;
        registry.register(Box::new(successful_embeddings.clone()))?;
        registry.register(Box::new(batch_processing_duration.clone()))?;
        registry.register(Box::new(batch_jobs_total.clone()))?;
        registry.register(Box::new(batch_jobs_failed.clone()))?;

        Ok(Self {
            embedding_generation_duration,
            images_analyzed_total,
            openai_request_duration,
            successful_openai_requests,
            successful_embeddings,
            batch_processing_duration,
            batch_jobs_total,
            batch_jobs_failed,
        })
    }
} 