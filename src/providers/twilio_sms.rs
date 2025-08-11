use reqwest::Client;
use tracing::info;

use crate::{
    config::TwilioSmsConfig,
    models::{notifications::SMSNotification, Notification},
    providers::errors::ProviderError,
    providers::providers::Provider,
};

pub struct TwilioSmsProvider {
    config: TwilioSmsConfig,
    client: Client,
    url: String,
}

impl TwilioSmsProvider {
    pub fn new(config: TwilioSmsConfig) -> Self {
        let client = Client::new();
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            config.account_sid
        );
        Self {
            config,
            client,
            url,
        }
    }

    async fn send_sms(&self, notification: &SMSNotification) -> Result<(), ProviderError> {
        let params = [
            ("From", &notification.from),
            ("To", &notification.to),
            ("Body", &notification.body),
        ];

        info!("Sending SMS with Twilio to: {}", notification.to);

        let response = self
            .client
            .post(&self.url)
            .basic_auth(&self.config.account_sid, Some(&self.config.auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            info!("SMS sent successfully via Twilio");
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            info!("Twilio API error: {} - {}", status, text);
            Err(ProviderError::ApiError(format!(
                "Twilio API error: {} - {}",
                status, text
            )))
        }
    }
}

impl Provider for TwilioSmsProvider {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError> {
        match notification {
            Notification::SMS(ref msg) => self.send_sms(msg).await,
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
    fn test_twilio_sms_provider_creation() {
        let config = TwilioSmsConfig {
            account_sid: "test_account_sid".to_string(),
            auth_token: "test_auth_token".to_string(),
        };

        let provider = TwilioSmsProvider::new(config.clone());
        assert_eq!(
            provider.url,
            format!(
                "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                config.account_sid
            )
        );
        assert_eq!(provider.config.account_sid, config.account_sid);
        assert_eq!(provider.config.auth_token, config.auth_token);
    }
}
