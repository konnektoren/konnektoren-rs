use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GameError {
    #[error("Challenge not found: {0}")]
    ChallengeNotFound(String),

    #[error("Game path not found")]
    GamePathNotFound,

    #[error("Invalid game state: {0}")]
    InvalidGameState(String),

    #[error("Challenge error: {0}")]
    ChallengeError(#[from] crate::challenges::ChallengeError),
}

pub type Result<T> = std::result::Result<T, GameError>;
