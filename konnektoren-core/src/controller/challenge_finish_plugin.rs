use super::GameControllerTrait;
use super::{ControllerPlugin, ControllerPluginError};
use crate::challenges::{Challenge, ChallengeResult};
use crate::commands::{ChallengeCommand, Command, CommandType};
use std::sync::Arc;

pub struct ChallengeFinishPlugin;

impl ChallengeFinishPlugin {
    fn handle_challenge_finish(
        game_controller: Arc<dyn GameControllerTrait>,
        challenge: &Challenge,
        result: &ChallengeResult,
    ) {
        {
            let mut game_state = game_controller.game_state().lock().unwrap();
            if game_state.challenge.challenge_config.id != challenge.challenge_config.id {
                return;
            }

            game_state.challenge.challenge_result = result.clone();
            game_state
                .game
                .challenge_history
                .add_challenge(challenge.clone());
        }

        game_controller.save_game_state().unwrap();
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
                    let challenge = {
                        let game_state = game_controller_clone.game_state().lock().unwrap();
                        game_state.challenge.clone()
                    };
                    ChallengeFinishPlugin::handle_challenge_finish(
                        game_controller_clone.clone(),
                        &challenge,
                        &result,
                    );
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
    use crate::challenges::{ChallengeConfig, ChallengeResult, MultipleChoiceOption};
    use crate::commands::{ChallengeCommand, Command};
    use crate::controller::game_controller::GameController;
    use crate::game::{Game, GameState};
    use crate::persistence::GameStatePersistence;
    use crate::prelude::ChallengeType;
    use anyhow::Result;
    use std::sync::Arc;

    struct MockPersistence;

    impl GameStatePersistence for MockPersistence {
        fn save_game_state(&self, _game_state: &GameState) -> Result<()> {
            Ok(())
        }

        fn load_game_state(&self) -> Result<GameState> {
            Ok(GameState::default())
        }
    }

    fn setup_game_controller() -> Arc<GameController> {
        let game = Game::default();
        let persistence = Arc::new(MockPersistence);
        GameController::new(game, persistence).init()
    }

    fn create_challenge(id: &str) -> (Challenge, ChallengeResult) {
        let mut config = ChallengeConfig::default();
        config.id = id.to_string();
        config.tasks = 3.into();

        let challenge = Challenge::new(&ChallengeType::default(), &config);
        let challenge_result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
            MultipleChoiceOption {
                id: 3,
                name: "Option 3".to_string(),
            },
        ]);

        (challenge, challenge_result)
    }

    fn finish_challenge(
        game_controller: &Arc<GameController>,
        challenge: &Challenge,
        result: &ChallengeResult,
    ) {
        {
            let mut game_state = game_controller.game_state().lock().unwrap();
            game_state.challenge = challenge.clone();
        }

        game_controller
            .command_bus()
            .publish(Command::Challenge(ChallengeCommand::Finish(Some(
                result.clone(),
            ))));

        // Wait a short time for the command to be processed
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    #[test]
    fn test_challenge_finish_plugin() {
        let game_controller = setup_game_controller();
        let plugin = ChallengeFinishPlugin;
        plugin.load(game_controller.clone()).unwrap();

        let (challenge, challenge_result) = create_challenge("test_challenge");
        finish_challenge(&game_controller, &challenge, &challenge_result);

        let game_state = game_controller.game_state().lock().unwrap();
        assert_eq!(
            game_state.challenge.challenge_config.name,
            challenge.challenge_config.name
        );
        assert_eq!(game_state.challenge.challenge_result, challenge_result);
    }

    #[test]
    fn test_challenge_history_update() {
        let game_controller = setup_game_controller();
        let plugin = ChallengeFinishPlugin;
        plugin.load(game_controller.clone()).unwrap();

        let (challenge, challenge_result) = create_challenge("test_challenge");

        {
            let game_state = game_controller.game_state().lock().unwrap();
            assert_eq!(game_state.game.challenge_history.len(), 0);
        }

        finish_challenge(&game_controller, &challenge, &challenge_result);

        let game_state = game_controller.game_state().lock().unwrap();

        // Check that the challenge history has been updated
        assert_eq!(game_state.game.challenge_history.len(), 1);
        let history_challenge = game_state
            .game
            .challenge_history
            .challenges
            .first()
            .unwrap();
        assert_eq!(history_challenge.challenge_config.id, "test_challenge");
        assert_eq!(history_challenge.challenge_result, challenge_result);
    }
}
