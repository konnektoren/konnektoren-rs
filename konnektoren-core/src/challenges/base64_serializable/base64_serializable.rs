use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::challenges::error::{ChallengeError, Result};

/// Trait for serializing and deserializing from base64 encoded data
pub trait Base64Serializable {
    /// Serialize to base64 encoded string
    fn to_base64(&self) -> Result<String>
    where
        Self: Serialize,
    {
        let serialized = serde_yaml::to_string(self)
            .map_err(|e| ChallengeError::Serialization(e.to_string()))?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(serialized);
        Ok(encoded)
    }

    /// Deserialize from base64 encoded string
    fn from_base64(data: &str) -> Result<Self>
    where
        Self: Sized + for<'de> Deserialize<'de>,
    {
        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| ChallengeError::Base64Decode(e.to_string()))?;

        let decoded_str = String::from_utf8(decoded_bytes)
            .map_err(|e| ChallengeError::Base64Decode(e.to_string()))?;

        let deserialized: Self = serde_yaml::from_str(&decoded_str)
            .map_err(|e| ChallengeError::Deserialization(e.to_string()))?;

        Ok(deserialized)
    }
}

// Implement the trait for ChallengeType
impl Base64Serializable for crate::challenges::challenge_type::ChallengeType {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::MultipleChoice;
    use crate::challenges::challenge_type::ChallengeType;

    #[test]
    fn test_encode_decode_multiple_choice() {
        // Create a sample challenge type
        let challenge_type = ChallengeType::MultipleChoice(MultipleChoice {
            id: "articles-1".to_string(),
            name: "Artikel".to_string(),
            lang: "de".to_string(),
            options: vec![
                crate::challenges::MultipleChoiceOption {
                    id: 0,
                    name: "der".to_string(),
                },
                crate::challenges::MultipleChoiceOption {
                    id: 1,
                    name: "die".to_string(),
                },
                crate::challenges::MultipleChoiceOption {
                    id: 2,
                    name: "das".to_string(),
                },
            ],
            questions: vec![crate::challenges::Question {
                question: "Haus".to_string(),
                help: "Ich habe ein neues Haus gekauft.".to_string(),
                option: 2,
                image: Some("fa-regular fa-house-chimney-window".to_string()),
            }],
        });

        // Encode to base64
        let encoded = challenge_type.to_base64().unwrap();
        assert!(!encoded.is_empty());

        // Decode from base64
        let decoded: ChallengeType = ChallengeType::from_base64(&encoded).unwrap();

        // Verify that the decoded content matches the original
        match (&challenge_type, &decoded) {
            (ChallengeType::MultipleChoice(orig), ChallengeType::MultipleChoice(decoded)) => {
                assert_eq!(orig.id, decoded.id);
                assert_eq!(orig.name, decoded.name);
                assert_eq!(orig.options.len(), decoded.options.len());
                assert_eq!(orig.questions.len(), decoded.questions.len());
            }
            _ => panic!("Decoded type doesn't match original"),
        }
    }

    #[test]
    fn test_encode_decode_empty_string() {
        let empty_data = "";
        let result = ChallengeType::from_base64(empty_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_invalid_base64() {
        let invalid_base64 = "This is not base64 content!";
        let result = ChallengeType::from_base64(invalid_base64);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_invalid_yaml() {
        let invalid_yaml = "!!invalid yaml content";
        let encoded = base64::engine::general_purpose::STANDARD.encode(invalid_yaml);
        let result = ChallengeType::from_base64(&encoded);
        assert!(result.is_err());
    }
}
