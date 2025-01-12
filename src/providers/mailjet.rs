use reqwest::Client;
use tracing::info;

use crate::{config::MailjetConfig, models::EmailNotification};

use super::{errors::ProviderError, EmailProvider};

pub struct MailjetProvider {
    config: MailjetConfig,
    client: Client,
    url: String,
}

impl MailjetProvider {
    pub fn new(config: MailjetConfig) -> Self {
        let client = Client::new();
        let url = if config.v31 {
            "https://api.mailjet.com/v3.1/send".to_string()
        } else {
            "https://api.mailjet.com/v3/send".to_string()
        };
        Self {
            config,
            client,
            url,
        }
    }

    async fn send_email(&self, notifications: &EmailNotification) -> Result<(), ProviderError> {
        let payload = if self.config.v31 {
            serde_json::json!({
                "Messages": [
                    {
                        "From": {
                            "Email": &notifications.from
                        },
                        "To": [
                            {
                                "Email": &notifications.to
                            }
                        ],
                        "Subject": &notifications.subject,
                        "HTMLPart": &notifications.body
                    }
                ]
            })
        } else {
            serde_json::json!({
                "FromEmail": &notifications.from,
                "Subject": &notifications.subject,
                "Html-part": &notifications.body,
                "Recipients": [
                    {
                        "Email": &notifications.to
                    }
                ]
            })
        };

        info!("Sending email with Mailjet: {:?}", payload);

        let response = self
            .client
            .post(&self.url)
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

impl EmailProvider for MailjetProvider {
    async fn send(&self, notification: EmailNotification) -> Result<(), ProviderError> {
        self.send_email(&notification).await
    }
}
