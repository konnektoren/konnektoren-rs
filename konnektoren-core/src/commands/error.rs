use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Game error: {0}")]
    GameError(#[from] crate::game::GameError),

    #[error("Challenge error: {0}")]
    ChallengeError(#[from] crate::challenges::ChallengeError),

    // Merge in the parse errors from errors.rs
    #[error("Failed to parse command: {0}")]
    ParseError(String),

    #[error("Unknown command type: {0}")]
    UnknownCommandType(String),

    #[error("Missing command data")]
    MissingData,

    #[error("Invalid command data: {0}")]
    InvalidData(String),

    #[error("Failed to lock state: {0}")]
    StateLock(String),
}

pub type Result<T> = std::result::Result<T, CommandError>;
