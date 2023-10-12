use std::collections::HashMap;
use std::time::{Duration, Instant};

struct NotificationService {
    rate_limits: HashMap<String, RateLimit>,
}

struct RateLimit {
    max_requests: u32,
    per_duration: Duration,
    recipient_counters: HashMap<String, RecipientCounter>,
}

struct RecipientCounter {
    allowed_requests: u32,
    last_request: Option<Instant>,
}

impl NotificationService {
    fn new() -> Self {
        NotificationService {
            rate_limits: HashMap::new(),
        }
    }

    fn create_rate_limit(max_requests: u32, per_duration: Duration) -> RateLimit {
        RateLimit {
            max_requests,
            per_duration,
            recipient_counters: HashMap::new(),
        }
    }

    fn send(
        &mut self,
        notification_type: &str,
        recipient: &str,
        message: &str,
    ) -> Result<(), String> {
        let rate_limit = self
            .rate_limits
            .entry(notification_type.to_string())
            .or_insert(Self::create_rate_limit(u32::MAX, Duration::from_secs(1)));

        let recipient_counter = rate_limit
            .recipient_counters
            .entry(recipient.to_string())
            .or_insert(RecipientCounter {
                allowed_requests: rate_limit.max_requests,
                last_request: Some(Instant::now()),
            });

        let now = Instant::now();
        let elapsed = now.duration_since(recipient_counter.last_request.unwrap_or(now));

        if elapsed <= rate_limit.per_duration {
            if recipient_counter.allowed_requests <= 0 {
                return Err(format!(
                    "Rate limit exceeded for {} notifications to {}. No more requests allowed.",
                    notification_type, recipient
                ));
            }
            recipient_counter.allowed_requests -= 1;
        } else {
            recipient_counter.last_request = Some(now);
            recipient_counter.allowed_requests = rate_limit.max_requests - 1;
        }

        // Perform the actual notification sending here.
        println!(
            "Sending {} notification to {}: {}",
            notification_type, recipient, message
        );

        Ok(())
    }
}

fn main() {
    let mut service = NotificationService::new();

    // Define rate limits for different notification types
    service.rate_limits.insert(
        "status".to_string(),
        NotificationService::create_rate_limit(2, Duration::from_secs(60)), // 2 per minute
    );
    service.rate_limits.insert(
        "news".to_string(),
        NotificationService::create_rate_limit(1, Duration::from_secs(24 * 60 * 60)), // 1 per day
    );
    service.rate_limits.insert(
        "marketing".to_string(),
        NotificationService::create_rate_limit(3, Duration::from_secs(60 * 60)), // 3 per hour
    );

    // Example usage:
    for _ in 0..5 {
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
