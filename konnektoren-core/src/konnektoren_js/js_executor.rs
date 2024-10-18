#[derive(Default)]
pub struct JsExecutor {}

impl JsExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, js_code: &str) {
        if let Err(err) = js_sys::eval(js_code) {
            log::error!("JavaScript execution failed: {:?}", err);
        }
    }
}
