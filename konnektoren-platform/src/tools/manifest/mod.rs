use std::fmt;
use std::marker::PhantomData;

use crate::manifest::{Manifest, ManifestExtensions, Package};
use serde::Serialize;

/// Error returned by [`ManifestBuilder::build`] or [`ManifestExporter::to_yaml`].
#[derive(Debug)]
pub enum ManifestToolError {
    /// figment failed to merge or extract the layers.
    #[cfg(feature = "manifest")]
    Figment(figment2::Error),
    /// serde_yaml failed to serialize the resolved manifest.
    Yaml(serde_yaml::Error),
}

#[cfg(feature = "manifest")]
impl From<figment2::Error> for ManifestToolError {
    fn from(e: figment2::Error) -> Self {
        Self::Figment(e)
    }
}

impl From<serde_yaml::Error> for ManifestToolError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e)
    }
}

impl fmt::Display for ManifestToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "manifest")]
            Self::Figment(e) => write!(f, "manifest merge error: {e}"),
            Self::Yaml(e) => write!(f, "yaml serialization error: {e}"),
        }
    }
}

/// Builds a fully resolved [`Manifest`] by layering YAML strings over base
/// defaults.
///
/// Later [`layer`](ManifestBuilder::layer) calls win over earlier ones, and all
/// layers win over the Cargo-derived package defaults — identical to how
/// [`load_manifest!`] works at compile time.
///
/// Requires the `manifest` feature.
///
/// # Example
///
/// ```rust,ignore
/// use konnektoren_platform::tools::manifest::ManifestBuilder;
/// use konnektoren_platform::manifest::{KonnektorenManifest, KonnektorenSections};
/// use konnektoren_platform::cargo_package;
///
/// let manifest: KonnektorenManifest = ManifestBuilder::new(cargo_package!())
///     .layer(include_str!("../assets/konnektoren.manifest.yml"))
///     .layer("domain:\n  hostname: staging.example.com\n")
///     .build()?;
/// ```
#[cfg(feature = "manifest")]
pub struct ManifestBuilder<Ext = crate::manifest::KonnektorenSections>
where
    Ext: ManifestExtensions + Serialize,
{
    package: Package,
    layers: Vec<String>,
    _ext: PhantomData<Ext>,
}

#[cfg(feature = "manifest")]
impl<Ext> ManifestBuilder<Ext>
where
    Ext: ManifestExtensions + Serialize + std::fmt::Debug + Clone + PartialEq,
{
    /// Start with package fields from `package` (use [`cargo_package!`] to
    /// populate from `Cargo.toml`) and no YAML layers.
    pub fn new(package: Package) -> Self {
        Self {
            package,
            layers: Vec::new(),
            _ext: PhantomData,
        }
    }

    /// Add a YAML layer. Each call appends one more overlay; later calls take
    /// precedence over earlier ones.
    pub fn layer(mut self, yaml: impl Into<String>) -> Self {
        self.layers.push(yaml.into());
        self
    }

    /// Merge all layers and return the resolved manifest.
    pub fn build(self) -> Result<Manifest<Ext>, ManifestToolError> {
        use figment2::Figment;
        use figment2::providers::{Format, Serialized, Yaml};

        let defaults = Manifest::<Ext> {
            package: self.package,
            ..Default::default()
        };

        let mut figment = Figment::new().merge(Serialized::defaults(defaults));
        for layer in &self.layers {
            figment = figment.merge(Yaml::string(layer));
        }

        Ok(figment.extract()?)
    }
}

/// Serializes a fully resolved [`Manifest`] back to a YAML string.
///
/// The output is the **complete merged state** — every field at its resolved
/// value — suitable for saving, diffing, or embedding as a canonical snapshot.
///
/// # Example
///
/// ```rust,ignore
/// use konnektoren_platform::tools::manifest::{ManifestBuilder, ManifestExporter};
/// use konnektoren_platform::cargo_package;
///
/// let manifest = ManifestBuilder::new(cargo_package!())
///     .layer(include_str!("../assets/konnektoren.manifest.yml"))
///     .build()?;
///
/// let yaml = ManifestExporter::new().to_yaml(&manifest)?;
/// println!("{yaml}");
/// ```
#[derive(Debug, Default)]
pub struct ManifestExporter;

impl ManifestExporter {
    pub fn new() -> Self {
        Self
    }

    /// Serialize `manifest` to a YAML string.
    pub fn to_yaml<Ext>(&self, manifest: &Manifest<Ext>) -> Result<String, ManifestToolError>
    where
        Ext: ManifestExtensions + Serialize,
    {
        Ok(serde_yaml::to_string(manifest)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{KonnektorenManifest, KonnektorenSections, NoExtensions};

    fn base_package() -> Package {
        Package {
            id: "test-game".into(),
            version: "1.0.0".into(),
            name: "Test Game".into(),
            ..Default::default()
        }
    }

    // ── ManifestExporter ─────────────────────────────────────────────────────

    #[test]
    fn exporter_serializes_package_fields() {
        let manifest = KonnektorenManifest::default();
        let yaml = ManifestExporter::new().to_yaml(&manifest).unwrap();
        assert!(yaml.contains("package"));
    }

    #[test]
    fn exporter_round_trips() {
        let yaml_in = r#"
package:
  id: round-trip
  name: Round Trip
game_paths:
  - level_a1.yml
i18n:
  default_language: de
"#;
        let manifest: KonnektorenManifest = serde_yaml::from_str(yaml_in).unwrap();
        let yaml_out = ManifestExporter::new().to_yaml(&manifest).unwrap();

        let manifest2: KonnektorenManifest = serde_yaml::from_str(&yaml_out).unwrap();
        assert_eq!(manifest, manifest2);
    }

    #[test]
    fn exporter_works_with_no_extensions() {
        let manifest = Manifest::<NoExtensions>::default();
        let yaml = ManifestExporter::new().to_yaml(&manifest).unwrap();
        assert!(yaml.contains("package"));
    }

    // ── ManifestBuilder ──────────────────────────────────────────────────────

    #[cfg(feature = "manifest")]
    #[test]
    fn builder_no_layers_returns_defaults() {
        let manifest: Manifest<NoExtensions> =
            ManifestBuilder::new(base_package()).build().unwrap();

        assert_eq!(manifest.package.id, "test-game");
        assert_eq!(manifest.assets.path, "challenges");
        assert!(manifest.game_paths.is_empty());
    }

    #[cfg(feature = "manifest")]
    #[test]
    fn builder_single_layer_overrides_defaults() {
        let yaml = r#"
assets:
  path: custom/challenges
game_paths:
  - level_a1.yml
  - level_a2.yml
"#;
        let manifest: Manifest<NoExtensions> = ManifestBuilder::new(base_package())
            .layer(yaml)
            .build()
            .unwrap();

        assert_eq!(manifest.assets.path, "custom/challenges");
        assert_eq!(manifest.game_paths.len(), 2);
        // package still comes from base_package()
        assert_eq!(manifest.package.id, "test-game");
    }

    #[cfg(feature = "manifest")]
    #[test]
    fn builder_later_layer_wins_over_earlier() {
        let base_yaml = "assets:\n  path: base/challenges\n";
        let override_yaml = "assets:\n  path: override/challenges\n";

        let manifest: Manifest<NoExtensions> = ManifestBuilder::new(base_package())
            .layer(base_yaml)
            .layer(override_yaml)
            .build()
            .unwrap();

        assert_eq!(manifest.assets.path, "override/challenges");
    }

    #[cfg(feature = "manifest")]
    #[test]
    fn builder_with_konnektoren_sections() {
        let yaml = r#"
domain:
  code: test
  name: Test
  hostname: test.example.com
i18n:
  default_language: de
  languages: [de, en]
game_paths:
  - level_a1.yml
"#;
        let manifest: KonnektorenManifest =
            ManifestBuilder::<KonnektorenSections>::new(base_package())
                .layer(yaml)
                .build()
                .unwrap();

        assert_eq!(
            manifest.ext.domain.as_ref().unwrap().hostname,
            "test.example.com"
        );
        assert_eq!(manifest.ext.i18n.default_language, "de");
        assert_eq!(manifest.ext.i18n.languages.len(), 2);
    }

    #[cfg(feature = "manifest")]
    #[test]
    fn builder_then_exporter_produces_complete_yaml() {
        let yaml = r#"
domain:
  code: konnektoren-de
  name: Konnektoren
  hostname: konnektoren.help
i18n:
  default_language: de
  languages: [de, en]
game_paths:
  - level_a1.yml
"#;
        let manifest: KonnektorenManifest =
            ManifestBuilder::<KonnektorenSections>::new(base_package())
                .layer(yaml)
                .build()
                .unwrap();

        let output = ManifestExporter::new().to_yaml(&manifest).unwrap();

        // The output is the complete resolved state
        assert!(output.contains("konnektoren.help"));
        assert!(output.contains("default_language: de"));
        assert!(output.contains("test-game")); // from base_package()
        assert!(output.contains("level_a1.yml"));
    }
}
