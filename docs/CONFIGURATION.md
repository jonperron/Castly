# Configuration

This document describes the configuration options for the notification service. The configuration is specified in the `config.yaml` file.

## Templates Configuration

The `templates` section configures the template settings for the notification service.

* `path` (required): The directory path where the templates are stored.
* `default_language` (required): The default language for the templates.

```yaml
templates:
  path: "templates"
  default_language: "en"
```

## Providers Configuration

The providers section configures the service providers. Several providers can be configured at once.

### Mailgun Configuration

The mailgun section configures the Mailgun email service provider.

* `domain` (required): The domain used for sending emails.
* `api_key` (required): The API key for authenticating with the Mailgun service.
* `base_url` (optional): The base URL for the Mailgun API.

```yaml
providers:
  mailgun:
    domain: "example.com"
    api_key: "key-1234567890abcdef"
    base_url: "https://api.mailgun.net/v3"
```

### Mailjet Configuration

The mailjet section configures the Mailjet email service provider.

* `api_key` (required): The API key for authenticating with the Mailjet service.
* `secret_key` (required): The API secret for authenticating with the Mailjet service.
* `v31` (required): A flag indicating whether to use the v3.1 API version.

```yaml
providers:
  mailjet:
    api_key: "key-1234567890abcdef"
    secret_key: "secret-1234567890abcdef"
    v31: true
```

### Telegram

The telegram section configures the Telegram service provider

* `bot_token` (required): the bot token for authenticating with Telegram.

```yaml
providers:
  telegram:
    bot_token: "my-bot:1234356ezds"
```

### Twilio SMS

The twilio_sms section configures the Twilio SMS service provider.

* `account_sid` (required): The Account SID for authenticating with Twilio.
* `auth_token` (required): The Auth Token for authenticating with Twilio.

**Note**: The sender phone number is provided via the `from` field in the notification request.

```yaml
providers:
  twilio_sms:
    account_sid: "ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    auth_token: "your_auth_token_here"
```

### Twilio Email

The twilio_email section configures the Twilio SendGrid Email service provider.

* `api_key` (required): The API key for authenticating with Twilio SendGrid.

**Note**: The sender email address is provided via the `from` field in the notification request.

```yaml
providers:
  twilio_email:
    api_key: "SG.your_api_key_here"
```
