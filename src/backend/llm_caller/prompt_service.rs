use std::sync::Arc;
use include_dir::{include_dir, Dir};
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use tracing::{info, warn};

use crate::backend::common::error::Result;

static PROMPTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/prompts");
static PROMPTS: OnceCell<Arc<PromptService>> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub version: String,
    pub content: String,
    pub parameters: Vec<String>,
}

pub struct PromptService {
    templates: std::collections::HashMap<String, PromptTemplate>,
}

impl PromptService {
    pub fn init() -> Result<()> {
        let mut service = Self {
            templates: std::collections::HashMap::new(),
        };

        // Load all prompt templates
        for entry in PROMPTS_DIR.find("**/*.md").unwrap() {
            if let Some(file) = entry.as_file() {
                let path = file.path().to_string_lossy();
                let content = String::from_utf8_lossy(file.contents());
                
                let template = PromptTemplate {
                    name: path.to_string(),
                    version: "1.0".to_string(),
                    content: content.to_string(),
                    parameters: extract_parameters(&content),
                };

                service.templates.insert(path.to_string(), template);
                info!("Loaded prompt template: {}", path);
            }
        }

        PROMPTS.set(Arc::new(service))?;
        Ok(())
    }

    pub fn get() -> Arc<Self> {
        PROMPTS.get()
            .expect("Prompts not initialized")
            .clone()
    }

    pub fn get_template(&self, name: &str) -> Option<&PromptTemplate> {
        self.templates.get(name)
    }

    pub fn render_template(&self, name: &str, params: &serde_json::Value) -> Result<String> {
        let template = self.get_template(name)?;
        let mut content = template.content.clone();
        
        for param in &template.parameters {
            if let Some(value) = params.get(param) {
                content = content.replace(&format!("{{{{{}}}}}", param), 
                    value.as_str().unwrap_or_default());
            }
        }

        Ok(content)
    }
}

fn extract_parameters(content: &str) -> Vec<String> {
    let mut params = Vec::new();
    let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
    
    for cap in re.captures_iter(content) {
        params.push(cap[1].to_string());
    }
    
    params
} 