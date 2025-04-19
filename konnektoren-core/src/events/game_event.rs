use serde::{Deserialize, Serialize};

use super::{EventType, event::EventTrait};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum GameEvent {
    #[default]
    Started,
}

impl EventTrait for GameEvent {
    fn get_type(&self) -> EventType {
        EventType::Game
    }

    fn get_action(&self) -> &str {
        match self {
            GameEvent::Started => "Started",
        }
    }
}
