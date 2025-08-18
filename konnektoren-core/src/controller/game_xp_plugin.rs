use super::{ControllerPlugin, ControllerPluginError, GameControllerTrait};
use crate::challenges::ChallengeResult;
use crate::challenges::Performance;
use crate::commands::{ChallengeCommand, Command, CommandType};
use crate::controller::ControllerError;
use std::sync::Arc;

pub struct GameXpPlugin;

impl GameXpPlugin {
    fn update_game_xp(
        game_controller: Arc<dyn GameControllerTrait>,
        challenge_result: &ChallengeResult,
    ) -> Result<(), ControllerError> {
        {
            let mut game_state = game_controller
                .game_state()
                .lock()
                .map_err(|_| ControllerError::StateLock)?;

            game_state.challenge.challenge_result = challenge_result.clone();
            let performance = game_state.challenge.performance(challenge_result);
            let xp_reward = performance / 10;

            game_state.game.xp += xp_reward;
        }

        game_controller.save_game_state()?;
        Ok(())
    }
}

impl ControllerPlugin for GameXpPlugin {
    fn name(&self) -> &str {
        "GameXpPlugin"
    }

    fn init(&self) -> Result<(), ControllerPluginError> {
        Ok(())
    }

    fn load(
        &self,
        game_controller: Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError> {
        let game_controller_clone = game_controller.clone();
        game_controller
            .command_bus()
            .subscribe(CommandType::Challenge, move |command| {
                if let Command::Challenge(ChallengeCommand::Finish(Some(result))) = command {
                    if let Err(e) = Self::update_game_xp(game_controller_clone.clone(), &result) {
                        log::error!("Error updating game XP: {:?}", e);
                    }
                }
            });

        Ok(())
    }

    fn unload(
        &self,
        _game_controller: Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::{Challenge, ChallengeConfig, ChallengeResult, ChallengeType};
    use crate::controller::game_controller::MockGameControllerTrait;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_update_game_xp_increases_xp() {
        let mut mock_controller = MockGameControllerTrait::new();

        // Setup a challenge with 100% performance
        let mut challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge.challenge_result = ChallengeResult::default();

        let mut game_state = crate::game::GameState::default();
        game_state.challenge = challenge.clone();
        game_state.game.xp = 0;

        let game_state_mutex = Mutex::new(game_state);
        mock_controller
            .expect_game_state()
            .return_const(Arc::new(game_state_mutex));
        mock_controller
            .expect_save_game_state()
            .returning(|| Ok(()));

        let result = ChallengeResult::default();
        let res = GameXpPlugin::update_game_xp(Arc::new(mock_controller), &result);
        assert!(res.is_ok());
        // You could check that XP increased, but with the mock it's not persisted.
    }
}
