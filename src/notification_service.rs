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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_service_creation() {
        let service = NotificationService::new();
        assert!(service.rate_limits.is_empty());
    }

    #[test]
    fn test_send_notification_within_rate_limit() {
        let mut service = NotificationService::new();

        // Send three notifications within the rate limit
        for _ in 0..3 {
            let result = service.send("test_type", "test_recipient", "Test message");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_send_notification_exceeds_rate_limit() {
        let mut service = NotificationService::new();

        // Set a rate limit of 2 per minute for "test_type"
        service.rate_limits.insert(
            "test_type".to_string(),
            RateLimit {
                max_requests: 2,
                per_duration: Duration::from_secs(60),
                recipient_counters: HashMap::new(),
            },
        );

        // Send two notifications within the rate limit
        for _ in 0..2 {
            let success_result = service.send("test_type", "test_recipient", "Test message");
            assert_eq!(success_result, Ok(()));
        }

        // Attempt to send a third notification, which should exceed the rate limit
        let exceed_limit_result = service.send("test_type", "test_recipient", "Test message");
        assert!(exceed_limit_result.is_err());
    }

    #[test]
    fn test_send_notification_after_rate_limit_duration() {
        let mut service = NotificationService::new();

        // Set a rate limit of 1 per second for "test_type"
        service.rate_limits.insert(
            "test_type".to_string(),
            RateLimit {
                max_requests: 1,
                per_duration: Duration::from_secs(1),
                recipient_counters: HashMap::new(),
            },
        );

        // Send a notification within the rate limit
        let success_result = service.send("test_type", "test_recipient", "Test message");
        assert_eq!(success_result, Ok(()));

        // Attempt to send another notification, which should exceed the rate limit
        let exceed_limit_result = service.send("test_type", "test_recipient", "Test message");
        assert!(exceed_limit_result.is_err());

        // Wait for 1 second (exceeding the rate limit duration)
        std::thread::sleep(Duration::from_secs(1));

        // Attempt to send another notification, which should be allowed as the elapsed time has exceeded the rate limit's duration
        let result_after_elapsed_time = service.send("test_type", "test_recipient", "Test message");
        assert_eq!(result_after_elapsed_time, Ok(()));
    }
}
