use crate::challenges::Custom;
use wasm_bindgen::JsValue;
use web_sys::Window;

pub struct ChallengeHandler<'a> {
    window: &'a Window,
}

impl<'a> ChallengeHandler<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn set_challenge_data(&self, challenge_data: Custom) {
        let js_challenge_data = serde_wasm_bindgen::to_value(&challenge_data).unwrap();
        let konnektoren_obj =
            js_sys::Reflect::get(self.window, &JsValue::from_str("konnektoren")).unwrap();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("challenge"),
            &js_challenge_data,
        )
        .unwrap();
    }
}
