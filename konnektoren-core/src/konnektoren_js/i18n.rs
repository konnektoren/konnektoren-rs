use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::Window;

pub struct I18nHandler<'a> {
    window: &'a Window,
    translations: HashMap<String, String>,
}

impl<'a> I18nHandler<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self {
            window,
            translations: HashMap::new(),
        }
    }

    pub fn set_i18n_data(&mut self, i18n_data: serde_json::Value) {
        let konnektoren_obj =
            js_sys::Reflect::get(self.window, &JsValue::from_str("konnektoren")).unwrap();
        let i18n_data_js = serde_wasm_bindgen::to_value(&i18n_data).unwrap();
        js_sys::Reflect::set(&konnektoren_obj, &JsValue::from_str("i18n"), &i18n_data_js).unwrap();

        self.translations = serde_json::from_value(i18n_data).unwrap();

        let translations = self.translations.clone();
        let tr_closure = Closure::wrap(Box::new(move |text: JsValue| {
            let text_str = text.as_string().unwrap_or_default();
            let translated = translations
                .get(&text_str)
                .cloned()
                .unwrap_or_else(|| text_str.clone());
            JsValue::from_str(&translated)
        }) as Box<dyn FnMut(JsValue) -> JsValue>);

        js_sys::Reflect::set(
            &konnektoren_obj,
            &JsValue::from_str("tr"),
            tr_closure.as_ref().unchecked_ref(),
        )
        .unwrap();
        tr_closure.forget();
    }
}
