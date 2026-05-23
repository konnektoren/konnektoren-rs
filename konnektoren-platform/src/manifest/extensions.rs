use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// A type that can appear as a named top-level section in a manifest YAML.
///
/// Implement via the [`manifest_section!`] macro.
///
/// ```rust,ignore
/// use konnektoren_platform::manifest_section;
///
/// #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
/// pub struct ThemeConfig {
///     pub primary_color: String,
/// }
///
/// manifest_section!(ThemeConfig, "theme");
///
/// assert_eq!(ThemeConfig::KEY, "theme");
/// ```
pub trait ManifestSection: Debug + Clone + PartialEq + Serialize + DeserializeOwned {
    /// The top-level YAML key this section occupies, e.g. `"domain"` or `"i18n"`.
    const KEY: &'static str;
}

/// A flat, composable bundle of [`ManifestSection`]s.
///
/// All fields in an `ManifestExtensions` struct are flattened into the parent
/// [`Manifest`](super::Manifest), so they appear as top-level YAML keys rather
/// than nested under `metadata:`.
///
/// Build one with the [`manifest_extensions!`] macro.
pub trait ManifestExtensions:
    Default + Debug + Clone + PartialEq + Serialize + DeserializeOwned
{
}

/// Empty extension bundle for `Manifest<NoExtensions>` — used when no sections
/// beyond `package`, `assets`, and `game_paths` are needed.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct NoExtensions {}

impl ManifestExtensions for NoExtensions {}
