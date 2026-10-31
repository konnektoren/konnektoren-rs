use crate::challenges::{ChallengeType, GameChallenge};
use crate::game::GameState;
use crate::persistence::error::Result;

pub trait GameStatePersistence<C: GameChallenge = ChallengeType>: Send + Sync {
    fn save_game_state(&self, state: &GameState<C>) -> Result<()>;
    fn load_game_state(&self) -> Result<GameState<C>>;
}
