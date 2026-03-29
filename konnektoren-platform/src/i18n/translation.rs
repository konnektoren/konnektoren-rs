use super::Language;

pub trait Translation {
    fn t(&self, key: &str) -> String;
    fn t_with_lang(&self, key: &str, lang: &Language) -> String;

    /// Returns `Some(translation)` if found in the default language, `None` if missing.
    fn try_t(&self, key: &str) -> Option<String> {
        let result = self.t(key);
        if result == key { None } else { Some(result) }
    }

    /// Returns `Some(translation)` if found in `lang`, `None` if missing.
    fn try_t_with_lang(&self, key: &str, lang: &Language) -> Option<String> {
        let result = self.t_with_lang(key, lang);
        if result == key { None } else { Some(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    /// Always translates: returns a prefixed string different from the key.
    struct DummyTranslation;
    impl Translation for DummyTranslation {
        fn t(&self, key: &str) -> String {
            format!("t:{}", key)
        }
        fn t_with_lang(&self, key: &str, lang: &Language) -> String {
            format!("t:{}:{}", key, lang.code())
        }
    }

    /// Simulates a missing translation: returns the key unchanged (fallback behaviour).
    struct FallbackTranslation;
    impl Translation for FallbackTranslation {
        fn t(&self, key: &str) -> String {
            key.to_string()
        }
        fn t_with_lang(&self, key: &str, _lang: &Language) -> String {
            key.to_string()
        }
    }

    #[test]
    fn test_translation_trait() {
        let dummy = DummyTranslation;
        assert_eq!(dummy.t("hello"), "t:hello");
        let lang = Language::from("de");
        assert_eq!(dummy.t_with_lang("hello", &lang), "t:hello:de");
    }

    #[test]
    fn try_t_returns_some_when_translation_differs_from_key() {
        let dummy = DummyTranslation;
        assert_eq!(dummy.try_t("hello"), Some("t:hello".to_string()));
    }

    #[test]
    fn try_t_returns_none_when_fallback_returns_key() {
        let fallback = FallbackTranslation;
        assert_eq!(fallback.try_t("missing"), None);
    }

    #[test]
    fn try_t_with_lang_returns_some_when_translation_differs_from_key() {
        let dummy = DummyTranslation;
        let lang = Language::from("de");
        assert_eq!(
            dummy.try_t_with_lang("hello", &lang),
            Some("t:hello:de".to_string())
        );
    }

    #[test]
    fn try_t_with_lang_returns_none_when_fallback_returns_key() {
        let fallback = FallbackTranslation;
        let lang = Language::from("de");
        assert_eq!(fallback.try_t_with_lang("missing", &lang), None);
    }

    #[test]
    fn try_t_returns_none_when_key_equals_translation() {
        // Edge case: a valid translation that happens to equal the key
        // The default impl cannot distinguish this from a missing translation —
        // this documents that known limitation.
        struct IdentityTranslation;
        impl Translation for IdentityTranslation {
            fn t(&self, key: &str) -> String {
                key.to_string()
            }
            fn t_with_lang(&self, key: &str, _lang: &Language) -> String {
                key.to_string()
            }
        }
        let identity = IdentityTranslation;
        // Returns None even though the translation technically exists (key == value)
        assert_eq!(identity.try_t("hello"), None);
    }
}
