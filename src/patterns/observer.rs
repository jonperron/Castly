use async_trait::async_trait;
use std::sync::Arc;

use crate::models::{Notification, NotificationType};
use crate::providers::errors::ProviderError;

#[derive(Debug, Clone)]
pub enum NotificationEvent {
    Sent {
        notification_type: NotificationType,
        recipient: String,
        success: bool,
        error: Option<String>,
    },
    Failed {
        notification_type: NotificationType,
        recipient: String,
        error: String,
    },
    HealthCheck {
        provider_name: String,
        healthy: bool,
        error: Option<String>,
    },
}

#[async_trait]
pub trait NotificationObserver: Send + Sync {
    async fn on_event(&self, event: NotificationEvent);
}

pub struct NotificationSubject {
    observers: Vec<Arc<dyn NotificationObserver>>,
}

impl NotificationSubject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Arc<dyn NotificationObserver>) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, observer_to_remove: &Arc<dyn NotificationObserver>) {
        self.observers
            .retain(|observer| !Arc::ptr_eq(observer, observer_to_remove));
    }

    pub async fn notify(&self, event: NotificationEvent) {
        for observer in &self.observers {
            observer.on_event(event.clone()).await;
        }
    }
}

impl Default for NotificationSubject {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LoggingObserver;

#[async_trait]
impl NotificationObserver for LoggingObserver {
    async fn on_event(&self, event: NotificationEvent) {
        match event {
            NotificationEvent::Sent {
                notification_type,
                recipient,
                success,
                error,
            } => {
                if success {
                    tracing::info!(
                        "Notification sent successfully: {:?} to {}",
                        notification_type,
                        recipient
                    );
                } else {
                    tracing::error!(
                        "Failed to send notification: {:?} to {} - {:?}",
                        notification_type,
                        recipient,
                        error
                    );
                }
            }
            NotificationEvent::Failed {
                notification_type,
                recipient,
                error,
            } => {
                tracing::error!(
                    "Notification failed: {:?} to {} - {}",
                    notification_type,
                    recipient,
                    error
                );
            }
            NotificationEvent::HealthCheck {
                provider_name,
                healthy,
                error,
            } => {
                if healthy {
                    tracing::info!("Provider {} is healthy", provider_name);
                } else {
                    tracing::warn!(
                        "Provider {} health check failed: {:?}",
                        provider_name,
                        error
                    );
                }
            }
        }
    }
}

pub struct MetricsObserver {
    // This would typically connect to a metrics system like Prometheus
    pub sent_count: std::sync::atomic::AtomicU64,
    pub failed_count: std::sync::atomic::AtomicU64,
}

impl MetricsObserver {
    pub fn new() -> Self {
        Self {
            sent_count: std::sync::atomic::AtomicU64::new(0),
            failed_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn get_sent_count(&self) -> u64 {
        self.sent_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_failed_count(&self) -> u64 {
        self.failed_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for MetricsObserver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NotificationObserver for MetricsObserver {
    async fn on_event(&self, event: NotificationEvent) {
        match event {
            NotificationEvent::Sent { success, .. } => {
                if success {
                    self.sent_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    self.failed_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            NotificationEvent::Failed { .. } => {
                self.failed_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            NotificationEvent::HealthCheck { .. } => {
                // Health check events don't affect notification metrics
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notification_subject() {
        let mut subject = NotificationSubject::new();
        let logging_observer = Arc::new(LoggingObserver);
        let metrics_observer = Arc::new(MetricsObserver::new());

        subject.add_observer(logging_observer.clone());
        subject.add_observer(metrics_observer.clone());

        let event = NotificationEvent::Sent {
            notification_type: NotificationType::MailMailgun,
            recipient: "test@example.com".to_string(),
            success: true,
            error: None,
        };

        subject.notify(event).await;

        assert_eq!(metrics_observer.get_sent_count(), 1);
        assert_eq!(metrics_observer.get_failed_count(), 0);
    }

    #[tokio::test]
    async fn test_observer_removal() {
        let mut subject = NotificationSubject::new();
        let observer: Arc<dyn NotificationObserver> = Arc::new(LoggingObserver);

        subject.add_observer(observer.clone());
        assert_eq!(subject.observers.len(), 1);

        subject.remove_observer(&observer);
        assert_eq!(subject.observers.len(), 0);
    }
}
