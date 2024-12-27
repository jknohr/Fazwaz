use prometheus::{
    IntCounter, 
    Histogram, 
    HistogramOpts, 
    IntCounterVec, 
    IntGauge,
    Opts, 
    Registry,
};

pub struct HealthMetrics {
    pub health_checks: IntCounter,
    pub health_check_failures: IntCounter,
    pub component_status: IntCounterVec,
    pub image_processing_duration: Histogram,
}

pub struct LLMMetrics {
    pub successful_embeddings: IntCounter,
    pub embedding_generation_duration: Histogram,
    pub tokenization_duration: Histogram,
    pub token_counts: Histogram,
    pub model_errors: IntCounterVec,
    pub embedding_dimensions: IntCounterVec,
    pub model_load_duration: Histogram,
    pub batch_size_distribution: Histogram,
    pub batch_processing_duration: Histogram,
    pub batch_jobs_total: IntCounter,
    pub batch_jobs_failed: IntCounter,
}

pub struct OpenAIMetrics {
    pub api_calls: IntCounterVec,
    pub api_latency: Histogram,
    pub api_errors: IntCounterVec,
    pub token_usage: IntCounterVec,
}

pub struct StorageMetrics {
    pub successful_uploads: IntCounter,
    pub failed_uploads: IntCounter,
    pub upload_duration: Histogram,
    pub download_duration: Histogram,
    pub bytes_transferred: IntCounterVec,
    pub storage_errors: IntCounterVec,
    pub bucket_operations: IntCounterVec,
    pub files_stored: IntGauge,
    pub total_storage_bytes: IntGauge,
    pub collection_duration: Histogram,
}

impl HealthMetrics {
    pub fn new(registry: &Registry) -> Self {
        let health_checks = IntCounter::new(
            "health_checks_total",
            "Total number of health checks performed"
        ).unwrap();

        let health_check_failures = IntCounter::new(
            "health_check_failures_total", 
            "Total number of health check failures"
        ).unwrap();

        let component_status = IntCounterVec::new(
            Opts::new(
                "component_status",
                "Status of each component"
            ),
            &["component", "status"]
        ).unwrap();

        let image_processing_duration = Histogram::with_opts(
            HistogramOpts::new(
                "image_processing_duration_seconds",
                "Time spent processing images"
            )
        ).unwrap();

        registry.register(Box::new(health_checks.clone())).unwrap();
        registry.register(Box::new(health_check_failures.clone())).unwrap();
        registry.register(Box::new(component_status.clone())).unwrap();
        registry.register(Box::new(image_processing_duration.clone())).unwrap();

        Self {
            health_checks,
            health_check_failures,
            component_status,
            image_processing_duration,
        }
    }
}

impl LLMMetrics {
    pub fn new(registry: &Registry) -> Self {
        let successful_embeddings = IntCounter::new(
            "successful_embeddings_total",
            "Total number of successfully generated embeddings"
        ).unwrap();

        let embedding_generation_duration = Histogram::with_opts(
            HistogramOpts::new(
                "embedding_generation_duration_seconds",
                "Time spent generating embeddings"
            )
        ).unwrap();

        let tokenization_duration = Histogram::with_opts(
            HistogramOpts::new(
                "tokenization_duration_seconds",
                "Time spent on tokenization"
            )
        ).unwrap();

        let token_counts = Histogram::with_opts(
            HistogramOpts::new(
                "token_counts_total",
                "Distribution of token counts per request"
            )
        ).unwrap();

        let model_errors = IntCounterVec::new(
            Opts::new(
                "llm_model_errors_total",
                "Total number of model errors by type"
            ),
            &["error_type"]
        ).unwrap();

        let embedding_dimensions = IntCounterVec::new(
            Opts::new(
                "embedding_dimensions_total",
                "Distribution of embedding dimensions by model"
            ),
            &["model"]
        ).unwrap();

        let model_load_duration = Histogram::with_opts(
            HistogramOpts::new(
                "model_load_duration_seconds",
                "Time spent loading models"
            )
        ).unwrap();

        let batch_size_distribution = Histogram::with_opts(
            HistogramOpts::new(
                "batch_size_distribution",
                "Distribution of batch sizes"
            )
        ).unwrap();

        let batch_processing_duration = Histogram::with_opts(
            HistogramOpts::new(
                "batch_processing_duration_seconds",
                "Time spent processing batches"
            )
        ).unwrap();

        let batch_jobs_total = IntCounter::new(
            "batch_jobs_total",
            "Total number of batch jobs processed"
        ).unwrap();

        let batch_jobs_failed = IntCounter::new(
            "batch_jobs_failed",
            "Total number of failed batch jobs"
        ).unwrap();

        registry.register(Box::new(successful_embeddings.clone())).unwrap();
        registry.register(Box::new(embedding_generation_duration.clone())).unwrap();
        registry.register(Box::new(tokenization_duration.clone())).unwrap();
        registry.register(Box::new(token_counts.clone())).unwrap();
        registry.register(Box::new(model_errors.clone())).unwrap();
        registry.register(Box::new(embedding_dimensions.clone())).unwrap();
        registry.register(Box::new(model_load_duration.clone())).unwrap();
        registry.register(Box::new(batch_size_distribution.clone())).unwrap();
        registry.register(Box::new(batch_processing_duration.clone())).unwrap();
        registry.register(Box::new(batch_jobs_total.clone())).unwrap();
        registry.register(Box::new(batch_jobs_failed.clone())).unwrap();

        Self {
            successful_embeddings,
            embedding_generation_duration,
            tokenization_duration,
            token_counts,
            model_errors,
            embedding_dimensions,
            model_load_duration,
            batch_size_distribution,
            batch_processing_duration,
            batch_jobs_total,
            batch_jobs_failed,
        }
    }
}

impl OpenAIMetrics {
    pub fn new(registry: &Registry) -> Self {
        let api_calls = IntCounterVec::new(
            Opts::new(
                "openai_api_calls_total",
                "Total number of OpenAI API calls by endpoint"
            ),
            &["endpoint"]
        ).unwrap();

        let api_latency = Histogram::with_opts(
            HistogramOpts::new(
                "openai_api_latency_seconds",
                "Latency of OpenAI API calls"
            )
        ).unwrap();

        let api_errors = IntCounterVec::new(
            Opts::new(
                "openai_api_errors_total",
                "Total number of OpenAI API errors by type"
            ),
            &["error_type"]
        ).unwrap();

        let token_usage = IntCounterVec::new(
            Opts::new(
                "openai_token_usage_total",
                "Total token usage by model"
            ),
            &["model"]
        ).unwrap();

        registry.register(Box::new(api_calls.clone())).unwrap();
        registry.register(Box::new(api_latency.clone())).unwrap();
        registry.register(Box::new(api_errors.clone())).unwrap();
        registry.register(Box::new(token_usage.clone())).unwrap();

        Self {
            api_calls,
            api_latency,
            api_errors,
            token_usage,
        }
    }
}

impl StorageMetrics {
    pub fn new(registry: &Registry) -> Self {
        let successful_uploads = IntCounter::new(
            "storage_uploads_total",
            "Total number of successful file uploads"
        ).unwrap();
        
        let failed_uploads = IntCounter::new(
            "storage_uploads_failed",
            "Total number of failed file uploads"
        ).unwrap();
        
        let upload_duration = Histogram::with_opts(
            HistogramOpts::new(
                "storage_upload_duration_seconds",
                "Time spent uploading files"
            )
        ).unwrap();
        
        let download_duration = Histogram::with_opts(
            HistogramOpts::new(
                "storage_download_duration_seconds",
                "Time spent downloading files"
            )
        ).unwrap();
        
        let bytes_transferred = IntCounterVec::new(
            Opts::new(
                "storage_bytes_transferred_total",
                "Total bytes transferred"
            ),
            &["direction"]
        ).unwrap();
        
        let storage_errors = IntCounterVec::new(
            Opts::new(
                "storage_errors_total",
                "Total number of storage errors"
            ),
            &["error_type"]
        ).unwrap();
        
        let bucket_operations = IntCounterVec::new(
            Opts::new(
                "storage_bucket_operations_total",
                "Total number of bucket operations"
            ),
            &["operation"]
        ).unwrap();

        let files_stored = IntGauge::new(
            "storage_files_total",
            "Total number of files currently stored"
        ).unwrap();

        let total_storage_bytes = IntGauge::new(
            "storage_bytes_total",
            "Total bytes stored"
        ).unwrap();

        let collection_duration = Histogram::with_opts(
            HistogramOpts::new(
                "storage_collection_duration_seconds",
                "Time spent collecting storage metrics"
            )
        ).unwrap();

        registry.register(Box::new(successful_uploads.clone())).unwrap();
        registry.register(Box::new(failed_uploads.clone())).unwrap();
        registry.register(Box::new(upload_duration.clone())).unwrap();
        registry.register(Box::new(download_duration.clone())).unwrap();
        registry.register(Box::new(bytes_transferred.clone())).unwrap();
        registry.register(Box::new(storage_errors.clone())).unwrap();
        registry.register(Box::new(bucket_operations.clone())).unwrap();
        registry.register(Box::new(files_stored.clone())).unwrap();
        registry.register(Box::new(total_storage_bytes.clone())).unwrap();
        registry.register(Box::new(collection_duration.clone())).unwrap();
        
        Self {
            successful_uploads,
            failed_uploads,
            upload_duration,
            download_duration,
            bytes_transferred,
            storage_errors,
            bucket_operations,
            files_stored,
            total_storage_bytes,
            collection_duration,
        }
    }
} 