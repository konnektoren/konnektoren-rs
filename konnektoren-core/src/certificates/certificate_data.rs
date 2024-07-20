use super::keypair_from_static_str;
use crate::challenges::PerformanceRecord;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use ed25519_dalek::{ed25519::SignatureBytes, Signature, Signer, Verifier};
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
            .expect("Failed to serialize certificate data to msgpack.");

        general_purpose::STANDARD.encode(buf).to_string()
    }

    pub fn from_base64(encoded: &str) -> Result<Self> {
        let decoded = match general_purpose::STANDARD.decode(encoded) {
            Ok(decoded) => decoded,
            Err(_) => return Err(anyhow!("Failed to decode the certificate data.")),
        };

        match rmp_serde::from_slice(&decoded) {
            Ok(certificate_data) => Ok(certificate_data),
            Err(_) => Err(anyhow!(
                "Failed to deserialize certificate data from msgpack."
            )),
        }
    }

    pub fn create_signature(&mut self) {
        let (signing_key, _) = keypair_from_static_str();
        let certificate_data_copy = CertificateData::new_data_copy(self);

        let serialized = serde_cbor::to_vec(&certificate_data_copy)
            .expect("Failed to serialize certificate data");
        let signature: Signature = signing_key.sign(&serialized);

        self.signature = Some(signature.to_bytes().to_vec());
    }

    pub fn verify(&self) -> bool {
        let (_, verifying_key) = keypair_from_static_str();

        let certificate_data_copy = CertificateData::new_data_copy(self);

        let serialized = serde_cbor::to_vec(&certificate_data_copy)
            .expect("Failed to serialize certificate data for verification");

        match &self.signature {
            Some(signature) => {
                let signature_bytes = SignatureBytes::try_from(signature.as_slice())
                    .expect("Failed to convert signature bytes");
                let signature = Signature::from_bytes(&signature_bytes);

                verifying_key.verify(&serialized, &signature).is_ok()
            }
            None => false,
        }
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

        certificate_data.create_signature();
        let is_verified = certificate_data.verify();

        assert!(
            is_verified,
            "The signature should be verified successfully."
        );
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

        certificate_data.create_signature();

        let certificate_data_copy = certificate_data.new_data_copy();
        assert_ne!(certificate_data, certificate_data_copy);
    }
}
