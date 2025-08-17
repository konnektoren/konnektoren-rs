//! This module contains the implementation of challenge-level commands.

use super::command::CommandTrait;
use super::command_type::CommandType;
use crate::challenges::Timed;
use crate::challenges::error::ChallengeError;
use crate::challenges::{
    Challenge, ChallengeInput, ChallengeResult, ChallengeType, MultipleChoiceOption, Solvable,
};
use crate::commands::error::{CommandError, Result};
use crate::game::GamePath;
use crate::game::GameState;
use crate::game::error::GameError;

/// Represents challenge-level commands that can be executed on the game state.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeCommand {
    Start(Challenge),
    /// Command to move to the next task within a challenge.
    NextTask,
    /// Command to move to the previous task within a challenge.
    PreviousTask,
    /// Command to solve a multiple choice option.
    SolveOption(usize),
    /// Command to finish the challenge with a custom result.
    Finish(Option<ChallengeResult>),
}

impl CommandTrait for ChallengeCommand {
    /// Executes the challenge command on the given game state.
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
            ChallengeCommand::Start(challenge_type) => Self::start_challenge(state, challenge_type),
            ChallengeCommand::NextTask => Self::next_task(state),
            ChallengeCommand::PreviousTask => Self::previous_task(state),
            ChallengeCommand::SolveOption(option_index) => Self::solve_option(state, *option_index),
            ChallengeCommand::Finish(result) => Self::finish_challenge(state, result),
        }
    }

    /// Gets the type of the command.
    fn get_type(&self) -> CommandType {
        CommandType::Challenge
    }
}

impl ChallengeCommand {
    /// Starts a new challenge with the given challenge configuration.
    fn start_challenge(state: &mut GameState, challenge: &Challenge) -> Result<()> {
        let mut challenge = challenge.clone();
        challenge.start();
        state.challenge = challenge;
        state.current_task_index = 0;
        Ok(())
    }

    /// Moves the game state to the next task within the current challenge.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if there are no more tasks.
    fn next_task(state: &mut GameState) -> Result<()> {
        let current_game_path: &GamePath = state
            .game
            .game_paths
            .get(state.current_game_path)
            .ok_or(CommandError::GameError(GameError::GamePathNotFound))?;

        let challenge_config = &current_game_path.challenges[state.current_challenge_index];
        let max_questions = challenge_config.tasks.len();

        if state.current_task_index >= max_questions - 1 {
            return Err(CommandError::ChallengeError(ChallengeError::NoMoreTasks));
        }

        state.current_task_index += 1;
        Ok(())
    }

    /// Moves the game state to the previous task within the current challenge.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if there are no previous tasks.
    fn previous_task(state: &mut GameState) -> Result<()> {
        if state.current_task_index == 0 {
            return Err(CommandError::ChallengeError(
                ChallengeError::NoPreviousTasks,
            ));
        }
        state.current_task_index -= 1;
        Ok(())
    }

    /// Solves the current task with the selected option and moves to the next task.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    /// * `option_index` - The index of the selected option.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the solution is invalid.
    fn solve_option(state: &mut GameState, option_index: usize) -> Result<()> {
        let challenge_input = match state.challenge.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let option =
                    dataset
                        .options
                        .get(option_index)
                        .ok_or(CommandError::ChallengeError(
                            ChallengeError::InvalidOptionId(option_index),
                        ))?;

                ChallengeInput::MultipleChoice(MultipleChoiceOption {
                    id: option.id,
                    name: option.name.clone(),
                })
            }
            _ => {
                return Err(CommandError::ChallengeError(
                    ChallengeError::InvalidChallengeType,
                ));
            }
        };

        state
            .challenge
            .solve(challenge_input)
            .map_err(CommandError::ChallengeError)?;

        // Attempt to move to the next task, but ignore "no more tasks" errors
        let _ = Self::next_task(state);

        Ok(())
    }

    /// Finishes the current challenge with a custom result.
    fn finish_challenge(
        state: &mut GameState,
        custom_result: &Option<ChallengeResult>,
    ) -> Result<()> {
        state.challenge.update_end_time();
        // Logic to handle finishing the challenge
        if let Some(result) = custom_result {
            state.challenge.challenge_result = result.clone();
        }
        // Additional logic to mark the challenge as finished
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn test_solve_option() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 0;

        let result = ChallengeCommand::solve_option(&mut state, 0);
        assert!(result.is_ok());
        assert_eq!(state.current_task_index, 1);
    }

    #[test]
    fn test_solve_option_invalid() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 0;

        // Try to solve with an invalid option index
        let result = ChallengeCommand::solve_option(&mut state, 999);
        assert!(result.is_err());

        // Verify error type
        if let Err(error) = result {
            match error {
                CommandError::ChallengeError(ChallengeError::InvalidOptionId(999)) => {}
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }

    #[test]
    fn test_next_task() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 0;

        let result = ChallengeCommand::next_task(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_task_index, 1);
    }

    #[test]
    fn test_next_task_no_more() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;

        // Set to last task
        let current_game_path = &state.game.game_paths[state.current_game_path];
        let challenge_config = &current_game_path.challenges[state.current_challenge_index];
        let max_tasks = challenge_config.tasks.len();
        state.current_task_index = max_tasks - 1;

        let result = ChallengeCommand::next_task(&mut state);
        assert!(result.is_err());

        // Verify error type
        if let Err(error) = result {
            match error {
                CommandError::ChallengeError(ChallengeError::NoMoreTasks) => {}
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }

    #[test]
    fn test_previous_task() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 1;

        let result = ChallengeCommand::previous_task(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_task_index, 0);
    }

    #[test]
    fn test_previous_task_no_more() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 0;

        let result = ChallengeCommand::previous_task(&mut state);
        assert!(result.is_err());

        // Verify error type
        if let Err(error) = result {
            match error {
                CommandError::ChallengeError(ChallengeError::NoPreviousTasks) => {}
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }

    #[test]
    fn test_finish_challenge() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 1;

        let result = ChallengeCommand::finish_challenge(&mut state, &None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 0;

        let result = ChallengeCommand::SolveOption(0).execute(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_task_index, 1);
    }

    #[test]
    fn test_invalid_challenge_type() {
        // Create a challenge with a non-multiple-choice type
        let mut state = GameState::default();

        // Modify challenge type to something other than MultipleChoice
        // This is a bit of a hack for testing - ideally we'd create a proper challenge with a different type
        state.challenge.challenge_type = ChallengeType::Informative(Default::default());

        let result = ChallengeCommand::solve_option(&mut state, 0);
        assert!(result.is_err());

        // Verify error type
        if let Err(error) = result {
            match error {
                CommandError::ChallengeError(ChallengeError::InvalidChallengeType) => {}
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }

    #[test]
    fn test_game_path_not_found() {
        let mut state = GameState::default();
        state.current_game_path = 999; // Invalid index

        let result = ChallengeCommand::next_task(&mut state);
        assert!(result.is_err());

        // Verify error type
        if let Err(error) = result {
            match error {
                CommandError::GameError(GameError::GamePathNotFound) => {}
                _ => panic!("Unexpected error type: {:?}", error),
            }
        }
    }
}
