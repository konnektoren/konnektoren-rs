use crate::challenges::error::{ChallengeError, Result};
use base64::Engine;
use rmp_serde;
use serde::{Deserialize, Serialize};

/// Trait for serializing and deserializing from base64 encoded data using MessagePack (rmp_serde)
pub trait RmpBase64Serializable {
    /// Serialize to base64 encoded string using MessagePack
    fn to_rmp_base64(&self) -> Result<String>
    where
        Self: Serialize,
    {
        let serialized_bytes =
            rmp_serde::to_vec(self).map_err(|e| ChallengeError::Serialization(e.to_string()))?;

        let encoded = base64::engine::general_purpose::STANDARD.encode(serialized_bytes);
        Ok(encoded)
    }

    /// Deserialize from base64 encoded string using MessagePack
    fn from_rmp_base64(data: &str) -> Result<Self>
    where
        Self: Sized + for<'de> Deserialize<'de>,
    {
        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| ChallengeError::Base64Decode(e.to_string()))?;

        let deserialized: Self = rmp_serde::from_slice(&decoded_bytes)
            .map_err(|e| ChallengeError::Deserialization(e.to_string()))?;

        Ok(deserialized)
    }
}

// Implement the new trait for ChallengeType
impl RmpBase64Serializable for crate::challenges::challenge_type::ChallengeType {}

#[cfg(test)]
mod tests {
    use super::super::Base64Serializable;
    use super::*;
    use crate::challenges::MultipleChoice;
    use crate::challenges::challenge_type::ChallengeType;

    // Re-use your existing YAML tests (if Base64Serializable is kept)
    #[test]
    fn test_encode_decode_multiple_choice_yaml() {
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

        let encoded = challenge_type.to_base64().unwrap();
        assert!(!encoded.is_empty());
        println!("YAML Encoded length: {}", encoded.len()); // For comparison

        let decoded: ChallengeType = ChallengeType::from_base64(&encoded).unwrap();

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

    // New tests for RmpBase64Serializable
    #[test]
    fn test_encode_decode_multiple_choice_rmp() {
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

        // Encode to rmp_serde + base64
        let encoded = challenge_type.to_rmp_base64().unwrap();
        assert!(!encoded.is_empty());
        println!("RMP Encoded length: {}", encoded.len()); // For comparison

        // Decode from rmp_serde + base64
        let decoded: ChallengeType = ChallengeType::from_rmp_base64(&encoded).unwrap();

        // Verify that the decoded content matches the original
        match (&challenge_type, &decoded) {
            (ChallengeType::MultipleChoice(orig), ChallengeType::MultipleChoice(decoded)) => {
                assert_eq!(orig.id, decoded.id);
                assert_eq!(orig.name, decoded.name);
                assert_eq!(orig.options.len(), decoded.options.len());
                assert_eq!(orig.questions.len(), decoded.questions.len());
                assert_eq!(orig.questions[0].image, decoded.questions[0].image); // More thorough check
            }
            _ => panic!("Decoded type doesn't match original"),
        }
    }

    #[test]
    fn test_encode_decode_empty_string_rmp() {
        let empty_data = "";
        let result = ChallengeType::from_rmp_base64(empty_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_invalid_base64_rmp() {
        let invalid_base64 = "This is not base64 content!";
        let result = ChallengeType::from_rmp_base64(invalid_base64);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_invalid_rmp() {
        // This is valid base64, but invalid MessagePack data
        let invalid_rmp_data = b"\xc0\xc1\xc2"; // Example of invalid MessagePack
        let encoded = base64::engine::general_purpose::STANDARD.encode(invalid_rmp_data);
        let result = ChallengeType::from_rmp_base64(&encoded);
        assert!(result.is_err());
    }
}
