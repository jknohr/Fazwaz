use serde::{Serialize, Deserialize};
use std::fs;
use anyhow::Result;

// Re-export the image analysis module
mod image_analysis;
pub use image_analysis::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFormat {
    pub format: String,
    pub schema: serde_json::Value,
}

impl Role {
    fn load() -> Result<Self> {
        let content = fs::read_to_string("src/backend/llm_caller/prompts/role.json")?;
        Ok(serde_json::from_str(&content)?)
    }
}

impl ResponseFormat {
    fn load() -> Result<Self> {
        let content = fs::read_to_string("src/backend/llm_caller/prompts/responseformat.json")?;
        Ok(serde_json::from_str(&content)?)
    }
}

// Export common prompt utilities
pub fn load_system_prompt() -> Result<Role> {
    Role::load()
}

pub fn load_response_format() -> Result<ResponseFormat> {
    ResponseFormat::load()
} 