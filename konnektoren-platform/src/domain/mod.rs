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
