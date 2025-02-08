pub mod domain;
pub mod i18n;

#[cfg(feature = "tools")]
pub mod tools;

pub mod prelude {
    pub use crate::domain::{Domain, DomainConfig, LanguageDomain, LanguageDomainConfig};
    pub use crate::i18n::Language;
}
