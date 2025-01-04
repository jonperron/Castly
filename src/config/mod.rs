use serde::Deserialize;
use serde_yaml;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct TemplatesConfig {
    pub path: String,
    pub default_language: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub service: ServiceConfig,
    pub templates: TemplatesConfig,
    pub providers: ProvidersConfig,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct ProvidersConfig {
    pub mailgun: Option<MailgunConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MailgunConfig {
    pub domain: String,
    pub api_key: String,
    pub base_url: Option<String>,
}

impl ProvidersConfig {
    pub fn is_empty(&self) -> bool {
        self.mailgun.is_none()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_file() {
        let config_content = r#"
        service:
          name: "test_service"
          port: 8080
        templates:
          path: "/templates"
          default_language: "en"
        providers:
          mailgun:
            domain: "example.com"
            api_key: "key-123"
            base_url: "https://api.mailgun.net"
        "#;

        let config_path = "/tmp/test_config.yaml";
        fs::write(config_path, config_content).unwrap();

        let config = Config::load_from_file(config_path).unwrap();
        assert_eq!(config.service.name, "test_service");
        assert_eq!(config.service.port, 8080);
        assert_eq!(config.templates.path, "/templates");
        assert_eq!(config.templates.default_language, "en");
        assert!(config.providers.mailgun.is_some());

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_providers_config_is_empty() {
        let providers_config = ProvidersConfig { mailgun: None };
        assert!(providers_config.is_empty());

        let mailgun_config = MailgunConfig {
            domain: "example.com".to_string(),
            api_key: "key-123".to_string(),
            base_url: Some("https://api.mailgun.net".to_string()),
        };
        let providers_config = ProvidersConfig {
            mailgun: Some(mailgun_config),
        };
        assert!(!providers_config.is_empty());
    }
}
