use crate::challenges::challenge_config::ChallengeConfig;
use crate::game::Map;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GamePath {
    pub id: String,
    pub name: String,
    pub challenges: Vec<ChallengeConfig>,
    pub map: Option<Map>,
}

impl Default for GamePath {
    fn default() -> Self {
        let data = include_str!("../assets/konnektoren_path.yml");
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
}
