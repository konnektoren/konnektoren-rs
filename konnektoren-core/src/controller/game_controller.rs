use crate::commands::{Command, CommandBus};
use crate::events::{Event, EventBus};
use crate::game::Game;
use std::time::Duration;

pub struct GameController {
    game: Game,
    event_bus: EventBus,
    command_bus: CommandBus,
}

impl GameController {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            event_bus: EventBus::new(),
            command_bus: CommandBus::new(),
        }
    }
}
