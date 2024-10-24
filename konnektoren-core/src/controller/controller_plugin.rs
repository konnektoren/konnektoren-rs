use super::game_controller::GameController;
use std::sync::Arc;

pub enum ControllerPluginError {
    PluginError(String),
}

pub trait ControllerPlugin {
    fn name(&self) -> &str;
    fn init(&self) -> Result<(), ControllerPluginError>;
    fn load(&self, game_controller: Arc<GameController>) -> Result<(), ControllerPluginError>;
    fn unload(&self, game_controller: Arc<GameController>) -> Result<(), ControllerPluginError>;
}
