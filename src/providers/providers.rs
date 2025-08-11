use crate::models::Notification;
use crate::providers::errors::ProviderError;
use async_trait::async_trait;

#[async_trait]
pub trait Provider: Send + Sync {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError>;

    /// Health check for the provider
    async fn health_check(&self) -> Result<(), ProviderError> {
        // Default implementation - providers can override
        Ok(())
    }

    /// Get provider name for logging and debugging
    fn name(&self) -> &'static str;

    /// Check if provider supports the given notification type
    fn supports_notification(&self, notification: &Notification) -> bool;
}
