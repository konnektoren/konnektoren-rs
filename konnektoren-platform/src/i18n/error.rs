use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssetLoadError {
    #[error("asset not found: {0}")]
    NotFound(String),

    #[error("invalid UTF-8 in asset '{file}': {source}")]
    Utf8Error {
        file: String,
        source: std::string::FromUtf8Error,
    },

    #[error("YAML parse error in '{file}': {source}")]
    YamlError {
        file: String,
        source: serde_yaml::Error,
    },

    #[error("unexpected YAML structure in '{file}': {detail}")]
    InvalidStructure { file: String, detail: String },
}
