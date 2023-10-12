# Rate-Limited Notification Service

Rust-based application that provides rate limiting for sending notifications to recipients. This service allows you to control the rate at which notifications can be sent based on the notification type, recipient, and a predefined rate limit.

## Table of Contents

- [Rate-Limited Notification Service](#rate-limited-notification-service)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Usage](#usage)
    - [Prerequisites](#prerequisites)
  - [Testing](#testing)
  - [License](#license)

## Features

- Rate limiting for various notification types.
- Customizable rate limits for each notification type.
- Rate limits are enforced per recipient.
- Logs notification attempts and rate-limit violations.

## Usage

### Prerequisites

- Rust and Cargo are installed on your system. If not, you can install them from [Rust's official website](https://www.rust-lang.org/tools/install).

1. Clone the repository to your local machine.

```bash
git clone https://github.com/zejiran/rate-limited-notification-service.git
```

2. Build the project.

```bash
cargo build
```

3. Run the application.

```bash
cargo run
```

4. Define Rate Limits

You can define rate limits for different notification types in the `main.rs` file. For example:

```rust
service.rate_limits.insert(
    "status".to_string(),
    notification_service::RateLimit {
        max_requests: 2,
        per_duration: Duration::from_secs(60), // 2 per minute
        recipient_counters: HashMap::new(),
    },
);
```

5. Send Notifications

You can send notifications using the `send` method in the `NotificationService` struct. The service will enforce rate limits based on the configuration you set.

```rust
match service.send("status", "user123", "This is a status update") {
    Ok(()) => println!("Notification sent successfully."),
    Err(err) => println!("Failed to send notification: {}", err),
}
```

## Testing

Unit tests are provided, covering various aspects of the notification service. You can run the tests using Cargo.

```bash
cargo test
```

## License

[![License](http://img.shields.io/:license-mit-blue.svg?style=flat-square)](http://badges.mit-license.org)

- **[MIT license](LICENSE)**
- Copyright 2023 © Juan Alegría
