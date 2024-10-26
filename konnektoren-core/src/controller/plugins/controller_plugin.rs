use crate::controller::GameControllerTrait;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum ControllerPluginError {
    PluginError(String),
}

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
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
