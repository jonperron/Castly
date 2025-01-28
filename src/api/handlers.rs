use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::{
    api::helpers::{create_email_notifications, create_messaging_notifications},
    models::{Notification, NotificationType, SendNotificationRequest},
    providers::{
        errors::ProviderError, MailgunProvider, MailjetProvider, Provider, TelegramProvider,
    },
    templates_engines::TemplateEngine,
};

#[derive(Clone)]
pub struct AppState {
    pub template_engine: Arc<TemplateEngine>,
    pub mailgun_provider: Option<Arc<MailgunProvider>>,
    pub mailjet_provider: Option<Arc<MailjetProvider>>,
    pub telegram_provider: Option<Arc<TelegramProvider>>,
}

pub async fn send_notifications_with_provider<T: Provider>(
    provider: Arc<T>,
    provider_name: &str,
    notifications: Vec<Notification>,
) -> Result<(), ProviderError> {
    for notification in notifications {
        if let Err(e) = provider.send(notification).await {
            return Err(ProviderError::provider_error(
                format!(
                    "Failed to send notification with {}: {:?}",
                    provider_name, e
                )
                .as_str(),
            ));
        }
    }
    Ok(())
}

pub async fn send_notification(
    State(state): State<AppState>,
    Json(request): Json<SendNotificationRequest>,
) -> impl IntoResponse {
    // Validate the request
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        );
    }

    // Create notification regarding type
    let notifications = match request.notification_type {
        NotificationType::MailMailgun | NotificationType::MailMailjet => {
            create_email_notifications(&request, &state.template_engine)
        }
        NotificationType::Telegram => {
            create_messaging_notifications(&request, &state.template_engine)
        }
        _ => Err(ProviderError::invalid_config(
            "Unsupported notification type",
        )),
    };

    // Generic helper function
    async fn handle_provider<T: Provider>(
        provider: Option<Arc<T>>,
        provider_name: &str,
        notifications: Result<Vec<Notification>, ProviderError>,
    ) -> Result<(), ProviderError> {
        if let Some(provider) = provider {
            match notifications {
                Ok(n) => send_notifications_with_provider(provider.clone(), provider_name, n).await,
                Err(e) => Err(e),
            }
        } else {
            Err(ProviderError::invalid_config(&format!(
                "{} provider not configured",
                provider_name
            )))
        }
    }

    // Find provider based on notification type and provider provided
    let provider_result = match request.notification_type {
        NotificationType::MailMailgun => {
            handle_provider(state.mailgun_provider.clone(), "Mailgun", notifications).await
        }
        NotificationType::MailMailjet => {
            handle_provider(state.mailjet_provider.clone(), "Mailjet", notifications).await
        }
        NotificationType::Telegram => {
            handle_provider(state.telegram_provider.clone(), "Telegram", notifications).await
        }
        _ => Err(ProviderError::invalid_config("Unsupported provider")),
    };

    // Select provider
    match provider_result {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "Notification sent"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub fn send_router(state: AppState) -> axum::Router {
    Router::new()
        .route("/send", post(send_notification))
        .with_state(state)
}
