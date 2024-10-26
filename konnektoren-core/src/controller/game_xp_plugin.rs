use super::{ControllerPlugin, ControllerPluginError, GameControllerTrait};
use crate::challenges::ChallengeResult;
use crate::challenges::Performance;
use crate::commands::{ChallengeCommand, Command, CommandType};
use std::sync::Arc;

pub struct GameXpPlugin;

impl GameXpPlugin {
    fn update_game_xp(
        game_controller: Arc<dyn GameControllerTrait>,
        challenge_result: &ChallengeResult,
    ) {
        {
            let mut game_state = game_controller.game_state().lock().unwrap();
            game_state.challenge.challenge_result = challenge_result.clone();
            let performance = game_state.challenge.performance(challenge_result);
            let xp_reward = performance / 10;

            game_state.game.xp += xp_reward;
        }
        game_controller.save_game_state().unwrap();
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
                    Self::update_game_xp(game_controller_clone.clone(), &result);
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
    use crate::controller::game_controller::GameController;
    use crate::game::{Game, GameState};
    use crate::persistence::GameStatePersistence;
    use crate::prelude::{Challenge, ChallengeFactory, ChallengeType};
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

    fn create_challenge() -> (Challenge, ChallengeResult) {
        let mut challenge_factory = ChallengeFactory::new();
        let challenge_type = ChallengeType::default();
        challenge_factory.challenge_types.push(challenge_type);
        let challenge_config = ChallengeConfig {
            challenge: "konnektoren".to_string(),
            tasks: 3.into(),
            ..Default::default()
        };

        let challenge = challenge_factory
            .create_challenge(&challenge_config)
            .unwrap();

        let challenge_result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 0,
                name: "Konnektoren mit Infinitivgruppe".to_string(),
            },
            MultipleChoiceOption {
                id: 0,
                name: "Konnektoren mit Infinitivgruppe".to_string(),
            },
            MultipleChoiceOption {
                id: 0,
                name: "Konnektoren mit Infinitivgruppe".to_string(),
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

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    #[test]
    fn test_game_xp_update() {
        let game = Game::default();
        let persistence = Arc::new(MockPersistence);
        let game_controller = GameController::new(game, persistence).init();

        let plugin = GameXpPlugin;
        plugin.load(game_controller.clone()).unwrap();

        let (challenge, challenge_result) = create_challenge();
        finish_challenge(&game_controller, &challenge, &challenge_result);

        let game_state = game_controller.game_state().lock().unwrap();
        assert!(game_state.game.xp > 0, "XP should have been updated");
    }
}
