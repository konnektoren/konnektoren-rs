use wasm_bindgen::prelude::*;
use web_sys::Window;

pub struct EventHandler<'a> {
    window: &'a Window,
}

impl<'a> EventHandler<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    pub fn expose_send_event<F>(&self, on_event: F)
    where
        F: 'static + FnMut(JsValue),
    {
        let closure = Closure::wrap(Box::new(on_event) as Box<dyn FnMut(JsValue)>);
        let konnektoren_obj =
            js_sys::Reflect::get(self.window, &JsValue::from_str("konnektoren")).unwrap();
        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("sendEvent"),
            closure.as_ref().unchecked_ref(),
        )
        .unwrap();
        closure.forget();
    }
}
