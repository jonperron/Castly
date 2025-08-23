use crate::{models::SendNotificationRequest, services::notification_service::NotificationService};
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use std::sync::Arc;
use tokio_stream::StreamExt;

pub async fn run_kafka_consumer(
    brokers: &str,
    topic: &str,
    group_id: &str,
    notification_service: Arc<NotificationService>,
) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer
        .subscribe(&[topic])
        .expect("Can't subscribe to specified topic");

    let mut stream = consumer.stream();

    while let Some(Ok(msg)) = stream.next().await {
        if let Some(payload) = msg.payload() {
            match serde_json::from_slice::<SendNotificationRequest>(payload) {
                Ok(request) => {
                    if let Err(e) = request.validate() {
                        tracing::error!("Invalid notification request: {}", e);
                        continue;
                    }
                    if let Err(e) = notification_service.send_notification(request).await {
                        tracing::error!("Failed to send notification: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to deserialize message: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SendNotificationRequest;
    use crate::providers::ProviderRegistry;
    use crate::services::notification_service::NotificationService;
    use crate::templates_engines::tera_engine::TemplateEngine;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_kafka_consumer_handles_valid_message() {
        let provider_registry = Arc::new(ProviderRegistry::new());
        let template_engine = Arc::new(TemplateEngine::new("templates/**/*").unwrap());
        let notification_service =
            Arc::new(NotificationService::new(provider_registry, template_engine));
        let request = SendNotificationRequest {
            to: vec!["test@example.com".to_string()],
            from: "sender@example.com".to_string(),
            subject: "Test".to_string(),
            use_raw_text: Some(crate::models::UseRawText {
                text: "Hello".to_string(),
            }),
            use_template: None,
            notification_type: crate::models::NotificationType::MailMailgun,
        };
        let payload = serde_json::to_vec(&request).unwrap();

        // Simulate Kafka message handling
        let result = serde_json::from_slice::<SendNotificationRequest>(&payload);
        assert!(result.is_ok());
        let req = result.unwrap();
        assert!(req.validate().is_ok());
    }
}
