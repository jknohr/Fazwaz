use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Vec<MessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text { text: String },
    
    #[serde(rename = "image_url")]
    ImageUrl { 
        image_url: ImageUrl,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    pub detail: Option<String>,
}

// Add other shared types... 