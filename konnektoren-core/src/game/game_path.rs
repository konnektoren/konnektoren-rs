use crate::challenges::challenge_config::ChallengeConfig;
use crate::game::Map;
#[cfg(feature = "schema")]
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct GamePath {
    pub id: String,
    pub name: String,
    pub challenges: Vec<ChallengeConfig>,
    pub map: Option<Map>,
}

impl Default for GamePath {
    fn default() -> Self {
        let data = include_str!("../../assets/konnektoren_path.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

impl GamePath {
    pub fn get_challenge_config(&self, challenge_config_id: &str) -> Option<&ChallengeConfig> {
        self.challenges
            .iter()
            .find(|challenge| challenge.id == challenge_config_id)
    }

    pub fn challenge_ids(&self) -> Vec<String> {
        self.challenges
            .iter()
            .map(|challenge| challenge.id.clone())
            .collect()
    }

    pub fn next_challenge_id(&self, challenge_id: &str) -> Option<String> {
        let mut iter = self.challenges.iter();
        while let Some(challenge) = iter.next() {
            if challenge.id == challenge_id {
                return iter.next().map(|c| c.id.clone());
            }
        }
        None
    }
}

#[cfg(feature = "schema")]
impl GamePath {
    /// Get the JSON schema for `GamePath` (for documentation / sharing with third parties)
    pub fn schema() -> serde_json::Value {
        let schema = schema_for!(GamePath);
        serde_json::to_value(schema).expect("schemars Schema is always JSON-serializable")
    }

    /// Get the JSON schema as a pretty-printed string
    pub fn schema_json() -> String {
        serde_json::to_string_pretty(&Self::schema())
            .expect("serde_json::Value is always serializable to string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_game_path() {
        let game_path = GamePath::default();
        assert_eq!(game_path.name, "Konnektoren");
        assert!(!game_path.challenges.is_empty());
    }

    #[test]
    fn get_challenge_config() {
        let game_path = GamePath::default();
        let challenge_config = game_path.get_challenge_config("unknown");
        assert!(challenge_config.is_none());
        let challenge_config = game_path.get_challenge_config("konnektoren-1");
        assert!(challenge_config.is_some());
    }

    #[test]
    fn test_challenge_ids() {
        let game_path = GamePath::default();
        let challenge_ids = game_path.challenge_ids();
        assert_eq!(challenge_ids.len(), game_path.challenges.len());
    }

    #[test]
    fn test_next_challenge_id() {
        let game_path = GamePath::default();
        let next_challenge_id = game_path.next_challenge_id("konnektoren-1");
        assert_eq!(next_challenge_id, Some("konnektoren-2".to_string()));
    }

    #[test]
    fn test_game_path_json_round_trip() {
        let game_path = GamePath::default();

        // Serialize to JSON
        let json = serde_json::to_string(&game_path)
            .expect("GamePath should serialize to JSON");
        assert!(!json.is_empty());

        // Deserialize back
        let from_json: GamePath = serde_json::from_str(&json)
            .expect("GamePath should deserialize from JSON");
        assert_eq!(game_path, from_json);
    }

    #[cfg(feature = "schema")]
    #[test]
    fn test_game_path_json_schema() {
        let schema = GamePath::schema();
        assert!(schema.is_object());

        let schema_str = serde_json::to_string_pretty(&schema)
            .expect("schema should serialize to string");
        // The schema must describe the key fields
        assert!(schema_str.contains("challenges"), "schema should mention 'challenges'");
        assert!(schema_str.contains("id"), "schema should mention 'id'");

        // schema_json convenience method should produce the same output
        let schema_json = GamePath::schema_json();
        assert!(!schema_json.is_empty());
    }
}
