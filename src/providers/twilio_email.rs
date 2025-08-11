use reqwest::Client;
use tracing::info;

use crate::{
    config::TwilioEmailConfig,
    models::{EmailNotification, Notification},
    providers::errors::ProviderError,
    providers::providers::Provider,
};

pub struct TwilioEmailProvider {
    config: TwilioEmailConfig,
    client: Client,
    url: String,
}

impl TwilioEmailProvider {
    pub fn new(config: TwilioEmailConfig) -> Self {
        let client = Client::new();
        let url = "https://email.twilio.com/v3/mail/send".to_string();
        Self {
            config,
            client,
            url,
        }
    }

    async fn send_email(&self, notification: &EmailNotification) -> Result<(), ProviderError> {
        let payload = serde_json::json!({
            "personalizations": [{
                "to": [{"email": &notification.to}],
                "subject": &notification.subject
            }],
            "from": {
                "email": &notification.from
            },
            "content": [{
                "type": if notification.is_raw_text { "text/plain" } else { "text/html" },
                "value": &notification.body
            }]
        });

        info!("Sending email with Twilio to: {}", notification.to);

        let response = self
            .client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            info!("Email sent successfully via Twilio");
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            info!("Twilio Email API error: {} - {}", status, text);
            Err(ProviderError::ApiError(format!(
                "Twilio Email API error: {} - {}",
                status, text
            )))
        }
    }
}

impl Provider for TwilioEmailProvider {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError> {
        match notification {
            Notification::Email(ref msg) => self.send_email(msg).await,
            _ => Err(ProviderError::UnexpectedError(
                "Invalid notification type".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twilio_email_provider_creation() {
        let config = TwilioEmailConfig {
            api_key: "test_api_key".to_string(),
        };

        let provider = TwilioEmailProvider::new(config.clone());
        assert_eq!(provider.url, "https://email.twilio.com/v3/mail/send");
        assert_eq!(provider.config.api_key, config.api_key);
    }
}
