use serde::Deserialize;
use anyhow::Result;
use tracing_subscriber;
use tracing_subscriber::fmt::time::UtcTime;



#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub namespace: String,
    pub database: String,
    username: Option<String>,
    pub password: Option<String>,
}

impl DatabaseConfig {
    pub fn get_credentials(&self) -> surrealdb::opt::auth::Root {
        surrealdb::opt::auth::Root {
            username: self.username.as_deref().unwrap_or("root"),
            password: self.password.as_deref().unwrap_or("root"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum LogFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "text")]
    Text,
}

pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.level));

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(UtcTime::rfc_3339())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true);

    match config.format {
        LogFormat::Json => {
            subscriber.json().init();
        }
        LogFormat::Text => {
            subscriber.init();
        }
    }

    Ok(())
} 