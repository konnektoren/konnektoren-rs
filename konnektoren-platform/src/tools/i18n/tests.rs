use super::*;
use crate::i18n::{CombinedTranslationAsset, I18nAssets, I18nConfig, Language};
use crate::i18n_patterns;

use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;

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
    assert!(human.contains("(de) - 2 missing:"));
    assert!(human.contains("(en) - 1 missing:"));
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

// --- I18nChecker tests ---

fn make_config(translations: &[(&str, serde_json::Value)]) -> I18nConfig {
    let mut config = I18nConfig::default();
    for (lang, trans) in translations {
        config.merge_translation(&Language::from(*lang), trans.clone());
    }
    config
}

#[test]
fn checker_detects_keys_from_i18n_t() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("main.rs"),
        r#"i18n.t("Hello") i18n.t("World")"#,
    )
    .unwrap();

    let config = make_config(&[
        ("en", json!({"Hello": "Hello", "World": "World"})),
        ("de", json!({"Hello": "Hallo", "World": "Welt"})),
    ]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(report.source_keys.contains("Hello"));
    assert!(report.source_keys.contains("World"));
    assert_eq!(report.source_keys.len(), 2);
}

#[test]
fn checker_detects_keys_from_t_with_lang() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), r#"t_with_lang("Greeting")"#).unwrap();

    let config = make_config(&[("en", json!({"Greeting": "Hello"}))]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(report.source_keys.contains("Greeting"));
}

#[test]
fn checker_no_errors_when_all_translated() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), r#"i18n.t("Save")"#).unwrap();

    let config = make_config(&[
        ("en", json!({"Save": "Save"})),
        ("de", json!({"Save": "Speichern"})),
    ]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    // "Save" must not be missing for en or de
    assert!(
        report
            .missing_translations
            .get("en")
            .map_or(true, |v| !v.contains(&"Save".to_string()))
    );
    assert!(
        report
            .missing_translations
            .get("de")
            .map_or(true, |v| !v.contains(&"Save".to_string()))
    );
}

#[test]
fn checker_reports_missing_translation_for_language() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), r#"i18n.t("Cancel")"#).unwrap();

    // "Cancel" only translated in English, missing in German
    let config = make_config(&[("en", json!({"Cancel": "Cancel"}))]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(report.has_errors);
    let missing_de = report.missing_translations.get("de");
    assert!(
        missing_de.map_or(false, |v| v.contains(&"Cancel".to_string())),
        "Expected 'Cancel' to be missing for 'de'"
    );
}

#[test]
fn checker_reports_unused_translations() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("lib.rs"), r#"i18n.t("Used")"#).unwrap();

    let config = make_config(&[("en", json!({"Used": "Used", "Unused": "Unused"}))]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(report.unused_translations.contains("Unused"));
    assert!(!report.unused_translations.contains("Used"));
}

#[test]
fn checker_empty_directory_no_errors() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_config(&[("en", json!({}))]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(report.source_keys.is_empty());
    assert!(!report.has_errors);
}

#[test]
fn checker_ignores_non_rust_files() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("readme.md"), r#"i18n.t("ShouldBeIgnored")"#).unwrap();

    let config = make_config(&[("en", json!({}))]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(!report.source_keys.contains("ShouldBeIgnored"));
    assert!(report.source_keys.is_empty());
}

#[test]
fn checker_scans_subdirectories_recursively() {
    let dir = tempfile::tempdir().unwrap();
    let sub = dir.path().join("components");
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("button.rs"), r#"i18n.t("Submit")"#).unwrap();

    let config = make_config(&[
        ("en", json!({"Submit": "Submit"})),
        ("de", json!({"Submit": "Absenden"})),
    ]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    assert!(report.source_keys.contains("Submit"));
    assert!(
        report
            .missing_translations
            .get("en")
            .map_or(true, |v| !v.contains(&"Submit".to_string()))
    );
    assert!(
        report
            .missing_translations
            .get("de")
            .map_or(true, |v| !v.contains(&"Submit".to_string()))
    );
}

#[test]
fn checker_coverage_percentage_is_correct() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("lib.rs"),
        r#"i18n.t("A") i18n.t("B") i18n.t("C") i18n.t("D")"#,
    )
    .unwrap();

    // German has 3 out of 4
    let config = make_config(&[
        ("en", json!({"A": "A", "B": "B", "C": "C", "D": "D"})),
        ("de", json!({"A": "A", "B": "B", "C": "C"})),
    ]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    let de_stats = report.language_stats.get("de").unwrap();
    assert_eq!(de_stats.total_keys, 4);
    assert_eq!(de_stats.missing_keys, 1);
    assert!((de_stats.coverage_percentage - 75.0).abs() < 0.01);
}

#[test]
fn checker_full_coverage_has_no_errors() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_config(&[("en", json!({}))]);
    let report = I18nChecker::new(config).check_directory(dir.path());

    let en_stats = report.language_stats.get("en").unwrap();
    assert_eq!(en_stats.coverage_percentage, 100.0);
}

// --- i18n_patterns! macro + with_patterns tests ---

#[test]
fn i18n_patterns_macro_generates_patterns() {
    let patterns = i18n_patterns!["i18n.t", "t_with_lang"];
    assert_eq!(patterns.len(), 2);
}

#[test]
fn i18n_patterns_macro_matches_method_call() {
    let patterns = i18n_patterns!["i18n.t"];
    let caps: Vec<_> = patterns[0]
        .captures_iter(r#"i18n.t("Hello")"#)
        .map(|c| c[1].to_string())
        .collect();
    assert_eq!(caps, vec!["Hello"]);
}

#[test]
fn i18n_patterns_macro_does_not_match_wrong_receiver() {
    let patterns = i18n_patterns!["i18n.t"];
    // "config.t" should NOT be matched by a pattern for "i18n.t"
    assert!(!patterns[0].is_match(r#"config.t("Hello")"#));
}

#[test]
fn i18n_patterns_macro_escapes_dot() {
    // Without escaping, "i18nXt" would match "i18n.t" pattern (. matches any char)
    let patterns = i18n_patterns!["i18n.t"];
    assert!(!patterns[0].is_match(r#"i18nXt("Hello")"#));
}

#[test]
fn with_patterns_scans_custom_calling_convention() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("lib.rs"),
        r#"config.t("CustomKey")"#,
    )
    .unwrap();

    let config = make_config(&[("en", json!({"CustomKey": "Custom"}))]);
    let report = I18nChecker::with_patterns(config, i18n_patterns!["config.t"])
        .check_directory(dir.path());

    assert!(report.source_keys.contains("CustomKey"));
}

#[test]
fn with_patterns_multiple_conventions_finds_all_keys() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("lib.rs"),
        r#"i18n.t("KeyA") config.translate("KeyB")"#,
    )
    .unwrap();

    let config = make_config(&[("en", json!({"KeyA": "A", "KeyB": "B"}))]);
    let report =
        I18nChecker::with_patterns(config, i18n_patterns!["i18n.t", "config.translate"])
            .check_directory(dir.path());

    assert!(report.source_keys.contains("KeyA"));
    assert!(report.source_keys.contains("KeyB"));
}

#[test]
fn with_patterns_does_not_pick_up_default_pattern() {
    let dir = tempfile::tempdir().unwrap();
    // Only the default i18n.t call — but checker uses a custom pattern only
    fs::write(dir.path().join("lib.rs"), r#"i18n.t("ShouldNotAppear")"#).unwrap();

    let config = make_config(&[("en", json!({}))]);
    let report =
        I18nChecker::with_patterns(config, i18n_patterns!["config.t"])
            .check_directory(dir.path());

    assert!(!report.source_keys.contains("ShouldNotAppear"));
}

// --- Self-coverage integration test ---

#[test]
fn platform_src_has_no_missing_translations() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let config = I18nConfig::with_assets(
        CombinedTranslationAsset::<I18nAssets>::new("i18n.yml"),
    );
    let report = I18nChecker::new(config).exclude_tests().check_directory(&src);

    if report.has_errors {
        eprintln!("{}", report.as_report().expect("report format failed"));
        panic!("missing translations in konnektoren-platform/src — see report above");
    }
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
    assert_eq!(json, "{}");
    let human = report.as_report().expect("Human formatting failed");
    assert!(human.contains("Translation Report"));
}
