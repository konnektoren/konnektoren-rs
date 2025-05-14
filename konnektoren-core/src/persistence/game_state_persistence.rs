use crate::game::GameState;
use crate::persistence::error::Result;

pub trait GameStatePersistence: Send + Sync {
    fn save_game_state(&self, state: &GameState) -> Result<()>;
    fn load_game_state(&self) -> Result<GameState>;
}
