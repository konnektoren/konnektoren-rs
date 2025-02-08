use super::{Domain, DomainConfig};
use serde::{Deserialize, Serialize};

/// Example implementation for language learning domains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageDomainConfig {
    code: String,
    name: String,
    base_path: String,
    locale: String,
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
            },
        }
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
        // Create a domain config
        let config = LanguageDomainConfig {
            code: "de".to_string(),
            name: "Learn German".to_string(),
            base_path: "/de".to_string(),
            locale: "de-DE".to_string(),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&config).expect("Failed to serialize to JSON");

        // Expected JSON structure
        let expected_json =
            r#"{"code":"de","name":"Learn German","base_path":"/de","locale":"de-DE"}"#;
        assert_eq!(json, expected_json);

        // Deserialize from JSON
        let deserialized: LanguageDomainConfig =
            serde_json::from_str(&json).expect("Failed to deserialize from JSON");
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_language_domain_yaml_serialization() {
        // Create a domain
        let domain = LanguageDomain::new("en", "Learn English", "en-US");

        // Create YAML string with multiple domains
        let yaml = r#"
- code: en
  name: Learn English
  base_path: /en
  locale: en-US
- code: de
  name: Learn German
  base_path: /de
  locale: de-DE
"#;

        // Deserialize YAML to vec of configs
        let configs: Vec<LanguageDomainConfig> =
            serde_yaml::from_str(yaml).expect("Failed to deserialize YAML");

        // Verify first config matches our domain
        assert_eq!(configs[0].code, domain.config().code);
        assert_eq!(configs[0].name, domain.config().name);
        assert_eq!(configs[0].base_path, domain.config().base_path);
        assert_eq!(configs[0].locale, domain.config().locale);

        // Test serializing back to YAML
        let serialized = serde_yaml::to_string(&configs).expect("Failed to serialize to YAML");
        let deserialized: Vec<LanguageDomainConfig> =
            serde_yaml::from_str(&serialized).expect("Failed to deserialize YAML");
        assert_eq!(configs, deserialized);
    }

    #[test]
    fn test_language_domain_conversion() {
        // Test From<LanguageDomainConfig> for LanguageDomain
        let config = LanguageDomainConfig {
            code: "es".to_string(),
            name: "Learn Spanish".to_string(),
            base_path: "/es".to_string(),
            locale: "es-ES".to_string(),
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
        locale: en-US    # Wrong indentation
          - invalid:     # Invalid nesting
    "#;

        let result: Result<Vec<LanguageDomainConfig>, _> = serde_yaml::from_str(invalid_yaml);
        assert!(result.is_err(), "Should fail with invalid YAML syntax");

        // Test with missing required field
        let missing_field_yaml = r#"
    - code: en
      name: Learn English
      base_path: /en
    "#;

        let result: Result<Vec<LanguageDomainConfig>, _> = serde_yaml::from_str(missing_field_yaml);
        assert!(result.is_err(), "Should fail with missing required field");

        // Test with invalid type
        let invalid_type_yaml = r#"
    - code: [this, is, an, array]  # should be string
      name: Learn English
      base_path: /en
      locale: en-US
    "#;

        let result: Result<Vec<LanguageDomainConfig>, _> = serde_yaml::from_str(invalid_type_yaml);
        assert!(result.is_err(), "Should fail with invalid type");
    }
}
