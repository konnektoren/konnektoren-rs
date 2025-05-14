use super::{ChallengeCommand, Command, CommandError, GameCommand};
use crate::challenges::{ChallengeResult, CustomChallengeResult};
use serde_json::Value;
use wasm_bindgen::prelude::*;

impl From<CommandError> for JsValue {
    fn from(error: CommandError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

impl TryFrom<JsValue> for Command {
    type Error = CommandError;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let obj: Value = serde_wasm_bindgen::from_value(value)
            .map_err(|e| CommandError::ParseError(e.to_string()))?;
        Command::try_from(obj)
    }
}

impl TryFrom<Value> for Command {
    type Error = CommandError;

    fn try_from(obj: Value) -> Result<Self, Self::Error> {
        match obj.get("type").and_then(|v| v.as_str()) {
            Some("Game") => Ok(Command::Game(GameCommand::try_from(obj)?)),
            Some("Challenge") => Ok(Command::Challenge(ChallengeCommand::try_from(obj)?)),
            Some(unknown_type) => Err(CommandError::UnknownCommandType(unknown_type.to_string())),
            None => Err(CommandError::MissingData),
        }
    }
}

impl TryFrom<Value> for GameCommand {
    type Error = CommandError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.get("action").and_then(|v| v.as_str()) {
            Some("NextChallenge") => Ok(GameCommand::NextChallenge),
            Some("PreviousChallenge") => Ok(GameCommand::PreviousChallenge),
            Some(unknown_action) => {
                Err(CommandError::UnknownCommandType(unknown_action.to_string()))
            }
            None => Err(CommandError::MissingData),
        }
    }
}

impl TryFrom<Value> for ChallengeCommand {
    type Error = CommandError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value.get("action").and_then(|v| v.as_str()) {
            Some("NextTask") => Ok(ChallengeCommand::NextTask),
            Some("PreviousTask") => Ok(ChallengeCommand::PreviousTask),
            Some("SolveOption") => {
                let option_index = value
                    .get("optionIndex")
                    .ok_or(CommandError::MissingData)?
                    .as_u64()
                    .ok_or_else(|| {
                        CommandError::InvalidData("optionIndex must be a number".to_string())
                    })?;
                Ok(ChallengeCommand::SolveOption(option_index as usize))
            }
            Some("Finish") => {
                let result = value.get("result").ok_or(CommandError::MissingData)?;
                let result: CustomChallengeResult = serde_json::from_value(result.clone())
                    .map_err(|e| CommandError::ParseError(e.to_string()))?;
                Ok(ChallengeCommand::Finish(Some(ChallengeResult::Custom(
                    result.clone(),
                ))))
            }
            Some(unknown_action) => {
                Err(CommandError::UnknownCommandType(unknown_action.to_string()))
            }
            None => Err(CommandError::MissingData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::ChallengeResult;

    #[test]
    fn test_parse_game_command() {
        let json = r#"{"type":"Game","action":"NextChallenge"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let command = Command::try_from(value).unwrap();
        assert_eq!(command, Command::Game(GameCommand::NextChallenge));
    }

    #[test]
    fn test_parse_challenge_command() {
        let json = r#"{"type":"Challenge","action":"NextTask"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let command = Command::try_from(value).unwrap();
        assert_eq!(command, Command::Challenge(ChallengeCommand::NextTask));
    }

    #[test]
    fn test_parse_challenge_command_with_option() {
        let json = r#"{"type":"Challenge","action":"SolveOption","optionIndex":0}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let command = Command::try_from(value).unwrap();
        assert_eq!(
            command,
            Command::Challenge(ChallengeCommand::SolveOption(0))
        );
    }

    #[test]
    fn test_parse_challenge_command_with_result() {
        let json = r#"{"type":"Challenge","action":"Finish","result":{"id":"123","performance":0.0,"data":{}}}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let command = Command::try_from(value).unwrap();
        assert_eq!(
            command,
            Command::Challenge(ChallengeCommand::Finish(Some(ChallengeResult::Custom(
                CustomChallengeResult {
                    id: "123".to_string(),
                    performance: 0.0,
                    data: serde_json::json!({}),
                }
            ))))
        );
    }

    #[test]
    fn test_parse_unknown_command() {
        let json = r#"{"type":"Unknown","action":"NextTask"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = Command::try_from(value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CommandError::UnknownCommandType("Unknown".to_string())
        );
    }

    #[test]
    fn test_parse_missing_data() {
        let json = r#"{"type":"Challenge"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = Command::try_from(value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CommandError::MissingData);
    }

    #[test]
    fn test_parse_missing_option_index() {
        let json = r#"{"type":"Challenge","action":"SolveOption"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = Command::try_from(value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CommandError::MissingData);
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{"type":"Challenge","action":"SolveOption","optionIndex":"invalid"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let result = Command::try_from(value);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::InvalidData(_)));
    }

    #[cfg(target_arch = "wasm32")]
    mod wasm_tests {
        use super::*;
        use wasm_bindgen_test::*;

        #[wasm_bindgen_test]
        fn test_parse_game_event_js() {
            let json = r#"{"type":"Game", "action":"NextChallenge"}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let command = Command::try_from(value).unwrap();
            assert_eq!(command, Command::Game(GameCommand::NextChallenge));
        }

        #[wasm_bindgen_test]
        fn test_parse_challenge_event_js() {
            let json = r#"{"type":"Challenge", "action":"NextTask"}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let command = Command::try_from(value).unwrap();
            assert_eq!(command, Command::Challenge(ChallengeCommand::NextTask));
        }

        #[wasm_bindgen_test]
        fn test_parse_challenge_event_with_option_js() {
            let json = r#"{"type":"Challenge", "action":"SolveOption", "optionIndex":0}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let command = Command::try_from(value).unwrap();
            assert_eq!(
                command,
                Command::Challenge(ChallengeCommand::SolveOption(0))
            );
        }

        #[wasm_bindgen_test]
        fn test_parse_challenge_event_with_result_js() {
            let json = r#"{"type":"Challenge", "action":"Finish", "result":{"id":"123","performance":0.0,"data":{}}}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let command = Command::try_from(value).unwrap();
            assert_eq!(
                command,
                Command::Challenge(ChallengeCommand::Finish(Some(ChallengeResult::Custom(
                    CustomChallengeResult {
                        id: "123".to_string(),
                        performance: 0.0,
                        data: serde_json::json!({}),
                    }
                ))))
            );
        }

        #[wasm_bindgen_test]
        fn test_parse_unknown_command_js() {
            let json = r#"{"type":"Unknown", "action":"NextTask"}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let result = Command::try_from(value);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                CommandError::UnknownCommandType("Unknown".to_string())
            );
        }

        #[wasm_bindgen_test]
        fn test_parse_missing_data_js() {
            let json = r#"{"type":"Challenge"}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let result = Command::try_from(value);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), CommandError::MissingData);
        }

        #[wasm_bindgen_test]
        fn test_parse_missing_option_index_js() {
            let json = r#"{"type":"Challenge", "action":"SolveOption"}"#;
            let value: JsValue = js_sys::JSON::parse(json).unwrap();
            let result = Command::try_from(value);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), CommandError::MissingData);
        }
    }
}
