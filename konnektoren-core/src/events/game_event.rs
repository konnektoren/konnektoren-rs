use serde::{Deserialize, Serialize};

use super::event::EventTrait;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    #[default]
    Started,
}

impl EventTrait for GameEvent {
    fn get_type(&self) -> &str {
        "Game"
    }

    fn get_action(&self) -> &str {
        match self {
            GameEvent::Started => "Started",
        }
    }
}
