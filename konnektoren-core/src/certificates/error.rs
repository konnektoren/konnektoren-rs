use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CertificateError {
    #[error("Failed to decode certificate data")]
    DecodingError,

    #[error("Failed to deserialize certificate data: {0}")]
    DeserializationError(String),

    #[error("Failed to serialize certificate data: {0}")]
    SerializationError(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Verification failed")]
    VerificationFailed,

    #[error("Certificate creation failed: {0}")]
    CreationFailed(String),

    #[error("Image processing error: {0}")]
    ImageProcessingError(String),
}

pub type Result<T> = std::result::Result<T, CertificateError>;
