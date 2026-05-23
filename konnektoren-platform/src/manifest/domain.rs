use crate::domain::DomainConfig;
use serde::{Deserialize, Serialize};

/// A serializable domain configuration that can be declared in a manifest YAML.
///
/// Implements [`DomainConfig`] so it can be used directly wherever the trait
/// is required — no extra adapter needed.
///
/// ```yaml
/// domain:
///   code: konnektoren-de
///   name: Konnektoren
///   base_path: /
///   icon: "🎓"
///   hostname: konnektoren.help
///   description: German grammar learning platform
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DomainManifest {
    /// Unique identifier for this deployment, e.g. `"konnektoren-de"`.
    pub code: String,
    /// Human-readable name shown in the UI.
    pub name: String,
    /// Base path for routing, e.g. `"/"` or `"/de"`.
    #[serde(default = "default_base_path")]
    pub base_path: String,
    /// Emoji or icon identifier, e.g. `"🎓"`.
    #[serde(default = "default_icon")]
    pub icon: String,
    /// Fully-qualified hostname of the deployment, e.g. `"konnektoren.help"`.
    #[serde(default = "default_hostname")]
    pub hostname: String,
    /// Optional description shown on about/landing pages.
    #[serde(default)]
    pub description: Option<String>,
}

fn default_base_path() -> String {
    "/".to_string()
}

fn default_icon() -> String {
    "🎓".to_string()
}

fn default_hostname() -> String {
    "localhost".to_string()
}

impl DomainConfig for DomainManifest {
    fn code(&self) -> &str {
        &self.code
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn base_path(&self) -> &str {
        &self.base_path
    }

    fn icon(&self) -> &str {
        &self.icon
    }

    fn hostname(&self) -> &str {
        &self.hostname
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML: &str = r#"
code: konnektoren-de
name: Konnektoren
base_path: /
icon: "🎓"
hostname: konnektoren.help
description: German grammar learning
"#;

    #[test]
    fn test_deserializes_from_yaml() {
        let d: DomainManifest = serde_yaml::from_str(YAML).unwrap();
        assert_eq!(d.code(), "konnektoren-de");
        assert_eq!(d.name(), "Konnektoren");
        assert_eq!(d.base_path(), "/");
        assert_eq!(d.icon(), "🎓");
        assert_eq!(d.hostname(), "konnektoren.help");
        assert_eq!(d.description(), Some("German grammar learning"));
    }

    #[test]
    fn test_optional_fields_default() {
        let yaml = "code: test\nname: Test\n";
        let d: DomainManifest = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(d.base_path(), "/");
        assert_eq!(d.icon(), "🎓");
        assert_eq!(d.hostname(), "localhost");
        assert_eq!(d.description(), None);
    }

    #[test]
    fn test_implements_domain_config() {
        let d: DomainManifest = serde_yaml::from_str(YAML).unwrap();
        // Verify DomainConfig methods compile and return expected values
        assert_eq!(DomainConfig::code(&d), "konnektoren-de");
        assert_eq!(DomainConfig::hostname(&d), "konnektoren.help");
    }
}
