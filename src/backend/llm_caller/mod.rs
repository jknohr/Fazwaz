// Sub-modules
mod analysis_client;
mod openai_client;
mod prompt_service;
mod embedding_service;
mod batch;
mod prompts;

// Re-exports
pub use analysis_client::AnalysisClient;
pub use openai_client::OpenAIClient;
pub use prompt_service::PromptService;
pub use embedding_service::EmbeddingService;
pub use batch::BatchProcessor;

// Common imports for the module
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use tokio;
use tracing::{info, warn, error, debug};

// Constants
const DEFAULT_MODEL: &str = "gpt-1o-mini";
const DEFAULT_TEMPERATURE: f32 = 0.7;
const MAX_TOKENS: u32 = 4096; 
