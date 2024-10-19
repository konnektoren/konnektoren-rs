use super::ChallengeEvent;
use super::GameEvent;
use serde::{Deserialize, Serialize};

pub trait EventTrait {
    fn get_type(&self) -> &str;
    fn get_action(&self) -> &str;
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Event {
    #[default]
    Game(GameEvent),
    Challenge(ChallengeEvent),
}

impl EventTrait for Event {
    fn get_type(&self) -> &str {
        match self {
            Event::Game(_) => "Game",
            Event::Challenge(_) => "Challenge",
        }
    }

    fn get_action(&self) -> &str {
        match self {
            Event::Game(event) => event.get_action(),
            Event::Challenge(event) => event.get_action(),
        }
    }
}
