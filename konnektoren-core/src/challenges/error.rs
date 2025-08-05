use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ChallengeError {
    #[error("Challenge type not found")]
    ChallengeTypeNotFound,

    #[error("Challenge config not found: {0}")]
    ChallengeConfigNotFound(String),

    #[error("Invalid challenge input: {0}")]
    InvalidInput(String),

    #[error("No more tasks")]
    NoMoreTasks,

    #[error("No previous tasks")]
    NoPreviousTasks,

    #[error("Invalid option id: {0}")]
    InvalidOptionId(usize),

    #[error("Invalid challenge type for operation")]
    InvalidChallengeType,

    #[error("Challenge validation failed: {0}")]
    ValidationFailed(String),

    #[error("Base64 decode error: {0}")]
    Base64Decode(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Challenge not found: {0}")]
    ChallengeNotFound(String),
}

pub type Result<T> = std::result::Result<T, ChallengeError>;
