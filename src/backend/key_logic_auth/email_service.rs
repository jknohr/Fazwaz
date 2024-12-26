use handlebars::Handlebars;
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde_json::json;
use tracing::{info, instrument};

use crate::backend::common::{Result, AppError};

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
        handlebars.register_template_string("key_email", include_str!("../assets/key_email.html"))?;
        handlebars.register_template_string("key_email_text", include_str!("../assets/key_email_text.txt"))?;

        Ok(Self {
            mailer,
            handlebars,
            from_email,
        })
    }

    #[instrument(skip(self))]
    pub async fn send_key_email(&self, email: &str, name: &str, key: &str) -> Result<()> {
        info!("Sending key email to: {}", email);

        let data = json!({
            "name": name,
            "key": key,
            "year": chrono::Utc::now().year(),
        });

        let html_body = self.handlebars.render("key_email", &data)?;
        let text_body = self.handlebars.render("key_email_text", &data)?;

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(email.parse()?)
            .subject("Your Access Key")
            .multipart(
                MultiPart::alternative()
                    .singlepart(SinglePart::builder().header(header::ContentType::TEXT_PLAIN).body(text_body))
                    .singlepart(SinglePart::builder().header(header::ContentType::TEXT_HTML).body(html_body))
            )?;

        self.mailer.send(email).await?;
        info!("Successfully sent key email to: {}", email);
        Ok(())
    }
} 