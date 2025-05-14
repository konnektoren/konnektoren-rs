use super::keypair_from_static_str;
use crate::certificates::error::{CertificateError, Result};
use crate::challenges::PerformanceRecord;
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, Verifier, ed25519::SignatureBytes};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq, Default)]
pub struct CertificateData {
    pub game_path_name: String,
    pub total_challenges: usize,
    pub solved_challenges: usize,
    pub performance_percentage: u8,
    pub profile_name: String,
    pub date: DateTime<Utc>,
    pub signature: Option<Vec<u8>>,
}

impl CertificateData {
    pub fn new(
        game_path_name: String,
        total_challenges: usize,
        solved_challenges: usize,
        player_name: String,
        date: DateTime<Utc>,
    ) -> Self {
        let performance_percentage =
            ((solved_challenges as f64 / total_challenges as f64) * 100.0) as u8;
        CertificateData {
            game_path_name,
            total_challenges,
            solved_challenges,
            performance_percentage,
            profile_name: player_name,
            date,
            signature: None,
        }
    }

    pub fn new_data_copy(&self) -> Self {
        CertificateData {
            game_path_name: self.game_path_name.clone(),
            total_challenges: self.total_challenges,
            solved_challenges: self.solved_challenges,
            performance_percentage: self.performance_percentage,
            profile_name: self.profile_name.clone(),
            date: self.date,
            signature: None,
        }
    }

    pub fn to_sha256(&self) -> Vec<u8> {
        let data = self.to_base64();

        let mut sha256 = Sha256::new();
        sha256.update(data);
        format!("{:X}", sha256.finalize()).into_bytes()
    }

    pub fn to_base64(&self) -> String {
        let mut buf = Vec::new();

        self.serialize(&mut rmp_serde::Serializer::new(&mut buf))
            .map_err(|e| {
                CertificateError::SerializationError(format!(
                    "Failed to serialize certificate data to msgpack: {}",
                    e
                ))
            })
            .expect("Failed to serialize certificate data to msgpack.");

        general_purpose::STANDARD.encode(buf).to_string()
    }

    pub fn from_base64(encoded: &str) -> Result<Self> {
        let decoded = general_purpose::STANDARD
            .decode(encoded)
            .map_err(|_| CertificateError::DecodingError)?;

        rmp_serde::from_slice(&decoded).map_err(|e| {
            CertificateError::DeserializationError(format!(
                "Failed to deserialize certificate data: {}",
                e
            ))
        })
    }

    pub fn create_signature(&mut self) -> Result<()> {
        let (signing_key, _) = keypair_from_static_str();
        let certificate_data_copy = CertificateData::new_data_copy(self);

        let serialized = serde_cbor::to_vec(&certificate_data_copy).map_err(|e| {
            CertificateError::SerializationError(format!(
                "Failed to serialize certificate data: {}",
                e
            ))
        })?;

        let signature: Signature = signing_key.sign(&serialized);
        self.signature = Some(signature.to_bytes().to_vec());

        Ok(())
    }

    pub fn verify(&self) -> Result<bool> {
        if self.signature.is_none() {
            return Ok(false);
        }

        let (_, verifying_key) = keypair_from_static_str();
        let certificate_data_copy = CertificateData::new_data_copy(self);

        let serialized = serde_cbor::to_vec(&certificate_data_copy).map_err(|e| {
            CertificateError::SerializationError(format!(
                "Failed to serialize certificate data for verification: {}",
                e
            ))
        })?;

        let signature = self.signature.as_ref().unwrap();
        let signature_bytes = SignatureBytes::try_from(signature.as_slice()).map_err(|_| {
            CertificateError::SignatureError("Failed to convert signature bytes".to_string())
        })?;

        let signature = Signature::from_bytes(&signature_bytes);
        Ok(verifying_key.verify(&serialized, &signature).is_ok())
    }
}

impl From<PerformanceRecord> for CertificateData {
    fn from(record: PerformanceRecord) -> Self {
        CertificateData {
            game_path_name: record.game_path_id,
            total_challenges: record.total_challenges,
            solved_challenges: record.challenges_performance.len(),
            performance_percentage: record.performance_percentage,
            profile_name: record.profile_name,
            date: record.date,
            signature: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_serialization_deserialization() {
        let certificate_data = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Utc::now(),
        );
        let base64_encoded = certificate_data.to_base64();
        let decoded_certificate_data = CertificateData::from_base64(&base64_encoded).unwrap();

        assert_eq!(certificate_data, decoded_certificate_data);
    }

    #[test]
    fn test_base64_error() {
        let base64_encoded = "invalid_base64";
        let decoded_certificate_data = CertificateData::from_base64(base64_encoded);
        assert!(decoded_certificate_data.is_err());

        // Verify error type
        match decoded_certificate_data {
            Err(CertificateError::DecodingError) => {}
            _ => panic!("Expected DecodingError"),
        }
    }

    #[test]
    fn test_certificate_data_new() {
        let certificate_data = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Utc::now(),
        );
        assert_eq!(certificate_data.game_path_name, "Level A1");
        assert_eq!(certificate_data.total_challenges, 12);
        assert_eq!(certificate_data.solved_challenges, 10);
        assert_eq!(certificate_data.performance_percentage, 83);
    }

    #[test]
    fn test_signature_verification() {
        let mut certificate_data = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Utc::now(),
        );

        certificate_data.create_signature().unwrap();
        let is_verified = certificate_data.verify().unwrap();

        assert!(
            is_verified,
            "The signature should be verified successfully."
        );
    }

    #[test]
    fn test_missing_signature_verification() {
        let certificate_data = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Utc::now(),
        );

        // Signature hasn't been created yet
        let is_verified = certificate_data.verify().unwrap();
        assert!(!is_verified, "Verification should fail with no signature");
    }

    #[test]
    fn test_sha256() {
        let certificate_data = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Utc::now(),
        );
        let sha256 = certificate_data.to_sha256();
        assert_eq!(sha256.len(), 64);
    }

    #[test]
    fn test_new_data_copy() {
        let mut certificate_data = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Utc::now(),
        );
        let certificate_data_copy = certificate_data.new_data_copy();
        assert_eq!(certificate_data, certificate_data_copy);

        certificate_data.create_signature().unwrap();

        let certificate_data_copy = certificate_data.new_data_copy();
        assert_ne!(certificate_data, certificate_data_copy);
    }

    #[test]
    fn test_certificate_data_from_performance_record() {
        let performance_record = PerformanceRecord {
            game_path_id: "Level A1".to_string(),
            total_challenges: 12,
            challenges_performance: vec![],
            performance_percentage: 0,
            profile_name: "Player".to_string(),
            date: Utc::now(),
        };

        let certificate_data: CertificateData = performance_record.into();
        assert_eq!(certificate_data.game_path_name, "Level A1");
        assert_eq!(certificate_data.total_challenges, 12);
        assert_eq!(certificate_data.solved_challenges, 0);
        assert_eq!(certificate_data.performance_percentage, 0);
    }
}
