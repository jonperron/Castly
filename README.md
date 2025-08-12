# 📢 Castly

[![Build Status](https://img.shields.io/github/actions/workflow/status/your-org/castly/ci.yml?branch=main)](https://github.com/your-org/castly/actions)

**Castly** is a flexible, provider-agnostic notification service built in Rust.  
Send messages via email, SMS, or messaging apps using a single API—no matter the underlying provider.

---

## ⚠️ Status

> **Castly is under active development.**
Expect breaking changes until the first stable release.

---

## 🚀 Key Features

- **Multi-Provider Support**  
  Integrate with providers like **Mailgun**, **Mailjet**, **Twilio**, and **Telegram** for email, SMS, or messaging.  
  Future-ready for push notifications (e.g., Firebase).

- **Templated Notifications**  
  Dynamic, customizable templates powered by [Tera](https://keats.github.io/tera/docs/).

- **Synchronous & Asynchronous Modes**  
  - **Synchronous** – Send instantly via `/send` HTTP endpoint.  
  - **Asynchronous** – (Coming soon) Consume messages from a queue.

- **Configurable via YAML**  
  Works out-of-the-box and adaptable for Kubernetes deployments.

- **Observability Built-In**  
  Health checks (`/health`), structured logging, and distributed tracing via [tracing](https://github.com/tokio-rs/tracing).

---

## 📦 Supported Providers

| Name     | Type      | Value in request |
| -------- | --------- | ---------------- |
| Mailgun  | Email     | `mail_mailgun`   |
| Mailjet  | Email     | `mail_mailjet`   |
| Twilio   | Email     | `mail_twilio`    |
| Telegram | Messaging | `telegram`       |
| Twilio   | SMS       | `sms_twilio`     |

---

## 🛠️ Usage

**Send a Notification via API**

**Endpoint**  
`POST /send`

**Headers**  
`Content-Type: application/json`

**Example Request**  

```json
{
  "to": ["receiver@example.com"],
  "from": "sender@example.com",
  "subject": "Hello, World!",
  "notification_type": "mail_mailgun",
  "use_raw_text": {
    "text": "Hello world!"
  }
}
