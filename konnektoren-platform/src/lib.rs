pub mod domain;
pub mod i18n;
pub mod manifest;

#[cfg(feature = "tools")]
pub mod tools;

/// Constructs a [`manifest::Package`] populated from this crate's `Cargo.toml`
/// at compile time.
///
/// Must be invoked in the *calling* crate so that `env!("CARGO_PKG_*")`
/// resolves to that crate's values, not `konnektoren-platform`'s.
///
/// ```rust,ignore
/// use konnektoren_platform::cargo_package;
///
/// let manifest = Manifest::figment(
///     cargo_package!(),
///     include_str!("../assets/konnektoren.manifest.yml"),
/// )
/// .extract::<KonnektorenManifest>()?;
/// ```
#[macro_export]
macro_rules! cargo_package {
    () => {
        $crate::manifest::Package {
            id: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            name: env!("CARGO_PKG_NAME").to_string(),
            description: {
                let d = env!("CARGO_PKG_DESCRIPTION");
                if d.is_empty() {
                    None
                } else {
                    Some(d.to_string())
                }
            },
            authors: env!("CARGO_PKG_AUTHORS")
                .split(':')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect(),
            repository: {
                let r = env!("CARGO_PKG_REPOSITORY");
                if r.is_empty() {
                    None
                } else {
                    Some(r.to_string())
                }
            },
            license: {
                let l = env!("CARGO_PKG_LICENSE");
                if l.is_empty() {
                    None
                } else {
                    Some(l.to_string())
                }
            },
            keywords: env!("CARGO_PKG_KEYWORDS")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
        }
    };
}

/// Loads a [`manifest::KonnektorenManifest`] for the *calling* crate.
///
/// # Behaviour
///
/// 1. Populates `package` from the calling crate's `Cargo.toml` via
///    `cargo_package!()` — identical to how `cargo_package!()` works on its own.
/// 2. Applies `i18n` defaults (`path = "assets/i18n"`, `default_language = "en"`).
/// 3. If the calling crate ships `assets/konnektoren.manifest.yml` (the path
///    declared in `[package.metadata.konnektoren] manifest = …`), that file is
///    overlaid on top using **figment**, so any key present in the YAML wins
///    over the defaults.
///
/// # Feature requirement
///
/// Requires the `manifest` feature of `konnektoren-platform`:
///
/// ```toml
/// konnektoren-platform = { …, features = ["manifest"] }
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use konnektoren_platform::load_manifest;
///
/// let manifest = load_manifest!().expect("valid manifest");
/// println!("hostname = {}", manifest.domain().map(|d| d.hostname()).unwrap_or("-"));
/// println!("i18n path = {}", manifest.i18n().path);
/// ```
#[macro_export]
#[cfg(feature = "manifest")]
macro_rules! load_manifest {
    () => {{
        $crate::manifest::Manifest::figment(
            $crate::cargo_package!(),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/konnektoren.manifest.yml"
            )),
        )
        .extract::<$crate::manifest::KonnektorenManifest>()
    }};
}

pub mod prelude {
    pub use crate::domain::{Domain, DomainConfig, LanguageDomain, LanguageDomainConfig};
    pub use crate::i18n::Language;
    pub use crate::manifest::{
        Assets, DEFAULT_MANIFEST, DomainManifest, I18nManifest, KonnektorenManifest,
        KonnektorenMeta, Manifest, ManifestConfig, Package,
    };
}
