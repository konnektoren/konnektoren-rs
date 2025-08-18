use thiserror::Error;

#[derive(Error, Debug)]
pub enum I18nReportError {
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Formatting error: {0}")]
    FmtError(#[from] std::fmt::Error),
    #[error("Unknown error: {0}")]
    Other(String),
}
