mod config;
mod language_domain;

pub use config::DomainConfig;
pub use language_domain::{LanguageDomain, LanguageDomainConfig};

/// Generic domain trait
pub trait Domain {
    /// The configuration type for this domain
    type Config: DomainConfig;

    /// Get the domain configuration
    fn config(&self) -> &Self::Config;

    /// Get the domain code
    fn code(&self) -> &str {
        self.config().code()
    }

    /// Get the domain name
    fn name(&self) -> &str {
        self.config().name()
    }

    /// Get the domain base path
    fn base_path(&self) -> &str {
        self.config().base_path()
    }

    /// Get the domain icon
    fn icon(&self) -> &str {
        self.config().icon()
    }

    /// Get the domain hostname
    fn hostname(&self) -> &str {
        // or host
        self.config().hostname()
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

    struct DummyDomain {
        config: DummyConfig,
    }
    impl Domain for DummyDomain {
        type Config = DummyConfig;
        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    #[test]
    fn test_domain_trait_defaults() {
        let config = DummyConfig {
            code: "abc".into(),
            name: "ABC Domain".into(),
            base_path: "/abc".into(),
            icon: "🔤".into(),
            hostname: "abc.local".into(),
            description: None,
        };
        let domain = DummyDomain { config };
        assert_eq!(domain.code(), "abc");
        assert_eq!(domain.name(), "ABC Domain");
        assert_eq!(domain.base_path(), "/abc");
        assert_eq!(domain.icon(), "🔤");
        assert_eq!(domain.hostname(), "abc.local");
    }
}
