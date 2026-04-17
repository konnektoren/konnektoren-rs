//! Integration test that writes JSON schema files to `schemas/` for human review.
//!
//! Run with:
//! ```bash
//! cargo test -p konnektoren-platform --features tools,schema --test schema_export -- --nocapture
//! ```
//!
//! Output files:
//! - `konnektoren-platform/schemas/challenge_type.schema.json`
//! - `konnektoren-platform/schemas/game_path.schema.json`
//! - `konnektoren-platform/schemas/all.schema.json`

#![cfg(all(feature = "tools", feature = "schema"))]

use konnektoren_platform::tools::SchemaExporter;
use std::path::PathBuf;

fn schemas_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas")
}

#[test]
fn generate_challenge_type_schema() {
    let dir = schemas_dir();
    std::fs::create_dir_all(&dir).expect("should create schemas/ directory");

    let path = dir.join("challenge_type.schema.json");
    let content = SchemaExporter::new().challenge_type();
    std::fs::write(&path, &content).expect("should write challenge_type.schema.json");

    // Sanity-check: file is non-empty and parses as a JSON object.
    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("challenge_type schema must be valid JSON");
    assert!(parsed.is_object());

    println!("wrote {}", path.display());
}

#[test]
fn generate_game_path_schema() {
    let dir = schemas_dir();
    std::fs::create_dir_all(&dir).expect("should create schemas/ directory");

    let path = dir.join("game_path.schema.json");
    let content = SchemaExporter::new().game_path();
    std::fs::write(&path, &content).expect("should write game_path.schema.json");

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("game_path schema must be valid JSON");
    assert!(parsed.is_object());

    println!("wrote {}", path.display());
}

#[test]
fn generate_all_schemas() {
    let dir = schemas_dir();
    std::fs::create_dir_all(&dir).expect("should create schemas/ directory");

    let path = dir.join("all.schema.json");
    let content = SchemaExporter::new().all();
    std::fs::write(&path, &content).expect("should write all.schema.json");

    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("all schema bundle must be valid JSON");
    assert!(parsed.get("challengeType").is_some());
    assert!(parsed.get("gamePath").is_some());

    println!("wrote {}", path.display());
    println!(
        "\nReview schema files in:\n  {}",
        dir.display()
    );
}
