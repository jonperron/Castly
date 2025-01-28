// Declaration
pub mod errors;
pub mod mailgun;
pub mod mailjet;
pub mod providers;
pub mod telegram;

// Limit import to only what is useful
pub use mailgun::MailgunProvider;
pub use mailjet::MailjetProvider;
pub use providers::Provider;
pub use telegram::TelegramProvider;
