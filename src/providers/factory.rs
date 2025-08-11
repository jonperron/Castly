use crate::config::ProvidersConfig;
use crate::providers::{
    MailgunProvider, MailjetProvider, TelegramProvider, TwilioEmailProvider, TwilioSmsProvider,
};

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create_mailgun_provider(config: &ProvidersConfig) -> Option<MailgunProvider> {
        config
            .mailgun
            .as_ref()
            .map(|cfg| MailgunProvider::new(cfg.clone()))
    }

    pub fn create_mailjet_provider(config: &ProvidersConfig) -> Option<MailjetProvider> {
        config
            .mailjet
            .as_ref()
            .map(|cfg| MailjetProvider::new(cfg.clone()))
    }

    pub fn create_telegram_provider(config: &ProvidersConfig) -> Option<TelegramProvider> {
        config
            .telegram
            .as_ref()
            .map(|cfg| TelegramProvider::new(cfg.clone()))
    }

    pub fn create_twilio_sms_provider(config: &ProvidersConfig) -> Option<TwilioSmsProvider> {
        config
            .twilio_sms
            .as_ref()
            .map(|cfg| TwilioSmsProvider::new(cfg.clone()))
    }

    pub fn create_twilio_email_provider(config: &ProvidersConfig) -> Option<TwilioEmailProvider> {
        config
            .twilio_email
            .as_ref()
            .map(|cfg| TwilioEmailProvider::new(cfg.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{MailgunConfig, ProvidersConfig, TelegramConfig};

    #[test]
    fn test_create_mailgun_provider() {
        let config = ProvidersConfig {
            mailgun: Some(MailgunConfig {
                domain: "test.com".to_string(),
                api_key: "test-key".to_string(),
                base_url: None,
            }),
            mailjet: None,
            telegram: None,
            twilio_sms: None,
            twilio_email: None,
        };

        let provider = ProviderFactory::create_mailgun_provider(&config);
        assert!(provider.is_some());
    }

    #[test]
    fn test_create_telegram_provider() {
        let config = ProvidersConfig {
            mailgun: None,
            mailjet: None,
            telegram: Some(TelegramConfig {
                bot_token: "test-token".to_string(),
            }),
            twilio_sms: None,
            twilio_email: None,
        };

        let provider = ProviderFactory::create_telegram_provider(&config);
        assert!(provider.is_some());
    }
}
