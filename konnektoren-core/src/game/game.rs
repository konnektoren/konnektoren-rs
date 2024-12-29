use crate::challenges::{
    Challenge, ChallengeConfig, ChallengeFactory, ChallengeHistory, Performance,
};
use crate::Xp;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::GamePath;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    pub game_paths: Vec<GamePath>,
    pub challenge_factory: ChallengeFactory,
    pub challenge_history: ChallengeHistory,
    pub xp: Xp,
}

impl Default for Game {
    fn default() -> Self {
        let game_path = GamePath::default();
        Game {
            game_paths: vec![game_path],
            challenge_factory: ChallengeFactory::default(),
            challenge_history: Default::default(),
            xp: Default::default(),
        }
    }
}

impl Game {
    pub fn find_game_path_index(&self, challenge_id: &str) -> Option<usize> {
        self.game_paths.iter().position(|game_path| {
            game_path
                .challenge_ids()
                .contains(&challenge_id.to_string())
        })
    }

    pub fn create_challenge(&self, challenge_config_id: &str) -> Result<Challenge> {
        let challenge_config: Option<ChallengeConfig> =
            self.get_challenge_config(challenge_config_id);

        let challenge_config =
            challenge_config.ok_or_else(|| anyhow::anyhow!("Challenge config not found"))?;
        self.challenge_factory.create_challenge(&challenge_config)
    }

    pub fn get_challenge_config(&self, challenge_config_id: &str) -> Option<ChallengeConfig> {
        self.game_paths
            .iter()
            .map(|game_path| game_path.get_challenge_config(challenge_config_id))
            .find(|challenge_config| challenge_config.is_some())
            .flatten()
            .cloned()
    }

    pub fn calculate_xp_reward(&self, challenge: &Challenge) -> Xp {
        challenge.performance(&challenge.challenge_result)
            * challenge.stars(&challenge.challenge_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_challenge() {
        let game = Game::default();
        let challenge = game.create_challenge("unknown");
        assert!(challenge.is_err());
        assert_eq!(game.game_paths[0].challenge_ids().len(), 5);
        assert_eq!(
            game.game_paths[0].challenge_ids(),
            vec![
                "konnektoren-1",
                "konnektoren-2",
                "konnektoren-3",
                "konnektoren-4",
                "konnektoren-5"
            ]
        );
        let challenge = game.create_challenge("konnektoren-1");
        assert!(challenge.is_ok());
    }

    #[test]
    fn calculate_xp_reward() {
        let game = Game::default();
        let challenge = game.create_challenge("konnektoren-1").unwrap();
        let xp = game.calculate_xp_reward(&challenge);
        assert_eq!(xp, 0);
    }

    #[test]
    fn get_challenge_config() {
        let game = Game::default();
        let challenge_config = game.get_challenge_config("konnektoren-1");
        assert!(challenge_config.is_some());
    }

    #[test]
    fn find_game_path_index() {
        let game = Game::default();
        let index = game.find_game_path_index("konnektoren-1");
        assert_eq!(index, Some(0));
    }
}
