use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coupon {
    pub id: Uuid,                       // Unique identifier for the coupon
    pub code: String,                   // User-friendly coupon code (e.g., "SPRING25")
    pub challenge_ids: Vec<String>,     // IDs of challenges this coupon applies to
    pub max_uses: u32,                  // Maximum number of times this coupon can be used
    pub uses_remaining: u32,            // Number of uses left
    pub expiration_date: DateTime<Utc>, // Date and time the coupon expires
    pub used_by: Vec<String>, // Trace IDs of requests that used this coupon.  This will be a trace ID in practice.
}

#[derive(Debug, Error)]
pub enum CouponRedemptionError {
    #[error("Coupon expired on {0}")]
    Expired(DateTime<Utc>),
    #[error("Coupon already used with this trace ID")]
    AlreadyUsed,
    #[error("Invalid challenge ID for this coupon")]
    InvalidChallenge,
    #[error("Coupon has no uses remaining")]
    NoUsesRemaining,
}

impl Coupon {
    pub fn new(
        code: String,
        challenge_ids: Vec<String>,
        max_uses: u32,
        expiration_date: DateTime<Utc>,
    ) -> Self {
        Coupon {
            id: Uuid::new_v4(),
            code,
            challenge_ids,
            max_uses,
            uses_remaining: max_uses,
            expiration_date,
            used_by: Vec::new(),
        }
    }

    pub fn is_valid(&self, challenge_id: &str, user_id: &str) -> bool {
        // Check for expiration
        if self.expiration_date < Utc::now() {
            return false;
        }

        // Check if uses remaining
        if self.uses_remaining == 0 {
            return false;
        }

        // Check if challenge is valid for this coupon
        if !self.challenge_ids.contains(&challenge_id.to_string()) {
            return false;
        }

        // Check if user_id (trace_id) already exists
        if self.used_by.contains(&user_id.to_string()) {
            return false;
        }

        true
    }

    pub fn redeem(&mut self, user_id: String) -> Result<(), CouponRedemptionError> {
        if self.expiration_date < Utc::now() {
            return Err(CouponRedemptionError::Expired(self.expiration_date));
        }
        if self.uses_remaining == 0 {
            return Err(CouponRedemptionError::NoUsesRemaining);
        }
        if self.used_by.contains(&user_id) {
            return Err(CouponRedemptionError::AlreadyUsed);
        }
        self.uses_remaining -= 1;
        self.used_by.push(user_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_coupon_creation() {
        let expiration_date = Utc::now() + Duration::days(7);
        let coupon = Coupon::new(
            "TEST1234".to_string(),
            vec!["challenge1".to_string(), "challenge2".to_string()],
            2,
            expiration_date,
        );
        assert_eq!(coupon.code, "TEST1234");
        assert_eq!(coupon.max_uses, 2);
        assert_eq!(coupon.uses_remaining, 2);
    }

    #[test]
    fn test_coupon_redemption() {
        let expiration_date = Utc::now() + Duration::days(7);
        let mut coupon = Coupon::new(
            "TEST1234".to_string(),
            vec!["challenge1".to_string()],
            1,
            expiration_date,
        );
        let trace_id = "trace123".to_string();

        assert!(coupon.redeem(trace_id.clone()).is_ok());
        assert!(coupon.redeem(trace_id.clone()).is_err()); // Already used
        assert_eq!(coupon.uses_remaining, 0);

        let expiration_date_past = Utc::now() - Duration::days(1);
        let mut coupon_past = Coupon::new(
            "TEST1235".to_string(),
            vec!["challenge1".to_string()],
            1,
            expiration_date_past,
        );
        assert!(coupon_past.redeem(trace_id.clone()).is_err()); // Expired
    }

    #[test]
    fn test_coupon_redemption_expired() {
        let expiration_date_past = Utc::now() - Duration::days(1);
        let mut coupon = Coupon::new(
            "TEST1235".to_string(),
            vec!["challenge1".to_string()],
            1,
            expiration_date_past,
        );
        let trace_id = "trace123".to_string();
        match coupon.redeem(trace_id) {
            Err(CouponRedemptionError::Expired(date)) => {
                assert_eq!(date, expiration_date_past);
            }
            _ => panic!("Expected Expired error"),
        }
    }
}
