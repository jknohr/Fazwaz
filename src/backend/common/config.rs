use config::{Config as ConfigBuilder, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use crate::backend::common::error::error::{Result, AppError};

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
    username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub endpoint: String,
    pub region: String,
    pub bucket_prefix: String,
    pub access_key: String,
    pub secret_key: String,
    pub locations: LocationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LocationConfig {
    pub country: String,
    pub district: String,
    pub subdistrict: String,
}

impl StorageConfig {
    pub fn validate(&self) -> Result<()> {
        if self.endpoint.is_empty() || self.access_key.is_empty() || self.secret_key.is_empty() {
            return Err(AppError::Validation("Storage credentials cannot be empty".into()));
        }
        if self.locations.country.is_empty() || self.locations.district.is_empty() || self.locations.subdistrict.is_empty() {
            return Err(AppError::Validation("Location fields cannot be empty".into()));
        }
        Ok(())
    }

    pub fn get_bucket_name(&self) -> String {
        format!("{}-{}-{}-{}", 
            self.bucket_prefix,
            self.locations.country.to_lowercase(),
            self.locations.district.to_lowercase(),
            self.locations.subdistrict.to_lowercase()
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub organization: Option<String>,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub timeout: Option<u64>,
    pub base_url: Option<String>,
}

impl OpenAIConfig {
    pub fn into_client_config(self) -> async_openai::config::OpenAIConfig {
        let mut config = async_openai::config::OpenAIConfig::new()
            .with_api_key(self.api_key);
        
        if let Some(org) = self.organization {
            config = config.with_org_id(org);
        }
        
        if let Some(url) = self.base_url {
            config = config.with_api_base(url);
        }

        config
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    pub username: String,
    pub password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub from_name: String,
    pub from_address: String,
    pub two_factor: Option<TwoFactorConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwoFactorConfig {
    pub enabled: bool,
    pub app_password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmtpEncryption {
    None,
    Ssl,
    StartTls,
} 