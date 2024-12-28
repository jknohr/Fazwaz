use serde::Deserialize;
use anyhow::Result;
use tracing_subscriber;
use tracing_subscriber::fmt::time::UtcTime;
use surrealdb::opt::auth::Root;



#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn get_credentials(&self) -> Root<'_> {
        Root {
            username: &self.username,
            password: &self.password,
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

    let timer = UtcTime::new(time::macros::format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
    ));

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_timer(timer)
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