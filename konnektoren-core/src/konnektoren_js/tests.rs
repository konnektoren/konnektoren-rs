use super::konnektoren_js::KonnektorenJs;
use crate::challenges::Custom;
use crate::challenges::CustomChallengeResult;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::Window;

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

    let konnektoren_obj = js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();

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

    let konnektoren_obj = js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();
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

    let konnektoren_obj = js_sys::Reflect::get(&window, &JsValue::from_str("konnektoren")).unwrap();

    let result_data = js_sys::Reflect::get(&konnektoren_obj, &JsValue::from_str("result")).unwrap();

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
