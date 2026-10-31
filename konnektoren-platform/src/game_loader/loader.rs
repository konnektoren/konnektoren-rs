use super::{GameLoadError, LevelDefinition, Yaml};
use crate::manifest::ManifestConfig;
use konnektoren_asset_loader::{AssetFormat, AssetServer, AssetSource};
use konnektoren_core::{
    challenges::{ChallengeDefinition, ChallengeFactory, ChallengeType},
    game::{Game, GamePath},
};
use std::{collections::HashSet, rc::Rc, sync::Arc};

/// Loads assets through one shared server, without caching mutable game state.
///
/// Construct the source at `manifest.asset_path()` (for example HTTP `/assets/`
/// or native `assets/`). Embedded sources use the same `challenges/...` keys.
/// The manifest's ordered filenames are copied at construction, so custom
/// manifest extensions are supported without tying the loader to their types.
///
/// [`load_game_paths`](Self::load_game_paths) reads metadata only;
/// [`load_challenge`](Self::load_challenge) reads one dataset on demand.
/// [`preload_game`](GameLoader::preload_game) explicitly loads all referenced content.
/// Successful parsed assets are cached; a failed load can be retried.
pub struct GameLoader<S: AssetSource + ?Sized, L = Yaml<GamePath>, C = Yaml<ChallengeType>> {
    level_format: L,
    challenge_format: C,
    server: AssetServer<S>,
    level_paths: Vec<String>,
    challenge_path: Rc<dyn Fn(&str) -> String>,
}

impl<S: AssetSource + ?Sized, L: Clone, C: Clone> Clone for GameLoader<S, L, C> {
    fn clone(&self) -> Self {
        Self {
            level_format: self.level_format.clone(),
            challenge_format: self.challenge_format.clone(),
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
            level_format: Yaml::new(),
            challenge_format: Yaml::new(),
            server,
            level_paths: manifest.game_paths().to_vec(),
            challenge_path: Rc::new(|id| format!("challenges/{id}.yml")),
        }
    }
}

impl<S: AssetSource + ?Sized, L, C> GameLoader<S, L, C> {
    /// Replace parsers while retaining the source, cache, manifest order and path resolver.
    pub fn with_formats<NL, NC>(
        self,
        level_format: NL,
        challenge_format: NC,
    ) -> GameLoader<S, NL, NC> {
        GameLoader {
            server: self.server,
            level_paths: self.level_paths,
            challenge_path: self.challenge_path,
            level_format,
            challenge_format,
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
}

impl<S, L, C> GameLoader<S, L, C>
where
    S: AssetSource + ?Sized + 'static,
    L: AssetFormat + Clone + 'static,
    C: AssetFormat + Clone + 'static,
    L::Asset: Clone + Send + Sync + 'static,
    C::Asset: ChallengeDefinition + Clone + Send + Sync + 'static,
{
    /// Loads a filename relative to `challenges/`, without fetching datasets.
    pub async fn load_game_path(&self, filename: &str) -> Result<Arc<L::Asset>, GameLoadError> {
        Ok(self
            .server
            .load(self.level_format.clone(), format!("challenges/{filename}"))
            .await?)
    }

    /// Loads level metadata in manifest order. No challenge datasets are read.
    pub async fn load_game_paths(&self) -> Result<Vec<L::Asset>, GameLoadError> {
        let mut levels = Vec::with_capacity(self.level_paths.len());
        for filename in &self.level_paths {
            levels.push((*self.load_game_path(filename).await?).clone());
        }
        Ok(levels)
    }

    /// Loads a dataset by `ChallengeConfig.challenge`, not by its entry ID.
    pub async fn load_challenge(&self, content_id: &str) -> Result<Arc<C::Asset>, GameLoadError> {
        let path = (self.challenge_path)(content_id);
        let asset = self
            .server
            .load(self.challenge_format.clone(), path.clone())
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
        levels: &[L::Asset],
    ) -> Result<ChallengeFactory<C::Asset>, GameLoadError>
    where
        L::Asset: LevelDefinition,
    {
        let mut seen = HashSet::new();
        let mut factory = ChallengeFactory {
            challenge_types: Vec::new(),
        };
        for level in levels {
            for (entry_id, content_id) in level.challenge_references() {
                if seen.insert(content_id) {
                    let asset = self.load_challenge(content_id).await.map_err(|source| {
                        GameLoadError::Reference {
                            level_id: level.id().to_owned(),
                            entry_id: entry_id.to_owned(),
                            source: Box::new(source),
                        }
                    })?;
                    factory.challenge_types.push((*asset).clone());
                }
            }
        }
        Ok(factory)
    }

    /// Validate every level and referenced challenge using the configured formats.
    ///
    /// Build scripts can call this to reject invalid bundled assets. Cargo file
    /// change tracking remains the build script's responsibility.
    pub async fn validate_assets(&self) -> Result<(), GameLoadError>
    where
        L::Asset: LevelDefinition,
    {
        let levels = self.load_game_paths().await?;
        self.preload_factory(&levels).await?;
        Ok(())
    }
}

impl<S, L, C> GameLoader<S, L, C>
where
    S: AssetSource + ?Sized + 'static,
    L: AssetFormat<Asset = GamePath> + Clone + 'static,
    C: AssetFormat<Asset = ChallengeType> + Clone + 'static,
{
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
