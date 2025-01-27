# Messenger

Messenger is a flexible service built in Rust for sending notifications via multiple providers. It supports both synchronous API calls and asynchronous message consumption, allowing seamless integration into modern architectures. The service is designed to be provider-agnostic, configurable, and extensible for various notification types.

##  Features

* Multi-Provider Support: Easily integrate providers like Mailgun, Mailjet, or others for email notifications. Future-proof for adding push notifications (e.g., Firebase).
* Templated Notifications: Supports dynamic content generation using [Tera](https://keats.github.io/tera/docs/) for customizable templates.
* Synchronous and Asynchronous Modes:
  * Synchronous: Use the /send HTTP endpoint to send notifications via API.
  * Asynchronous: TBA
* Configurable via YAML. Support for k8s to be added
* Health Checks: Includes a /health endpoint for monitoring and readiness probes.
* Tracing and Observability: Built-in support for structured logging and distributed tracing using [tracing](https://github.com/tokio-rs/tracing).

## How It Works

* Configuration: Define provider settings, template paths, and notification preferences in a config.yaml file.
* Templating: Use Tera templates for dynamic content generation. Templates can be organized by language or type (e.g., templates/hello_world.en.html).
* Sending Notifications:
  * API: Send a POST request to /send with the notification payload.
* Extensibility: Add new providers by implementing the EmailProvider trait.

## Supported providers

| Name    | Type  | Value in request |
| ------- | ----- | ---------------- |
| Mailgun | Email | `mail_mailgun`   |
| Mailjet | Email | `mail_mailjet`   |

## API Example

`/send` Endpoint

Send a notification via API:

* Request: POST /send
* Headers: Content-Type: application/json
* Body:

```json
{
  "to": ["receiver@example.com"],
  "from": "sender@example.com",
  "subject": "Hello, World!",
  "notification_type": "mail_mailgun",
  "use_raw_text": {
    "text": "Hello world!"
  }
}

```

Open API specifications are available [here](docs/open_api_specifications.yml).

## Running the Service

Build and run the service:

```bash
cargo run
```

Access the API on <http://localhost:3000>.

## Configuration

See config.yaml:

```yaml
templates:
  path: "templates"
  default_language: "en"

providers:
  mailgun:
    domain: "your-mailgun-domain.com"
    api_key: "your-mailgun-api-key"
    base_url: "https://api.mailgun.net"

  mailjet:
    api_key: "your-mailjet-api-key"
    secret_key: "your-mailjet-secret-key"
```

## Monitoring

Use the /health endpoint to monitor the service's status:

```bash
curl http://localhost:3000/health
```

## Contributing

TBA
