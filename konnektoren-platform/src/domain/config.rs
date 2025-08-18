use std::fmt::Debug;

/// Generic trait for domain configuration
pub trait DomainConfig: Debug + Clone + PartialEq {
    /// Unique identifier for the domain
    fn code(&self) -> &str;

    /// Human readable name of the domain
    fn name(&self) -> &str;

    /// Base path for routing
    fn base_path(&self) -> &str;

    /// Visual representation of the domain
    fn icon(&self) -> &str;

    /// Hostname of the domain
    fn hostname(&self) -> &str;

    /// Optional description of the domain
    fn description(&self) -> Option<&str> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct DummyConfig {
        code: String,
        name: String,
        base_path: String,
        icon: String,
        hostname: String,
        description: Option<String>,
    }

    impl DomainConfig for DummyConfig {
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

    #[test]
    fn test_domain_config_trait() {
        let config = DummyConfig {
            code: "test".into(),
            name: "Test Domain".into(),
            base_path: "/test".into(),
            icon: "🧪".into(),
            hostname: "test.local".into(),
            description: Some("A test domain".into()),
        };
        assert_eq!(config.code(), "test");
        assert_eq!(config.name(), "Test Domain");
        assert_eq!(config.base_path(), "/test");
        assert_eq!(config.icon(), "🧪");
        assert_eq!(config.hostname(), "test.local");
        assert_eq!(config.description(), Some("A test domain"));
    }

    #[test]
    fn test_domain_config_default_description() {
        let config = DummyConfig {
            code: "test".into(),
            name: "Test Domain".into(),
            base_path: "/test".into(),
            icon: "🧪".into(),
            hostname: "test.local".into(),
            description: None,
        };
        assert_eq!(config.description(), None);
    }
}
