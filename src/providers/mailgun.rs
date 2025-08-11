use async_trait::async_trait;
use reqwest::Client;

use crate::{
    config::MailgunConfig,
    models::{EmailNotification, Notification},
    providers::errors::ProviderError,
    providers::providers::Provider,
};

pub struct MailgunProvider {
    config: MailgunConfig,
    client: Client,
    url: String,
}

impl MailgunProvider {
    pub fn new(config: MailgunConfig) -> Self {
        let client = Client::new();
        let url = format!(
            "{}/v3/{}/messages",
            config
                .base_url
                .clone()
                .unwrap_or_else(|| "https://api.mailgun.net".to_string()),
            config.domain
        );
        Self {
            config,
            client,
            url,
        }
    }

    // Send notification to Mailgun API
    async fn send_email(&self, notification: &EmailNotification) -> Result<(), ProviderError> {
        let params = [
            ("from", &notification.from),
            ("to", &notification.to),
            ("subject", &notification.subject),
            ("text", &notification.body),
        ];

        let response = self
            .client
            .post(&self.url)
            .basic_auth("api", Some(&self.config.api_key))
            .form(&params)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(ProviderError::ApiError(format!(
                "Mailgun API error: {} - {}",
                status, text
            )))
        }
    }
}

#[async_trait]
impl Provider for MailgunProvider {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError> {
        match notification {
            Notification::Email(ref msg) => self.send_email(msg).await,
            _ => Err(ProviderError::UnexpectedError(
                "Invalid notification type".to_string(),
            )),
        }
    }

    fn name(&self) -> &'static str {
        "mail_mailgun"
    }

    fn supports_notification(&self, notification: &Notification) -> bool {
        matches!(notification, Notification::Email(_))
    }

    async fn health_check(&self) -> Result<(), ProviderError> {
        // Simple health check - verify domain configuration
        if self.config.domain.is_empty() || self.config.api_key.is_empty() {
            return Err(ProviderError::invalid_config(
                "Invalid Mailgun configuration",
            ));
        }
        Ok(())
    }
}
