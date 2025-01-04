use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::errors::ProviderError;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    MailMailgun, // Mailgun for email notifications
    #[serde(other)] // Handle unknown notification types
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SendNotificationRequest {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub template_name: String,
    pub context: Value,
    pub notification_type: NotificationType,
}

impl SendNotificationRequest {
    pub fn validate(&self) -> Result<(), ProviderError> {
        match self.notification_type {
            NotificationType::MailMailgun => Ok(()), // Accept mailgun for email notifications
            _ => Err(ProviderError::invalid_config(
                "Notification type is not supported".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_send_notification_request_serialization() {
        let send_notification_request = SendNotificationRequest {
            to: "receiver@example.com".to_string(),
            from: "sender@example.com".to_string(),
            subject: "Test subject".to_string(),
            template_name: "hello_world.html".to_string(),
            context: serde_json::json!({"foo": "bar"}),
            notification_type: NotificationType::MailMailgun,
        };
        let json = serde_json::to_string(&send_notification_request).expect("Serialization failed");
        assert!(json.contains("sender@example.com"));
        assert!(json.contains("receiver@example.com"));

        let deserialized_send_notification_request: SendNotificationRequest =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(
            deserialized_send_notification_request,
            send_notification_request
        );
    }

    #[test]
    fn test_send_notification_request_validation() {
        // Valid notification type
        let valid_send_notification_request = SendNotificationRequest {
            to: "receiver@example.com".to_string(),
            from: "sender@example.com".to_string(),
            subject: "Test subject".to_string(),
            template_name: "hello_world.html".to_string(),
            context: serde_json::json!({"foo": "bar"}),
            notification_type: NotificationType::MailMailgun,
        };

        assert!(valid_send_notification_request.validate().is_ok());

        // Invalid notification type
        let invalid_send_notification_request_json = r#"
        {
            "to": "receiver@example.com",
            "from": "sender@example.com",
            "subject": "Test subject",
            "template_name": "hello_world.html",
            "context": {"foo": "bar"},
            "notification_type": "buzz"
        }"#;

        let invalid_send_notification_request: SendNotificationRequest =
            serde_json::from_str(invalid_send_notification_request_json)
                .expect("Deserialization failed");

        assert!(invalid_send_notification_request.validate().is_err());
    }
}
