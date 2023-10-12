use std::collections::HashMap;
use std::time::{Duration, Instant};

struct NotificationService {
    rate_limits: HashMap<String, RateLimit>,
}

struct RateLimit {
    max_requests: u32,
    allowed_requests: u32,
    per_duration: Duration,
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
            allowed_requests: max_requests,
            per_duration,
            last_request: Some(Instant::now()),
        }
    }

    fn send(
        &mut self,
        notification_type: &str,
        recipient: &str,
        message: &str,
    ) -> Result<(), String> {
        if let Some(rate_limit) = self.rate_limits.get_mut(notification_type) {
            let per_duration = rate_limit.per_duration;
            let now = Instant::now();
            let elapsed = now.duration_since(rate_limit.last_request.unwrap_or(now));

            if elapsed < per_duration {
                if rate_limit.allowed_requests == 0 {
                    return Err(format!(
                        "Rate limit exceeded for {} notifications. No more requests allowed.",
                        notification_type
                    ));
                }
                // Update the number of allowed requests if it's not zero
                rate_limit.allowed_requests = rate_limit.allowed_requests.saturating_sub(1);
            } else {
                // Reset the rate limit if the duration has passed
                rate_limit.last_request = Some(now);
                rate_limit.allowed_requests = rate_limit.max_requests;
            }
        } else {
            // If no rate limit rule is defined, treat it as no rate limiting.
            self.rate_limits.insert(
                notification_type.to_string(),
                Self::create_rate_limit(u32::MAX, Duration::from_secs(1)),
            );
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

    // Example usage:
    for _ in 0..5 {
        match service.send("news", "user", "This is a news update") {
            Ok(()) => println!("Notification sent successfully."),
            Err(err) => println!("Failed to send notification: {}", err),
        }
    }
}
