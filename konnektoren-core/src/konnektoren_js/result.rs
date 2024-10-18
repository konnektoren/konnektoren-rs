use crate::challenges::CustomChallengeResult;
use wasm_bindgen::JsValue;
use web_sys::Window;

pub struct ResultHandler<'a> {
    window: &'a Window,
}

impl<'a> ResultHandler<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn set_result_data(&self, result_data: CustomChallengeResult) {
        let js_result_data = serde_wasm_bindgen::to_value(&result_data).unwrap();
        let konnektoren_obj =
            js_sys::Reflect::get(self.window, &JsValue::from_str("konnektoren")).unwrap();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("result"),
            &js_result_data,
        )
        .unwrap();
    }
}
