use super::*;
use crate::i18n::Language;
use crate::tools::i18n::I18nReportError;
use serde_json::json;
use std::collections::{HashMap, HashSet};

fn dummy_report() -> I18nReport {
    let mut source_keys = HashSet::new();
    source_keys.insert("Hello".to_string());
    source_keys.insert("World".to_string());
    source_keys.insert("Description".to_string());

    let mut translation_keys = HashMap::new();
    translation_keys.insert(
        "en".to_string(),
        ["Hello", "World"].iter().map(|s| s.to_string()).collect(),
    );
    translation_keys.insert(
        "de".to_string(),
        ["Hello"].iter().map(|s| s.to_string()).collect(),
    );

    let mut missing_translations = HashMap::new();
    missing_translations.insert(
        "de".to_string(),
        vec!["World".to_string(), "Description".to_string()],
    );
    missing_translations.insert("en".to_string(), vec!["Description".to_string()]);

    let mut unused_translations = HashSet::new();
    unused_translations.insert("Unused".to_string());

    let mut language_stats = HashMap::new();
    language_stats.insert(
        "en".to_string(),
        LanguageStats {
            total_keys: 3,
            missing_keys: 1,
            coverage_percentage: 66.7,
        },
    );
    language_stats.insert(
        "de".to_string(),
        LanguageStats {
            total_keys: 3,
            missing_keys: 2,
            coverage_percentage: 33.3,
        },
    );

    let mut translations = HashMap::new();
    translations.insert(
        "en".to_string(),
        json!({"Hello": "Hello", "World": "World"}),
    );
    translations.insert("de".to_string(), json!({"Hello": "Hallo"}));

    I18nReport {
        source_keys,
        translation_keys,
        missing_translations,
        unused_translations,
        language_stats,
        has_errors: true,
        translations,
    }
}

#[test]
fn test_yaml_formatter() {
    let report = dummy_report();
    let yaml = report.missing_as_yaml().expect("YAML formatting failed");
    assert!(yaml.contains("i18n:"));
    assert!(yaml.contains("\"World\":"));
    assert!(yaml.contains("\"Description\":"));
    assert!(yaml.contains("\"de\": \"\""));
    assert!(yaml.contains("\"en\": \"\""));
}

#[test]
fn test_json_formatter() {
    let report = dummy_report();
    let json = report.missing_as_json().expect("JSON formatting failed");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON parse failed");
    assert!(parsed.get("de").is_some());
    assert!(parsed.get("en").is_some());
    assert_eq!(parsed["de"]["World"], "");
    assert_eq!(parsed["en"]["Description"], "");
}

#[test]
fn test_human_formatter() {
    let report = dummy_report();
    let human = report.as_report().expect("Human formatting failed");
    assert!(human.contains("Translation Report"));
    assert!(human.contains("Missing Translations:"));
    assert!(human.contains("de (de) - 2 missing:"));
    assert!(human.contains("en (en) - 1 missing:"));
    assert!(human.contains("Unused Translations:"));
    assert!(human.contains("Summary:"));
    assert!(human.contains("Total missing translations: 3"));
    assert!(human.contains("❌ Missing translations"));
}

#[test]
fn test_format_with_trait_object() {
    let report = dummy_report();
    let formatter: Box<dyn I18nReportFormatter> = Box::new(I18nYamlFormatter);
    let yaml = report
        .format_with(&*formatter)
        .expect("Trait object formatting failed");
    assert!(yaml.contains("i18n:"));
}

#[test]
fn test_empty_report() {
    let report = I18nReport {
        source_keys: HashSet::new(),
        translation_keys: HashMap::new(),
        missing_translations: HashMap::new(),
        unused_translations: HashSet::new(),
        language_stats: HashMap::new(),
        has_errors: false,
        translations: HashMap::new(),
    };
    let yaml = report.missing_as_yaml().expect("YAML formatting failed");
    assert!(yaml.contains("i18n:"));
    let json = report.missing_as_json().expect("JSON formatting failed");
    assert_eq!(json, "{\n\n}");
    let human = report.as_report().expect("Human formatting failed");
    assert!(human.contains("Translation Report"));
}
