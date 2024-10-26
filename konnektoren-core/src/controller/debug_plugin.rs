use super::GameControllerTrait;
use super::{ControllerPlugin, ControllerPluginError};
use crate::commands::CommandType;
use log;
use std::sync::Arc;

#[derive(Clone)]
pub struct DebugPlugin {
    logger: Arc<dyn log::Log>,
}

impl DebugPlugin {
    pub fn new(logger: Arc<dyn log::Log>) -> Self {
        Self { logger }
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
        let logger = self.logger.clone();

        game_controller
            .command_bus()
            .subscribe(CommandType::Game, move |command| {
                logger.log(
                    &log::Record::builder()
                        .level(log::Level::Debug)
                        .target("GameCommand")
                        .args(format_args!("Command: {:?}", command))
                        .build(),
                );
            });

        let logger = self.logger.clone();

        game_controller
            .command_bus()
            .subscribe(CommandType::Challenge, move |command| {
                logger.log(
                    &log::Record::builder()
                        .level(log::Level::Debug)
                        .target("ChallengeCommand")
                        .args(format_args!("Command: {:?}", command))
                        .build(),
                );
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
    use std::sync::Mutex;

    #[derive(Default)]
    pub struct MockLogger {
        messages: Mutex<Vec<String>>,
    }

    impl MockLogger {
        pub fn get_messages(&self) -> Vec<String> {
            self.messages.lock().unwrap().clone()
        }
    }

    impl log::Log for MockLogger {
        fn enabled(&self, _metadata: &log::Metadata) -> bool {
            true
        }

        fn log(&self, record: &log::Record) {
            let mut messages = self.messages.lock().unwrap();
            messages.push(format!("{}: {}", record.level(), record.args()));
        }

        fn flush(&self) {}
    }

    #[test]
    fn test_debug_plugin() {
        let mut mock_game_controller = MockGameControllerTrait::new();

        let command_bus = CommandBus::new();

        mock_game_controller
            .expect_command_bus()
            .return_const(command_bus.clone());

        let mock_logger = Arc::new(MockLogger::default());
        let debug_plugin = DebugPlugin::new(mock_logger.clone());

        assert_eq!(debug_plugin.name(), "DebugPlugin");
        assert_eq!(debug_plugin.init(), Ok(()));
        assert_eq!(debug_plugin.load(Arc::new(mock_game_controller)), Ok(()));

        command_bus.publish(Command::Game(GameCommand::NextChallenge));

        let messages = mock_logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "DEBUG: Command: Game(NextChallenge)");
        assert!(messages.contains(&"DEBUG: Command: Game(NextChallenge)".to_string()));
    }
}
