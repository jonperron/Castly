use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::models::{EmailNotification, NotificationType, SendNotificationRequest};
use crate::providers::{errors::ProviderError, EmailProvider, MailgunProvider, MailjetProvider};
use crate::templates_engines::TemplateEngine;

#[derive(Clone)]
pub struct AppState {
    pub template_engine: Arc<TemplateEngine>,
    pub mailgun_provider: Option<Arc<MailgunProvider>>,
    pub mailjet_provider: Option<Arc<MailjetProvider>>,
}

pub fn create_email_notification(
    request: &SendNotificationRequest,
    template_engine: &TemplateEngine,
) -> Result<EmailNotification, ProviderError> {
    let body = if let Some(template_request) = &request.use_template {
        // Load the template using the template name
        let template = template_engine
            .load(&template_request.template_name)
            .map_err(|e| {
                tracing::error!("Failed to load template: {:?}", e);
                ProviderError::template_error("Template not found")
            })?;

        // Render the template with the provided context
        template
            .render(template_request.context.clone())
            .map_err(|_| ProviderError::template_error("Failed to render template"))?
    } else if let Some(raw_text_request) = &request.use_raw_text {
        // Use the raw text directly
        raw_text_request.text.clone()
    } else {
        unreachable!(
            "Either use_template or use_raw_text should always be present due to validation"
        );
    };

    // Create and return the Notification object
    Ok(EmailNotification {
        from: request.from.clone(),
        to: request.to.clone(),
        subject: request.subject.clone(),
        body,
        is_raw_text: request.use_raw_text.is_some(),
    })
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
    let notification = match request.notification_type {
        NotificationType::MailMailgun => {
            create_email_notification(&request, &state.template_engine)
        }
        NotificationType::MailMailjet => {
            create_email_notification(&request, &state.template_engine)
        }
        _ => Err(ProviderError::invalid_config(
            "Unsupported notification type",
        )),
    };

    // Find provider based on notification type and provider provided
    let provider_result = match request.notification_type {
        NotificationType::MailMailgun => match notification {
            Ok(n) => {
                if let Some(provider) = state.mailgun_provider.as_ref() {
                    provider.send(n).await
                } else {
                    Err(ProviderError::invalid_config(
                        "Mailgun provider not configured",
                    ))
                }
            }
            Err(e) => Err(e),
        },
        NotificationType::MailMailjet => match notification {
            Ok(n) => {
                if let Some(provider) = state.mailjet_provider.as_ref() {
                    provider.send(n).await
                } else {
                    Err(ProviderError::invalid_config(
                        "Mailjet provider not configured",
                    ))
                }
            }
            Err(e) => Err(e),
        },
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
