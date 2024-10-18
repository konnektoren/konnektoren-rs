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
}
