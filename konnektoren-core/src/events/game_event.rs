use serde::{Deserialize, Serialize};

use super::{event::EventTrait, EventType};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    Started,
}

impl Default for GameEvent {
    fn default() -> Self {
        GameEvent::Started
    }
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
