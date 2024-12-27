//! LLM caller module for handling AI model interactions
//! 
//! This module provides services for:
//! - Batch image analysis
//! - Prompt management
//! - Type definitions

pub mod batch_analysis_service;
pub mod prompt_service;
pub mod prompts;
pub mod types;

pub use batch_analysis_service::BatchAnalysisService;
pub use prompt_service::PromptService;

// Constants
const DEFAULT_MODEL: &str = "gpt-1o-mini";
const DEFAULT_TEMPERATURE: f32 = 0.7;
const MAX_TOKENS: u32 = 4096; 
