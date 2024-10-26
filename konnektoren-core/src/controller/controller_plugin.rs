use super::GameControllerTrait;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum ControllerPluginError {
    PluginError(String),
}

pub trait ControllerPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn init(&self) -> Result<(), ControllerPluginError>;
    fn load(
        &self,
        game_controller: Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError>;
    fn unload(
        &self,
        game_controller: Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError>;
}
