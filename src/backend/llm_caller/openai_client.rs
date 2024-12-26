use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use reqwest::Client;
use tokio;
use tracing::{info, error};

use crate::backend::llm_caller::prompts::{load_system_prompt, load_response_format};

pub struct OpenAIClient {
    api_key: String,
    client: Client,
    base_url: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    async fn analyze_image(&self, image_data: &[u8]) -> Result<serde_json::Value> {
        // Implementation details...
        todo!()
    }
} 