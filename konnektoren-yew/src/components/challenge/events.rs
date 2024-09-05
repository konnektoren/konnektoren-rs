use js_sys::{Object, Reflect};
use konnektoren_core::challenges::ChallengeResult;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChallengeEvent {
    NextTask(usize),
    PreviousTask(usize),
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    Finish(ChallengeResult),
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use gloo::utils::format::JsValueSerdeExt;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_challenge_event_from_js_value() {
        // Test case for NextTask
        let event = JsValue::from_serde(&serde_json::json!({
            "type": "NextTask",
            "index": 1,
        }))
        .unwrap();
        let challenge_event = ChallengeEvent::from(event);
        assert_eq!(challenge_event, ChallengeEvent::NextTask(1));

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
