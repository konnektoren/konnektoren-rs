use super::report::LanguageStats;
use crate::i18n::{I18nConfig, Language};
use crate::tools::i18n::I18nReportError;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;
use walkdir::WalkDir;

static SOURCE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:i18n\.t|t_with_lang)\("([^"]+)"\)"#)
        .expect("i18n source pattern is a valid regex")
});

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
    pub fn format_with<F: super::report_format::I18nReportFormatter + ?Sized>(
        &self,
        formatter: &F,
    ) -> Result<String, I18nReportError> {
        formatter.format(self)
    }
    pub fn missing_as_yaml(&self) -> Result<String, I18nReportError> {
        self.format_with(&super::report_format::I18nYamlFormatter)
    }
    pub fn missing_as_json(&self) -> Result<String, I18nReportError> {
        self.format_with(&super::report_format::I18nJsonFormatter)
    }
    pub fn as_report(&self) -> Result<String, I18nReportError> {
        self.format_with(&super::report_format::I18nHumanFormatter)
    }
}

pub struct I18nChecker {
    config: I18nConfig,
    source_patterns: Vec<Regex>,
    skip_test_files: bool,
}

/// Build a `Vec<Regex>` that matches `fn_name("key")` for each given name.
/// Dots in names are escaped automatically, e.g. `"i18n.t"` → `i18n\.t\("([^"]+)"\)`.
///
/// # Example
/// ```rust
/// use konnektoren_platform::i18n_patterns;
/// let patterns = i18n_patterns!["i18n.t", "config.t", "t_with_lang"];
/// ```
#[macro_export]
macro_rules! i18n_patterns {
    [$($name:literal),+ $(,)?] => {
        vec![
            $(
                regex::Regex::new(
                    &format!(r#"{}\("([^"]+)"\)"#, $name.replace('.', r"\."))
                ).expect(concat!("valid i18n pattern for ", $name))
            ),+
        ]
    };
}

impl I18nChecker {
    pub fn new(config: I18nConfig) -> Self {
        Self {
            config,
            source_patterns: vec![SOURCE_PATTERN.clone()],
            skip_test_files: false,
        }
    }

    /// Create a checker with custom scan patterns.
    /// Use the [`i18n_patterns!`] macro to build the pattern list.
    pub fn with_patterns(config: I18nConfig, patterns: Vec<Regex>) -> Self {
        Self {
            config,
            source_patterns: patterns,
            skip_test_files: false,
        }
    }

    /// Skip files named `tests.rs` or ending in `_test.rs` during scanning.
    pub fn exclude_tests(mut self) -> Self {
        self.skip_test_files = true;
        self
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
            let entry = match entry {
                Ok(e) => e,
                Err(err) => {
                    log::warn!("skipping unreadable path: {}", err);
                    continue;
                }
            };
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "rs") {
                continue;
            }
            if self.skip_test_files
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
                    && (name == "tests.rs" || name.ends_with("_test.rs")) {
                        continue;
                    }
            if let Ok(content) = std::fs::read_to_string(path) {
                let effective = if self.skip_test_files {
                    Self::strip_test_blocks(&content)
                } else {
                    content
                };
                for pattern in &self.source_patterns {
                    for cap in pattern.captures_iter(&effective) {
                        keys.insert(cap[1].to_string());
                    }
                }
            }
        }
        keys
    }

    /// Remove `#[cfg(test)]` blocks from source content so that test-only
    /// translation keys are not treated as production keys.
    fn strip_test_blocks(content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = String::with_capacity(content.len());
        let mut i = 0;

        while i < lines.len() {
            let trimmed = lines[i].trim();
            if trimmed.starts_with("#[cfg(test)]") {
                // Skip this attribute line and the block that follows.
                i += 1;
                // Skip blank lines between the attribute and the block opener.
                while i < lines.len() && lines[i].trim().is_empty() {
                    i += 1;
                }
                // Consume lines until the cfg(test) block is fully closed.
                if i < lines.len() && lines[i].contains('{') {
                    let mut depth: i32 = 0;
                    while i < lines.len() {
                        for c in lines[i].chars() {
                            match c {
                                '{' => depth += 1,
                                '}' => depth -= 1,
                                _ => {}
                            }
                        }
                        i += 1;
                        if depth <= 0 {
                            break;
                        }
                    }
                }
            } else {
                result.push_str(lines[i]);
                result.push('\n');
                i += 1;
            }
        }
        result
    }

    fn collect_translation_keys(&self) -> HashMap<String, HashSet<String>> {
        let mut translation_keys = HashMap::new();
        for lang in Language::builtin() {
            let lang_code = lang.code().to_string();
            if let Some(translations) = self.config.translations.get(&lang_code)
                && let Some(obj) = translations.as_object() {
                    translation_keys.insert(lang_code, obj.keys().cloned().collect());
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
            let empty_set = HashSet::new();
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
