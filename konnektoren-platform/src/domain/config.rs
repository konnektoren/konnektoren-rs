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
}
