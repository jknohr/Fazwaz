use std::sync::Arc;
use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor, Message,
};
use serde::Serialize;
use handlebars::Handlebars;
use crate::backend::common::error::error::{Result, AppError};
use tracing::{info, error, instrument};

#[derive(Clone)]
pub struct EmailService {
    mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    templates: Arc<Handlebars<'static>>,
    from_address: String,
}

#[derive(Serialize)]
struct ListingEmailContext {
    fullname: String,
    api_key: String,
    listing_id: String,
    upload_url: String,
    support_email: String,
}

impl EmailService {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        username: String,
        password: String,
        from_address: String,
    ) -> Result<Self> {
        let creds = Credentials::new(username, password);
        
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        let mut templates = Handlebars::new();
        templates.register_template_string(
            "listing_confirmation",
            include_str!("../templates/listing_confirmation.html"),
        )?;

        Ok(Self {
            mailer: Arc::new(mailer),
            templates: Arc::new(templates),
            from_address,
        })
    }

    #[instrument(skip(self))]
    pub async fn send_listing_confirmation(
        &self,
        to_email: &str,
        fullname: &str,
        api_key: &str,
        listing_id: &str,
    ) -> Result<()> {
        let context = ListingEmailContext {
            fullname: fullname.to_string(),
            api_key: api_key.to_string(),
            listing_id: listing_id.to_string(),
            upload_url: format!("https://upload.example.com/{}", listing_id),
            support_email: "support@example.com".to_string(),
        };

        let body = self.templates
            .render("listing_confirmation", &context)
            .map_err(|e| AppError::Template(e.to_string()))?;

        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to_email.parse()?)
            .subject("Your Listing API Key")
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(body)?;

        match self.mailer.send(email).await {
            Ok(_) => {
                info!(
                    to_email = %to_email,
                    listing_id = %listing_id,
                    "Sent listing confirmation email"
                );
                Ok(())
            }
            Err(e) => {
                error!(
                    error = %e,
                    to_email = %to_email,
                    listing_id = %listing_id,
                    "Failed to send listing confirmation email"
                );
                Err(AppError::Email(e.to_string()))
            }
        }
    }
} 