use isolang::Language as IsoLanguage;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A language supported by the platform.
///
/// Builtin languages (en, de, es, ar, zh, uk, pl, tr, vi) are represented as
/// `Builtin` variants backed by [`isolang`]. Any other valid ISO 639-1 code can
/// be stored as `Other` with a custom flag emoji and RTL flag.
///
/// # Construction
/// - [`Language::builtin()`] — list of all built-in languages
/// - [`Language::from_code()`] — infallible, falls back to `default()` on unknown codes
/// - [`Language::try_from_code()`] / `FromStr` — returns `Err` on unknown codes
/// - [`Language::new()`] — create a custom `Other` variant with a specific flag
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Builtin(IsoLanguage),
    Other {
        iso: IsoLanguage,
        flag: String,
        rtl: bool,
    },
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Language::Builtin(iso) => {
                let code = iso.to_639_1().unwrap_or("und");
                serializer.serialize_str(code)
            }
            Language::Other { iso, flag, rtl } => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct("Language", 3)?;
                state.serialize_field("code", iso.to_639_1().unwrap_or("und"))?;
                state.serialize_field("flag", flag)?;
                state.serialize_field("rtl", rtl)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum LanguageData {
            Code(String),
            Other {
                code: String,
                flag: String,
                rtl: bool,
            },
        }

        let data = LanguageData::deserialize(deserializer)?;
        match data {
            LanguageData::Code(code) => {
                let iso = IsoLanguage::from_639_1(&code).ok_or_else(|| {
                    serde::de::Error::custom(format!("Invalid language code: {}", code))
                })?;
                Ok(Language::Builtin(iso))
            }
            LanguageData::Other { code, flag, rtl } => {
                let iso = IsoLanguage::from_639_1(&code).ok_or_else(|| {
                    serde::de::Error::custom(format!("Invalid language code: {}", code))
                })?;
                Ok(Language::Other { iso, flag, rtl })
            }
        }
    }
}

impl From<&str> for Language {
    fn from(code: &str) -> Self {
        Self::from_code(code)
    }
}

impl From<String> for Language {
    fn from(code: String) -> Self {
        Self::from_code(&code)
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        if let Some(iso) = IsoLanguage::from_639_1(code) {
            // Check if it's a builtin language
            if Language::builtin().iter().any(|lang| lang.code() == code) {
                Ok(Language::Builtin(iso))
            } else {
                // For non-builtin languages, create as Other with default flag
                Ok(Language::Other {
                    iso,
                    flag: "🌐".to_string(),
                    rtl: matches!(iso, IsoLanguage::Ara | IsoLanguage::Heb),
                })
            }
        } else {
            Err(format!("Invalid language code: {}", code))
        }
    }
}

impl Language {
    /// Returns the ISO 639-1 two-letter code, e.g. `"en"`, `"de"`. Falls back to `"und"` for
    /// languages without a 639-1 code (should not happen for builtin languages).
    pub fn code(&self) -> &'static str {
        match self {
            Language::Builtin(iso) | Language::Other { iso, .. } => iso.to_639_1().unwrap_or("und"),
        }
    }

    /// Returns the English name of the language, e.g. `"German"`.
    pub fn name(&self) -> &str {
        match self {
            Language::Builtin(iso) | Language::Other { iso, .. } => iso.to_name(),
        }
    }

    /// Returns the language name in the language itself, e.g. `"Deutsch"` for German.
    /// Falls back to the English name for languages without an explicit native name.
    pub fn native_name(&self) -> &str {
        match self {
            Language::Builtin(iso) | Language::Other { iso, .. } => match iso {
                IsoLanguage::Eng => "English",
                IsoLanguage::Deu => "Deutsch",
                IsoLanguage::Spa => "Español",
                IsoLanguage::Ara => "العربية",
                IsoLanguage::Zho => "中文",
                IsoLanguage::Ukr => "Українська",
                IsoLanguage::Pol => "Polski",
                IsoLanguage::Tur => "Türkçe",
                IsoLanguage::Vie => "Tiếng Việt",
                _ => iso.to_name(),
            },
        }
    }

    /// Returns the flag emoji for the language, e.g. `"🇩🇪"` for German.
    pub fn flag(&self) -> String {
        match self {
            Language::Builtin(iso) => self.iso_to_flag(iso).to_string(),
            Language::Other { flag, .. } => flag.clone(),
        }
    }

    fn iso_to_flag(&self, iso: &IsoLanguage) -> &'static str {
        match iso {
            IsoLanguage::Eng => "🇺🇸",
            IsoLanguage::Deu => "🇩🇪",
            IsoLanguage::Spa => "🇪🇸",
            IsoLanguage::Ara => "🇸🇦",
            IsoLanguage::Zho => "🇨🇳",
            IsoLanguage::Ukr => "🇺🇦",
            IsoLanguage::Pol => "🇵🇱",
            IsoLanguage::Tur => "🇹🇷",
            IsoLanguage::Vie => "🇻🇳",
            _ => "🌐",
        }
    }

    /// Returns `true` if the language is written right-to-left (Arabic, Hebrew).
    pub fn is_rtl(&self) -> bool {
        match self {
            Language::Builtin(iso) => matches!(iso, IsoLanguage::Ara | IsoLanguage::Heb),
            Language::Other { rtl, .. } => *rtl,
        }
    }

    /// Returns all languages with first-class platform support (en, de, es, ar, zh, uk, pl, tr, vi).
    pub fn builtin() -> Vec<Language> {
        vec![
            Language::Builtin(IsoLanguage::Eng),
            Language::Builtin(IsoLanguage::Deu),
            Language::Builtin(IsoLanguage::Spa),
            Language::Builtin(IsoLanguage::Ara),
            Language::Builtin(IsoLanguage::Zho),
            Language::Builtin(IsoLanguage::Ukr),
            Language::Builtin(IsoLanguage::Pol),
            Language::Builtin(IsoLanguage::Tur),
            Language::Builtin(IsoLanguage::Vie),
        ]
    }

    /// Returns builtin languages plus any additional ones provided.
    pub fn all(others: Option<Vec<Language>>) -> Vec<Language> {
        let mut languages = Self::builtin();
        if let Some(additional) = others {
            languages.extend(additional);
        }
        languages
    }

    /// Creates a custom `Other` language with the given ISO 639-1 `code`, `flag` emoji, and
    /// `rtl` direction. Returns `None` if `code` is not a valid ISO 639-1 code.
    pub fn new(code: &str, flag: impl Into<String>, rtl: bool) -> Option<Self> {
        IsoLanguage::from_639_1(code).map(|iso| Language::Other {
            iso,
            flag: flag.into(),
            rtl,
        })
    }

    /// Parses a language from an ISO 639-1 code. Falls back to `default()` for unknown codes.
    /// Use [`Language::try_from_code()`] if you need to detect invalid codes.
    pub fn from_code(code: &str) -> Self {
        Self::from_str(code).unwrap_or_else(|_| Self::default())
    }

    /// Parses a language from an ISO 639-1 code. Returns `Err` for unknown codes.
    pub fn try_from_code(code: &str) -> Result<Self, String> {
        Self::from_str(code)
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Builtin(IsoLanguage::Eng)
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.native_name(), self.flag())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let lang = Language::from("de".to_string());
        assert_eq!(lang, Language::Builtin(IsoLanguage::Deu));
    }

    #[test]
    fn test_from_str_ref() {
        let lang = Language::from("de");
        assert_eq!(lang, Language::Builtin(IsoLanguage::Deu));
    }

    #[test]
    fn test_default_fallback() {
        let invalid_lang = Language::from("invalid");
        assert_eq!(invalid_lang, Language::default());
        assert_eq!(invalid_lang, Language::Builtin(IsoLanguage::Eng));
    }

    #[test]
    fn test_language_equality() {
        let lang1 = Language::from("de");
        let lang2 = Language::Builtin(IsoLanguage::Deu);
        let lang3 = Language::from_code("de");

        assert_eq!(lang1, lang2);
        assert_eq!(lang2, lang3);
        assert_eq!(lang1, lang3);
    }

    #[test]
    fn test_builtin_languages() {
        let languages = Language::builtin();
        assert!(languages.contains(&Language::Builtin(IsoLanguage::Eng)));
        assert!(languages.contains(&Language::Builtin(IsoLanguage::Deu)));
        assert_eq!(languages.len(), 9);
    }

    #[test]
    fn test_all_with_additional() {
        let french = Language::new("fr", "🇫🇷", false).unwrap();
        let italian = Language::new("it", "🇮🇹", false).unwrap();

        let languages = Language::all(Some(vec![french.clone(), italian.clone()]));

        assert!(languages.contains(&french));
        assert!(languages.contains(&italian));
        assert!(languages.contains(&Language::Builtin(IsoLanguage::Eng)));
        assert_eq!(languages.len(), Language::builtin().len() + 2);
    }

    #[test]
    fn test_all_with_no_additional() {
        let languages = Language::all(None);
        assert_eq!(languages.len(), Language::builtin().len());
    }

    #[test]
    fn test_language_properties() {
        let french = Language::new("fr", "🇫🇷", false).unwrap();
        assert_eq!(french.code(), "fr");
        assert_eq!(french.name(), "French");
        assert!(french.native_name().contains("French")); // Changed because isolang might not have native name
        assert_eq!(french.flag(), "🇫🇷");
        assert!(!french.is_rtl());
    }

    #[test]
    fn test_rtl_language() {
        let arabic = Language::Builtin(IsoLanguage::Ara);
        assert!(arabic.is_rtl());
    }

    #[test]
    fn test_invalid_language_code() {
        assert!(Language::new("xx", "🌐", false).is_none());
    }

    #[test]
    fn test_display_format() {
        let german = Language::Builtin(IsoLanguage::Deu);
        assert_eq!(german.to_string(), "Deutsch 🇩🇪");
    }

    #[test]
    fn test_serialization_deserialization() {
        let german = Language::Builtin(IsoLanguage::Deu);
        let serialized = serde_json::to_string(&german).unwrap();
        let deserialized: Language = serde_json::from_str(&serialized).unwrap();
        assert_eq!(german, deserialized);

        let french = Language::new("fr", "🇫🇷", false).unwrap();
        let serialized = serde_json::to_string(&french).unwrap();
        let deserialized: Language = serde_json::from_str(&serialized).unwrap();
        assert_eq!(french, deserialized);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            Language::from_str("en").unwrap(),
            Language::Builtin(IsoLanguage::Eng)
        );
        assert_eq!(
            Language::from_str("de").unwrap(),
            Language::Builtin(IsoLanguage::Deu)
        );
        assert!(Language::from_str("xx").is_err());
    }

    #[test]
    fn test_from_code() {
        assert_eq!(
            Language::from_code("en"),
            Language::Builtin(IsoLanguage::Eng)
        );
        assert_eq!(
            Language::from_code("de"),
            Language::Builtin(IsoLanguage::Deu)
        );
        // Invalid code should return default language (English)
        assert_eq!(Language::from_code("xx"), Language::default());
    }

    #[test]
    fn test_try_from_code() {
        assert!(Language::try_from_code("en").is_ok());
        assert!(Language::try_from_code("xx").is_err());
    }

    #[test]
    fn test_non_builtin_language() {
        let french = Language::from_str("fr").unwrap();
        assert_eq!(french.code(), "fr");
        assert_eq!(french.flag(), "🌐");
        assert!(!french.is_rtl());
    }
}
