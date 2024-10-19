use crate::game::GameState;
use anyhow::Result;

pub trait GameStatePersistence {
    fn save_game_state(&self, state: &GameState) -> Result<()>;
    fn load_game_state(&self) -> Result<GameState>;
}
