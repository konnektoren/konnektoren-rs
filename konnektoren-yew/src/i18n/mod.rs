mod config;
mod provider;
mod selected_language;

pub const LANGUAGE_KEY: &str = "selected_language";

pub const LANGUAGES: [&str; 8] = ["en", "ua", "ar", "de", "cn", "pl", "tr", "es"];

pub use config::I18nConfig;
pub use provider::{use_i18n, use_selected_language, I18nProvider, I18nProviderProps};
pub use selected_language::SelectedLanguage;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
