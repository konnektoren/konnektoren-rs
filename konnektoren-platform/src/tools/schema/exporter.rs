use konnektoren_core::challenges::ChallengeType;
use konnektoren_core::game::GamePath;

/// Output format for schema export.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SchemaFormat {
    /// Human-readable indented JSON (default).
    #[default]
    JsonPretty,
    /// Compact single-line JSON.
    JsonCompact,
}

#[derive(Debug, Default)]
pub struct SchemaExporter {
    format: SchemaFormat,
}

impl SchemaExporter {
    pub fn new() -> Self { Self::default() }
    pub fn compact(mut self) -> Self { self.format = SchemaFormat::JsonCompact; self }
    pub fn pretty(mut self) -> Self { self.format = SchemaFormat::JsonPretty; self }
    fn render(&self, value: &serde_json::Value) -> String {
        match self.format {
            SchemaFormat::JsonPretty => serde_json::to_string_pretty(value).expect("always serializable"),
            SchemaFormat::JsonCompact => serde_json::to_string(value).expect("always serializable"),
        }
    }
    pub fn challenge_type(&self) -> String { self.render(&ChallengeType::schema()) }
    pub fn game_path(&self) -> String { self.render(&GamePath::schema()) }
    pub fn all(&self) -> String {
        let bundle = serde_json::json!({
            "challengeType": ChallengeType::schema(),
            "gamePath": GamePath::schema(),
        });
        self.render(&bundle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_type_schema_is_valid_json() {
        let exporter = SchemaExporter::new();
        let schema = exporter.challenge_type();
        assert!(!schema.is_empty());
        let value: serde_json::Value =
            serde_json::from_str(&schema).expect("challenge_type schema must be valid JSON");
        assert!(value.is_object(), "schema root must be a JSON object");
    }

    #[test]
    fn test_game_path_schema_is_valid_json() {
        let exporter = SchemaExporter::new();
        let schema = exporter.game_path();
        let value: serde_json::Value =
            serde_json::from_str(&schema).expect("game_path schema must be valid JSON");
        assert!(value.is_object());
    }

    #[test]
    fn test_game_path_schema_describes_key_fields() {
        let exporter = SchemaExporter::new();
        let schema = exporter.game_path();
        assert!(schema.contains("challenges"), "schema should describe challenges");
        assert!(schema.contains("id"), "schema should describe id");
        assert!(schema.contains("name"), "schema should describe name");
    }

    #[test]
    fn test_challenge_type_schema_describes_variants() {
        let exporter = SchemaExporter::new();
        let schema = exporter.challenge_type();
        assert!(
            schema.contains("multiple-choice") || schema.contains("MultipleChoice"),
            "schema must reference the MultipleChoice variant"
        );
    }

    #[test]
    fn test_compact_output_has_no_newlines() {
        let exporter = SchemaExporter::new().compact();
        let schema = exporter.challenge_type();
        assert!(!schema.contains('\n'), "compact output must not contain newlines");
    }

    #[test]
    fn test_pretty_output_is_indented() {
        let exporter = SchemaExporter::new().pretty();
        let schema = exporter.challenge_type();
        assert!(schema.contains('\n'), "pretty output must contain newlines");
    }

    #[test]
    fn test_all_schema_bundle_has_both_keys() {
        let exporter = SchemaExporter::new();
        let all = exporter.all();
        let value: serde_json::Value =
            serde_json::from_str(&all).expect("all() output must be valid JSON");
        assert!(value.get("challengeType").is_some(), "bundle must contain challengeType");
        assert!(value.get("gamePath").is_some(), "bundle must contain gamePath");
    }

    #[test]
    fn test_compact_and_pretty_same_data() {
        let compact: serde_json::Value =
            serde_json::from_str(&SchemaExporter::new().compact().game_path()).unwrap();
        let pretty: serde_json::Value =
            serde_json::from_str(&SchemaExporter::new().pretty().game_path()).unwrap();
        assert_eq!(compact, pretty, "compact and pretty must represent the same schema");
    }
}
