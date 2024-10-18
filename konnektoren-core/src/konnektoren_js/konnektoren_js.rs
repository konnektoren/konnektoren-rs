use crate::challenges::{Custom, CustomChallengeResult};
use wasm_bindgen::JsValue;

use super::{
    ChallengeHandler, CommandHandler, EventHandler, I18nHandler, JsExecutor, ResultHandler,
};

use web_sys::Window;

pub struct KonnektorenJs {
    challenge: ChallengeHandler<'static>,
    command: CommandHandler<'static>,
    event: EventHandler<'static>,
    i18n: I18nHandler<'static>,
    js_executor: JsExecutor,
    result: ResultHandler<'static>,
}

impl KonnektorenJs {
    pub fn new(window: &Window) -> Self {
        let window = window.clone();

        // Ensure that `window.konnektoren` exists, or create it if it doesn't.
        if js_sys::Reflect::has(&window, &JsValue::from_str("konnektoren")).unwrap() == false {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&window, &JsValue::from_str("konnektoren"), &obj).unwrap();
        }

        // Use a static reference to the cloned window for handlers
        let static_window: &'static Window = Box::leak(Box::new(window.clone()));

        Self {
            challenge: ChallengeHandler::new(static_window),
            command: CommandHandler::new(static_window),
            event: EventHandler::new(static_window),
            i18n: I18nHandler::new(static_window),
            js_executor: JsExecutor::new(),
            result: ResultHandler::new(static_window),
        }
    }

    // Implement the public methods using the handlers
    pub fn set_challenge_data(&self, challenge_data: Custom) {
        self.challenge.set_challenge_data(challenge_data);
    }

    pub fn set_result_data(&self, result_data: CustomChallengeResult) {
        self.result.set_result_data(result_data);
    }

    pub fn expose_send_event<F>(&self, on_event: F)
    where
        F: 'static + FnMut(JsValue),
    {
        self.event.expose_send_event(on_event);
    }

    pub fn expose_execute_command<F>(&self, on_command: F)
    where
        F: 'static + FnMut(JsValue),
    {
        self.command.expose_execute_command(on_command);
    }

    pub fn execute_js(&self, js_code: &str) {
        self.js_executor.execute(js_code);
    }

    pub fn set_i18n_data(&mut self, i18n_data: serde_json::Value) {
        self.i18n.set_i18n_data(i18n_data);
    }

    pub fn finish_challenge(&self, result: Option<CustomChallengeResult>) {
        self.command.finish_challenge(result);
    }
}
