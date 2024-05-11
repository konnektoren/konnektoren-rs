use crate::game::GameState;

use super::GameCommand;

pub struct NextChallengeCommand();

impl GameCommand for NextChallengeCommand {
    fn execute(&self, state: &mut GameState) -> Result<(), String> {
        if state.current_challenge_index + 1 >= state.game.game_path.challenge_ids().len() {
            return Err("No more challenges".to_string());
        }
        state.current_challenge_index += 1;
        Ok(())
    }
}

pub struct PreviousChallengeCommand();

impl GameCommand for PreviousChallengeCommand {
    fn execute(&self, state: &mut GameState) -> Result<(), String> {
        if state.current_challenge_index == 0 {
            return Err("No previous challenges".to_string());
        }
        state.current_challenge_index -= 1;
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
}
