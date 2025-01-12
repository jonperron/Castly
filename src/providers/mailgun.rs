use reqwest::Client;

use crate::config::MailgunConfig;
use crate::models::EmailNotification;
use crate::providers::errors::ProviderError;
use crate::providers::providers::EmailProvider;

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

impl EmailProvider for MailgunProvider {
    async fn send(&self, notification: EmailNotification) -> Result<(), ProviderError> {
        self.send_email(&notification).await
    }
}
