//! Integration test that verifies all challenges in the default game path have
//! translations for every builtin language.
//!
//! Run with:
//! ```bash
//! cargo test -p konnektoren-platform --features tools --test challenge_i18n -- --nocapture
//! ```

use konnektoren_core::game::GamePath;
use konnektoren_platform::i18n::JsonTranslationAsset;
use konnektoren_platform::tools::ChallengeI18nChecker;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets/challenges/i18n/"]
struct ChallengeI18nAssets;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets/challenges/i18n/"]
#[include = "konnektoren_path_de.json"]
struct DeOnlyAssets;

#[test]
fn konnektoren_path_all_langs_translated() {
    let asset = JsonTranslationAsset::<ChallengeI18nAssets>::new();
    let report = ChallengeI18nChecker::new(&asset).check(&GamePath::default());

    if report.has_errors {
        for (lang, keys) in &report.missing {
            println!("[{lang}] missing {} key(s):", keys.len());
            for key in keys {
                println!("  - {key:?}");
            }
        }
    }

    report.assert_complete();
}

#[test]
fn missing_translations_detected_when_only_de_loaded() {
    let asset = JsonTranslationAsset::<DeOnlyAssets>::new();
    let report = ChallengeI18nChecker::new(&asset).check(&GamePath::default());

    assert!(
        report.has_errors,
        "expected missing translations for all languages except de"
    );
    assert!(
        !report.missing.contains_key("de"),
        "de should be fully translated"
    );

    for lang in ["en", "es", "ar", "zh", "uk", "pl", "tr", "vi"] {
        assert!(
            report.missing.contains_key(lang),
            "{lang} should have missing translations"
        );
    }
}
