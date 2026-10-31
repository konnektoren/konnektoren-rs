use konnektoren_asset_loader::AssetLoadError;

/// Failures retain the requested asset path and, during preload, its reference.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GameLoadError {
    #[error(transparent)]
    Asset(#[from] AssetLoadError),
    #[error("challenge ID mismatch in {path}: expected {expected}, found {actual}")]
    IdMismatch {
        path: String,
        expected: String,
        actual: String,
    },
    #[error("level {level_id}, entry {entry_id}: {source}")]
    Reference {
        level_id: String,
        entry_id: String,
        #[source]
        source: Box<GameLoadError>,
    },
}
