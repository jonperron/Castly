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
use crate::providers::MailgunProvider;
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

    if config.providers.is_empty() {
        panic!("No provider is configured, please check your config file");
    }

    let mailgun_provider = Arc::new(MailgunProvider::new(
        config
            .providers
            .mailgun
            .clone()
            .expect("Mailgun config is missing"),
    ));

    let state = AppState {
        template_engine,
        mailgun_provider,
    };

    let app = send_router(state).merge(Router::new().route("/health", get(health_handler)));

    let addr: SocketAddr = format!("0.0.0.0:{}", config.service.port).parse().unwrap();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
