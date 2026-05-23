use serde::{Deserialize, Serialize};

/// I18n configuration declared in a manifest YAML.
///
/// Tells loaders where translation files live, which languages are supported,
/// and which one to use when no preference is expressed.
///
/// ```yaml
/// i18n:
///   path: assets/i18n
///   default_language: en
///   languages: [de, en, es, pl, tr, uk, ar, zh]
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct I18nManifest {
    /// Path to the directory containing per-language JSON/YAML files,
    /// relative to the crate's asset root.
    #[serde(default = "default_i18n_path")]
    pub path: String,

    /// BCP-47 language code for the fallback / default language.
    #[serde(default = "default_language")]
    pub default_language: String,

    /// Ordered list of BCP-47 language codes the deployment supports.
    /// An empty list means "accept whatever files are found in `path`".
    #[serde(default)]
    pub languages: Vec<String>,
}

impl Default for I18nManifest {
    fn default() -> Self {
        Self {
            path: default_i18n_path(),
            default_language: default_language(),
            languages: vec![],
        }
    }
}

fn default_i18n_path() -> String {
    "i18n".to_string()
}

fn default_language() -> String {
    "en".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_YAML: &str = r#"
path: i18n
default_language: de
languages:
  - de
  - en
  - es
  - pl
  - tr
  - uk
  - ar
  - zh
"#;

    #[test]
    fn test_deserializes_full() {
        let i: I18nManifest = serde_yaml::from_str(FULL_YAML).unwrap();
        assert_eq!(i.path, "i18n");
        assert_eq!(i.default_language, "de");
        assert_eq!(i.languages.len(), 8);
        assert!(i.languages.contains(&"uk".to_string()));
    }

    #[test]
    fn test_defaults_when_empty() {
        let i: I18nManifest = serde_yaml::from_str("{}").unwrap();
        assert_eq!(i.path, "i18n");
        assert_eq!(i.default_language, "en");
        assert!(i.languages.is_empty());
    }

    #[test]
    fn test_default_impl() {
        let i = I18nManifest::default();
        assert_eq!(i.path, "i18n");
        assert_eq!(i.default_language, "en");
    }

    #[test]
    fn test_partial_override() {
        let yaml = "default_language: tr\n";
        let i: I18nManifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(i.default_language, "tr");
        assert_eq!(i.path, "i18n"); // still default
    }
}
