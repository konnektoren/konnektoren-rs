use super::language::Language;
use super::translation_asset::TranslationAsset;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct I18nConfig {
    pub translations: HashMap<String, Value>,
    pub default_language: Language,
    additional_languages: Option<Vec<Language>>,
}

impl I18nConfig {
    pub fn new(
        translations: HashMap<String, Value>,
        default_language: Language,
        additional_languages: Option<Vec<Language>>,
    ) -> Self {
        Self {
            translations,
            default_language,
            additional_languages,
        }
    }

    pub fn with_assets<T: TranslationAsset>(asset: T) -> Self {
        let mut config = Self::default();
        let translations = asset.load_translations();

        for (lang, trans) in translations {
            config.merge_translation(
                &Language::builtin()
                    .into_iter()
                    .find(|l| l.code() == lang)
                    .unwrap_or_else(Language::default),
                trans,
            );
        }

        config
    }

    pub fn supported_languages(&self) -> Vec<Language> {
        Language::all(self.additional_languages.clone())
    }

    // Safely truncate a UTF-8 string to a maximum character count
    fn safe_truncate(s: &str, max_chars: usize) -> String {
        if s.chars().count() <= max_chars {
            return s.to_string();
        }

        let mut result = String::with_capacity(max_chars);
        for (i, c) in s.chars().enumerate() {
            if i >= max_chars {
                break;
            }
            result.push(c);
        }

        result.push_str("...");
        result
    }

    pub fn get_translation(&self, text: &str, lang: Option<&Language>) -> String {
        let language = lang
            .map(|l| l.code().to_string())
            .unwrap_or_else(|| self.default_language.code().to_string());

        let translation = self.translations.get(&language).unwrap_or(&Value::Null);

        // Check if translation exists
        let result = translation[text].as_str().unwrap_or(text).to_string();

        // Log a warning if the translation is missing (i.e., returns the original text)
        if result == text {
            // Safely truncate the text for logging if it's too long
            let truncated_text = if text.chars().count() > 20 {
                Self::safe_truncate(text, 17)
            } else {
                text.to_string()
            };

            log::warn!("⚠️ no '{}' in '{}'", truncated_text, language);
        }

        result
    }

    pub fn merge_translation(&mut self, lang: &Language, translation: Value) {
        match self.translations.get(lang.code()) {
            Some(existing) => {
                let mut merged = existing.clone();
                merged
                    .as_object_mut()
                    .unwrap()
                    .extend(translation.as_object().unwrap().clone());
                self.translations.insert(lang.code().to_string(), merged);
            }
            None => {
                self.translations
                    .insert(lang.code().to_string(), translation);
            }
        }
    }

    pub fn supported_codes(&self) -> Vec<&str> {
        self.supported_languages()
            .iter()
            .map(|lang| lang.code())
            .collect()
    }
}

impl I18nConfig {
    pub fn t(&self, key: &str) -> String {
        self.get_translation(key, None)
    }

    pub fn t_with_lang(&self, key: &str, lang: &Language) -> String {
        self.get_translation(key, Some(lang))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_config() -> I18nConfig {
        let mut config = I18nConfig::default();

        // Add test translations
        config.merge_translation(
            &Language::from("en"),
            json!({
                "Language": "Language",
                "Hello": "Hello",
                "Test": "Test",
                "Umlauts Test": "With umlauts"
            }),
        );

        config.merge_translation(
            &Language::from("de"),
            json!({
                "Language": "Sprache",
                "Hello": "Hallo",
                "Test": "Test",
                "Umlauts Test": "Mit Umlauten: äöüß"
            }),
        );

        config
    }

    #[test]
    fn test_default() {
        let config = I18nConfig::default();
        assert_eq!(
            config.supported_languages().len(),
            Language::builtin().len()
        );
        assert_eq!(config.default_language, Language::default());
    }

    #[test]
    fn test_supported_languages() {
        let config = I18nConfig::default();
        let languages = config.supported_languages();
        assert!(languages.contains(&Language::default()));
        assert_eq!(languages.len(), Language::builtin().len());

        // Test with additional languages
        let french = Language::new("fr", "🇫🇷", false).unwrap();
        let config = I18nConfig::new(
            HashMap::new(),
            Language::default(),
            Some(vec![french.clone()]),
        );
        let languages = config.supported_languages();
        assert!(languages.contains(&french));
        assert_eq!(languages.len(), Language::builtin().len() + 1);
    }

    #[test]
    fn test_get_translation() {
        let mut translations = HashMap::new();
        translations.insert(
            "en".to_string(),
            json!({ "Hello": "Hello", "World": "World" }),
        );

        let config = I18nConfig::new(translations, Language::default(), None);

        assert_eq!(config.get_translation("Hello", None), "Hello");
        assert_eq!(
            config.get_translation("Hello", Some(&Language::default())),
            "Hello"
        );
    }

    #[test]
    fn test_merge_translation() {
        let mut config = I18nConfig::default();
        let lang = Language::default();

        config.merge_translation(
            &lang,
            json!({
                "Hello": "Hello",
                "World": "World"
            }),
        );

        assert_eq!(config.get_translation("Hello", Some(&lang)), "Hello");

        config.merge_translation(
            &lang,
            json!({
                "World": "New World",
                "Test": "Test"
            }),
        );

        assert_eq!(config.get_translation("World", Some(&lang)), "New World");
        assert_eq!(config.get_translation("Test", Some(&lang)), "Test");
    }

    #[test]
    fn test_supported_codes() {
        let config = I18nConfig::default();
        let codes = config.supported_codes();
        assert!(codes.contains(&"en"));
        assert!(codes.contains(&"de"));
        assert_eq!(codes.len(), Language::builtin().len());
    }

    #[test]
    fn test_t_method() {
        let i18n = create_test_config();
        assert_eq!(i18n.t("Test"), "Test");
        assert_eq!(i18n.t("Hello"), "Hello");
        assert_eq!(i18n.t("NonExistent"), "NonExistent");
    }

    #[test]
    fn test_t_with_lang() {
        let i18n = create_test_config();
        assert_eq!(
            i18n.t_with_lang("Language", &Language::from("de")),
            "Sprache"
        );
        assert_eq!(
            i18n.t_with_lang("Language", &Language::from("en")),
            "Language"
        );
    }

    #[test]
    fn test_safe_truncate() {
        // Test regular ASCII strings
        assert_eq!(I18nConfig::safe_truncate("Hello World", 5), "Hello...");
        assert_eq!(I18nConfig::safe_truncate("Short", 10), "Short");

        // Test UTF-8 characters
        assert_eq!(I18nConfig::safe_truncate("äöüß", 2), "äö...");

        // Test string with UTF-8 characters at the truncation boundary
        assert_eq!(
            I18nConfig::safe_truncate(
                "Ordnen Sie die Wörter zu einem korrekten deutschen Satz",
                17
            ),
            "Ordnen Sie die Wö..."
        );

        // Test empty string
        assert_eq!(I18nConfig::safe_truncate("", 5), "");

        // Test mixed ASCII and UTF-8
        assert_eq!(I18nConfig::safe_truncate("Hello 世界", 6), "Hello ...");

        // Test string with emoji
        assert_eq!(I18nConfig::safe_truncate("Hello 🌍🌎🌏", 7), "Hello 🌍...");
    }

    #[test]
    fn test_unicode_translations() {
        let mut config = I18nConfig::default();
        let lang = Language::from("de");

        // Add a translation with Unicode characters
        config.merge_translation(
            &lang,
            json!({
                "German with Umlauts": "Deutsch mit Umlauten: äöüß",
                "Long Unicode": "Eine sehr lange Zeichenkette mit Umlauten äöüß und mehr Text, der abgeschnitten werden müsste"
            }),
        );

        // Test retrieval of the Unicode text
        assert_eq!(
            config.get_translation("German with Umlauts", Some(&lang)),
            "Deutsch mit Umlauten: äöüß"
        );

        // Test that warning truncation works with Unicode
        // This should not panic when truncating for logging
        let _ = config.get_translation(
            "Long Unicode and more text that isn't in the translations",
            Some(&lang),
        );

        // Test keys with Unicode characters
        config.merge_translation(
            &lang,
            json!({
                "Schlüssel mit Umlauten": "Value with key containing umlauts"
            }),
        );

        assert_eq!(
            config.get_translation("Schlüssel mit Umlauten", Some(&lang)),
            "Value with key containing umlauts"
        );
    }
}
