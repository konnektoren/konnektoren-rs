use serde::{Deserialize, Serialize};

use crate::{game::Game, prelude::Challenge};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub game: Game,
    pub challenge: Challenge,
    pub current_game_path: usize,
    pub current_challenge_index: usize,
    pub current_task_index: usize,
}

impl GameState {
    pub fn new(game: Game) -> Self {
        let challenge = game.create_challenge("konnektoren-1").unwrap();

        GameState {
            game,
            challenge,
            current_game_path: 0,
            current_challenge_index: 0,
            current_task_index: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new(Game::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state() {
        let state = GameState::default();
        assert_eq!(state.game.game_paths[0].challenge_ids().len(), 7);
    }

    #[test]
    fn default_state() {
        let state = GameState::default();
        assert_eq!(state.current_challenge_index, 0);

        let challenge = state.game.create_challenge("konnektoren-1").unwrap();
        assert_eq!(state.challenge, challenge);
    }
}
