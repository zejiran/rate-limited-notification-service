use std::{collections::HashMap, time::Duration};

mod notification_service;

fn main() {
    let mut service = notification_service::NotificationService::new();

    // Define rate limits for different notification types
    let rate_limits = vec![
        ("status", 2, Duration::from_secs(60)),         // 2 per minute
        ("news", 1, Duration::from_secs(24 * 60 * 60)), // 1 per day
        ("marketing", 3, Duration::from_secs(60 * 60)), // 3 per hour
    ];

    for (notification_type, max_requests, per_duration) in rate_limits {
        service.rate_limits.insert(
            notification_type.to_string(),
            notification_service::RateLimit {
                max_requests,
                per_duration,
                recipient_counters: HashMap::new(),
            },
        );
    }

    // Example usage:
    for _ in 0..3 {
        match service.send("news", "user 1", "This is a news update") {
            Ok(()) => println!("Notification sent successfully."),
            Err(err) => println!("Failed to send notification: {}", err),
        }
        match service.send("marketing", "user 1", "This is a marketing update") {
            Ok(()) => println!("Notification sent successfully."),
            Err(err) => println!("Failed to send notification: {}", err),
        }
        match service.send("news", "user 2", "This is a news update") {
            Ok(()) => println!("Notification sent successfully."),
            Err(err) => println!("Failed to send notification: {}", err),
        }
    }
}
