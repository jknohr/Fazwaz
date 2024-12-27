use handlebars::Handlebars;
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde_json::json;
use tracing::{info, instrument};
use crate::backend::common::error::error::Result;

#[derive(Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    handlebars: Handlebars<'static>,
    from_email: String,
}

impl EmailService {
    #[instrument(skip(smtp_password))]
    pub fn new(
        smtp_host: String,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
    ) -> Result<Self> {
        info!("Initializing email service");
        
        let creds = Credentials::new(smtp_username, smtp_password);
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
            .credentials(creds)
            .build();

        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("key_email", include_str!("templates/key_email.html"))?;
        handlebars.register_template_string("key_email_text", include_str!("templates/key_email_text.txt"))?;

        Ok(Self {
            mailer,
            handlebars,
            from_email,
        })
    }

    // ... rest of implementation
} 