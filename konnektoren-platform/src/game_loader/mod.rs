#![doc = include_str!("README.md")]

use crate::manifest::ManifestConfig;
use konnektoren_asset_loader::{AssetFormat, AssetLoadError, AssetServer, AssetSource};
use konnektoren_core::{
    challenges::{ChallengeFactory, ChallengeType},
    game::{Game, GamePath},
};
use serde::de::DeserializeOwned;
use std::{collections::HashSet, marker::PhantomData, rc::Rc, sync::Arc};

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

/// Loads assets through one shared server, without caching mutable game state.
///
/// Construct the source at `manifest.asset_path()` (for example HTTP `/assets/`
/// or native `assets/`). Embedded sources use the same `challenges/...` keys.
/// The manifest's ordered filenames are copied at construction, so custom
/// manifest extensions are supported without tying the loader to their types.
///
/// [`load_game_paths`](Self::load_game_paths) reads metadata only;
/// [`load_challenge`](Self::load_challenge) reads one dataset on demand.
/// [`preload_game`](Self::preload_game) explicitly loads all referenced content.
/// Successful parsed assets are cached; a failed load can be retried.
pub struct GameLoader<S: AssetSource + ?Sized> {
    server: AssetServer<S>,
    level_paths: Vec<String>,
    challenge_path: Rc<dyn Fn(&str) -> String>,
}

impl<S: AssetSource + ?Sized> Clone for GameLoader<S> {
    fn clone(&self) -> Self {
        Self {
            server: self.server.clone(),
            level_paths: self.level_paths.clone(),
            challenge_path: self.challenge_path.clone(),
        }
    }
}

impl<S: AssetSource + 'static> GameLoader<S> {
    /// Uses a source rooted at the manifest's asset directory.
    pub fn from_manifest(manifest: &impl ManifestConfig, source: S) -> Self {
        Self::with_server(manifest, AssetServer::new(Rc::new(source)))
    }
}

impl<S: AssetSource + ?Sized + 'static> GameLoader<S> {
    /// Shares an existing server's cache and pending requests.
    pub fn with_server(manifest: &impl ManifestConfig, server: AssetServer<S>) -> Self {
        Self {
            server,
            level_paths: manifest.game_paths().to_vec(),
            challenge_path: Rc::new(|id| format!("challenges/{id}.yml")),
        }
    }

    /// Overrides dataset filename resolution, without changing dataset IDs.
    /// Returned paths are relative to the supplied source's asset root.
    /// The resolver must be deterministic; clone the loader to share it.
    pub fn with_challenge_path_resolver(
        mut self,
        resolver: impl Fn(&str) -> String + 'static,
    ) -> Self {
        self.challenge_path = Rc::new(resolver);
        self
    }

    /// Filenames in manifest order, relative to the challenges directory.
    pub fn level_paths(&self) -> &[String] {
        &self.level_paths
    }

    /// Access to cache invalidation using source-relative logical paths.
    pub fn server(&self) -> &AssetServer<S> {
        &self.server
    }

    /// Loads a filename relative to `challenges/`, without fetching datasets.
    pub async fn load_game_path(&self, filename: &str) -> Result<Arc<GamePath>, GameLoadError> {
        Ok(self
            .server
            .load(Yaml::<GamePath>::new(), format!("challenges/{filename}"))
            .await?)
    }

    /// Loads level metadata in manifest order. No challenge datasets are read.
    pub async fn load_game_paths(&self) -> Result<Vec<GamePath>, GameLoadError> {
        let mut levels = Vec::with_capacity(self.level_paths.len());
        for filename in &self.level_paths {
            levels.push((*self.load_game_path(filename).await?).clone());
        }
        Ok(levels)
    }

    /// Loads a dataset by `ChallengeConfig.challenge`, not by its entry ID.
    pub async fn load_challenge(
        &self,
        content_id: &str,
    ) -> Result<Arc<ChallengeType>, GameLoadError> {
        let path = (self.challenge_path)(content_id);
        let asset = self
            .server
            .load(Yaml::<ChallengeType>::new(), path.clone())
            .await?;
        if asset.id() != content_id {
            // A corrected remote file must be retried rather than kept in cache.
            self.server.invalidate(&path);
            return Err(GameLoadError::IdMismatch {
                path,
                expected: content_id.to_owned(),
                actual: asset.id().to_owned(),
            });
        }
        Ok(asset)
    }

    /// Loads each unique content ID once, preserving first-reference order.
    pub async fn preload_factory(
        &self,
        levels: &[GamePath],
    ) -> Result<ChallengeFactory, GameLoadError> {
        let mut seen = HashSet::new();
        let mut factory = ChallengeFactory::new();
        for level in levels {
            for config in &level.challenges {
                if seen.insert(config.challenge.as_str()) {
                    let asset = self
                        .load_challenge(&config.challenge)
                        .await
                        .map_err(|source| GameLoadError::Reference {
                            level_id: level.id.clone(),
                            entry_id: config.id.clone(),
                            source: Box::new(source),
                        })?;
                    factory.challenge_types.push((*asset).clone());
                }
            }
        }
        Ok(factory)
    }

    /// Constructs a fresh game with all manifest content, zero XP and no history.
    /// App-specific repositories, saved progress and session initialization remain
    /// outside this loader.
    pub async fn preload_game(&self) -> Result<Game, GameLoadError> {
        let game_paths = self.load_game_paths().await?;
        let challenge_factory = self.preload_factory(&game_paths).await?;
        Ok(Game {
            game_paths,
            challenge_factory,
            challenge_history: Default::default(),
            xp: 0,
        })
    }
}

struct Yaml<T>(PhantomData<fn() -> T>);
impl<T> Yaml<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T: DeserializeOwned> AssetFormat for Yaml<T> {
    type Asset = T;
    type Error = serde_yaml::Error;

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }
    fn cache_key(&self) -> Option<String> {
        Some(String::new())
    }
    fn parse(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        serde_yaml::from_slice(bytes)
    }
}
