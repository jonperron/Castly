use reqwest::Client;
use tracing::info;

use crate::{
    config::MailjetConfig,
    models::{EmailNotification, Notification},
    providers::errors::ProviderError,
    providers::providers::Provider,
};

pub struct MailjetProvider {
    config: MailjetConfig,
    client: Client,
    urls: MailjeSendUrls,
}

pub struct MailjeSendUrls {
    pub url_v31: String,
    pub url_v3: String,
}

impl Default for MailjeSendUrls {
    fn default() -> Self {
        Self {
            url_v31: "https://api.mailjet.com/v3.1/send".to_string(),
            url_v3: "https://api.mailjet.com/v3/send".to_string(),
        }
    }
}
impl MailjetProvider {
    pub fn new(config: MailjetConfig) -> Self {
        let client = Client::new();
        let urls: MailjeSendUrls = Default::default();
        Self {
            config,
            client,
            urls,
        }
    }

    async fn send_email(&self, notifications: &EmailNotification) -> Result<(), ProviderError> {
        let payload = if self.config.v31 {
            let message = if notifications.is_raw_text {
                serde_json::json!({
                    "From": { "Email": &notifications.from },
                    "To": [{ "Email": &notifications.to }],
                    "Subject": &notifications.subject,
                    "TextPart": &notifications.body,
                })
            } else {
                serde_json::json!({
                    "From": { "Email": &notifications.from },
                    "To": [{ "Email": &notifications.to }],
                    "Subject": &notifications.subject,
                    "HTMLPart": &notifications.body,
                })
            };
            serde_json::json!({ "Messages": [message] })
        } else {
            if notifications.is_raw_text {
                serde_json::json!({
                    "FromEmail": &notifications.from,
                    "Subject": &notifications.subject,
                    "Text-part": &notifications.body,
                    "Recipients": [{ "Email": &notifications.to }]
                })
            } else {
                serde_json::json!({
                    "FromEmail": &notifications.from,
                    "Subject": &notifications.subject,
                    "Html-part": &notifications.body,
                    "Recipients": [{ "Email": &notifications.to }]
                })
            }
        };

        info!("Sending email with Mailjet: {:?}", payload);

        let response = self
            .client
            .post(if self.config.v31 {
                &self.urls.url_v31
            } else {
                &self.urls.url_v3
            })
            .header("Content-type", "application/json")
            .basic_auth(&self.config.api_key, Some(&self.config.api_secret))
            .json(&payload)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            info!("Mailjet API error: {} - {}", status, text);
            Err(ProviderError::ApiError(format!(
                "Mailjet API error: {} - {}",
                status, text
            )))
        }
    }
}

impl Provider for MailjetProvider {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError> {
        match notification {
            Notification::Email(ref msg) => self.send_email(msg).await,
            _ => Err(ProviderError::UnexpectedError(
                "Invalid notification type".to_string(),
            )),
        }
    }
}
