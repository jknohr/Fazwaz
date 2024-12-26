pub const ANALYZE_IMAGE: &str = include_str!("image_analysis.md");
pub const SYSTEM_ROLE: &str = include_str!("role.json");
pub const RESPONSE_SCHEMA: &str = include_str!("response.json");

pub struct ImageAnalysisPrompt;

impl ImageAnalysisPrompt {
    pub fn get_prompt() -> &'static str {
        ANALYZE_IMAGE
    }

    pub fn get_role() -> &'static str {
        SYSTEM_ROLE
    }

    pub fn get_schema() -> &'static str {
        RESPONSE_SCHEMA
    }
} 