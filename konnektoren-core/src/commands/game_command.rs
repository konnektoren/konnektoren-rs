//! This module contains the implementation of game-level commands.

use super::command::CommandTrait;
use super::command_type::CommandType;
use crate::challenges::Timed;
use crate::commands::error::{CommandError, Result};
use crate::game::GamePath;
use crate::game::GameState;
use crate::game::error::GameError;

/// Represents game-level commands that can be executed on the game state.
#[derive(Debug, Clone, PartialEq)]
pub enum GameCommand {
    /// Command to move to the next challenge.
    NextChallenge,
    /// Command to move to the previous challenge.
    PreviousChallenge,
}

impl CommandTrait for GameCommand {
    /// Executes the game command on the given game state.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the command execution failed.
    fn execute(&self, state: &mut GameState) -> Result<()> {
        match self {
            GameCommand::NextChallenge => Self::next_challenge(state),
            GameCommand::PreviousChallenge => Self::previous_challenge(state),
        }
    }

    /// Gets the type of the command.
    fn get_type(&self) -> CommandType {
        CommandType::Game
    }
}

impl GameCommand {
    /// Moves the game state to the next challenge.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if there are no more challenges.
    pub fn next_challenge(state: &mut GameState) -> Result<()> {
        let current_game_path: &GamePath = state
            .game
            .game_paths
            .get(state.current_game_path)
            .ok_or(CommandError::GameError(GameError::GamePathNotFound))?;

        if state.current_challenge_index + 1 >= current_game_path.challenge_ids().len() {
            return Err(CommandError::GameError(GameError::InvalidGameState(
                "No more challenges".to_string(),
            )));
        }
        state.current_challenge_index += 1;

        let challenge_config = &current_game_path.challenges[state.current_challenge_index];
        state.challenge = state
            .game
            .create_challenge(&challenge_config.id)
            .map_err(CommandError::GameError)?;

        state.challenge.start();
        state.current_task_index = 0;

        Ok(())
    }

    /// Moves the game state to the previous challenge.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if there are no previous challenges.
    pub fn previous_challenge(state: &mut GameState) -> Result<()> {
        if state.current_challenge_index == 0 {
            return Err(CommandError::GameError(GameError::InvalidGameState(
                "No previous challenges".to_string(),
            )));
        }

        state.current_challenge_index -= 1;

        let current_game_path: &GamePath = state
            .game
            .game_paths
            .get(state.current_game_path)
            .ok_or(CommandError::GameError(GameError::GamePathNotFound))?;

        let challenge_config = &current_game_path.challenges[state.current_challenge_index];

        match state.game.create_challenge(&challenge_config.id) {
            Ok(challenge) => {
                state.challenge = challenge;
                state.challenge.start();
                state.current_task_index = 0;
                Ok(())
            }
            Err(err) => Err(CommandError::GameError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn next_challenge() {
        let mut state = GameState::default();
        let command = GameCommand::NextChallenge;
        let result = command.execute(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_challenge_index, 1);
    }

    #[test]
    fn previous_challenge() {
        let mut state = GameState::default();
        let command = GameCommand::NextChallenge;
        let result = command.execute(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_challenge_index, 1);
        let command = GameCommand::PreviousChallenge;
        let result = command.execute(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_challenge_index, 0);
    }

    #[test]
    fn next_challenge_no_more() {
        let mut state = GameState::default();
        let command = GameCommand::NextChallenge;
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        assert_eq!(state.current_challenge_index, 7);
        let result = command.execute(&mut state);
        assert!(result.is_err());

        // Check the error type
        if let Err(error) = result {
            match error {
                CommandError::GameError(GameError::InvalidGameState(msg)) => {
                    assert_eq!(msg, "No more challenges");
                }
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }

    #[test]
    fn previous_challenge_no_more() {
        let mut state = GameState::default();
        let command = GameCommand::PreviousChallenge;
        let result = command.execute(&mut state);
        assert!(result.is_err());

        // Check the error type
        if let Err(error) = result {
            match error {
                CommandError::GameError(GameError::InvalidGameState(msg)) => {
                    assert_eq!(msg, "No previous challenges");
                }
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }

    #[test]
    fn test_game_path_not_found() {
        let mut state = GameState::default();
        state.current_game_path = 999; // Invalid index

        let command = GameCommand::NextChallenge;
        let result = command.execute(&mut state);
        assert!(result.is_err());

        // Check the error type
        if let Err(error) = result {
            match error {
                CommandError::GameError(GameError::GamePathNotFound) => {}
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }
}
