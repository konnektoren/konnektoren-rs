use super::EventParseError;
use super::{ChallengeEvent, Event, GameEvent};
use serde_json::Value;
use wasm_bindgen::prelude::*;

impl From<EventParseError> for JsValue {
    fn from(error: EventParseError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl TryFrom<JsValue> for Event {
    type Error = EventParseError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let obj: Value = serde_wasm_bindgen::from_value(value)
            .map_err(|e| EventParseError::ParseError(e.to_string()))?;
        Event::try_from(obj)
    }
}

impl TryFrom<Value> for Event {
    type Error = EventParseError;

    fn try_from(obj: Value) -> Result<Self, Self::Error> {
        match obj.get("type").and_then(|v| v.as_str()) {
            Some("Game") => Ok(Event::Game(GameEvent::try_from(obj)?)),
            Some("Challenge") => Ok(Event::Challenge(ChallengeEvent::try_from(obj)?)),
            Some(unknown_type) => Err(EventParseError::UnknownEventType(unknown_type.to_string())),
            None => Err(EventParseError::MissingData),
        }
    }
}

impl TryFrom<Value> for GameEvent {
    type Error = EventParseError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.get("action").and_then(|v| v.as_str()) {
            Some("Started") => Ok(GameEvent::Started),
            Some(unknown_action) => Err(EventParseError::UnknownEventType(
                unknown_action.to_string(),
            )),
            None => Err(EventParseError::MissingData),
        }
    }
}

impl TryFrom<Value> for ChallengeEvent {
    type Error = EventParseError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.get("action").and_then(|v| v.as_str()) {
            Some("SolvedCorrect") => {
                let index = value
                    .get("index")
                    .ok_or(EventParseError::MissingData)?
                    .as_u64()
                    .ok_or_else(|| {
                        EventParseError::InvalidData("index must be a number".to_string())
                    })?;
                Ok(ChallengeEvent::SolvedCorrect(index as usize))
            }
            Some("SolvedIncorrect") => {
                let index = value
                    .get("index")
                    .ok_or(EventParseError::MissingData)?
                    .as_u64()
                    .ok_or_else(|| {
                        EventParseError::InvalidData("index must be a number".to_string())
                    })?;
                Ok(ChallengeEvent::SolvedIncorrect(index as usize))
            }
            Some("Started") => Ok(ChallengeEvent::Started),
            Some("Completed") => Ok(ChallengeEvent::Completed),
            Some(unknown_action) => Err(EventParseError::UnknownEventType(
                unknown_action.to_string(),
            )),
            None => Err(EventParseError::MissingData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_event() {
        let json = r#"{"type":"Game","action":"Started"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let event = Event::try_from(value).unwrap();
        assert_eq!(event, Event::Game(GameEvent::Started));
    }

    #[test]
    fn test_parse_challenge_event() {
        let json = r#"{"type":"Challenge","action":"SolvedCorrect","index":2}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let event = Event::try_from(value).unwrap();
        assert_eq!(event, Event::Challenge(ChallengeEvent::SolvedCorrect(2)));
    }

    #[test]
    fn test_parse_challenge_event_solved_incorrect() {
        let json = r#"{"type":"Challenge","action":"SolvedIncorrect","index":2}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let event = Event::try_from(value).unwrap();
        assert_eq!(event, Event::Challenge(ChallengeEvent::SolvedIncorrect(2)));
    }

    #[test]
    fn test_parse_challenge_event_started() {
        let json = r#"{"type":"Challenge","action":"Started"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let event = Event::try_from(value).unwrap();
        assert_eq!(event, Event::Challenge(ChallengeEvent::Started));
    }

    #[test]
    fn test_parse_challenge_event_completed() {
        let json = r#"{"type":"Challenge","action":"Completed"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let event = Event::try_from(value).unwrap();
        assert_eq!(event, Event::Challenge(ChallengeEvent::Completed));
    }
}
