use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersistenceError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("State not found")]
    StateNotFound,

    #[error("Access error: {0}")]
    AccessError(String),
}

pub type Result<T> = std::result::Result<T, PersistenceError>;
