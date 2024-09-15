use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Review {
    pub challenge_id: String,
    pub rating: u8,
    pub comment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_review() {
        let challenge_id = "123".to_string();
        let rating = 5;
        let review = Review {
            challenge_id: challenge_id.clone(),
            rating,
            comment: None,
        };

        assert_eq!(review.challenge_id, challenge_id);
        assert_eq!(review.rating, rating);
        assert_eq!(review.comment, None);
    }

    #[test]
    fn serialize_review() {
        let json_str = r#"{"challenge_id":"123","rating":5,"comment":null}"#;
        let review = Review {
            challenge_id: "123".to_string(),
            rating: 5,
            comment: None,
        };

        let serialized = serde_json::to_string(&review).unwrap();
        assert_eq!(serialized, json_str);
    }
}
