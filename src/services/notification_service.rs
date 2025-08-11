use std::sync::Arc;

use crate::api::helpers::{
    create_email_notifications, create_messaging_notifications, create_sms_notifications,
};
use crate::models::{NotificationType, SendNotificationRequest};
use crate::providers::{errors::ProviderError, ProviderRegistry};
use crate::templates_engines::TemplateEngine;

pub struct NotificationService {
    provider_registry: Arc<ProviderRegistry>,
    template_engine: Arc<TemplateEngine>,
}

impl NotificationService {
    pub fn new(
        provider_registry: Arc<ProviderRegistry>,
        template_engine: Arc<TemplateEngine>,
    ) -> Self {
        Self {
            provider_registry,
            template_engine,
        }
    }

    pub async fn send_notification(
        &self,
        request: SendNotificationRequest,
    ) -> Result<(), ProviderError> {
        // Validate the request
        request.validate()?;

        // Create notifications based on type
        let notifications = match request.notification_type {
            NotificationType::MailMailgun
            | NotificationType::MailMailjet
            | NotificationType::MailTwilio => {
                create_email_notifications(&request, &self.template_engine)
            }
            NotificationType::Telegram => {
                create_messaging_notifications(&request, &self.template_engine)
            }
            NotificationType::SmsTwilio => {
                create_sms_notifications(&request, &self.template_engine)
            }
            _ => Err(ProviderError::invalid_config(
                "Unsupported notification type",
            )),
        }?;

        // Send each notification
        for notification in notifications {
            self.provider_registry
                .send_notification(&request.notification_type, notification)
                .await?;
        }

        Ok(())
    }

    pub async fn health_check(
        &self,
    ) -> std::collections::HashMap<String, Result<(), ProviderError>> {
        self.provider_registry.health_check_all().await
    }

    pub fn list_available_providers(&self) -> Vec<&String> {
        self.provider_registry.list_providers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MailgunConfig;
    use crate::providers::MailgunProvider;

    #[tokio::test]
    async fn test_notification_service() {
        // Create provider registry
        let mut registry = ProviderRegistry::new();
        let mailgun_config = MailgunConfig {
            domain: "test.com".to_string(),
            api_key: "test-key".to_string(),
            base_url: None,
        };
        let mailgun_provider = Arc::new(MailgunProvider::new(mailgun_config));
        registry.register_provider("mail_mailgun".to_string(), mailgun_provider);

        // For testing purposes, create a minimal template engine
        // In a real scenario, you would properly initialize with template files
        let template_engine = Arc::new(
            TemplateEngine::new("templates/**/*.html").unwrap_or_else(|_| {
                // Fallback for test environment
                panic!("Template engine initialization failed - this is expected in test environment without proper template setup")
            })
        );

        let service = NotificationService::new(Arc::new(registry), template_engine);

        let providers = service.list_available_providers();
        assert_eq!(providers.len(), 1);
        assert!(providers.contains(&&"mail_mailgun".to_string()));
    }
}
