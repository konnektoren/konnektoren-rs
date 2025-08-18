use super::GameControllerTrait;
use super::{ControllerPlugin, ControllerPluginError};
use crate::challenges::{Challenge, ChallengeResult};
use crate::commands::{ChallengeCommand, Command, CommandType};
use crate::controller::ControllerError;
use std::sync::Arc;

pub struct ChallengeFinishPlugin;

impl ChallengeFinishPlugin {
    fn handle_challenge_finish(
        game_controller: Arc<dyn GameControllerTrait>,
        challenge: &Challenge,
        result: &ChallengeResult,
    ) -> Result<(), ControllerError> {
        {
            let mut game_state = game_controller
                .game_state()
                .lock()
                .map_err(|_| ControllerError::StateLock)?;

            if game_state.challenge.challenge_config.id != challenge.challenge_config.id {
                return Ok(());
            }

            game_state.challenge.challenge_result = result.clone();
            game_state
                .game
                .challenge_history
                .add_challenge(challenge.clone());
        }

        game_controller.save_game_state()?;
        Ok(())
    }
}

impl ControllerPlugin for ChallengeFinishPlugin {
    fn name(&self) -> &str {
        "ChallengeFinishPlugin"
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
                    let challenge = match game_controller_clone.game_state().lock() {
                        Ok(state) => state.challenge.clone(),
                        Err(_) => {
                            log::error!("Failed to lock game state in ChallengeFinishPlugin");
                            return;
                        }
                    };

                    if let Err(e) = Self::handle_challenge_finish(
                        game_controller_clone.clone(),
                        &challenge,
                        &result,
                    ) {
                        log::error!("Error in challenge finish handler: {:?}", e);
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
    fn test_handle_challenge_finish_updates_state_and_saves() {
        let mut mock_controller = MockGameControllerTrait::new();

        // Setup a challenge and result
        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        let result = ChallengeResult::default();

        // Setup game state
        let game_state = Mutex::new(crate::game::GameState {
            challenge: challenge.clone(),
            ..Default::default()
        });
        mock_controller
            .expect_game_state()
            .return_const(Arc::new(game_state));
        mock_controller
            .expect_save_game_state()
            .returning(|| Ok(()));

        // Should update state and call save_game_state
        let res = ChallengeFinishPlugin::handle_challenge_finish(
            Arc::new(mock_controller),
            &challenge,
            &result,
        );
        assert!(res.is_ok());
    }
}
