//! The `i18n` module contains the internationalization (i18n) functionality for the application.

/// The `config` module contains the configuration for internationalization (i18n).
mod config;

/// The `provider` module defines the components and hooks for managing i18n within the application.
mod provider;

/// The `selected_language` module manages the state of the currently selected language.
mod selected_language;

/// A constant key used to store the selected language in storage.
pub const LANGUAGE_KEY: &str = "selected_language";

/// A list of supported languages represented by their ISO codes.
pub const LANGUAGES: [&str; 8] = ["en", "ua", "ar", "de", "cn", "pl", "tr", "es"];

/// Re-export the `I18nConfig` struct from the `config` module.
pub use config::I18nConfig;

/// Re-export the i18n hooks and components from the `provider` module.
///
/// - `use_i18n`: A hook to access i18n functionality.
/// - `use_selected_language`: A hook to get or set the selected language.
/// - `I18nProvider`: A component that provides i18n context to the application.
/// - `I18nProviderProps`: The properties for the `I18nProvider` component.
pub use provider::{use_i18n, use_selected_language, I18nProvider, I18nProviderProps};

/// Re-export the `SelectedLanguage` type from the `selected_language` module.
pub use selected_language::SelectedLanguage;

/// Checks if the provided language is supported by the application.
///
/// # Parameters
///
/// - `lang`: An optional string slice representing the language code.
///
/// # Returns
///
/// If the language is supported, it returns `Some(String)` with the language code as a `String`.
/// Otherwise, it returns `None`.
///
/// # Examples
///
/// ```
/// use konnektoren_yew::i18n::supported_language;
/// let lang = supported_language(Some("en"));
/// assert_eq!(lang, Some("en".to_string()));
/// ```
pub fn supported_language(lang: Option<&str>) -> Option<String> {
    match lang {
        Some(lang) => {
            if LANGUAGES.contains(&lang) {
                Some(lang.to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

/// Returns the flag emoji corresponding to the provided language code.
///
/// # Parameters
///
/// - `lang`: A static string slice representing the language code.
///
/// # Returns
///
/// A static string slice containing the flag emoji associated with the language code.
///
/// # Examples
///
/// ```
/// use konnektoren_yew::i18n::flag;
/// let flag = flag("en");
/// assert_eq!(flag, "🇺🇸");
/// ```
pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        "ua" => "🇺🇦",
        "cn" => "🇨🇳",
        "ar" => "🇸🇦",
        "pl" => "🇵🇱",
        "tr" => "🇹🇷",
        "es" => "🇪🇸",
        _ => "🌐",
    }
}

/// Unit tests for the `supported_language` and `flag` functions.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests the `supported_language` function with various inputs.
    fn test_supported_language() {
        assert_eq!(supported_language(Some("en")), Some("en".to_string()));
        assert_eq!(supported_language(Some("ua")), Some("ua".to_string()));
        assert_eq!(supported_language(Some("de")), Some("de".to_string()));
        assert_eq!(supported_language(Some("cn")), Some("cn".to_string()));
        assert_eq!(supported_language(Some("ar")), Some("ar".to_string()));
        assert_eq!(supported_language(Some("pl")), Some("pl".to_string()));
        assert_eq!(supported_language(Some("tr")), Some("tr".to_string()));
        assert_eq!(supported_language(Some("es")), Some("es".to_string()));
        assert_eq!(supported_language(Some("fr")), None);
        assert_eq!(supported_language(None), None);
    }

    #[test]
    /// Tests the `flag` function to ensure it returns the correct flag emoji.
    fn test_flag() {
        assert_eq!(flag("en"), "🇺🇸");
        assert_eq!(flag("de"), "🇩🇪");
        assert_eq!(flag("ua"), "🇺🇦");
        assert_eq!(flag("cn"), "🇨🇳");
        assert_eq!(flag("ar"), "🇸🇦");
        assert_eq!(flag("pl"), "🇵🇱");
        assert_eq!(flag("tr"), "🇹🇷");
        assert_eq!(flag("es"), "🇪🇸");
        assert_eq!(flag("fr"), "🌐");
    }
}
