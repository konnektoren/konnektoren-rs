use super::JsExecutor;
use crate::challenges::{Custom, CustomChallengeState};
use wasm_bindgen::JsValue;

pub struct ChallengeHandler<'a> {
    js_executor: &'a JsExecutor<'a>,
}

impl<'a> ChallengeHandler<'a> {
    pub fn new(js_executor: &'a JsExecutor<'a>) -> Self {
        Self { js_executor }
    }

    pub fn set_challenge_data(&self, challenge_data: Custom) {
        let js_challenge_data = serde_wasm_bindgen::to_value(&challenge_data).unwrap();
        let konnektoren_obj = self.js_executor.get_or_create_konnektoren_object();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("challenge"),
            &js_challenge_data,
        )
        .unwrap();
    }

    pub fn set_challenge_state(&self, state: &CustomChallengeState) {
        let js_state = serde_wasm_bindgen::to_value(&state).unwrap();
        let konnektoren_obj = self.js_executor.get_or_create_konnektoren_object();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("challengeState"),
            &js_state,
        )
        .unwrap();
    }

    pub fn get_challenge_state(&self) -> Option<CustomChallengeState> {
        let konnektoren_obj = self.js_executor.get_or_create_konnektoren_object();
        let state =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("challengeState")).ok()?;

        serde_wasm_bindgen::from_value(state).ok()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_challenge_handler() {
        let challenge = Custom::default();
        let window = web_sys::window().unwrap();
        let js_executor = JsExecutor::new(&window);
        let handler = ChallengeHandler::new(&js_executor);
        handler.set_challenge_data(challenge.clone());
        handler.set_challenge_state(&CustomChallengeState::default());
        let state = handler.get_challenge_state().unwrap();
        assert_eq!(state.is_finished, false);
    }
}
