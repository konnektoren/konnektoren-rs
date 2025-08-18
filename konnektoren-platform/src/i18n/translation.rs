use super::Language;

pub trait Translation {
    fn t(&self, key: &str) -> String;
    fn t_with_lang(&self, key: &str, lang: &Language) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    struct DummyTranslation;
    impl Translation for DummyTranslation {
        fn t(&self, key: &str) -> String {
            format!("t:{}", key)
        }
        fn t_with_lang(&self, key: &str, lang: &Language) -> String {
            format!("t:{}:{}", key, lang.code())
        }
    }

    #[test]
    fn test_translation_trait() {
        let dummy = DummyTranslation;
        assert_eq!(dummy.t("hello"), "t:hello");
        let lang = Language::from("de");
        assert_eq!(dummy.t_with_lang("hello", &lang), "t:hello:de");
    }
}
