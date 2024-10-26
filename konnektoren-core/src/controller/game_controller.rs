use super::ControllerPlugin;
use crate::commands::{Command, CommandBus, CommandTrait, CommandType};
use crate::events::EventBus;
use crate::game::Game;
use crate::game::GameState;
use crate::persistence::GameStatePersistence;
use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait GameControllerTrait: Send + Sync {
    fn save_game_state(&self) -> Result<()>;
    fn load_game_state(&self) -> Result<()>;

    fn handle_command(&self, command: Command);
    fn publish_command(&self, command: Command);

    // Getters for internal components
    fn game_state(&self) -> &Arc<Mutex<GameState>>;
    fn event_bus(&self) -> &EventBus;
    fn event_bus_mut(&mut self) -> &mut EventBus;
    fn command_bus(&self) -> &CommandBus;
    fn command_bus_mut(&mut self) -> &mut CommandBus;
}

pub struct GameController {
    game_state: Arc<Mutex<GameState>>,
    event_bus: EventBus,
    command_bus: CommandBus,
    persistence: Arc<dyn GameStatePersistence>,
    plugins: Vec<Arc<dyn ControllerPlugin>>,
}

impl PartialEq for GameController {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.game_state, &other.game_state)
    }
}

impl Debug for GameController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameController").finish()
    }
}

impl GameController {
    pub fn new(game: Game, persistence: Arc<dyn GameStatePersistence>) -> Self {
        let game_state = Arc::new(Mutex::new(GameState::new(game)));
        let event_bus = EventBus::new();
        let command_bus = CommandBus::new();

        Self {
            game_state,
            event_bus,
            command_bus,
            persistence,
            plugins: vec![],
        }
    }

    #[must_use]
    pub fn init(self) -> Arc<Self> {
        let controller = Arc::new(self);

        let controller_clone = Arc::clone(&controller);
        controller
            .command_bus
            .subscribe(CommandType::Game, move |command| {
                controller_clone.handle_command(command);
            });

        let controller_clone = Arc::clone(&controller);
        controller
            .command_bus
            .subscribe(CommandType::Challenge, move |command| {
                controller_clone.handle_command(command);
            });

        for plugin in &controller.plugins {
            match plugin.init() {
                Ok(_) => {}
                Err(e) => log::error!("Error initializing plugin: {:?}", e),
            }
        }

        let controller_clone = Arc::clone(&controller);

        for plugin in &controller.plugins {
            plugin.load(controller_clone.clone()).unwrap();
        }

        controller
    }

    pub fn register_plugin(&mut self, plugin: Arc<dyn ControllerPlugin>) {
        self.plugins.push(plugin);
    }
}

impl GameControllerTrait for GameController {
    fn save_game_state(&self) -> Result<()> {
        let game_state = self.game_state.lock().unwrap();
        self.persistence.save_game_state(&game_state)
    }

    fn load_game_state(&self) -> Result<()> {
        let loaded_state = self.persistence.load_game_state()?;
        let mut game_state = self.game_state.lock().unwrap();
        *game_state = loaded_state;
        Ok(())
    }

    fn handle_command(&self, command: Command) {
        let mut state = self.game_state.lock().unwrap();
        if let Err(e) = command.execute(&mut state) {
            eprintln!("Error executing command: {:?}", e);
        }
    }

    fn publish_command(&self, command: Command) {
        self.command_bus.publish(command);
    }

    // Getter for game_state
    fn game_state(&self) -> &Arc<Mutex<GameState>> {
        &self.game_state
    }

    // Getter for event_bus
    fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    // Mutable getter for event_bus
    fn event_bus_mut(&mut self) -> &mut EventBus {
        &mut self.event_bus
    }

    // Getter for command_bus
    fn command_bus(&self) -> &CommandBus {
        &self.command_bus
    }

    // Mutable getter for command_bus
    fn command_bus_mut(&mut self) -> &mut CommandBus {
        &mut self.command_bus
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::GameCommand;
    use crate::persistence::MemoryPersistence;

    #[test]
    fn test_handle_command() {
        let game = Game::default();
        let persistence = Arc::new(MemoryPersistence::default());
        let controller = GameController::new(game, persistence).init();

        let command = Command::Game(GameCommand::NextChallenge);
        controller.publish_command(command);

        let game_state = controller.game_state.lock().unwrap();
        assert_eq!(game_state.current_challenge_index, 1);
    }

    #[test]
    fn test_save_and_load_game_state() {
        let game = Game::default();
        let persistence = Arc::new(MemoryPersistence::default());
        let controller = GameController::new(game, persistence).init();

        // Modify the game state
        let command = Command::Game(GameCommand::NextChallenge);
        controller.publish_command(command);

        // Save the game state
        let save_result = controller.save_game_state();
        assert!(save_result.is_ok());

        // Modify the game state again
        let command = Command::Game(GameCommand::NextChallenge);
        controller.publish_command(command);

        // Load the saved game state
        let load_result = controller.load_game_state();
        assert!(load_result.is_ok());

        // Check if the loaded state matches the saved state
        let game_state = controller.game_state.lock().unwrap();
        assert_eq!(game_state.current_challenge_index, 1);
    }

    #[test]
    fn test_getters() {
        let game = Game::default();
        let persistence = Arc::new(MemoryPersistence::default());
        let mut controller = GameController::new(game, persistence);

        // Using immutable getters
        let game_state = controller.game_state();
        let event_bus = controller.event_bus();
        let command_bus = controller.command_bus();

        // Check immutable getters
        assert_eq!(game_state.lock().unwrap().current_challenge_index, 0);
        assert_eq!(event_bus.listeners.lock().unwrap().len(), 0);
        assert_eq!(command_bus.listeners.lock().unwrap().len(), 0);

        // Using mutable getters
        {
            let event_bus_mut = controller.event_bus_mut();
            assert_eq!(event_bus_mut.listeners.lock().unwrap().len(), 0);
        }
        {
            let command_bus_mut = controller.command_bus_mut();
            assert_eq!(command_bus_mut.listeners.lock().unwrap().len(), 0);
        }
    }
}
