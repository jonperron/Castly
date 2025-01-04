use crate::models::EmailNotification;
use crate::providers::errors::ProviderError;

#[allow(async_fn_in_trait)]
pub trait EmailProvider {
    async fn send(&self, notification: EmailNotification) -> Result<(), ProviderError>;
}
