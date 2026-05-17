use crate::i18n::{Language, TranslationAsset};
use konnektoren_core::game::GamePath;
use std::collections::{HashMap, HashSet};

/// Missing translations per language code for the challenges in a [`GamePath`].
#[derive(Debug, Default)]
pub struct ChallengeI18nReport {
    /// Keys present in the path that are missing from each language.
    pub missing: HashMap<String, Vec<String>>,
    /// `true` if any language is missing at least one translation.
    pub has_errors: bool,
}

impl ChallengeI18nReport {
    pub fn assert_complete(&self) {
        assert!(
            !self.has_errors,
            "missing challenge translations: {:#?}",
            self.missing
        );
    }
}

/// Checks that all `name` and `description` fields of the challenges in a
/// [`GamePath`] are translated for every builtin language.
///
/// Keys are the values from the path YAML. The convention is that the English
/// translation file maps each key to its English text, so a missing key in any
/// language file is caught immediately.
///
/// # Example
/// ```rust,no_run
/// use konnektoren_platform::tools::ChallengeI18nChecker;
/// use konnektoren_platform::i18n::JsonTranslationAsset;
/// use konnektoren_core::game::GamePath;
///
/// let asset = JsonTranslationAsset::<MyAssets>::new();
/// let report = ChallengeI18nChecker::new(&asset).check(&GamePath::default());
/// report.assert_complete();
/// ```
pub struct ChallengeI18nChecker {
    translations: HashMap<String, serde_json::Value>,
}

impl ChallengeI18nChecker {
    pub fn new<A: TranslationAsset>(asset: &A) -> Self {
        Self {
            translations: asset.load_translations(),
        }
    }

    pub fn check(&self, path: &GamePath) -> ChallengeI18nReport {
        let keys = extract_keys(path);
        let mut missing: HashMap<String, Vec<String>> = HashMap::new();

        for lang in Language::builtin() {
            let code = lang.code().to_string();
            let translated: HashSet<&str> = self
                .translations
                .get(&code)
                .and_then(|v| v.as_object())
                .map(|obj| obj.keys().map(|k| k.as_str()).collect())
                .unwrap_or_default();

            let missing_keys: Vec<String> = keys
                .iter()
                .filter(|k| !translated.contains(k.as_str()))
                .cloned()
                .collect();

            if !missing_keys.is_empty() {
                missing.insert(code, missing_keys);
            }
        }

        let has_errors = !missing.is_empty();
        ChallengeI18nReport {
            missing,
            has_errors,
        }
    }
}

fn extract_keys(path: &GamePath) -> Vec<String> {
    let mut keys = Vec::new();
    keys.push(path.name.clone());
    for challenge in &path.challenges {
        keys.push(challenge.name.clone());
        keys.push(challenge.description.clone());
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::JsonTranslationAsset;

    #[derive(rust_embed::RustEmbed)]
    #[folder = "$CARGO_MANIFEST_DIR/assets/challenges/i18n/"]
    struct ChallengeI18nAssets;

    #[test]
    fn konnektoren_path_all_langs_translated() {
        let asset = JsonTranslationAsset::<ChallengeI18nAssets>::new();
        let checker = ChallengeI18nChecker::new(&asset);
        let report = checker.check(&GamePath::default());
        report.assert_complete();
    }
}
