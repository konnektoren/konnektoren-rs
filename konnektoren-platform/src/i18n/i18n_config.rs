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

    pub fn get_translation(&self, text: &str, lang: Option<&Language>) -> String {
        let language = lang
            .map(|l| l.code().to_string())
            .unwrap_or_else(|| self.default_language.code().to_string());

        let translation = self.translations.get(&language).unwrap_or(&Value::Null);

        translation[text].as_str().unwrap_or(text).to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
}
