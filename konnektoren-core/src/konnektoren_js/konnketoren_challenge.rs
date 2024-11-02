use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::challenges::Custom;
use crate::challenges::CustomChallengeState;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct KonnektorenChallenge {
    pub id: String,
    pub challenge_type: String,
    pub state: CustomChallengeState,
    //pub data: serde_json::Value,
}

#[wasm_bindgen]
impl KonnektorenChallenge {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Result<KonnektorenChallenge, JsValue> {
        let config: serde_json::Value = serde_wasm_bindgen::from_value(config)?;

        Ok(Self {
            id: config["id"].as_str().unwrap_or_default().to_string(),
            challenge_type: config["type"].as_str().unwrap_or_default().to_string(),
            state: CustomChallengeState::new(),
            data: serde_wasm_bindgen::to_value(&config["data"]).unwrap(),
        })
    }

    #[wasm_bindgen]
    pub fn initialize(&self) -> Result<(), JsValue> {
        // Get window.konnektoren
        let window = web_sys::window().unwrap();
        let konnektoren = js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren"))?;

        // Set initial state
        let set_state = js_sys::Reflect::get(&konnektoren, &JsValue::from_str("setState"))?;
        let state_js = serde_wasm_bindgen::to_value(&self.state)?;
        js_sys::Reflect::apply(
            set_state.unchecked_ref(),
            &konnektoren,
            &js_sys::Array::of1(&state_js),
        )?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn calculate_performance(&self) -> f64 {
        let questions = js_sys::Reflect::get(&self.data, &JsValue::from_str("questions")).unwrap();
        let questions_length = js_sys::Array::from(&questions).length() as f64;

        if questions_length > 0.0 {
            self.state.correct_answers as f64 / questions_length
        } else {
            0.0
        }
    }

    #[wasm_bindgen]
    pub fn finish(&mut self) -> Result<(), JsValue> {
        self.state.end_time = Some(js_sys::Date::now());
        self.state.is_finished = true;

        let window = web_sys::window().unwrap();
        let konnektoren = js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren"))?;

        // Update state
        let set_state = js_sys::Reflect::get(&konnektoren, &JsValue::from_str("setState"))?;
        let state_js = serde_wasm_bindgen::to_value(&self.state)?;
        js_sys::Reflect::apply(
            set_state.unchecked_ref(),
            &konnektoren,
            &js_sys::Array::of1(&state_js),
        )?;

        // Create result
        let result = serde_json::json!({
            "id": self.id,
            "performance": self.calculate_performance(),
            "data": {
                "answers": self.state.user_answers,
                "timeSpent": self.state.end_time.unwrap() - self.state.start_time.unwrap()
            }
        });

        // Execute finish command
        let execute_command =
            js_sys::Reflect::get(&konnektoren, &JsValue::from_str("executeCommand"))?;
        let command = serde_json::json!({
            "type": "Challenge",
            "action": "Finish",
            "result": result
        });

        js_sys::Reflect::apply(
            execute_command.unchecked_ref(),
            &konnektoren,
            &js_sys::Array::of1(&JsValue::from_serde(&command).unwrap()),
        )?;

        Ok(())
    }
}
