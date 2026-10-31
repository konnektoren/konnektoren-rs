//! This module contains the implementation of challenge-level commands.

use super::command::CommandTrait;
use super::command_type::CommandType;
use crate::challenges::Timed;
use crate::challenges::error::ChallengeError;
use crate::challenges::{Attempt, ChallengeType, GameChallenge};
use crate::commands::error::{CommandError, Result};
use crate::game::GamePath;
use crate::game::GameState;
use crate::game::error::GameError;

/// Represents challenge-level commands that can be executed on the game state.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeCommand<C: GameChallenge = ChallengeType> {
    Start(Attempt<C>),
    /// Command to move to the next task within a challenge.
    NextTask,
    /// Command to move to the previous task within a challenge.
    PreviousTask,
    /// Command to solve the current task with the presented option at this index.
    SolveOption(usize),
    /// Command to solve the current task with an input and move on.
    Solve(C::Input),
    /// Command to finish the challenge with a custom result.
    Finish(Option<C::Result>),
}

impl<C: GameChallenge> CommandTrait<C> for ChallengeCommand<C> {
    /// Executes the challenge command on the given game state.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the command execution failed.
    fn execute(&self, state: &mut GameState<C>) -> Result<()> {
        match self {
            ChallengeCommand::Start(challenge) => Self::start_challenge(state, challenge),
            ChallengeCommand::NextTask => Self::next_task(state),
            ChallengeCommand::PreviousTask => Self::previous_task(state),
            ChallengeCommand::SolveOption(option_index) => Self::solve_option(state, *option_index),
            ChallengeCommand::Solve(input) => Self::solve(state, input),
            ChallengeCommand::Finish(result) => Self::finish_challenge(state, result),
        }
    }

    /// Gets the type of the command.
    fn get_type(&self) -> CommandType {
        CommandType::Challenge
    }
}

impl<C: GameChallenge> ChallengeCommand<C> {
    /// Starts a new challenge with the given challenge configuration.
    fn start_challenge(state: &mut GameState<C>, challenge: &Attempt<C>) -> Result<()> {
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
    fn next_task(state: &mut GameState<C>) -> Result<()> {
        let current_game_path: &GamePath<C::Variant> = state
            .game
            .game_paths
            .get(state.current_game_path)
            .ok_or(CommandError::GameError(GameError::GamePathNotFound))?;

        let challenge_config = &current_game_path.challenges[state.current_challenge_index];
        let max_questions = challenge_config.tasks.len();

        if state.current_task_index >= max_questions - 1 {
            return Err(CommandError::ChallengeError(ChallengeError::NoMoreTasks));
        }

        // QUICKFIX: If current task wasn't answered, add a default answer to keep indices aligned
        let result_len = state
            .challenge
            .challenge_result
            .as_ref()
            .map_or(0, C::result_len);
        if result_len <= state.current_task_index {
            let default_input = state
                .challenge
                .challenge_type
                .default_input(state.current_task_index)
                .map_err(CommandError::ChallengeError)?;

            if let Some(input) = default_input {
                state
                    .challenge
                    .challenge_type
                    .record_input(&mut state.challenge.challenge_result, input)
                    .map_err(CommandError::ChallengeError)?;
            }
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
    fn previous_task(state: &mut GameState<C>) -> Result<()> {
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
    fn solve_option(state: &mut GameState<C>, option_index: usize) -> Result<()> {
        let challenge_input = state
            .challenge
            .challenge_type
            .option_input(option_index)
            .map_err(CommandError::ChallengeError)?;

        Self::solve_input(state, challenge_input)
    }

    /// Solves the current task with the given input and moves to the next task.
    fn solve(state: &mut GameState<C>, input: &C::Input) -> Result<()> {
        Self::solve_input(state, input.clone())
    }

    fn solve_input(state: &mut GameState<C>, input: C::Input) -> Result<()> {
        C::solve_task(&mut state.challenge, input, state.current_task_index)
            .map_err(CommandError::ChallengeError)?;

        // Attempt to move to the next task, but ignore "no more tasks" errors
        let _ = Self::next_task(state);

        Ok(())
    }

    /// Finishes the current challenge with a custom result.
    fn finish_challenge(state: &mut GameState<C>, custom_result: &Option<C::Result>) -> Result<()> {
        // An attempt that is already in the history was finished before; finishing it
        // again must not award XP or add history again.
        let attempt = &state.challenge;
        if state.game.challenge_history.challenges.iter().any(|done| {
            done.challenge_config.id == attempt.challenge_config.id
                && done.start_time == attempt.start_time
        }) {
            return Err(CommandError::ChallengeError(
                ChallengeError::AlreadyFinished,
            ));
        }
        if let Some(result) = custom_result {
            // A result for a different challenge is rejected, not stored.
            state
                .challenge
                .challenge_type
                .try_performance(result)
                .map_err(CommandError::ChallengeError)?;
        }
        state.challenge.update_end_time();
        // Logic to handle finishing the challenge
        if let Some(result) = custom_result {
            state.challenge.challenge_result = Some(result.clone());
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
