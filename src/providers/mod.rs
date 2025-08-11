// Declaration
pub mod errors;
pub mod factory;
pub mod mailgun;
pub mod mailjet;
pub mod providers;
pub mod registry;
pub mod telegram;
pub mod twilio_email;
pub mod twilio_sms;

// Limit import to only what is useful
pub use factory::ProviderFactory;
pub use mailgun::MailgunProvider;
pub use mailjet::MailjetProvider;
pub use providers::Provider;
pub use registry::ProviderRegistry;
pub use telegram::TelegramProvider;
pub use twilio_email::TwilioEmailProvider;
pub use twilio_sms::TwilioSmsProvider;
