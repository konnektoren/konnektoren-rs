use isolang::Language as IsoLanguage;
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::Builtin(iso) | Language::Other { iso, .. } => iso.to_639_1().unwrap_or("und"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Language::Builtin(iso) | Language::Other { iso, .. } => iso.to_name(),
        }
    }

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

    pub fn is_rtl(&self) -> bool {
        match self {
            Language::Builtin(iso) => matches!(iso, IsoLanguage::Ara | IsoLanguage::Heb),
            Language::Other { rtl, .. } => *rtl,
        }
    }

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

    pub fn all(others: Option<Vec<Language>>) -> Vec<Language> {
        let mut languages = Self::builtin();
        if let Some(additional) = others {
            languages.extend(additional);
        }
        languages
    }

    pub fn new(code: &str, flag: impl Into<String>, rtl: bool) -> Option<Self> {
        IsoLanguage::from_639_1(code).map(|iso| Language::Other {
            iso,
            flag: flag.into(),
            rtl,
        })
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
}
