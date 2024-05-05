pub mod game_path;
use crate::challenges::{Challenge, ChallengeFactory, ChallengeHistory, Performance};
use crate::Xp;
use anyhow::Result;
pub use game_path::GamePath;

#[derive(Debug, Default, Clone)]
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
        Ok(self.challenge_factory.create_challenge(challenge_config)?)
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
        assert_eq!(game.game_path.challenge_ids().len(), 2);
        assert_eq!(
            game.game_path.challenge_ids(),
            vec!["konnektoren-1", "konnektoren-2"]
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
