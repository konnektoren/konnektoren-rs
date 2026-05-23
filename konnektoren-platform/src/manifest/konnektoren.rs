use super::Manifest;
use crate::manifest::domain::DomainManifest;
use crate::manifest::i18n::I18nManifest;

// Register DomainManifest and I18nManifest as named manifest sections.
// The KEY constant documents which top-level YAML key each section occupies.
crate::manifest_section!(DomainManifest, "domain");
crate::manifest_section!(I18nManifest, "i18n");

// The default extension bundle for the Konnektoren platform.
// Fields appear as top-level YAML keys, not nested under `metadata:`.
crate::manifest_extensions! {
    pub struct KonnektorenSections {
        pub domain: Option<DomainManifest>,
        pub i18n: I18nManifest
    }
}

/// Convenience alias — a [`Manifest`] typed for the Konnektoren platform.
pub type KonnektorenManifest = Manifest<KonnektorenSections>;

impl KonnektorenManifest {
    pub fn domain(&self) -> Option<&DomainManifest> {
        self.ext.domain.as_ref()
    }

    pub fn i18n(&self) -> &I18nManifest {
        &self.ext.i18n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::DomainConfig;
    use crate::manifest::ManifestConfig;
    use crate::manifest::extensions::ManifestSection;

    const YAML_WITH_ALL: &str = r#"
package:
  id: konnektoren-de
  version: "1.0.0"
  name: Konnektoren

game_paths:
  - level_a1.yml
  - level_a2.yml

domain:
  code: konnektoren-de
  name: Konnektoren
  hostname: konnektoren.help

i18n:
  default_language: de
  languages: [de, en, es]
"#;

    const YAML_MINIMAL: &str = r#"
package:
  id: konnektoren-de
  version: "1.0.0"
  name: Konnektoren

game_paths:
  - level_a1.yml
"#;

    #[test]
    fn test_domain_accessible() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_WITH_ALL).unwrap();
        let domain = manifest.domain().unwrap();
        assert_eq!(domain.hostname(), "konnektoren.help");
        assert_eq!(domain.code(), "konnektoren-de");
    }

    #[test]
    fn test_domain_absent_by_default() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_MINIMAL).unwrap();
        assert!(manifest.domain().is_none());
    }

    #[test]
    fn test_i18n_accessible() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_WITH_ALL).unwrap();
        assert_eq!(manifest.i18n().default_language, "de");
        assert_eq!(manifest.i18n().languages.len(), 3);
    }

    #[test]
    fn test_i18n_defaults_when_absent() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_MINIMAL).unwrap();
        // I18nManifest::default() is used when the key is absent
        assert_eq!(manifest.i18n().default_language, "en");
        assert_eq!(manifest.i18n().path, "assets/i18n");
    }

    #[test]
    fn test_section_keys() {
        assert_eq!(DomainManifest::KEY, "domain");
        assert_eq!(I18nManifest::KEY, "i18n");
    }

    #[test]
    fn test_base_fields_accessible_via_trait() {
        let manifest: KonnektorenManifest = serde_yaml::from_str(YAML_WITH_ALL).unwrap();
        assert_eq!(manifest.package().id, "konnektoren-de");
        assert_eq!(manifest.game_paths(), &["level_a1.yml", "level_a2.yml"]);
        assert_eq!(manifest.asset_path(), "challenges");
    }
}
