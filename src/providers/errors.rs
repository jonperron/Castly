use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error["Api error: {0}"]]
    ApiError(String),

    #[error["Provider error: {0}"]]
    ProviderError(String),
}

impl ProviderError {
    pub fn network_error(details: impl Into<String>) -> Self {
        ProviderError::NetworkError(details.into())
    }

    pub fn invalid_config(details: impl Into<String>) -> Self {
        ProviderError::InvalidConfig(details.into())
    }

    pub fn template_error(details: impl Into<String>) -> Self {
        ProviderError::TemplateError(details.into())
    }

    pub fn unexpected_error(details: impl Into<String>) -> Self {
        ProviderError::UnexpectedError(details.into())
    }

    pub fn api_error(details: impl Into<String>) -> Self {
        ProviderError::ApiError(details.into())
    }

    pub fn provider_error(details: impl Into<String>) -> Self {
        ProviderError::ProviderError(details.into())
    }
}
