use thiserror::Error;

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Game state error: {0}")]
    GameState(String),

    #[error("Persistence error: {0}")]
    Persistence(#[from] crate::persistence::PersistenceError),

    #[error("Command execution error: {0}")]
    CommandExecution(#[from] crate::commands::CommandError),

    #[error("Plugin error: {0}")]
    Plugin(#[from] crate::controller::ControllerPluginError),

    #[error("State lock error")]
    StateLock,

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub type Result<T> = std::result::Result<T, ControllerError>;
