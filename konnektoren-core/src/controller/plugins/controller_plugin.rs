use crate::controller::GameControllerTrait;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ControllerPluginError {
    #[error("Plugin initialization error: {0}")]
    InitError(String),

    #[error("Plugin loading error: {0}")]
    LoadError(String),

    #[error("Plugin unloading error: {0}")]
    UnloadError(String),

    #[error("General plugin error: {0}")]
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
