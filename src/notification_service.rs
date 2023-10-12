use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct NotificationService {
    pub rate_limits: HashMap<String, RateLimit>,
}

pub struct RateLimit {
    pub max_requests: u32,
    pub per_duration: Duration,
    pub recipient_counters: HashMap<String, RecipientCounter>,
}

pub struct RecipientCounter {
    allowed_requests: u32,
    last_request: Option<Instant>,
}

impl NotificationService {
    pub fn new() -> Self {
        NotificationService {
            rate_limits: HashMap::new(),
        }
    }

    pub fn send(
        &mut self,
        notification_type: &str,
        recipient: &str,
        message: &str,
    ) -> Result<(), String> {
        let rate_limit = self
            .rate_limits
            .entry(notification_type.to_string())
            .or_insert(RateLimit {
                max_requests: u32::MAX,
                per_duration: Duration::from_secs(1),
                recipient_counters: HashMap::new(),
            });

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
