// Declare modules part
mod api;
mod config;
mod models;
mod patterns;
mod providers;
mod services;
mod templates_engines;
mod tools;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber;

use crate::api::{send_router, AppState};
use crate::config::Config;
use crate::providers::{ProviderFactory, ProviderRegistry};
use crate::services::NotificationService;
use crate::templates_engines::tera_engine::TemplateEngine;
use crate::tools::health_handler;

#[tokio::main]
async fn main() {
    // Set up tracing subscriber to log at info level
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Service starting...");

    // Load the configuration
    let config = Arc::new(Config::load_from_file("config.yaml").expect("Failed to load config"));

    // Init dependencies
    let template_engine = Arc::new(
        TemplateEngine::new(format!("{}/**/*.html", &config.templates.path).as_str())
            .expect("Failed to load template engine"),
    );

    // Init providers using factory pattern
    if config.providers.is_empty() {
        panic!("No provider is configured, please check your config file");
    }

    let mut provider_registry = ProviderRegistry::new();

    // Register providers using factory
    if let Some(provider) = ProviderFactory::create_mailgun_provider(&config.providers) {
        provider_registry.register_provider("mailgun".to_string(), Arc::new(provider));
    }
    if let Some(provider) = ProviderFactory::create_mailjet_provider(&config.providers) {
        provider_registry.register_provider("mailjet".to_string(), Arc::new(provider));
    }
    if let Some(provider) = ProviderFactory::create_telegram_provider(&config.providers) {
        provider_registry.register_provider("telegram".to_string(), Arc::new(provider));
    }
    if let Some(provider) = ProviderFactory::create_twilio_sms_provider(&config.providers) {
        provider_registry.register_provider("twilio_sms".to_string(), Arc::new(provider));
    }
    if let Some(provider) = ProviderFactory::create_twilio_email_provider(&config.providers) {
        provider_registry.register_provider("twilio_email".to_string(), Arc::new(provider));
    }

    // Create notification service with dependency injection
    let notification_service = Arc::new(NotificationService::new(
        Arc::new(provider_registry),
        template_engine.clone(),
    ));

    // Legacy AppState for backward compatibility with existing handlers
    let state = AppState {
        template_engine,
        mailgun_provider: ProviderFactory::create_mailgun_provider(&config.providers).map(Arc::new),
        mailjet_provider: ProviderFactory::create_mailjet_provider(&config.providers).map(Arc::new),
        telegram_provider: ProviderFactory::create_telegram_provider(&config.providers)
            .map(Arc::new),
        twilio_sms_provider: ProviderFactory::create_twilio_sms_provider(&config.providers)
            .map(Arc::new),
        twilio_email_provider: ProviderFactory::create_twilio_email_provider(&config.providers)
            .map(Arc::new),
    };

    let app = send_router(state).merge(Router::new().route("/health", get(health_handler)));

    let addr: SocketAddr = format!("0.0.0.0:{}", config.service.port).parse().unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
