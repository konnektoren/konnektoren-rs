use crate::challenges::CustomChallengeResult;
use wasm_bindgen::prelude::*;
use web_sys::Window;

pub struct CommandHandler<'a> {
    window: &'a Window,
}

impl<'a> CommandHandler<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn expose_execute_command<F>(&self, on_command: F)
    where
        F: 'static + FnMut(JsValue),
    {
        let closure = Closure::wrap(Box::new(on_command) as Box<dyn FnMut(JsValue)>);
        let konnektoren_obj =
            js_sys::Reflect::get(self.window, &JsValue::from_str("konnektoren")).unwrap();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("executeCommand"),
            closure.as_ref().unchecked_ref(),
        )
        .unwrap();
        closure.forget();
    }

    pub fn finish_challenge(&self, result: Option<CustomChallengeResult>) {
        let konnektoren_obj =
            js_sys::Reflect::get(self.window, &JsValue::from_str("konnektoren")).unwrap();
        let execute_command =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("executeCommand")).unwrap();
        let command = serde_json::json!({
            "type": "Challenge",
            "action": "Finish",
            "result": result
        });
        let command_js = serde_wasm_bindgen::to_value(&command).unwrap();
        execute_command
            .dyn_ref::<js_sys::Function>()
            .unwrap()
            .call1(&JsValue::NULL, &command_js)
            .unwrap();
    }
}
