use futures::executor::block_on;
use konnektoren_asset_loader::{
    AssetError, AssetLoadError, AssetServer, AssetSource, EmbeddedSource, FileSource, LayeredSource,
};
use konnektoren_core::game::Game;
use konnektoren_platform::{
    game_loader::{GameLoadError, GameLoader},
    manifest::{Manifest, NoExtensions},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const LEVEL_B: &[u8] = br#"
id: level-b
name: Second file, first in manifest
challenges:
  - id: shared-first
    name: First presentation
    description: Shared content
    challenge: shared
    tasks: '1'
    unlock_points: 0
  - id: other-entry
    name: Other content
    description: Other content
    challenge: other
    tasks: '1'
    unlock_points: 0
"#;
const LEVEL_A: &[u8] = br#"
id: level-a
name: First file, second in manifest
challenges:
  - id: shared-again
    name: Second presentation
    description: Shared content
    challenge: shared
    tasks: '1'
    unlock_points: 0
"#;
const SHARED: &[u8] = br#"!multiple-choice
id: shared
name: Shared content
lang: de
options:
  - id: 0
    name: Hallo
questions:
  - question: Begruessung?
    help: Say hello
    option: 0
"#;
const OTHER: &[u8] = br#"!multiple-choice
id: other
name: Other content
lang: de
options:
  - id: 0
    name: Tschuess
questions:
  - question: Abschied?
    help: Say goodbye
    option: 0
"#;
static FILES: &[(&str, &[u8])] = &[
    ("challenges/level_b.yml", LEVEL_B),
    ("challenges/level_a.yml", LEVEL_A),
    ("challenges/shared.yml", SHARED),
    ("challenges/other.yml", OTHER),
];

struct MemorySource {
    files: RefCell<HashMap<String, Vec<u8>>>,
    requests: RefCell<Vec<String>>,
}
impl MemorySource {
    fn new() -> Self {
        Self {
            files: RefCell::new(
                FILES
                    .iter()
                    .map(|(path, bytes)| (path.to_string(), bytes.to_vec()))
                    .collect(),
            ),
            requests: RefCell::default(),
        }
    }
    fn count(&self, path: &str) -> usize {
        self.requests
            .borrow()
            .iter()
            .filter(|request| request.as_str() == path)
            .count()
    }
}
impl AssetSource for MemorySource {
    async fn load_bytes(&self, path: &str) -> Result<Vec<u8>, AssetError> {
        self.requests.borrow_mut().push(path.to_owned());
        self.files
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| AssetError::NotFound {
                path: path.to_owned(),
            })
    }
}
fn manifest() -> Manifest<NoExtensions> {
    Manifest {
        game_paths: vec!["level_b.yml".into(), "level_a.yml".into()],
        ..Default::default()
    }
}
fn fixture() -> (GameLoader<MemorySource>, Rc<MemorySource>) {
    let source = Rc::new(MemorySource::new());
    let loader = GameLoader::with_server(&manifest(), AssetServer::new(source.clone()));
    (loader, source)
}

#[test]
fn metadata_loading_preserves_manifest_order_without_loading_datasets() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        assert_eq!(loader.level_paths(), manifest().game_paths);
        let levels = loader.load_game_paths().await?;
        assert_eq!(
            levels
                .iter()
                .map(|level| level.id.as_str())
                .collect::<Vec<_>>(),
            ["level-b", "level-a"]
        );
        assert_eq!(
            *source.requests.borrow(),
            ["challenges/level_b.yml", "challenges/level_a.yml"]
        );
        assert_eq!(source.count("challenges/shared.yml"), 0);
        Ok(())
    })
}

#[test]
fn one_dataset_load_does_not_load_levels_and_is_shared_across_clones() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        let first = loader.load_challenge("shared").await?;
        let second = loader.clone().load_challenge("shared").await?;
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(*source.requests.borrow(), ["challenges/shared.yml"]);
        loader.server().invalidate("challenges/shared.yml");
        let fresh = loader.load_challenge("shared").await?;
        assert!(!Arc::ptr_eq(&first, &fresh));
        assert_eq!(source.count("challenges/shared.yml"), 2);
        Ok(())
    })
}

#[test]
fn factory_deduplicates_nonadjacent_references_across_levels() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        let levels = loader.load_game_paths().await?;
        let factory = loader.preload_factory(&levels).await?;
        assert_eq!(
            factory
                .challenge_types
                .iter()
                .map(|dataset| dataset.id())
                .collect::<Vec<_>>(),
            ["shared", "other"]
        );
        assert_eq!(source.count("challenges/shared.yml"), 1);
        assert_eq!(source.count("challenges/other.yml"), 1);
        assert!(factory.create_challenge(&levels[0].challenges[0]).is_ok());
        assert!(factory.create_challenge(&levels[1].challenges[0]).is_ok());
        Ok(())
    })
}

#[test]
fn preloaded_games_are_independent_and_do_not_include_default_content() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        let mut first = loader.preload_game().await?;
        first.xp = 100;
        first.challenge_factory.challenge_types.clear();
        let second = loader.preload_game().await?;
        assert_eq!(second.xp, 0);
        assert_eq!(second.challenge_factory.challenge_types.len(), 2);
        assert!(second.create_challenge("shared-first").is_ok());
        assert!(second.create_challenge("shared-again").is_ok());
        assert!(
            source
                .requests
                .borrow()
                .iter()
                .all(|path| path.starts_with("challenges/"))
        );
        for (path, _) in FILES {
            assert_eq!(source.count(path), 1);
        }
        Ok(())
    })
}

#[test]
fn missing_reference_reports_level_entry_and_asset_path_then_can_retry() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        source.files.borrow_mut().remove("challenges/shared.yml");
        let error = loader
            .preload_game()
            .await
            .err()
            .ok_or("expected missing reference error")?;
        let GameLoadError::Reference {
            level_id,
            entry_id,
            source: error,
        } = error
        else {
            return Err("expected reference context".into());
        };
        assert_eq!(level_id, "level-b");
        assert_eq!(entry_id, "shared-first");
        assert!(
            matches!(*error, GameLoadError::Asset(AssetLoadError::NotFound { path }) if path == "challenges/shared.yml")
        );
        source
            .files
            .borrow_mut()
            .insert("challenges/shared.yml".to_owned(), SHARED.to_vec());
        assert!(loader.preload_game().await.is_ok());
        assert_eq!(source.count("challenges/shared.yml"), 2);
        Ok(())
    })
}

#[test]
fn wrong_dataset_id_is_rejected_and_does_not_poison_the_cache() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        source
            .files
            .borrow_mut()
            .insert("challenges/shared.yml".to_owned(), OTHER.to_vec());
        assert!(matches!(loader.load_challenge("shared").await,
            Err(GameLoadError::IdMismatch { path, expected, actual })
            if path == "challenges/shared.yml" && expected == "shared" && actual == "other"));
        source
            .files
            .borrow_mut()
            .insert("challenges/shared.yml".to_owned(), SHARED.to_vec());
        assert_eq!(loader.load_challenge("shared").await?.id(), "shared");
        assert_eq!(source.count("challenges/shared.yml"), 2);
        Ok(())
    })
}

#[test]
fn malformed_level_reports_parse_path_and_can_be_fixed() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        source.files.borrow_mut().insert(
            "challenges/level_b.yml".to_owned(),
            b"[not a level".to_vec(),
        );
        assert!(matches!(loader.load_game_paths().await,
            Err(GameLoadError::Asset(AssetLoadError::Parse { path, .. })) if path == "challenges/level_b.yml"));
        source
            .files
            .borrow_mut()
            .insert("challenges/level_b.yml".to_owned(), LEVEL_B.to_vec());
        assert_eq!(loader.load_game_paths().await?.len(), 2);
        Ok(())
    })
}

#[test]
fn legacy_filename_resolution_keeps_content_id_validation() -> TestResult {
    block_on(async {
        let (loader, source) = fixture();
        source
            .files
            .borrow_mut()
            .insert("legacy/shared_content.yaml".into(), SHARED.to_vec());
        let loader = loader.with_challenge_path_resolver(|id| format!("legacy/{id}_content.yaml"));
        assert_eq!(loader.load_challenge("shared").await?.id(), "shared");
        assert_eq!(*source.requests.borrow(), ["legacy/shared_content.yaml"]);
        Ok(())
    })
}

#[test]
fn empty_catalogue_does_not_inject_embedded_defaults() -> TestResult {
    block_on(async {
        let source = Rc::new(MemorySource::new());
        let loader = GameLoader::with_server(
            &Manifest::<NoExtensions>::default(),
            AssetServer::new(source.clone()),
        );
        let game = loader.preload_game().await?;
        assert!(game.game_paths.is_empty());
        assert!(game.challenge_factory.challenge_types.is_empty());
        assert!(source.requests.borrow().is_empty());
        Ok(())
    })
}

#[test]
fn embedded_file_and_combined_sources_produce_equivalent_games() -> TestResult {
    block_on(async {
        let root = tempfile::tempdir()?;
        std::fs::create_dir(root.path().join("challenges"))?;
        for (path, bytes) in FILES {
            std::fs::write(root.path().join(path), bytes)?;
        }
        let mut manifest = manifest();
        manifest.assets.path = root.path().to_string_lossy().into_owned();
        let expected: Game = GameLoader::from_manifest(&manifest, EmbeddedSource::new(FILES))
            .preload_game()
            .await?;
        let file_game = GameLoader::from_manifest(&manifest, FileSource::from_dir(root.path()))
            .preload_game()
            .await?;
        assert_eq!(expected, file_game);
        let source = LayeredSource::default()
            .with_source(EmbeddedSource::new(&[("challenges/shared.yml", SHARED)]))
            .with_source(FileSource::from_dir(root.path()));
        let combined = GameLoader::from_manifest(&manifest, source)
            .preload_game()
            .await?;
        assert_eq!(expected, combined);
        Ok(())
    })
}

#[test]
fn custom_manifest_extensions_are_not_part_of_the_loader_type() -> TestResult {
    konnektoren_platform::manifest_extensions! {
        pub struct MusicSections { pub theme: String }
    }
    block_on(async {
        let manifest = Manifest::<MusicSections> {
            game_paths: manifest().game_paths,
            ext: MusicSections {
                theme: "music".into(),
            },
            ..Default::default()
        };
        let game = GameLoader::from_manifest(&manifest, EmbeddedSource::new(FILES))
            .preload_game()
            .await?;
        assert_eq!(game.game_paths.len(), 2);
        assert_eq!(manifest.ext.theme, "music");
        Ok(())
    })
}
