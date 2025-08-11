use serde_json::Value;

use crate::models::send_notification_request::{
    NotificationType, SendNotificationRequest, UseRawText, UseTemplate,
};
use crate::providers::errors::ProviderError;

pub struct NotificationRequestBuilder {
    to: Vec<String>,
    from: Option<String>,
    subject: Option<String>,
    raw_text: Option<String>,
    template_name: Option<String>,
    template_context: Option<Value>,
    notification_type: Option<NotificationType>,
}

impl NotificationRequestBuilder {
    pub fn new() -> Self {
        Self {
            to: Vec::new(),
            from: None,
            subject: None,
            raw_text: None,
            template_name: None,
            template_context: None,
            notification_type: None,
        }
    }

    pub fn to(mut self, recipients: Vec<String>) -> Self {
        self.to = recipients;
        self
    }

    pub fn add_recipient(mut self, recipient: String) -> Self {
        self.to.push(recipient);
        self
    }

    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_raw_text(mut self, text: String) -> Self {
        self.raw_text = Some(text);
        // Clear template fields if set
        self.template_name = None;
        self.template_context = None;
        self
    }

    pub fn with_template(mut self, template_name: String, context: Value) -> Self {
        self.template_name = Some(template_name);
        self.template_context = Some(context);
        // Clear raw text if set
        self.raw_text = None;
        self
    }

    pub fn notification_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = Some(notification_type);
        self
    }

    pub fn build(self) -> Result<SendNotificationRequest, ProviderError> {
        let from = self
            .from
            .ok_or_else(|| ProviderError::invalid_config("From field is required"))?;

        let subject = self
            .subject
            .ok_or_else(|| ProviderError::invalid_config("Subject field is required"))?;

        let notification_type = self
            .notification_type
            .ok_or_else(|| ProviderError::invalid_config("Notification type is required"))?;

        if self.to.is_empty() {
            return Err(ProviderError::invalid_config(
                "At least one recipient is required",
            ));
        }

        let (use_raw_text, use_template) =
            match (self.raw_text, self.template_name, self.template_context) {
                (Some(text), None, None) => (Some(UseRawText { text }), None),
                (None, Some(template_name), Some(context)) => (
                    None,
                    Some(UseTemplate {
                        template_name,
                        context,
                    }),
                ),
                (None, None, None) => {
                    return Err(ProviderError::invalid_config(
                        "Either raw text or template must be provided",
                    ));
                }
                _ => {
                    return Err(ProviderError::invalid_config(
                        "Cannot provide both raw text and template",
                    ));
                }
            };

        let request = SendNotificationRequest {
            to: self.to,
            from,
            subject,
            use_raw_text,
            use_template,
            notification_type,
        };

        // Validate the request
        request.validate()?;

        Ok(request)
    }
}

impl Default for NotificationRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_builder_with_raw_text() {
        let request = NotificationRequestBuilder::new()
            .to(vec!["test@example.com".to_string()])
            .from("sender@example.com".to_string())
            .subject("Test Subject".to_string())
            .with_raw_text("Hello, World!".to_string())
            .notification_type(NotificationType::MailMailgun)
            .build()
            .unwrap();

        assert_eq!(request.to, vec!["test@example.com"]);
        assert_eq!(request.from, "sender@example.com");
        assert_eq!(request.subject, "Test Subject");
        assert!(request.use_raw_text.is_some());
        assert!(request.use_template.is_none());
    }

    #[test]
    fn test_builder_with_template() {
        let context = json!({"name": "John"});
        let request = NotificationRequestBuilder::new()
            .to(vec!["test@example.com".to_string()])
            .from("sender@example.com".to_string())
            .subject("Test Subject".to_string())
            .with_template("hello.html".to_string(), context.clone())
            .notification_type(NotificationType::MailMailgun)
            .build()
            .unwrap();

        assert!(request.use_template.is_some());
        assert!(request.use_raw_text.is_none());

        let template = request.use_template.unwrap();
        assert_eq!(template.template_name, "hello.html");
        assert_eq!(template.context, context);
    }

    #[test]
    fn test_builder_missing_required_fields() {
        let result = NotificationRequestBuilder::new()
            .to(vec!["test@example.com".to_string()])
            .with_raw_text("Hello".to_string())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_add_recipient() {
        let request = NotificationRequestBuilder::new()
            .add_recipient("test1@example.com".to_string())
            .add_recipient("test2@example.com".to_string())
            .from("sender@example.com".to_string())
            .subject("Test Subject".to_string())
            .with_raw_text("Hello".to_string())
            .notification_type(NotificationType::MailMailgun)
            .build()
            .unwrap();

        assert_eq!(request.to.len(), 2);
        assert!(request.to.contains(&"test1@example.com".to_string()));
        assert!(request.to.contains(&"test2@example.com".to_string()));
    }
}
