use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::backend::common::{
    config::EmailConfig,
    error::{Result, AppError},
};

#[async_trait]
pub trait EmailTrigger: Send + Sync {
    async fn trigger_email(&self, request: EmailRequest) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub template_name: Option<String>,
}

pub struct AutomatedEmailService {
    config: EmailConfig,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl AutomatedEmailService {
    pub async fn new(config: EmailConfig) -> Result<Self> {
        let creds = if let Some(two_factor) = &config.two_factor {
            if two_factor.enabled {
                Credentials::new(config.username.clone(), two_factor.app_password.clone())
            } else {
                Credentials::new(config.username.clone(), config.password.clone())
            }
        } else {
            Credentials::new(config.username.clone(), config.password.clone())
        };

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)?
            .credentials(creds)
            .port(config.smtp_port)
            .build();

        Ok(Self {
            config,
            transport,
        })
    }
}

#[async_trait]
impl EmailTrigger for AutomatedEmailService {
    async fn trigger_email(&self, request: EmailRequest) -> Result<()> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.config.from_name, self.config.from_address).parse()?)
            .to(request.to.parse()?)
            .subject(request.subject)
            .body(request.body.to_string())?;

        self.transport
            .send(email)
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}

// Example usage in an agent/tool context
pub async fn send_automated_email(
    service: &impl EmailTrigger,
    to: String,
    subject: String,
    body: String,
) -> Result<()> {
    let request = EmailRequest {
        to,
        subject,
        body,
        template_name: None,
    };

    service.trigger_email(request).await
} 