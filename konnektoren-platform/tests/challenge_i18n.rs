//! Integration test that verifies all challenges in the default game path have
//! translations for every builtin language.
//!
//! Run with:
//! ```bash
//! cargo test -p konnektoren-platform --features tools --test challenge_i18n -- --nocapture
//! ```

#![cfg(feature = "tools")]

use konnektoren_core::game::GamePath;
use konnektoren_platform::i18n::JsonTranslationAsset;
use konnektoren_platform::tools::ChallengeI18nChecker;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets/challenges/i18n/"]
struct ChallengeI18nAssets;

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
