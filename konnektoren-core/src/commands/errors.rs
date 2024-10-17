use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CommandParseError {
    #[error("Failed to parse command: {0}")]
    ParseError(String),
    #[error("Unknown command type: {0}")]
    UnknownCommandType(String),
    #[error("Missing command data")]
    MissingData,
    #[error("Invalid command data: {0}")]
    InvalidData(String),
}
