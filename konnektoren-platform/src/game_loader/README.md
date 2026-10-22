# Game loader

`GameLoader` is the shared platform service for loading `GamePath` metadata,
challenge datasets and complete games. The manifest supplies level filenames and
their order. An injected `AssetSource` or `AssetServer` supplies the bytes.
No app-specific catalogue, browser repository or mutable session is stored here.

## Basic usage

```rust,no_run
use konnektoren_asset_loader::FileSource;
use konnektoren_platform::{
    game_loader::GameLoader,
    manifest::{Manifest, ManifestConfig, NoExtensions},
};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let manifest: Manifest<NoExtensions> = serde_yaml::from_str(
    "assets: { path: assets }\ngame_paths: [level_a0.yml, level_a1.yml]"
)?;
let source = FileSource::from_dir(manifest.asset_path());
let loader = GameLoader::from_manifest(&manifest, source);

// Metadata only: does not load challenge datasets.
let paths = loader.load_game_paths().await?;

// Load the dataset when the player opens a particular entry.
if let Some(config) = paths.first().and_then(|path| path.challenges.first()) {
    let dataset = loader.load_challenge(&config.challenge).await?;
    assert_eq!(dataset.id(), config.challenge);
}

// Alternatively, explicitly preload a factory or a complete game.
let factory = loader.preload_factory(&paths).await?;
let game = loader.preload_game().await?;
# let _ = (factory, game);
# Ok(())
# }
```

`GameLoader::with_server(&manifest, server.clone())` uses an existing asset
server and shares its cache and pending requests. Cloning a loader also shares
that server. `level_paths()` exposes the manifest's ordered filenames;
`load_game_path(filename)` loads one level and returns its cached `Arc<GamePath>`.
`load_game_paths()` returns owned level metadata for direct use in `Game`.

## Paths and source selection

Sources are rooted at `manifest.assets.path`. Source-relative paths are:

| Content | Logical path |
|---|---|
| Manifest entry `level_a0.yml` | `challenges/level_a0.yml` |
| Content reference `greetings` | `challenges/greetings.yml` |

For the default manifest asset root `assets`, a file source points to `assets/`
and an HTTP source to `/assets/`. The loader adds `challenges/` once. It does
not add the physical asset root, infer a backend, or fetch the manifest itself.
You can keep using `load_manifest!()` or supply any `ManifestConfig`, including
a manifest with custom extension sections. Filenames are relative to the
challenges directory; do not prefix manifest entries with `assets/challenges/`.

Embedded and layered sources use these same logical keys. For an Android
bundle, implement `AssetSource` for the app's embedded assets, or supply an
`EmbeddedSource` with `challenges/...` keys. No application assets are embedded
by this module. Existing flat sources must be adapted to this namespace.

With platform's `csr` feature, an embedded-first/HTTP-fallback setup is:

```rust,no_run
# #[cfg(feature = "csr")]
# async fn example(
#     manifest: &impl konnektoren_platform::manifest::ManifestConfig,
#     embedded: konnektoren_asset_loader::EmbeddedSource,
# ) -> Result<(), konnektoren_platform::game_loader::GameLoadError> {
use konnektoren_asset_loader::{LayeredSource, UrlSource};
use konnektoren_platform::game_loader::GameLoader;

let base_url = format!("/{}", manifest.asset_path().trim_matches('/'));
let source = LayeredSource::default()
    .with_source(embedded)
    .with_source(UrlSource::new(base_url));
let loader = GameLoader::from_manifest(manifest, source);
let paths = loader.load_game_paths().await?;
# let _ = paths;
# Ok(())
# }
```

Feature flags make backends available; source construction selects the backend
or combination. The shared loading API has no dependency on the optional
`manifest` feature, which only enables Figment-based manifest acquisition.

## References, caching and errors

`load_challenge` takes `ChallengeConfig.challenge` (the content ID), not the
presentation entry's `id`. It validates the dataset's actual ID. The default
filename resolver uses the exact content ID without replacing hyphens or
underscores. Legacy layouts can opt into a deterministic mapping:

```rust,no_run
# fn example<S: konnektoren_asset_loader::AssetSource + 'static>(
#     loader: konnektoren_platform::game_loader::GameLoader<S>,
# ) {
let loader = loader.with_challenge_path_resolver(|id| {
    format!("challenges/{}.yml", id.replace('-', "_"))
});
# let _ = loader;
# }
```

Preloading deduplicates by content ID across all supplied paths and preserves
first-reference order. It starts with an empty factory, never with core's
embedded default factory. `preload_game()` creates a fresh owned `Game` with
zero XP and empty history; progress, initial challenge selection and session
initialization belong to the caller.

Successful parsed metadata and datasets are cached by the asset server. Missing
files, parse failures and ID mismatches can be retried after fixing the source.
`GameLoadError` retains asset errors and ID mismatch details; factory-preload
errors additionally identify the level ID and presentation entry ID.

Use `loader.server().invalidate("challenges/greetings.yml")` to refresh a
particular asset or `loader.server().clear()` for the entire server cache.
Source-owned caches and browser HTTP caching remain independent. Loading is
local/async, matching `AssetServer`; the loader does not spawn background work.
