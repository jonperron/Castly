use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::SendNotificationRequest, services::notification_service::NotificationService};

#[derive(Clone)]
pub struct AppState {
    pub notification_service: Arc<NotificationService>,
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

    // Use the notification service to send the notification
    match state.notification_service.send_notification(request).await {
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
