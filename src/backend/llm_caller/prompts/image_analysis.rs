use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAnalysisPrompt {
    pub content: String,
    pub role: String,
    pub response_format: ResponseFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFormat {
    pub format: String,
    pub schema: serde_json::Value,
}

impl Default for ImageAnalysisPrompt {
    fn default() -> Self {
        Self {
            content: include_str!("image_analysis.md").to_string(),
            role: include_str!("role.json").to_string(),
            response_format: serde_json::from_str(include_str!("response.json"))
                .expect("Invalid response format JSON"),
        }
    }
}

impl ImageAnalysisPrompt {
    pub fn get_system_content(&self) -> String {
        format!("{}\n\nResponse format: {}", 
            self.content,
            serde_json::to_string_pretty(&self.response_format.schema).unwrap()
        )
    }
} 