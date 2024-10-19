use crate::commands::{Command, CommandBus, CommandTrait, CommandType};
use crate::events::EventBus;
use crate::game::Game;
use crate::game::GameState;
use crate::persistence::GameStatePersistence;
use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

pub struct GameController {
    game_state: Arc<Mutex<GameState>>,
    event_bus: EventBus,
    command_bus: CommandBus,
    persistence: Arc<dyn GameStatePersistence>,
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
        }
    }

    pub fn save_game_state(&self) -> Result<()> {
        let game_state = self.game_state.lock().unwrap();
        self.persistence.save_game_state(&game_state)
    }

    pub fn load_game_state(&self) -> Result<()> {
        let loaded_state = self.persistence.load_game_state()?;
        let mut game_state = self.game_state.lock().unwrap();
        *game_state = loaded_state;
        Ok(())
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

        controller
    }

    pub fn handle_command(&self, command: Command) {
        let mut state = self.game_state.lock().unwrap();
        if let Err(e) = command.execute(&mut state) {
            eprintln!("Error executing command: {:?}", e);
        }
    }

    pub fn publish_command(&self, command: Command) {
        self.command_bus.publish(command);
    }

    // Getter for game_state
    pub fn game_state(&self) -> &Arc<Mutex<GameState>> {
        &self.game_state
    }

    // Getter for event_bus
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    // Mutable getter for event_bus
    pub fn event_bus_mut(&mut self) -> &mut EventBus {
        &mut self.event_bus
    }

    // Getter for command_bus
    pub fn command_bus(&self) -> &CommandBus {
        &self.command_bus
    }

    // Mutable getter for command_bus
    pub fn command_bus_mut(&mut self) -> &mut CommandBus {
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
