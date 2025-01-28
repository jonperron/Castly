use crate::models::Notification;
use crate::providers::errors::ProviderError;

#[allow(async_fn_in_trait)]
pub trait Provider {
    async fn send(&self, notification: Notification) -> Result<(), ProviderError>;
}
