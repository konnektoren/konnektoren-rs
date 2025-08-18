use super::ChallengeEvent;
use super::EventType;
use super::GameEvent;
use serde::{Deserialize, Serialize};

pub trait EventTrait {
    fn get_type(&self) -> EventType;
    fn get_action(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{ChallengeEvent, EventType, GameEvent};

    #[test]
    fn test_default_event() {
        let event = Event::default();
        assert_eq!(event, Event::Game(GameEvent::Started));
    }

    #[test]
    fn test_get_type_game() {
        let event = Event::Game(GameEvent::Started);
        assert_eq!(event.get_type(), EventType::Game);
    }

    #[test]
    fn test_get_type_challenge() {
        let event = Event::Challenge(ChallengeEvent::Started);
        assert_eq!(event.get_type(), EventType::Challenge);
    }

    #[test]
    fn test_get_action_game() {
        let event = Event::Game(GameEvent::Started);
        assert_eq!(event.get_action(), "Started");
    }

    #[test]
    fn test_get_action_challenge() {
        let event = Event::Challenge(ChallengeEvent::SolvedCorrect(2));
        assert_eq!(event.get_action(), "SolvedCorrect");
    }

    #[test]
    fn test_serde_serialization_game() {
        let event = Event::Game(GameEvent::Started);
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: Event = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, event);
    }

    #[test]
    fn test_serde_serialization_challenge() {
        let event = Event::Challenge(ChallengeEvent::SolvedIncorrect(1));
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: Event = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, event);
    }
}
