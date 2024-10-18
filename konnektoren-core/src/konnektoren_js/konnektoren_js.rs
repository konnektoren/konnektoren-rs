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
    js_executor: JsExecutor<'static>,
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
            js_executor: JsExecutor::new(static_window),
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

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    fn get_window() -> Window {
        web_sys::window().expect("no global `window` exists")
    }
    #[wasm_bindgen_test]
    fn test_set_challenge_data() {
        let window = get_window();
        let konnektoren_js = KonnektorenJs::new(&window);

        let test_custom = Custom {
            id: "123".to_string(),
            name: "Test".to_string(),
            description: "".to_string(),
            html: "".to_string(),
            results_html: None,
            css: "".to_string(),
            js: "".to_string(),
            i18n: None,
            data: serde_json::json!({"key":"value"}),
            package_url: None,
        };
        konnektoren_js.set_challenge_data(test_custom.clone());

        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();

        let challenge_data =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("challenge")).unwrap();

        let js_challenge_data: Custom = challenge_data.into_serde().unwrap();

        assert_eq!(js_challenge_data.id, test_custom.id);
        assert_eq!(js_challenge_data.name, test_custom.name);
    }

    #[wasm_bindgen_test]
    fn test_expose_send_event() {
        let window = get_window();
        let konnektoren_js = KonnektorenJs::new(&window);

        let event_received = std::rc::Rc::new(std::cell::RefCell::new(false));

        {
            let event_received_clone = event_received.clone();

            konnektoren_js.expose_send_event(move |event: JsValue| {
                *event_received_clone.borrow_mut() = true;
                log::info!("Received event: {:?}", event);
            });
        }

        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();
        let send_event_func =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("sendEvent")).unwrap();

        let event_data = JsValue::from_serde(&serde_json::json!({ "type": "TestEvent" })).unwrap();
        let send_event_func = send_event_func.dyn_ref::<js_sys::Function>().unwrap();
        send_event_func.call1(&JsValue::NULL, &event_data).unwrap();

        assert!(*event_received.borrow());
    }

    #[wasm_bindgen_test]
    fn test_set_result_data() {
        let window = get_window();
        let konnektoren_js = KonnektorenJs::new(&window);

        let test_result = CustomChallengeResult {
            id: "123".to_string(),
            performance: 0.0,
            data: serde_json::json!({"key":"value"}),
        };
        konnektoren_js.set_result_data(test_result.clone());

        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();

        let result_data =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("result")).unwrap();

        let js_result_data: CustomChallengeResult = result_data.into_serde().unwrap();

        assert_eq!(js_result_data.id, test_result.id);
        assert_eq!(js_result_data.performance, test_result.performance);
    }

    #[wasm_bindgen_test]
    fn test_execute_js() {
        let window = get_window();
        let konnektoren_js = KonnektorenJs::new(&window);

        konnektoren_js.execute_js("console.log('Hello from Rust!')");
    }
}
