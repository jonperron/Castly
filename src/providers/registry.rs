use std::collections::HashMap;
use std::sync::Arc;

use crate::models::{Notification, NotificationType};
use crate::providers::{errors::ProviderError, Provider};

pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider + Send + Sync>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register_provider(&mut self, name: String, provider: Arc<dyn Provider + Send + Sync>) {
        self.providers.insert(name, provider);
    }

    pub fn get_provider(&self, name: &str) -> Option<&Arc<dyn Provider + Send + Sync>> {
        self.providers.get(name)
    }

    pub fn get_provider_for_notification_type(
        &self,
        notification_type: &NotificationType,
    ) -> Result<&Arc<dyn Provider + Send + Sync>, ProviderError> {
        let provider_name = match notification_type {
            NotificationType::MailMailgun => "mail_mailgun",
            NotificationType::MailMailjet => "mail_mailjet",
            NotificationType::MailTwilio => "mail_twilio",
            NotificationType::Telegram => "telegram",
            NotificationType::SmsTwilio => "sms_twilio",
            NotificationType::Unknown => {
                return Err(ProviderError::invalid_config("Unknown notification type"))
            }
        };

        self.get_provider(provider_name).ok_or_else(|| {
            ProviderError::invalid_config(&format!("{} provider not configured", provider_name))
        })
    }

    pub async fn send_notification(
        &self,
        notification_type: &NotificationType,
        notification: Notification,
    ) -> Result<(), ProviderError> {
        let provider = self.get_provider_for_notification_type(notification_type)?;

        if !provider.supports_notification(&notification) {
            return Err(ProviderError::invalid_config(
                "Provider does not support this notification type",
            ));
        }

        provider.send(notification).await
    }

    pub async fn health_check_all(&self) -> HashMap<String, Result<(), ProviderError>> {
        let mut results = HashMap::new();

        for (name, provider) in &self.providers {
            let result = provider.health_check().await;
            results.insert(name.clone(), result);
        }

        results
    }

    pub fn list_providers(&self) -> Vec<&String> {
        self.providers.keys().collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MailgunConfig;
    use crate::providers::MailgunProvider;

    #[test]
    fn test_provider_registry() {
        let mut registry = ProviderRegistry::new();

        let mailgun_config = MailgunConfig {
            domain: "test.com".to_string(),
            api_key: "test-key".to_string(),
            base_url: None,
        };
        let mailgun_provider = Arc::new(MailgunProvider::new(mailgun_config));

        registry.register_provider("mail_mailgun".to_string(), mailgun_provider);

        assert!(registry.get_provider("mail_mailgun").is_some());
        assert!(registry.get_provider("nonexistent").is_none());

        let providers = registry.list_providers();
        assert_eq!(providers.len(), 1);
        assert!(providers.contains(&&"mail_mailgun".to_string()));
    }

    #[test]
    fn test_get_provider_for_notification_type() {
        let mut registry = ProviderRegistry::new();

        let mailgun_config = MailgunConfig {
            domain: "test.com".to_string(),
            api_key: "test-key".to_string(),
            base_url: None,
        };
        let mailgun_provider = Arc::new(MailgunProvider::new(mailgun_config));
        registry.register_provider("mail_mailgun".to_string(), mailgun_provider);

        let result = registry.get_provider_for_notification_type(&NotificationType::MailMailgun);
        assert!(result.is_ok());

        let result = registry.get_provider_for_notification_type(&NotificationType::Telegram);
        assert!(result.is_err());
    }
}
