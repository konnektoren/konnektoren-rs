pub mod game_path;
use crate::challenges::{Challenge, ChallengeFactory, Performance};
use crate::Xp;
use anyhow::Result;
pub use game_path::GamePath;

pub struct Game {
    pub game_path: GamePath,
    pub challenge_factory: ChallengeFactory,
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
