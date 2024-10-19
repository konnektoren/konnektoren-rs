use super::ChallengeEvent;
use super::EventType;
use super::GameEvent;
use serde::{Deserialize, Serialize};

pub trait EventTrait {
    fn get_type(&self) -> EventType;
    fn get_action(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Game(GameEvent),
    Challenge(ChallengeEvent),
}

impl Default for Event {
    fn default() -> Self {
        Event::Game(GameEvent::default())
    }
}

impl EventTrait for Event {
    fn get_type(&self) -> EventType {
        match self {
            Event::Game(_) => EventType::Game,
            Event::Challenge(_) => EventType::Challenge,
        }
    }

    fn get_action(&self) -> &str {
        match self {
            Event::Game(event) => event.get_action(),
            Event::Challenge(event) => event.get_action(),
        }
    }
}
