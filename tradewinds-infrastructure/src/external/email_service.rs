use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, transport::smtp::authentication::Credentials,
};

use tradewinds_error::AppResult;

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> AppResult<()>;
}

pub struct SmtpEmailService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

impl SmtpEmailService {
    pub fn new(host: &str, username: &str, password: &str, from_email: &str) -> AppResult<Self> {
        let creds = Credentials::new(username.to_string(), password.to_string());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(host)
            .map_err(|e| format!("SMTP configuration error: {}", e))?
            .credentials(creds)
            .build();

        Ok(Self { transport, from_email: from_email.to_string() })
    }
}

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> AppResult<()> {
        let email = Message::builder()
            .from(self.from_email.parse().map_err(|e| format!("From email parse error: {}", e))?)
            .to(to.parse().map_err(|e| format!("To email parse error: {}", e))?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| format!("Email build error: {}", e))?;

        self.transport.send(email).await.map_err(|e| format!("Email send error: {}", e))?;

        Ok(())
    }
}
