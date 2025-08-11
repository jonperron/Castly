// Declare modules part
mod api;
mod config;
mod models;
mod providers;
mod templates_engines;
mod tools;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber;

use crate::api::{send_router, AppState};
use crate::config::Config;
use crate::providers::{
    MailgunProvider, MailjetProvider, TelegramProvider, TwilioEmailProvider, TwilioSmsProvider,
};
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

    // Init providers
    if config.providers.is_empty() {
        panic!("No provider is configured, please check your config file");
    }

    let mailgun_provider = if let Some(mailgun_config) = &config.providers.mailgun {
        Some(Arc::new(MailgunProvider::new(mailgun_config.clone())))
    } else {
        None
    };

    let mailjet_provider = if let Some(mailjet_config) = &config.providers.mailjet {
        Some(Arc::new(MailjetProvider::new(mailjet_config.clone())))
    } else {
        None
    };

    let telegram_provider = if let Some(telegram_config) = &config.providers.telegram {
        Some(Arc::new(TelegramProvider::new(telegram_config.clone())))
    } else {
        None
    };

    let twilio_sms_provider = if let Some(twilio_sms_config) = &config.providers.twilio_sms {
        Some(Arc::new(TwilioSmsProvider::new(twilio_sms_config.clone())))
    } else {
        None
    };

    let twilio_email_provider = if let Some(twilio_email_config) = &config.providers.twilio_email {
        Some(Arc::new(TwilioEmailProvider::new(
            twilio_email_config.clone(),
        )))
    } else {
        None
    };

    let state = AppState {
        template_engine,
        mailgun_provider,
        mailjet_provider,
        telegram_provider,
        twilio_sms_provider,
        twilio_email_provider,
    };

    let app = send_router(state).merge(Router::new().route("/health", get(health_handler)));

    let addr: SocketAddr = format!("0.0.0.0:{}", config.service.port).parse().unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
