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
