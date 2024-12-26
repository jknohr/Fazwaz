use anyhow::Result;
use serde_json::Value;
use tracing::debug;

use super::prompts::{Role, ResponseFormat, load_system_prompt, load_response_format};

pub struct PromptService {
    system_prompt: Role,
    response_format: ResponseFormat,
}

impl PromptService {
    pub fn new() -> Self {
        Self {
            system_prompt: load_system_prompt().expect("Failed to load system prompt"),
            response_format: load_response_format().expect("Failed to load response format"),
        }
    }

    pub fn get_system_prompt(&self) -> &str {
        &self.system_prompt.content
    }

    pub fn get_response_format(&self) -> &Value {
        &self.response_format.schema
    }

    pub fn format_image_prompt(&self, image_base64: &str) -> Value {
        debug!("Formatting image analysis prompt");
        
        serde_json::json!({
            "model": "gpt-4-vision-preview",
            "messages": [
                {
                    "role": "system",
                    "content": self.get_system_prompt()
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "Please analyze this property image."
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": format!("data:image/jpeg;base64,{}", image_base64),
                                "detail": "high"
                            }
                        }
                    ]
                }
            ],
            "max_tokens": 4096,
            "response_format": self.get_response_format()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_service_initialization() {
        let service = PromptService::new();
        assert!(!service.get_system_prompt().is_empty());
        assert!(service.get_response_format().is_object());
    }
} 