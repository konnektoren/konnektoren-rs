use crate::controller::game_controller::GameControllerTrait;
use crate::controller::{ControllerPlugin, ControllerPluginError};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PluginManager {
    plugins: HashMap<String, Arc<dyn ControllerPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Arc<dyn ControllerPlugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    pub fn init_plugins(&mut self) -> Result<(), ControllerPluginError> {
        for (_, plugin) in self.plugins.iter() {
            plugin.init()?;
        }
        Ok(())
    }

    pub fn load_plugins(
        &self,
        game_controller: &Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError> {
        for plugin in self.plugins.values() {
            plugin.load(game_controller.clone())?;
        }
        Ok(())
    }

    pub fn unload_plugins(
        &self,
        game_controller: &Arc<dyn GameControllerTrait>,
    ) -> Result<(), ControllerPluginError> {
        for (_, plugin) in self.plugins.iter() {
            plugin.unload(game_controller.clone())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::plugins::controller_plugin::MockControllerPlugin;
    use crate::controller::GameController;
    use crate::controller::GameControllerTrait;
    use crate::game::Game;
    use crate::persistence::MemoryPersistence;

    #[test]
    fn test_plugin_manager() {
        let game = Game::default();
        let persistence = Arc::new(MemoryPersistence::default());
        let controller = GameController::new(game, persistence).init();

        let mut plugin_manager = PluginManager::new();
        let mut mocked_plugin = MockControllerPlugin::new();
        mocked_plugin
            .expect_name()
            .return_const("MockedPlugin".to_string());
        mocked_plugin.expect_init().returning(|| Ok(()));
        mocked_plugin.expect_load().returning(|_| Ok(()));
        mocked_plugin.expect_unload().returning(|_| Ok(()));
        let plugin = Arc::new(mocked_plugin);
        plugin_manager.add_plugin(plugin);

        let game_controller = controller as Arc<dyn GameControllerTrait>;

        assert_eq!(plugin_manager.init_plugins().is_ok(), true);
        assert_eq!(plugin_manager.load_plugins(&game_controller).is_ok(), true);
        assert_eq!(
            plugin_manager.unload_plugins(&game_controller).is_ok(),
            true
        );
    }
}
