use serde::{Deserialize, Serialize};

use crate::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub game: Game,
}

impl GameState {
    pub fn new(game: Game) -> Self {
        GameState { game }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            game: Game::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state() {
        let state = GameState::default();
        assert_eq!(state.game.game_path.challenge_ids().len(), 3);
    }
}
