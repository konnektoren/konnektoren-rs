use crate::challenges::Custom;
use js_sys::Reflect;
use serde_wasm_bindgen::to_value;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

pub struct KonnektorenJs {
    js_event_callback: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>,
}

impl KonnektorenJs {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let global_obj = window.as_ref();

        // Ensure that `window.konnektoren` exists, or create it if it doesn't.
        if js_sys::Reflect::has(global_obj, &JsValue::from_str("konnektoren")).unwrap() == false {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(global_obj, &JsValue::from_str("konnektoren"), &obj).unwrap();
        }

        Self {
            js_event_callback: Rc::new(RefCell::new(None)),
        }
    }

    /// Sets the challenge data in the JavaScript `window.konnektoren.challenge` as a `JsValue`.
    pub fn set_challenge_data(&self, challenge_data: Custom) {
        let window = web_sys::window().unwrap();
        let global_obj = window.as_ref();

        // Convert the `Custom` struct to `JsValue` using `serde-wasm-bindgen::to_value`
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

    /// Executes arbitrary JavaScript code in the global context.
    pub fn execute_js(&self, js_code: &str) {
        if let Err(err) = js_sys::eval(js_code) {
            log::error!("JavaScript execution failed: {:?}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    // Enable `wasm_bindgen_test` for the test environment
    #[wasm_bindgen_test]
    fn test_set_challenge_data() {
        // Create a new instance of KonnektorenJs
        let konnektoren_js = KonnektorenJs::new();

        // Set challenge data
        konnektoren_js.set_challenge_data(r#"{"title": "Test Challenge"}"#);

        // Access the JavaScript window object to verify if the data is set correctly
        let window = web_sys::window().unwrap();
        let konnektoren_obj =
            js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();
        let challenge_data =
            js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("challenge")).unwrap();

        // Assert the challenge data is set correctly
        assert_eq!(
            challenge_data.as_string().unwrap(),
            r#"{"title": "Test Challenge"}"#
        );
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
