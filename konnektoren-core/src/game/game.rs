use crate::challenges::{Challenge, ChallengeFactory, ChallengeHistory, Performance};
use crate::Xp;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::GamePath;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Game {
    pub game_path: GamePath,
    pub challenge_factory: ChallengeFactory,
    pub challenge_history: ChallengeHistory,
}

impl Game {
    pub fn create_challenge(&self, challenge_config_id: &str) -> Result<Challenge> {
        let challenge_config = self
            .game_path
            .get_challenge_config(challenge_config_id)
            .ok_or_else(|| anyhow::anyhow!("Challenge config not found"))?;
        self.challenge_factory.create_challenge(challenge_config)
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
        assert_eq!(game.game_path.challenge_ids().len(), 5);
        assert_eq!(
            game.game_path.challenge_ids(),
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
}
