//! Integration tests for the manifest loading pipeline.
//!
//! These tests exercise the full round-trip:
//!   assets/konnektoren.manifest.yml  →  KonnektorenManifest  →  typed fields
//!
//! Run with:
//! ```bash
//! cargo test -p konnektoren-platform --features manifest --test manifest -- --nocapture
//! ```

use konnektoren_platform::manifest::{
    KonnektorenManifest, KonnektorenSections, Manifest, ManifestConfig, NoExtensions, Package,
};
use konnektoren_platform::{cargo_package, load_manifest};

// ── helpers ──────────────────────────────────────────────────────────────────

/// The manifest shipped with this crate, loaded the same way `load_manifest!`
/// does it in consuming crates.
fn platform_manifest() -> KonnektorenManifest {
    load_manifest!().expect("assets/konnektoren.manifest.yml must be valid")
}

// ── package defaults from Cargo.toml ─────────────────────────────────────────

#[test]
fn package_id_comes_from_cargo_toml() {
    let manifest = platform_manifest();
    // load_manifest! seeds package from cargo_package!(), so the id should
    // match the crate name declared in Cargo.toml.
    assert_eq!(manifest.package().id, "konnektoren-platform");
}

#[test]
fn package_version_is_semver() {
    let manifest = platform_manifest();
    let version = &manifest.package().version;
    assert!(
        version.contains('.'),
        "expected SemVer string, got {version:?}"
    );
}

// ── assets ────────────────────────────────────────────────────────────────────

#[test]
fn asset_path_is_set() {
    let manifest = platform_manifest();
    assert!(!manifest.asset_path().is_empty());
}

// ── game_paths ────────────────────────────────────────────────────────────────

#[test]
fn game_paths_are_non_empty() {
    let manifest = platform_manifest();
    assert!(
        !manifest.game_paths().is_empty(),
        "expected at least one game path"
    );
}

#[test]
fn game_paths_are_yaml_files() {
    let manifest = platform_manifest();
    for path in manifest.game_paths() {
        assert!(
            path.ends_with(".yml") || path.ends_with(".yaml"),
            "game_path {path:?} should be a YAML file"
        );
    }
}

// ── domain ────────────────────────────────────────────────────────────────────

#[test]
fn platform_manifest_has_domain() {
    let manifest = platform_manifest();
    assert!(
        manifest.domain().is_some(),
        "default manifest should declare a domain section"
    );
}

#[test]
fn domain_hostname_is_set() {
    let manifest = platform_manifest();
    let domain = manifest.domain().unwrap();
    assert!(!domain.hostname.is_empty());
}

// ── i18n ──────────────────────────────────────────────────────────────────────

#[test]
fn i18n_path_is_set() {
    let manifest = platform_manifest();
    assert!(!manifest.i18n().path.is_empty());
}

#[test]
fn i18n_default_language_is_set() {
    let manifest = platform_manifest();
    assert!(!manifest.i18n().default_language.is_empty());
}

#[test]
fn i18n_languages_are_non_empty() {
    let manifest = platform_manifest();
    assert!(
        !manifest.i18n().languages.is_empty(),
        "default manifest should list supported languages"
    );
}

// ── figment overlay ───────────────────────────────────────────────────────────

#[test]
fn figment_yaml_overrides_package_defaults() {
    let yaml = r#"
domain:
  code: overlay-test
  name: Overlay Test
  hostname: overlay.example.com
game_paths:
  - level_a1.yml
"#;
    let manifest: KonnektorenManifest =
        Manifest::<KonnektorenSections>::figment(cargo_package!(), yaml)
            .extract()
            .expect("figment extraction must succeed");

    // Package comes from cargo_package!() (this crate's Cargo.toml)
    assert_eq!(manifest.package().id, "konnektoren-platform");
    // Domain comes from YAML
    assert_eq!(manifest.domain().unwrap().hostname, "overlay.example.com");
    // i18n defaults are still present even though yaml didn't set them
    assert_eq!(manifest.i18n().default_language, "en");
}

#[test]
fn figment_works_with_no_extensions() {
    let yaml = r#"
assets:
  path: my/challenges
game_paths:
  - a.yml
  - b.yml
"#;
    let manifest: Manifest<NoExtensions> = Manifest::<NoExtensions>::figment(
        Package {
            id: "test".into(),
            name: "Test".into(),
            ..Default::default()
        },
        yaml,
    )
    .extract()
    .expect("figment extraction must succeed");

    assert_eq!(manifest.asset_path(), "my/challenges");
    assert_eq!(manifest.game_paths().len(), 2);
}
