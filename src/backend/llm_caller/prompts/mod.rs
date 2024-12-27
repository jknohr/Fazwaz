pub mod image_analysis;
pub mod types;
pub use image_analysis::ImageAnalysisPrompt;

// Constants for prompt configuration
pub const MAX_TOKENS: u32 = 4096;
pub const DEFAULT_TEMPERATURE: f32 = 0.7; 