use super::GameControllerTrait;
use super::{ControllerPlugin, ControllerPluginError};
use crate::commands::CommandType;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct DebugPlugin;

impl DebugPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl ControllerPlugin for DebugPlugin {
    fn name(&self) -> &str {
        "DebugPlugin"
    }
    fn init(&self) -> Result<(), ControllerPluginError> {
        Ok(())
    }
    fn load(
        &self,
        game_controller: Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError> {
        game_controller
            .command_bus()
            .subscribe(CommandType::Game, move |command| {
                tracing::debug!(target: "GameCommand", "Command: {:?}", command);
            });

        game_controller
            .command_bus()
            .subscribe(CommandType::Challenge, move |command| {
                tracing::debug!(target: "ChallengeCommand", "Command: {:?}", command);
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
mod test {
    use crate::commands::{Command, CommandBus, GameCommand};
    use crate::controller::game_controller::MockGameControllerTrait;

    use super::*;

    #[test]
    fn test_debug_plugin() {
        let mut mock_game_controller = MockGameControllerTrait::new();

        let command_bus = CommandBus::new();

        mock_game_controller
            .expect_command_bus()
            .return_const(command_bus.clone());

        let debug_plugin = DebugPlugin::new();

        assert_eq!(debug_plugin.name(), "DebugPlugin");
        assert_eq!(debug_plugin.init(), Ok(()));
        assert_eq!(debug_plugin.load(Arc::new(mock_game_controller)), Ok(()));

        command_bus.publish(Command::Game(GameCommand::NextChallenge));
    }
}
