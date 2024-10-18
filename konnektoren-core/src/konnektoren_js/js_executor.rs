use super::KONNEKTOREN_NAMESPACE;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Window;

pub struct JsExecutor<'a> {
    window: &'a Window,
}

impl<'a> JsExecutor<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn execute(&self, js_code: &str) {
        if let Err(err) = js_sys::eval(js_code) {
            log::error!("JavaScript execution failed: {:?}", err);
        }
    }

    pub fn get_or_create_konnektoren_object(&self) -> JsValue {
        let global = JsValue::from(self.window.clone());
        if js_sys::Reflect::has(&global, &JsValue::from_str(KONNEKTOREN_NAMESPACE)).unwrap_or(false)
        {
            js_sys::Reflect::get(&global, &JsValue::from_str(KONNEKTOREN_NAMESPACE)).unwrap()
        } else {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&global, &JsValue::from_str(KONNEKTOREN_NAMESPACE), &obj).unwrap();
            obj.into()
        }
    }

    pub fn is_konnektoren_object_empty(&self, konnektoren_obj: &JsValue) -> bool {
        js_sys::Object::keys(konnektoren_obj.unchecked_ref::<js_sys::Object>()).length() == 0
    }

    pub fn create_konnektoren_object(&self, konnektoren_obj: &JsValue) {
        let version = env!("CARGO_PKG_VERSION");
        js_sys::Reflect::set(
            konnektoren_obj,
            &JsValue::from_str("version"),
            &JsValue::from_str(version),
        )
        .unwrap();
    }
}
