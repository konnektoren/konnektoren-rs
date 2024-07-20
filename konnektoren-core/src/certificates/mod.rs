use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

mod certificate_data;

pub use certificate_data::CertificateData;

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
