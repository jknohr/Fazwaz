use config::{Config as ConfigBuilder, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use crate::backend::common::error::{Result, AppError};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub openai: OpenAIConfig,
    pub email: EmailConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok(); // Load .env file if it exists

        let config = ConfigBuilder::builder()
            // Start with default settings
            .add_source(File::with_name("config/default"))
            // Add environment-specific settings
            .add_source(File::with_name(&format!(
                "config/{}",
                env::var("RUN_ENV").unwrap_or_else(|_| "development".into())
            )).required(false))
            // Add environment variables with prefix "APP_"
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub endpoint: String,
    pub region: String,
    pub bucket_prefix: String,
    pub access_key: String,
    pub secret_key: String,
}

impl StorageConfig {
    pub fn validate(&self) -> Result<()> {
        if self.endpoint.is_empty() || self.access_key.is_empty() || self.secret_key.is_empty() {
            return Err(AppError::Validation("Storage credentials cannot be empty".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub organization: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    pub reply_to: Option<String>,
    pub encryption: SmtpEncryption,
    pub timeout_seconds: u64,
    pub templates_dir: String,
    pub max_emails_per_minute: u32,
    pub max_recipients_per_email: u32,
    pub two_factor: Option<TwoFactorConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorConfig {
    pub enabled: bool,
    pub app_password: String,
    pub backup_codes: Vec<String>,
    pub totp_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmtpEncryption {
    None,
    Ssl,
    StartTls,
} 