use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EventParseError {
    #[error("Failed to parse event: {0}")]
    ParseError(String),
    #[error("Unknown event type: {0}")]
    UnknownEventType(String),
    #[error("Missing event data")]
    MissingData,
    #[error("Invalid event data: {0}")]
    InvalidData(String),
}
