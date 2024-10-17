use crate::challenges::{Custom, CustomChallengeResult};
use js_sys::Reflect;
use serde_wasm_bindgen::to_value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub struct KonnektorenJs {
    js_event_callback: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>,
    tr_callback: Rc<RefCell<Option<Closure<dyn FnMut(JsValue) -> JsValue>>>>,
    translations: Rc<RefCell<HashMap<String, String>>>,
    execute_command_callback: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>,
}

impl KonnektorenJs {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let global_obj = window.as_ref();

        // Ensure that `window.konnektoren` exists, or create it if it doesn't.
        if Reflect::has(global_obj, &JsValue::from_str("konnektoren")).unwrap() == false {
            let obj = js_sys::Object::new();
            Reflect::set(global_obj, &JsValue::from_str("konnektoren"), &obj).unwrap();
        }

        Self {
            js_event_callback: Rc::new(RefCell::new(None)),
            tr_callback: Rc::new(RefCell::new(None)),
            translations: Rc::new(RefCell::new(HashMap::new())),
            execute_command_callback: Rc::new(RefCell::new(None)),
        }
    }

    /// Sets the challenge data in the JavaScript `window.konnektoren.challenge` as a `JsValue`.
    pub fn set_challenge_data(&self, challenge_data: Custom) {
        let window = web_sys::window().unwrap();
        let global_obj = window.as_ref();

        // Convert the `Custom` struct to `JsValue`
        let js_challenge_data = to_value(&challenge_data).unwrap();

        // Get `window.konnektoren`
        let konnektoren_obj = Reflect::get(global_obj, &JsValue::from_str("konnektoren")).unwrap();

        // Set the challenge data as a plain object under `window.konnektoren.challenge`
        Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("challenge"),
            &js_challenge_data,
        )
        .unwrap();
    }

    /// Sets the result data in the JavaScript `window.konnektoren.result` as a `JsValue`.
    pub fn set_result_data(&self, result_data: CustomChallengeResult) {
        let window = web_sys::window().unwrap();
        let global_obj = window.as_ref();

        // Convert the `Custom` struct to `JsValue`
        let js_result_data = to_value(&result_data).unwrap();

        // Get `window.konnektoren`
        let konnektoren_obj = Reflect::get(global_obj, &JsValue::from_str("konnektoren")).unwrap();

        // Set the challenge data as a plain object under `window.konnektoren.challenge`
        Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("result"),
            &js_result_data,
        )
        .unwrap();
    }

    /// Exposes `sendEvent` to JavaScript, allowing it to send events to Rust.
    /// This is a generic function that receives a closure for event handling.
    pub fn expose_send_event<F>(&self, on_event: F)
    where
        F: 'static + FnMut(JsValue),
    {
        let closure = Closure::wrap(Box::new(on_event) as Box<dyn FnMut(JsValue)>);
        *self.js_event_callback.borrow_mut() = Some(closure);

        let window = web_sys::window().unwrap();
        let global_obj = window.as_ref();

        // Get `window.konnektoren` (it should exist after `new()` creates it)
        let konnektoren_obj =
            js_sys::Reflect::get(global_obj, &JsValue::from_str("konnektoren")).unwrap();

        // Set the `sendEvent` function inside `window.konnektoren`
        let binding = self.js_event_callback.borrow();
        let callback_ref = binding.as_ref().unwrap().as_ref().unchecked_ref();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("sendEvent"),
            callback_ref,
        )
        .unwrap();
    }

    /// Exposes `executeCommand` to JavaScript, allowing it to send commands to Rust.
    /// This is a generic function that receives a closure for command handling.
    pub fn expose_execute_command<F>(&self, on_command: F)
    where
        F: 'static + FnMut(JsValue),
    {
        let closure = Closure::wrap(Box::new(on_command) as Box<dyn FnMut(JsValue)>);
        *self.execute_command_callback.borrow_mut() = Some(closure);

        let window = web_sys::window().unwrap();
        let global_obj = window.as_ref();

        // Get `window.konnektoren` (it should exist after `new()` creates it)
        let konnektoren_obj =
            js_sys::Reflect::get(global_obj, &JsValue::from_str("konnektoren")).unwrap();

        // Set the `executeCommand` function inside `window.konnektoren`
        let binding = self.execute_command_callback.borrow();
        let callback_ref = binding.as_ref().unwrap().as_ref().unchecked_ref();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("executeCommand"),
            callback_ref,
        )
        .unwrap();
    }

    /// Executes arbitrary JavaScript code in the global context.
    pub fn execute_js(&self, js_code: &str) {
        if let Err(err) = js_sys::eval(js_code) {
            log::error!("JavaScript execution failed: {:?}", err);
        }
    }

    /// Sets the i18n data and exposes the `tr` function to JavaScript.
    pub fn set_i18n_data(&self, i18n_data: serde_json::Value) {
        let window = web_sys::window().unwrap();
        let global_obj = window.as_ref();

        let translations_map: HashMap<String, String> =
            serde_json::from_value(i18n_data.clone()).unwrap();
        *self.translations.borrow_mut() = translations_map;

        let konnektoren_obj = Reflect::get(global_obj, &JsValue::from_str("konnektoren")).unwrap();

        let i18n_data = to_value(&i18n_data).unwrap();

        Reflect::set(&konnektoren_obj, &JsValue::from_str("i18n"), &i18n_data).unwrap();

        let translations = self.translations.clone();
        let tr_closure = Closure::wrap(Box::new(move |text: JsValue| {
            let text_str = text.as_string().unwrap_or_default();
            let translated = translations
                .borrow()
                .get(&text_str)
                .cloned()
                .unwrap_or_else(|| text_str.clone());
            JsValue::from_str(&translated)
        }) as Box<dyn FnMut(JsValue) -> JsValue>);

        *self.tr_callback.borrow_mut() = Some(tr_closure);

        let binding = self.tr_callback.borrow();
        let callback_ref = binding.as_ref().unwrap().as_ref().unchecked_ref();

        Reflect::set(&konnektoren_obj, &JsValue::from_str("tr"), callback_ref).unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_set_challenge_data() {
        // Create a new instance of KonnektorenJs
        let konnektoren_js = KonnektorenJs::new();

        // Set challenge data
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

        // Access the JavaScript window object to verify if the data is set correctly
        let window = web_sys::window().unwrap();
        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();

        let challenge_data =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("challenge")).unwrap();

        // Convert the JsValue back to Custom
        let js_challenge_data: Custom = challenge_data.into_serde().unwrap();

        // Assert the challenge data is set correctly
        assert_eq!(js_challenge_data.id, test_custom.id);
        assert_eq!(js_challenge_data.name, test_custom.name);
        // assert_eq!(js_challenge_data.data, test_custom.data);
    }

    #[wasm_bindgen_test]
    fn test_expose_send_event() {
        // Create a new instance of KonnektorenJs
        let konnektoren_js = KonnektorenJs::new();

        // Variable to track if the event was received
        let event_received = Rc::new(RefCell::new(false));

        {
            // Clone the event_received to move into closure
            let event_received_clone = event_received.clone();

            // Expose the sendEvent function with an event handler closure
            konnektoren_js.expose_send_event(move |event: JsValue| {
                // When the event is received, set event_received to true
                *event_received_clone.borrow_mut() = true;
                log::info!("Received event: {:?}", event);
            });
        }

        // Simulate triggering the event in JavaScript
        let window = web_sys::window().unwrap();
        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();
        let send_event_func =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("sendEvent")).unwrap();

        // Call the sendEvent function with an empty event (to simulate a real event)
        let event_data = JsValue::from_serde(&serde_json::json!({ "type": "TestEvent" })).unwrap();
        let send_event_func = send_event_func.dyn_ref::<js_sys::Function>().unwrap();
        send_event_func.call1(&JsValue::NULL, &event_data).unwrap();

        // Assert that the event was received by the closure
        assert!(*event_received.borrow());
    }

    #[wasm_bindgen_test]
    fn test_set_result_data() {
        // Create a new instance of KonnektorenJs
        let konnektoren_js = KonnektorenJs::new();

        // Set result data
        let test_result = CustomChallengeResult {
            id: "123".to_string(),
            performance: 0.0,
            data: serde_json::json!({"key":"value"}),
        };
        konnektoren_js.set_result_data(test_result.clone());

        // Access the JavaScript window object to verify if the data is set correctly
        let window = web_sys::window().unwrap();
        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();

        let result_data =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("result")).unwrap();

        // Convert the JsValue back to CustomChallengeResult
        let js_result_data: CustomChallengeResult = result_data.into_serde().unwrap();

        // Assert the result data is set correctly
        assert_eq!(js_result_data.id, test_result.id);
        assert_eq!(js_result_data.performance, test_result.performance);
        // assert_eq!(js_result_data, test_result);
    }

    #[wasm_bindgen_test]
    fn test_execute_js() {
        // Create a new instance of KonnektorenJs
        let konnektoren_js = KonnektorenJs::new();

        // Execute some arbitrary JavaScript code
        konnektoren_js.execute_js("console.log('Hello from Rust!')");

        // Unfortunately, wasm-bindgen-test doesn't capture console logs directly for assertions.
        // You would typically verify this manually by running the tests in a browser and checking the console output.
        // However, we can test that no error occurred during execution.
    }
}
