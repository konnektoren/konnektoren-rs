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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_event() {
        let event = GameEvent::default();
        assert_eq!(event, GameEvent::Started);
    }

    #[test]
    fn test_get_type() {
        let event = GameEvent::Started;
        assert_eq!(event.get_type(), EventType::Game);
    }

    #[test]
    fn test_get_action() {
        let event = GameEvent::Started;
        assert_eq!(event.get_action(), "Started");
    }

    #[test]
    fn test_serde_serialization() {
        let event = GameEvent::Started;
        let serialized = serde_json::to_string(&event).unwrap();
        assert_eq!(serialized, "\"Started\"");
        let deserialized: GameEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, GameEvent::Started);
    }
}
