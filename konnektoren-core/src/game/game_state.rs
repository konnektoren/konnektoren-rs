use serde::{Deserialize, Serialize};

use crate::challenges::{ChallengeType, GameChallenge};
use crate::game::error::GameError;
use crate::{game::Game, prelude::Challenge};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "C: Serialize, C::Result: Serialize",
    deserialize = "C: Deserialize<'de>, C::Result: Deserialize<'de>"
))]
pub struct GameState<C: GameChallenge = ChallengeType> {
    pub game: Game<C>,
    pub challenge: Challenge<C, C::Result>,
    pub current_game_path: usize,
    pub current_challenge_index: usize,
    pub current_task_index: usize,
}

impl<C: GameChallenge> GameState<C> {
    /// Start at the first challenge of the first game path.
    pub fn try_new(game: Game<C>) -> Result<Self, GameError> {
        let first_id = game
            .game_paths
            .first()
            .and_then(|path| path.challenges.first())
            .map(|config| config.id.clone())
            .ok_or(GameError::GamePathNotFound)?;
        let challenge = game.create_challenge(&first_id)?;

        Ok(GameState {
            game,
            challenge,
            current_game_path: 0,
            current_challenge_index: 0,
            current_task_index: 0,
        })
    }

    /// Like [`GameState::try_new`], panicking when the game has no challenge to start with.
    pub fn new(game: Game<C>) -> Self {
        Self::try_new(game).expect("game must start with a creatable challenge")
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
        assert_eq!(state.game.game_paths[0].challenge_ids().len(), 10);
    }

    #[test]
    fn default_state() {
        let state = GameState::default();
        assert_eq!(state.current_challenge_index, 0);

        let challenge = state.game.create_challenge("konnektoren-1").unwrap();
        assert_eq!(state.challenge, challenge);
    }
}
