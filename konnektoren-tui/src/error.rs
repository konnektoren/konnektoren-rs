use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Command execution error: {0}")]
    CommandError(#[from] konnektoren_core::commands::CommandError),

    #[error("UI error: {0}")]
    UiError(String),

    #[error("State error: {0}")]
    StateError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
