use super::{Domain, DomainConfig};
use serde::{Deserialize, Serialize};

/// Example implementation for language learning domains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LanguageDomainConfig {
    pub code: String,
    pub name: String,
    pub base_path: String,
    pub locale: String,
    pub icon: String,
    pub hostname: String,
}

impl DomainConfig for LanguageDomainConfig {
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
}

#[derive(Debug, Clone)]
pub struct LanguageDomain {
    config: LanguageDomainConfig,
}

impl LanguageDomain {
    pub fn new(code: &str, name: &str, locale: &str) -> Self {
        Self {
            config: LanguageDomainConfig {
                code: code.to_string(),
                name: name.to_string(),
                base_path: format!("/{}", code),
                locale: locale.to_string(),
                icon: match code {
                    "de" => "🇩🇪".to_string(),
                    "en" => "🇬🇧".to_string(),
                    "es" => "🇪🇸".to_string(),
                    _ => "🌐".to_string(),
                },
                hostname: match code {
                    "de" => "konnektoren.help".to_string(),
                    "en" => "en.konnektoren.help".to_string(),
                    "es" => "es.konnektoren.help".to_string(),
                    _ => "konnektoren.help".to_string(),
                },
            },
        }
    }

    pub fn with_icon(code: &str, name: &str, locale: &str, icon: &str) -> Self {
        let mut domain = Self::new(code, name, locale);
        domain.config.icon = icon.to_string();
        domain
    }

    pub fn with_hostname(code: &str, name: &str, locale: &str, hostname: &str) -> Self {
        let mut domain = Self::new(code, name, locale);
        domain.config.hostname = hostname.to_string();
        domain
    }
}

impl Domain for LanguageDomain {
    type Config = LanguageDomainConfig;

    fn config(&self) -> &Self::Config {
        &self.config
    }
}

impl From<LanguageDomainConfig> for LanguageDomain {
    fn from(config: LanguageDomainConfig) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::LanguageDomain;
    use serde_json;
    use serde_yaml;

    #[test]
    fn test_language_domain_json_serialization() {
        let config = LanguageDomainConfig {
            code: "de".to_string(),
            name: "Learn German".to_string(),
            base_path: "/de".to_string(),
            locale: "de-DE".to_string(),
            icon: "🇩🇪".to_string(),
            hostname: "konnektoren.help".to_string(),
        };

        let json = serde_json::to_string(&config).expect("Failed to serialize to JSON");
        let expected_json = r#"{"code":"de","name":"Learn German","base_path":"/de","locale":"de-DE","icon":"🇩🇪","hostname":"konnektoren.help"}"#;
        assert_eq!(json, expected_json);

        let deserialized: LanguageDomainConfig =
            serde_json::from_str(&json).expect("Failed to deserialize from JSON");
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_language_domain_yaml_serialization() {
        let domain = LanguageDomain::new("en", "Learn English", "en-US");

        let yaml = r#"
    - code: en
      name: Learn English
      base_path: /en
      locale: en-US
      icon: "🇬🇧"
      hostname: "en.konnektoren.help"
    - code: de
      name: Learn German
      base_path: /de
      locale: de-DE
      icon: "🇩🇪"
      hostname: "konnektoren.help"
    "#;

        let configs: Vec<LanguageDomainConfig> =
            serde_yaml::from_str(yaml).expect("Failed to deserialize YAML");

        assert_eq!(configs[0].code, domain.config().code);
        assert_eq!(configs[0].name, domain.config().name);
        assert_eq!(configs[0].base_path, domain.config().base_path);
        assert_eq!(configs[0].locale, domain.config().locale);
        assert_eq!(configs[0].icon, domain.config().icon);
        assert_eq!(configs[0].hostname, domain.config().hostname);

        let serialized = serde_yaml::to_string(&configs).expect("Failed to serialize to YAML");
        let deserialized: Vec<LanguageDomainConfig> =
            serde_yaml::from_str(&serialized).expect("Failed to deserialize YAML");
        assert_eq!(configs, deserialized);
    }

    #[test]
    fn test_language_domain_conversion() {
        let config = LanguageDomainConfig {
            code: "es".to_string(),
            name: "Learn Spanish".to_string(),
            base_path: "/es".to_string(),
            locale: "es-ES".to_string(),
            icon: "🇪🇸".to_string(),
            hostname: "es.konnektoren.help".to_string(),
        };

        let domain = LanguageDomain::from(config.clone());
        assert_eq!(domain.config(), &config);
    }

    #[test]
    fn test_invalid_yaml() {
        // Test with invalid YAML syntax (broken structure)
        let invalid_yaml = r#"
    - code: en
      name: Learn English  # Missing proper indentation
      base_path: /en
      locale: en-US
      icon: "🇬🇧"
      - invalid:     # Invalid nesting
    "#;

        let result: Result<Vec<LanguageDomainConfig>, _> = serde_yaml::from_str(invalid_yaml);
        assert!(result.is_err(), "Should fail with invalid YAML syntax");

        // Test with missing required field
        let missing_field_yaml = r#"
    - code: en
      name: Learn English
      base_path: /en
      locale: en-US
      # icon is missing
    "#;

        let result: Result<Vec<LanguageDomainConfig>, _> = serde_yaml::from_str(missing_field_yaml);
        assert!(result.is_err(), "Should fail with missing required field");

        // Test with invalid type
        let invalid_type_yaml = r#"
    - code: [this, is, an, array]  # should be string
      name: Learn English
      base_path: /en
      locale: en-US
      icon: "🇬🇧"
    "#;

        let result: Result<Vec<LanguageDomainConfig>, _> = serde_yaml::from_str(invalid_type_yaml);
        assert!(result.is_err(), "Should fail with invalid type");
    }

    #[test]
    fn test_language_domain_icons() {
        let german = LanguageDomain::new("de", "Learn German", "de-DE");
        assert_eq!(german.icon(), "🇩🇪");

        let english = LanguageDomain::new("en", "Learn English", "en-US");
        assert_eq!(english.icon(), "🇬🇧");

        let spanish = LanguageDomain::new("es", "Learn Spanish", "es-ES");
        assert_eq!(spanish.icon(), "🇪🇸");

        let other = LanguageDomain::new("fr", "Learn French", "fr-FR");
        assert_eq!(other.icon(), "🌐");

        let custom = LanguageDomain::with_icon("fr", "Learn French", "fr-FR", "🇫🇷");
        assert_eq!(custom.icon(), "🇫🇷");
    }

    #[test]
    fn test_language_domain_hostnames() {
        let german = LanguageDomain::new("de", "Learn German", "de-DE");
        assert_eq!(german.hostname(), "konnektoren.help");

        let english = LanguageDomain::new("en", "Learn English", "en-US");
        assert_eq!(english.hostname(), "en.konnektoren.help");

        let custom =
            LanguageDomain::with_hostname("fr", "Learn French", "fr-FR", "french.konnektoren.help");
        assert_eq!(custom.hostname(), "french.konnektoren.help");
    }
}
