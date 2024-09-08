use crate::challenges::ChallengeResult;
#[cfg(feature = "js")]
use js_sys::{Object, Reflect};
use serde::{Deserialize, Serialize};
#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChallengeEvent {
    NextTask(usize),
    PreviousTask(usize),
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    Finish(ChallengeResult),
}

#[cfg(feature = "js")]
impl From<JsValue> for ChallengeEvent {
    fn from(value: JsValue) -> ChallengeEvent {
        let obj = Object::from(value);
        let event_type = Reflect::get(&obj, &JsValue::from_str("type"))
            .unwrap()
            .as_string()
            .unwrap();

        match event_type.as_str() {
            "NextTask" => {
                let index = Reflect::get(&obj, &JsValue::from_str("index"))
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize;
                ChallengeEvent::NextTask(index)
            }
            "PreviousTask" => {
                let index = Reflect::get(&obj, &JsValue::from_str("index"))
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize;
                ChallengeEvent::PreviousTask(index)
            }
            "SolvedCorrect" => {
                let index = Reflect::get(&obj, &JsValue::from_str("index"))
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize;
                ChallengeEvent::SolvedCorrect(index)
            }
            "SolvedIncorrect" => {
                let index = Reflect::get(&obj, &JsValue::from_str("index"))
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize;
                ChallengeEvent::SolvedIncorrect(index)
            }
            "Finish" => {
                let result = Reflect::get(&obj, &JsValue::from_str("result")).unwrap();
                ChallengeEvent::Finish(ChallengeResult::Custom)
            }
            _ => panic!("Unknown event type: {}", event_type),
        }
    }
}

#[cfg(feature = "js")]
#[cfg(test)]
mod tests {
    use super::*;
    use gloo::utils::format::JsValueSerdeExt;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_challenge_event_from_js_value() {
        // Test case for NextTask
        let next_task_event = JsValue::from_serde(&serde_json::json!({
            "type": "NextTask",
            "index": 1,
        }))
        .unwrap();
        let challenge_event = ChallengeEvent::from(next_task_event);
        assert_eq!(challenge_event, ChallengeEvent::NextTask(1));

        // Test case for PreviousTask
        let prev_task_event = JsValue::from_serde(&serde_json::json!({
            "type": "PreviousTask",
            "index": 2,
        }))
        .unwrap();
        let challenge_event = ChallengeEvent::from(prev_task_event);
        assert_eq!(challenge_event, ChallengeEvent::PreviousTask(2));

        // Test case for SolvedCorrect
        let solved_correct_event = JsValue::from_serde(&serde_json::json!({
            "type": "SolvedCorrect",
            "index": 3,
        }))
        .unwrap();
        let challenge_event = ChallengeEvent::from(solved_correct_event);
        assert_eq!(challenge_event, ChallengeEvent::SolvedCorrect(3));

        // Test case for SolvedIncorrect
        let solved_incorrect_event = JsValue::from_serde(&serde_json::json!({
            "type": "SolvedIncorrect",
            "index": 4,
        }))
        .unwrap();
        let challenge_event = ChallengeEvent::from(solved_incorrect_event);
        assert_eq!(challenge_event, ChallengeEvent::SolvedIncorrect(4));

        // Test case for Finish (with a simple result)
        let finish_event = JsValue::from_serde(&serde_json::json!({
            "type": "Finish",
            "result": {} // Minimal structure for result
        }))
        .unwrap();
        let finish_challenge_event = ChallengeEvent::from(finish_event);
        assert_eq!(
            finish_challenge_event,
            ChallengeEvent::Finish(ChallengeResult::Custom)
        );
    }
}
