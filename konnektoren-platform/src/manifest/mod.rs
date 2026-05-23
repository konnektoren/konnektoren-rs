pub mod domain;
pub mod i18n;
pub mod konnektoren;

#[cfg(feature = "manifest")]
use figment2::Figment;
#[cfg(feature = "manifest")]
use figment2::providers::{Format, Serialized, Yaml};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub use domain::DomainManifest;
pub use i18n::I18nManifest;
pub use konnektoren::{KonnektorenManifest, KonnektorenMeta};

/// The default manifest shipped with this crate
/// (`assets/konnektoren.manifest.yml`).
///
/// Declared in `[package.metadata.konnektoren] manifest = ...` in `Cargo.toml`.
pub const DEFAULT_MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/konnektoren.manifest.yml"
));

/// Identity of a deployment — mirrors Cargo's `[package]`.
///
/// Required: `id` and `name`. All other fields are optional.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct Package {
    /// Unique identifier, e.g. `"konnektoren-de"`.
    pub id: String,
    /// SemVer version string, e.g. `"1.0.0"`.
    #[serde(default)]
    pub version: String,
    /// Human-readable display name, e.g. `"Konnektoren"`.
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

/// Where challenge/asset files live.
///
/// Analogous to path configuration in a Cargo build script.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Assets {
    /// Sub-path within the asset root where challenge files are stored.
    #[serde(default = "default_asset_path")]
    pub path: String,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            path: default_asset_path(),
        }
    }
}

fn default_asset_path() -> String {
    "challenges".to_string()
}

/// Base deployment manifest.
///
/// Follows the structure of `Cargo.toml`:
///
/// | Cargo                  | Manifest              | Purpose                     |
/// |------------------------|-----------------------|-----------------------------|
/// | `[package]`            | `package`             | Identity & metadata         |
/// | `[[bin]]` targets      | `game_paths`          | Ordered content to load     |
/// | `[package.metadata.*]` | `metadata`            | Tool-specific extension     |
///
/// ## Extension via `M`
///
/// `M` is the typed extension point — identical in concept to
/// `[package.metadata]` in `Cargo.toml`. Layers that don't need
/// typed metadata use `M = ()` (the default). Downstream crates
/// (bevy, web-game) define their own `M` and deserialize it from
/// the `metadata:` section without changing this struct.
///
/// ```yaml
/// # minimal — any consumer
/// package:
///   id: konnektoren-de
///   version: "1.0.0"
///   name: Konnektoren
///
/// game_paths:
///   - level_a1.yml
///   - level_a2.yml
///
/// # bevy consumer adds its own typed section here
/// metadata:
///   title_key: games.konnektoren.title
///   default_language: de
///   splash:
///     logo: images/logo.png
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(bound(
    deserialize = "M: Default + DeserializeOwned",
    serialize = "M: Serialize"
))]
pub struct Manifest<M = ()>
where
    M: Default + Serialize + DeserializeOwned,
{
    #[serde(default)]
    pub package: Package,

    #[serde(default)]
    pub assets: Assets,

    /// Domain configuration — hostname, routing base-path, icon, etc.
    /// Absent by default; set in `konnektoren.manifest.yml` for deployments
    /// that need it.
    #[serde(default)]
    pub domain: Option<DomainManifest>,

    /// Internationalisation configuration — where translation files live,
    /// which languages are supported, and the fallback language.
    #[serde(default)]
    pub i18n: I18nManifest,

    /// Ordered list of game-path filenames to load — analogous to `[[bin]]`
    /// targets in `Cargo.toml`. Each entry is resolved relative to `assets.path`.
    #[serde(default)]
    pub game_paths: Vec<String>,

    /// Tool/framework-specific configuration — like `[package.metadata]` in
    /// `Cargo.toml`. Ignored by layers that don't recognize it; consumed by
    /// those that do via a concrete `M`.
    #[serde(default)]
    pub metadata: M,
}

/// Protocol all manifests satisfy, enabling generic framework code.
///
/// Implement this on any concrete manifest type to make it usable
/// with loaders and validators that don't need to know about `M`.
pub trait ManifestConfig {
    fn package(&self) -> &Package;
    fn asset_path(&self) -> &str;
    fn game_paths(&self) -> &[String];
    fn domain(&self) -> Option<&DomainManifest>;
    fn i18n(&self) -> &I18nManifest;
}

impl<M> ManifestConfig for Manifest<M>
where
    M: Default + Serialize + DeserializeOwned,
{
    fn package(&self) -> &Package {
        &self.package
    }

    fn asset_path(&self) -> &str {
        &self.assets.path
    }

    fn game_paths(&self) -> &[String] {
        &self.game_paths
    }

    fn domain(&self) -> Option<&DomainManifest> {
        self.domain.as_ref()
    }

    fn i18n(&self) -> &I18nManifest {
        &self.i18n
    }
}

#[cfg(feature = "manifest")]
impl<M> Manifest<M>
where
    M: Default + Serialize + DeserializeOwned,
{
    /// Build a [`Figment`] layering `default_package` and i18n defaults under
    /// the given YAML.
    ///
    /// Fields present in `yaml` override the defaults; absent fields fall
    /// back to `default_package` — which should be constructed with the
    /// [`cargo_package!`] macro so `env!("CARGO_PKG_*")` resolves in the
    /// *calling* crate rather than in `konnektoren-platform`.
    ///
    /// ```rust,ignore
    /// let manifest: KonnektorenManifest = Manifest::figment(
    ///     cargo_package!(),
    ///     include_str!("../assets/konnektoren.manifest.yml"),
    /// )
    /// .extract()?;
    /// ```
    pub fn figment(default_package: Package, yaml: &str) -> Figment {
        Figment::new()
            .merge(Serialized::default("package", &default_package))
            .merge(Serialized::default("i18n", &I18nManifest::default()))
            .merge(Yaml::string(yaml))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_YAML: &str = r#"
package:
  id: test-deployment
  version: "1.0.0"
  name: Test

game_paths:
  - level_a1.yml
  - level_a2.yml
"#;

    const WITH_METADATA_YAML: &str = r#"
package:
  id: bevy-game
  version: "0.1.0"
  name: My Bevy Game
  description: A bevy game manifest with typed metadata

assets:
  path: assets/challenges

game_paths:
  - level_a1.yml

metadata:
  title_key: games.my_game.title
  default_language: de
"#;

    #[test]
    fn test_minimal_manifest_deserializes() {
        let manifest: Manifest = serde_yaml::from_str(MINIMAL_YAML).unwrap();
        assert_eq!(manifest.package.id, "test-deployment");
        assert_eq!(manifest.game_paths, vec!["level_a1.yml", "level_a2.yml"]);
        assert_eq!(manifest.assets.path, "challenges"); // default
        assert_eq!(manifest.metadata, ());
    }

    #[test]
    fn test_manifest_with_typed_metadata() {
        #[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
        struct BevyMeta {
            title_key: String,
            #[serde(default = "default_lang")]
            default_language: String,
        }
        fn default_lang() -> String {
            "en".to_string()
        }

        let manifest: Manifest<BevyMeta> = serde_yaml::from_str(WITH_METADATA_YAML).unwrap();

        assert_eq!(manifest.package.id, "bevy-game");
        assert_eq!(manifest.assets.path, "assets/challenges");
        assert_eq!(manifest.metadata.title_key, "games.my_game.title");
        assert_eq!(manifest.metadata.default_language, "de");
    }

    #[test]
    fn test_manifest_config_trait() {
        let manifest: Manifest = serde_yaml::from_str(MINIMAL_YAML).unwrap();
        assert_eq!(manifest.package().id, "test-deployment");
        assert_eq!(manifest.asset_path(), "challenges");
        assert_eq!(manifest.game_paths().len(), 2);
    }

    #[test]
    fn test_default_manifest_parses() {
        // DEFAULT_MANIFEST has no [package] — those come from cargo_package!().
        // Without the manifest feature we can still parse the non-package fields.
        let manifest: KonnektorenManifest =
            serde_yaml::from_str(DEFAULT_MANIFEST).expect("DEFAULT_MANIFEST must be valid YAML");
        assert!(!manifest.game_paths.is_empty());
        assert_eq!(manifest.konnektoren_file(), "konnektoren.yml");
    }

    #[cfg(feature = "manifest")]
    #[test]
    fn test_figment_fills_package_from_defaults() {
        let default_package = Package {
            id: "my-game".to_string(),
            version: "2.0.0".to_string(),
            name: "My Game".to_string(),
            description: Some("A test game".to_string()),
            ..Default::default()
        };

        // YAML overrides only `assets.path`; package comes from defaults.
        let yaml = r#"
assets:
  path: custom/challenges
game_paths:
  - level_a1.yml
"#;

        let manifest: Manifest<()> = Manifest::<()>::figment(default_package, yaml)
            .extract()
            .expect("figment extraction must succeed");

        assert_eq!(manifest.package.id, "my-game");
        assert_eq!(manifest.package.version, "2.0.0");
        assert_eq!(manifest.assets.path, "custom/challenges");
        assert_eq!(manifest.game_paths, vec!["level_a1.yml"]);
    }
}
