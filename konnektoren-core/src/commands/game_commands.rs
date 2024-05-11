use multiple_choice::MultipleChoiceOption;

use crate::{
    challenges::{multiple_choice, Solvable},
    game::GameState,
    prelude::{ChallengeInput, ChallengeType},
};

use anyhow::{anyhow, Result};

use super::GameCommand;

pub struct NextChallengeCommand();

impl GameCommand for NextChallengeCommand {
    fn execute(&self, state: &mut GameState) -> Result<()> {
        if state.current_challenge_index + 1 >= state.game.game_path.challenge_ids().len() {
            return Err(anyhow!("No more challenges"));
        }
        state.current_challenge_index += 1;

        let challenge_config = &state.game.game_path.challenges[state.current_challenge_index];
        state.challenge = state
            .game
            .create_challenge(&challenge_config.id)
            .unwrap_or_default();

        Ok(())
    }
}

pub struct PreviousChallengeCommand();

impl GameCommand for PreviousChallengeCommand {
    fn execute(&self, state: &mut GameState) -> Result<()> {
        if state.current_challenge_index == 0 {
            return Err(anyhow!("No previous challenges"));
        }
        state.current_challenge_index -= 1;

        let challenge_config = &state.game.game_path.challenges[state.current_challenge_index];
        state.challenge = state
            .game
            .create_challenge(&challenge_config.id)
            .unwrap_or_default();
        Ok(())
    }
}

pub struct SolveOptionCommand {
    pub option_index: usize,
}

impl GameCommand for SolveOptionCommand {
    fn execute(&self, state: &mut GameState) -> Result<()> {
        let challenge_input = match state.challenge.challenge_type {
            ChallengeType::MultipleChoice(ref dataset) => {
                let option = match dataset.options.get(self.option_index) {
                    Some(option) => option,
                    None => {
                        return Err(anyhow!("Invalid option id: {}", self.option_index));
                    }
                };
                ChallengeInput::MultipleChoice(MultipleChoiceOption {
                    id: option.id,
                    name: option.name.clone(),
                })
            }
        };
        state.challenge.solve(challenge_input)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn next_challenge() {
        let mut state = GameState::default();
        let command = NextChallengeCommand();
        command.execute(&mut state).unwrap();
        assert_eq!(state.current_challenge_index, 1);
    }

    #[test]
    fn previous_challenge() {
        let mut state = GameState::default();
        state.current_challenge_index = 1;
        let command = PreviousChallengeCommand();
        command.execute(&mut state).unwrap();
        assert_eq!(state.current_challenge_index, 0);
    }

    #[test]
    fn previous_challenge_noop() {
        let mut state = GameState::default();
        let command = PreviousChallengeCommand();
        assert!(command.execute(&mut state).is_err());
        assert_eq!(state.current_challenge_index, 0);
    }

    #[test]
    fn next_challenge_noop() {
        let mut state = GameState::default();
        let command = NextChallengeCommand();
        command.execute(&mut state).unwrap();
        command.execute(&mut state).unwrap();
        assert!(command.execute(&mut state).is_err());
        assert_eq!(state.current_challenge_index, 2);
    }

    #[test]
    fn solve_option() {
        let mut state = GameState::default();
        let command = NextChallengeCommand();
        command.execute(&mut state).unwrap();
        let command = SolveOptionCommand { option_index: 0 };
        command.execute(&mut state).unwrap();
    }
}
