use crate::challenges::challenge_config::ChallengeConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GamePath {
    pub id: String,
    pub name: String,
    pub challenges: Vec<ChallengeConfig>,
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
}
