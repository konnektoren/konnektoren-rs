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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_game_path() {
        let game_path = GamePath::default();
        assert_eq!(game_path.name, "Konnektoren");
        assert!(!game_path.challenges.is_empty());
    }
}
