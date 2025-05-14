//! Module for creating certificates for the Konnektoren.
use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

mod certificate_data;
mod certificate_image;
pub mod error;

pub use certificate_data::CertificateData;
pub use certificate_image::{create_certificate, create_certificate_data_url};
pub use error::*;

pub fn keypair_from_static_str() -> (SigningKey, VerifyingKey) {
    let mut hasher = Sha256::new();
    hasher.update(option_env!("SIGNATURE_PRIVATE_KEY").unwrap_or_default());
    let result = hasher.finalize();

    let seed: [u8; 32] = result[..]
        .try_into()
        .expect("Hash output size does not match ed25519 seed size");

    let signing_key = SigningKey::from_bytes(&seed);
    let verify_key = signing_key.verifying_key();

    (signing_key, verify_key)
}
