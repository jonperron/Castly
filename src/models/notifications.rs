use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EmailNotification {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub is_raw_text: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageNotification {
    pub recipient: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SMSNotification {
    pub from: String,
    pub to: String,
    pub body: String,
}

pub enum Notification {
    Email(EmailNotification),
    Message(MessageNotification),
    SMS(SMSNotification),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_email_notification_serialization() {
        let email_notification = EmailNotification {
            from: "sender@example.com".to_string(),
            to: "receiver@example.com".to_string(),
            subject: "Test email".to_string(),
            body: "This is a test email".to_string(),
            is_raw_text: true,
        };

        let json = serde_json::to_string(&email_notification).expect("Serialization failed");
        assert!(json.contains("sender@example.com"));
        assert!(json.contains("receiver@example.com"));

        let deserialized_email: EmailNotification =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(deserialized_email, email_notification);
    }

    #[test]
    fn test_sms_notification_serialization() {
        let sms_notification = SMSNotification {
            from: "+1234567890".to_string(),
            to: "+0987654321".to_string(),
            body: "This is a test SMS".to_string(),
        };

        let json = serde_json::to_string(&sms_notification).expect("Serialization failed");
        assert!(json.contains("+1234567890"));
        assert!(json.contains("+0987654321"));

        let deserialized_sms: SMSNotification =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(deserialized_sms, sms_notification);
    }
}
