use async_trait::async_trait;
use reqwest::Client;

use crate::{
    config::TelegramConfig,
    models::{MessageNotification, Notification},
    providers::errors::ProviderError,
    providers::providers::Provider,
};

pub struct TelegramProvider {
    client: Client,
    url: String,
}

impl TelegramProvider {
    pub fn new(config: TelegramConfig) -> Self {
        let client = Client::new();
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            config.bot_token
        );
        Self { client, url }
    }

    async fn send_message(&self, notification: &MessageNotification) -> Result<(), ProviderError> {
        let params = serde_json::json!({
            "chat_id": &notification.recipient,
            "text": &notification.message,
        });

        let response = self
            .client
            .post(&self.url)
            .header("Content-type", "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(ProviderError::ApiError(format!(
                "Telegram API error: {} - {}",
                status, text
            )))
        }
    }
}

#[async_trait]
impl Provider for TelegramProvider {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError> {
        match notification {
            Notification::Message(ref msg) => self.send_message(msg).await,
            _ => Err(ProviderError::UnexpectedError(
                "Invalid notification type".to_string(),
            )),
        }
    }

    fn name(&self) -> &'static str {
        "telegram"
    }

    fn supports_notification(&self, notification: &Notification) -> bool {
        matches!(notification, Notification::Message(_))
    }

    async fn health_check(&self) -> Result<(), ProviderError> {
        // Health check by calling getMe API
        let health_url = self.url.replace("sendMessage", "getMe");
        let response = self
            .client
            .get(&health_url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(ProviderError::ApiError(
                "Telegram bot token invalid".to_string(),
            ))
        }
    }
}
