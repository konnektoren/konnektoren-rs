use super::Manifest;
use serde::{Deserialize, Serialize};

/// Konnektoren-platform metadata — the concrete `M` for `Manifest<M>`.
///
/// Add this to the `metadata:` section of a manifest YAML to tell
/// the loader which konnektoren challenge file to load in addition
/// to the regular `game_paths`.
///
/// ```yaml
/// package:
///   id: konnektoren-de
///   version: "1.0.0"
///   name: Konnektoren
///
/// game_paths:
///   - level_a1.yml
///   - level_a2.yml
///
/// metadata:
///   konnektoren_file: challenges/konnektoren.yml
/// ```
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct KonnektorenMeta {
    /// Path to a konnektoren challenge YAML file, resolved relative to
    /// `assets.path`.  When absent the loader uses whatever its built-in
    /// default is (typically `konnektoren.yml`).
    #[serde(default)]
    pub konnektoren_file: Option<String>,
}

/// Convenience alias — a `Manifest` typed for the Konnektoren platform.
pub type KonnektorenManifest = Manifest<KonnektorenMeta>;

impl KonnektorenManifest {
    /// Returns the konnektoren file path if specified in `metadata`,
    /// otherwise falls back to `"konnektoren.yml"`.
    pub fn konnektoren_file(&self) -> &str {
        self.metadata
            .konnektoren_file
            .as_deref()
            .unwrap_or("konnektoren.yml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::ManifestConfig;

    const YAML_WITH_FILE: &str = r#"
package:
  id: konnektoren-de
  version: "1.0.0"
  name: Konnektoren

game_paths:
  - level_a1.yml
  - level_a2.yml

metadata:
  konnektoren_file: challenges/konnektoren.yml
"#;

    const YAML_WITHOUT_FILE: &str = r#"
package:
  id: konnektoren-de
  version: "1.0.0"
  name: Konnektoren

game_paths:
  - level_a1.yml
"#;

    #[test]
    fn test_konnektoren_file_explicit() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_WITH_FILE).unwrap();
        assert_eq!(manifest.konnektoren_file(), "challenges/konnektoren.yml");
    }

    #[test]
    fn test_konnektoren_file_default() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_WITHOUT_FILE).unwrap();
        assert_eq!(manifest.konnektoren_file(), "konnektoren.yml");
    }

    #[test]
    fn test_base_fields_still_accessible() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_WITH_FILE).unwrap();
        assert_eq!(manifest.package().id, "konnektoren-de");
        assert_eq!(manifest.game_paths(), &["level_a1.yml", "level_a2.yml"]);
    }
}
