use super::JsExecutor;
use crate::challenges::CustomChallengeResult;
use wasm_bindgen::JsValue;

pub struct ResultHandler<'a> {
    js_executor: &'a JsExecutor<'a>,
}

impl<'a> ResultHandler<'a> {
    pub fn new(js_executor: &'a JsExecutor<'a>) -> Self {
        Self { js_executor }
    }

    pub fn set_result_data(&self, result_data: CustomChallengeResult) {
        let js_result_data = serde_wasm_bindgen::to_value(&result_data).unwrap();
        let konnektoren_obj = self.js_executor.get_or_create_konnektoren_object();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("result"),
            &js_result_data,
        )
        .unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::{Custom, CustomChallengeResult};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_result_handler() {
        let challenge = Custom::default();
        let window = web_sys::window().unwrap();
        let js_executor = JsExecutor::new(&window);
        let result = CustomChallengeResult::default();
        let handler = ResultHandler::new(&js_executor);
        handler.set_result_data(result.clone());
    }
}
