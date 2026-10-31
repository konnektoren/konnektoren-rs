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

mod external_formats {
    use super::*;
    use konnektoren_asset_loader::AssetFormat;
    use konnektoren_core::challenges::{
        ChallengeConfig, ChallengeDefinition, ChallengeFamily, task_pattern::TaskPattern,
    };
    use konnektoren_platform::game_loader::{
        ChallengeFamilyFormat, FamilyFormatError, LevelDefinition, TaggedYaml, Yaml,
    };
    use serde::Deserialize;

    #[derive(Clone, Deserialize)]
    struct ExternalLevel {
        id: String,
        description: String,
        entries: Vec<(String, String)>,
    }
    impl LevelDefinition for ExternalLevel {
        fn id(&self) -> &str {
            &self.id
        }
        fn challenge_references(&self) -> impl Iterator<Item = (&str, &str)> {
            self.entries
                .iter()
                .map(|(entry, content)| (entry.as_str(), content.as_str()))
        }
    }

    #[derive(Clone, Deserialize)]
    struct ExternalContent {
        id: String,
        value: u32,
    }
    impl ChallengeDefinition for ExternalContent {
        type Result = Vec<bool>;
        fn id(&self) -> &str {
            &self.id
        }
        fn of_tasks(&self, _: &TaskPattern) -> Self {
            self.clone()
        }
    }

    #[test]
    fn one_factory_loads_builtin_and_external_challenges() -> TestResult {
        block_on(async {
            const MIXED_FILES: &[(&str, &[u8])] = &[
                (
                    "challenges/mixed.yml",
                    b"id: mixed\ndescription: Mixed content\nentries: [[first, shared], [second, extra]]",
                ),
                ("challenges/shared.yml", SHARED),
                ("challenges/extra.yml", b"!example\nid: extra\nvalue: 5"),
                ("challenges/broken.yml", b"!example\nid: broken\nvalue: invalid"),
                ("challenges/unknown.yml", b"!unknown\nid: unknown"),
            ];
            let manifest: Manifest<NoExtensions> = serde_yaml::from_str("game_paths: [mixed.yml]")?;
            let format = ChallengeFamilyFormat::new(
                Yaml::<konnektoren_core::challenges::ChallengeType>::new(),
                TaggedYaml::<ExternalContent>::new(["example"]),
            )?;
            let loader = GameLoader::from_manifest(&manifest, EmbeddedSource::new(MIXED_FILES))
                .with_formats(Yaml::<ExternalLevel>::new(), format);
            loader.validate_assets().await?;
            let factory = loader
                .preload_factory(&loader.load_game_paths().await?)
                .await?;
            assert_eq!(factory.challenge_types.len(), 2);
            for (id, is_core) in [("shared", true), ("extra", false)] {
                let config = ChallengeConfig {
                    challenge: id.into(),
                    ..ChallengeConfig::default()
                };
                let challenge = factory.create_challenge(&config)?;
                assert_eq!(challenge.challenge_type.id(), id);
                assert_eq!(
                    matches!(challenge.challenge_type, ChallengeFamily::First(_)),
                    is_core
                );
                assert!(challenge.challenge_result.is_none());
            }
            for id in ["broken", "unknown"] {
                assert!(matches!(
                    loader.load_challenge(id).await,
                    Err(GameLoadError::Asset(AssetLoadError::Parse { .. }))
                ));
            }
            Ok(())
        })
    }

    #[test]
    fn family_formats_compose_and_reject_conflicting_or_invalid_types() -> TestResult {
        let core = Yaml::<konnektoren_core::challenges::ChallengeType>::new();
        assert!(matches!(
            ChallengeFamilyFormat::new(core.clone(), TaggedYaml::<ExternalContent>::new(["multiple-choice"])),
            Err(FamilyFormatError::DuplicateOrInvalidType(name)) if name == "multiple-choice"
        ));
        assert!(matches!(
            ChallengeFamilyFormat::new(TaggedYaml::<ExternalContent>::new(["same", "same"]), core.clone()),
            Err(FamilyFormatError::DuplicateOrInvalidType(name)) if name == "same"
        ));
        let inner =
            ChallengeFamilyFormat::new(core, TaggedYaml::<ExternalContent>::new(["example"]))?;
        let nested =
            ChallengeFamilyFormat::new(inner, TaggedYaml::<ExternalContent>::new(["more"]))?;
        assert!(matches!(
            nested.parse(b"!more\nid: other\nvalue: 7")?,
            ChallengeFamily::Second(_)
        ));
        assert!(matches!(
            nested.parse(b"!example\nid: other\nvalue: 7")?,
            ChallengeFamily::First(ChallengeFamily::Second(_))
        ));
        assert!(matches!(
            nested.parse(SHARED)?,
            ChallengeFamily::First(ChallengeFamily::First(_))
        ));
        assert!(matches!(
            nested.parse(b"!example\nid: other\nvalue: invalid"),
            Err(FamilyFormatError::Parse { .. })
        ));
        assert!(matches!(
            nested.parse(b"!unknown\nid: other"),
            Err(FamilyFormatError::UnknownType(_))
        ));
        assert!(matches!(
            nested.parse(b"id: other"),
            Err(FamilyFormatError::MissingType)
        ));
        Ok(())
    }

    #[derive(Clone)]
    struct ContentFormat {
        minimum: u32,
    }
    impl AssetFormat for ContentFormat {
        type Asset = ExternalContent;
        type Error = serde_json::Error;
        fn cache_key(&self) -> Option<String> {
            Some(self.minimum.to_string())
        }
        fn parse(&self, bytes: &[u8]) -> Result<Self::Asset, Self::Error> {
            let content: ExternalContent = serde_json::from_slice(bytes)?;
            if content.value < self.minimum {
                return Err(serde::de::Error::custom("value below configured minimum"));
            }
            Ok(content)
        }
    }

    const FILES: &[(&str, &[u8])] = &[
        ("challenges/external.yml", b"id: external\ndescription: App metadata\nentries: [[first, shared], [second, shared]]"),
        ("content/shared.json", br#"{"id":"shared","value":5}"#),
        ("content/wrong.json", br#"{"id":"different","value":5}"#),
    ];

    #[test]
    fn app_formats_preserve_metadata_and_preload_a_typed_factory() -> TestResult {
        block_on(async {
            let manifest: Manifest<NoExtensions> =
                serde_yaml::from_str("game_paths: [external.yml]")?;
            let loader = GameLoader::from_manifest(&manifest, EmbeddedSource::new(FILES))
                .with_challenge_path_resolver(|id| format!("content/{id}.json"))
                .with_formats(Yaml::<ExternalLevel>::new(), ContentFormat { minimum: 0 });
            let levels = loader.load_game_paths().await?;
            assert_eq!(levels[0].description, "App metadata");
            let factory = loader.preload_factory(&levels).await?;
            assert_eq!(factory.challenge_types.len(), 1);
            let config = ChallengeConfig {
                challenge: "shared".into(),
                ..ChallengeConfig::default()
            };
            let challenge = factory.create_challenge(&config)?;
            assert_eq!(challenge.challenge_type.value, 5);
            assert!(challenge.challenge_result.is_none());
            let first = loader.load_challenge("shared").await?;
            assert!(Arc::ptr_eq(
                &first,
                &loader.clone().load_challenge("shared").await?
            ));
            assert!(matches!(
                loader.load_challenge("wrong").await,
                Err(GameLoadError::IdMismatch { .. })
            ));
            Ok(())
        })
    }

    #[test]
    fn configured_format_cache_and_reference_errors_remain_distinct() -> TestResult {
        block_on(async {
            let manifest: Manifest<NoExtensions> =
                serde_yaml::from_str("game_paths: [external.yml]")?;
            let loader = GameLoader::from_manifest(&manifest, EmbeddedSource::new(FILES))
                .with_formats(Yaml::<ExternalLevel>::new(), ContentFormat { minimum: 0 })
                .with_challenge_path_resolver(|id| format!("content/{id}.json"));
            loader.load_challenge("shared").await?;
            let strict =
                loader.with_formats(Yaml::<ExternalLevel>::new(), ContentFormat { minimum: 10 });
            match strict.validate_assets().await {
                Err(GameLoadError::Reference {
                    level_id,
                    entry_id,
                    source,
                }) => {
                    assert_eq!(level_id, "external");
                    assert_eq!(entry_id, "first");
                    assert!(matches!(
                        *source,
                        GameLoadError::Asset(AssetLoadError::Parse { .. })
                    ));
                }
                _ => panic!("strict format must reject cached content parsed by lenient format"),
            }
            Ok(())
        })
    }
}
