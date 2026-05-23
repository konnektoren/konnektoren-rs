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
            keywords: option_env!("CARGO_PKG_KEYWORDS")
                .unwrap_or("")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
        }
    };
}

/// Declare a type as a named top-level manifest section.
///
/// Implements [`manifest::ManifestSection`] for the type, setting `KEY` to the
/// given string literal. The key becomes the YAML key when the section is
/// included in a [`manifest_extensions!`] bundle.
///
/// # Example
///
/// ```rust,ignore
/// use konnektoren_platform::manifest_section;
///
/// #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
/// pub struct ThemeConfig { pub primary_color: String }
///
/// manifest_section!(ThemeConfig, "theme");
/// assert_eq!(<ThemeConfig as konnektoren_platform::manifest::ManifestSection>::KEY, "theme");
/// ```
#[macro_export]
macro_rules! manifest_section {
    ($type:ty, $key:literal) => {
        impl $crate::manifest::ManifestSection for $type {
            const KEY: &'static str = $key;
        }
    };
}

/// Declare a flat extension bundle for [`manifest::Manifest`].
///
/// Generates a struct whose fields are all flattened into `Manifest` at the
/// top level — so each field appears as its own top-level YAML key, not nested
/// under `metadata:`.
///
/// Each field type must implement [`Default`]. Fields are automatically
/// wrapped with `#[serde(default)]`.
///
/// The generated struct implements [`manifest::ManifestExtensions`], which is
/// the required bound for the `Ext` type parameter of `Manifest<Ext>`.
///
/// # Example
///
/// ```rust,ignore
/// use konnektoren_platform::{manifest_extensions, manifest_section};
///
/// #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
/// pub struct ThemeConfig { pub dark_mode: bool }
/// manifest_section!(ThemeConfig, "theme");
///
/// manifest_extensions! {
///     pub struct MyExtensions {
///         pub theme: ThemeConfig,
///     }
/// }
///
/// // In YAML:
/// // theme:
/// //   dark_mode: true
/// ```
#[macro_export]
macro_rules! manifest_extensions {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        $vis struct $name {
            $(
                #[serde(default)]
                $(#[$field_attr])*
                pub $field: $ty,
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $( $field: Default::default(), )*
                }
            }
        }

        impl $crate::manifest::ManifestExtensions for $name {}
    };
}

/// Loads a [`manifest::KonnektorenManifest`] for the *calling* crate.
///
/// Seeds package fields from `Cargo.toml`, applies all section defaults, then
/// overlays `assets/konnektoren.manifest.yml` from the calling crate. Requires
/// the `manifest` feature.
///
/// ```toml
/// # Cargo.toml of the consuming crate
/// [package.metadata.konnektoren]
/// manifest = "assets/konnektoren.manifest.yml"
///
/// [dependencies]
/// konnektoren-platform = { ..., features = ["manifest"] }
/// ```
#[macro_export]
#[cfg(feature = "manifest")]
macro_rules! load_manifest {
    () => {{
        $crate::manifest::Manifest::<$crate::manifest::KonnektorenSections>::figment(
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
        KonnektorenSections, Manifest, ManifestConfig, ManifestExtensions, ManifestSection,
        NoExtensions, Package,
    };
}
