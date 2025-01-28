use crate::{
    models::{EmailNotification, MessageNotification, Notification, SendNotificationRequest},
    providers::errors::ProviderError,
    templates_engines::TemplateEngine,
};

// Create email notification from SendNotificationRequest
pub fn create_email_notifications(
    request: &SendNotificationRequest,
    template_engine: &TemplateEngine,
) -> Result<Vec<Notification>, ProviderError> {
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

    // Create and return the Notification objects
    let notifications = request
        .to
        .iter()
        .map(|to| {
            Notification::Email(EmailNotification {
                from: request.from.clone(),
                to: to.clone(),
                subject: request.subject.clone(),
                body: body.clone(),
                is_raw_text: request.use_raw_text.is_some(),
            })
        })
        .collect();

    Ok(notifications)
}

// Create messaging notification from SendNotificationRequest
pub fn create_messaging_notifications(
    request: &SendNotificationRequest,
    template_engine: &TemplateEngine,
) -> Result<Vec<Notification>, ProviderError> {
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

    let notifications = request
        .to
        .iter()
        .map(|to| {
            Notification::Message(MessageNotification {
                recipient: to.clone(),
                message: body.clone(),
            })
        })
        .collect();

    Ok(notifications)
}
