use crate::i18n::{I18nConfig, Language};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct LanguageStats {
    pub total_keys: usize,
    pub missing_keys: usize,
    pub coverage_percentage: f64,
}

#[derive(Debug)]
pub struct I18nReport {
    pub source_keys: HashSet<String>,
    pub translation_keys: HashMap<String, HashSet<String>>,
    pub missing_translations: HashMap<String, Vec<String>>,
    pub unused_translations: HashSet<String>,
    pub language_stats: HashMap<String, LanguageStats>,
    pub has_errors: bool,
    pub translations: HashMap<String, serde_json::Value>,
}

impl I18nReport {
    pub fn print_report(&self) {
        log::info!("\nTranslation Report");
        log::info!("=================");
        log::info!("Source keys found: {}", self.source_keys.len());

        // Print statistics for each language
        log::info!("\nLanguage Statistics:");
        log::info!("-------------------");
        for lang in Language::builtin() {
            if let Some(stats) = self.language_stats.get(lang.code()) {
                log::info!(
                    "{} ({}): {}/{} keys ({:.1}% coverage)",
                    lang.native_name(),
                    lang.code(),
                    stats.total_keys - stats.missing_keys,
                    stats.total_keys,
                    stats.coverage_percentage
                );
            }
        }

        // Print missing translations by language
        if !self.missing_translations.is_empty() {
            log::warn!("\nMissing Translations:");
            log::warn!("-------------------");
            for lang in Language::builtin() {
                if let Some(missing) = self.missing_translations.get(lang.code()) {
                    if !missing.is_empty() {
                        log::warn!(
                            "{} ({}) - {} missing:",
                            lang.native_name(),
                            lang.code(),
                            missing.len()
                        );
                        for key in missing {
                            // Show the English translation as reference if available
                            if let Some(en_trans) = self
                                .translations
                                .get("en")
                                .and_then(|t| t.get(key))
                                .and_then(|v| v.as_str())
                            {
                                log::warn!("  - {}: \"{}\"", key, en_trans);
                            } else {
                                log::warn!("  - {}", key);
                            }
                        }
                    }
                }
            }
        }

        // Print unused translations
        if !self.unused_translations.is_empty() {
            log::warn!("\nUnused Translations:");
            log::warn!("-------------------");
            for key in &self.unused_translations {
                log::warn!("  - {}", key);
                // Show translations in all languages
                for lang in Language::builtin() {
                    if let Some(trans) = self
                        .translations
                        .get(lang.code())
                        .and_then(|t| t.get(key))
                        .and_then(|v| v.as_str())
                    {
                        log::info!(
                            "    {} ({}): \"{}\"",
                            lang.native_name(),
                            lang.code(),
                            trans
                        );
                    }
                }
            }
        }

        // Print summary
        log::info!("\nSummary:");
        log::info!("--------");
        log::info!("Total source keys: {}", self.source_keys.len());
        log::info!(
            "Languages: {}",
            Language::builtin()
                .iter()
                .map(|l| l.code())
                .collect::<Vec<_>>()
                .join(", ")
        );
        log::info!(
            "Overall status: {}",
            if self.has_errors {
                "❌ Missing translations"
            } else {
                "✅ All translations complete"
            }
        );
    }
}

pub struct I18nChecker {
    config: I18nConfig,
    source_patterns: Vec<Regex>,
}

impl I18nChecker {
    pub fn new(config: I18nConfig) -> Self {
        Self {
            config,
            source_patterns: vec![Regex::new(r#"(?:i18n\.t|t_with_lang)\("([^"]+)"\)"#).unwrap()],
        }
    }

    pub fn check_directory<P: AsRef<Path>>(&self, dir: P) -> I18nReport {
        let source_keys = self.collect_source_keys(dir);
        let translation_keys = self.collect_translation_keys();
        let missing_translations = self.find_missing_translations(&source_keys, &translation_keys);
        let unused_translations = self.find_unused_translations(&source_keys, &translation_keys);
        let language_stats = self.calculate_language_stats(&source_keys, &translation_keys);
        let has_errors = missing_translations.values().any(|v| !v.is_empty());

        I18nReport {
            source_keys,
            translation_keys,
            missing_translations,
            unused_translations,
            language_stats,
            has_errors,
            translations: self.config.translations.clone(),
        }
    }

    fn collect_source_keys<P: AsRef<Path>>(&self, dir: P) -> HashSet<String> {
        let mut keys = HashSet::new();

        for entry in WalkDir::new(dir) {
            let entry = entry.unwrap();
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    for pattern in &self.source_patterns {
                        for cap in pattern.captures_iter(&content) {
                            keys.insert(cap[1].to_string());
                        }
                    }
                }
            }
        }

        keys
    }

    fn collect_translation_keys(&self) -> HashMap<String, HashSet<String>> {
        let mut translation_keys = HashMap::new();

        for lang in Language::builtin() {
            let lang_code = lang.code().to_string();
            if let Some(translations) = self.config.translations.get(&lang_code) {
                if let Some(obj) = translations.as_object() {
                    translation_keys.insert(lang_code, obj.keys().cloned().collect());
                }
            }
        }

        translation_keys
    }

    fn find_missing_translations(
        &self,
        source_keys: &HashSet<String>,
        translation_keys: &HashMap<String, HashSet<String>>,
    ) -> HashMap<String, Vec<String>> {
        let mut missing = HashMap::new();

        for lang in Language::builtin() {
            let lang_code = lang.code().to_string();

            // Create the empty set first and bind it to a variable
            let empty_set = HashSet::new();
            // Get the reference to either the existing keys or the empty set
            let keys = translation_keys.get(&lang_code).unwrap_or(&empty_set);

            let missing_keys: Vec<_> = source_keys
                .iter()
                .filter(|key| !keys.contains(*key))
                .cloned()
                .collect();

            if !missing_keys.is_empty() {
                missing.insert(lang_code, missing_keys);
            }
        }

        missing
    }

    fn find_unused_translations(
        &self,
        source_keys: &HashSet<String>,
        translation_keys: &HashMap<String, HashSet<String>>,
    ) -> HashSet<String> {
        let mut unused = HashSet::new();

        for keys in translation_keys.values() {
            for key in keys {
                if !source_keys.contains(key) {
                    unused.insert(key.clone());
                }
            }
        }

        unused
    }

    fn calculate_language_stats(
        &self,
        source_keys: &HashSet<String>,
        translation_keys: &HashMap<String, HashSet<String>>,
    ) -> HashMap<String, LanguageStats> {
        let mut stats = HashMap::new();

        for lang in Language::builtin() {
            let lang_code = lang.code().to_string();
            let total_keys = source_keys.len();
            let empty_set = HashSet::new();
            let keys = translation_keys.get(&lang_code).unwrap_or(&empty_set);
            let missing_keys = source_keys
                .iter()
                .filter(|key| !keys.contains(*key))
                .count();
            let coverage_percentage = if total_keys > 0 {
                ((total_keys - missing_keys) as f64 / total_keys as f64) * 100.0
            } else {
                100.0
            };

            stats.insert(
                lang_code,
                LanguageStats {
                    total_keys,
                    missing_keys,
                    coverage_percentage,
                },
            );
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_i18n_checker() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init();

        let mut config = I18nConfig::default();
        config.merge_translation(
            &Language::from("en"),
            json!({
                "Hello": "Hello",
                "Unused": "Unused",
                "Test": "Test",
            }),
        );
        config.merge_translation(
            &Language::from("de"),
            json!({
                "Hello": "Hallo",
                "Test": "Test",
            }),
        );

        let checker = I18nChecker::new(config);
        let report = checker.check_directory("src/tools");

        // Verify statistics
        assert_eq!(report.source_keys.len(), 0);
        assert!(!report.translation_keys.is_empty());

        if let Some(en_stats) = report.language_stats.get("en") {
            assert_eq!(en_stats.total_keys, 0);
            assert_eq!(en_stats.coverage_percentage, 100.0);
        }

        report.print_report();
    }
}
