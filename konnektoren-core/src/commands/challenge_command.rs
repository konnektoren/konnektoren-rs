//! This module contains the implementation of challenge-level commands.

use super::command::CommandTrait;
use crate::challenges::{ChallengeInput, ChallengeType, MultipleChoiceOption, Solvable};
use crate::game::GamePath;
use crate::game::GameState;
use anyhow::{anyhow, Result};

/// Represents challenge-level commands that can be executed on the game state.
#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeCommand {
    /// Command to move to the next task within a challenge.
    NextTask,
    /// Command to move to the previous task within a challenge.
    PreviousTask,
    /// Command to solve a multiple choice option.
    SolveOption(usize),
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
            ChallengeCommand::NextTask => Self::next_task(state),
            ChallengeCommand::PreviousTask => Self::previous_task(state),
            ChallengeCommand::SolveOption(option_index) => Self::solve_option(state, *option_index),
        }
    }
}

impl ChallengeCommand {
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
        let current_game_path: &GamePath = &state
            .game
            .game_paths
            .get(state.current_game_path)
            .expect("Invalid game path index");
        let challenge_config = &current_game_path.challenges[state.current_challenge_index];
        let max_questions = challenge_config.tasks.len();
        if state.current_task_index >= max_questions - 1 {
            return Err(anyhow!("No more tasks"));
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
            return Err(anyhow!("No previous tasks"));
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
                let option = match dataset.options.get(option_index) {
                    Some(option) => option,
                    None => {
                        return Err(anyhow!("Invalid option id: {}", option_index));
                    }
                };
                ChallengeInput::MultipleChoice(MultipleChoiceOption {
                    id: option.id,
                    name: option.name.clone(),
                })
            }
            _ => {
                return Err(anyhow!("Invalid challenge type"));
            }
        };
        state.challenge.solve(challenge_input)?;

        Self::next_task(state).unwrap_or_default();

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
    fn test_previous_task() {
        let mut state = GameState::default();
        state.current_game_path = 0;
        state.current_challenge_index = 0;
        state.current_task_index = 1;

        let result = ChallengeCommand::previous_task(&mut state);
        assert!(result.is_ok());
        assert_eq!(state.current_task_index, 0);
    }
}
