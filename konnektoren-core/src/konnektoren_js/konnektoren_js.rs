use crate::challenges::{Custom, CustomChallengeResult};
use wasm_bindgen::JsValue;

use super::{
    ChallengeHandler, CommandHandler, EventHandler, I18nHandler, JsExecutor, ResultHandler,
};

use web_sys::Window;

/// `KonnektorenJs` is the main struct that provides an interface between Rust and JavaScript
/// for the Konnektoren application. It manages various handlers for different aspects of
/// the application's functionality.
pub struct KonnektorenJs {
    challenge: ChallengeHandler<'static>,
    command: CommandHandler<'static>,
    event: EventHandler<'static>,
    i18n: I18nHandler<'static>,
    js_executor: &'static JsExecutor<'static>,
    result: ResultHandler<'static>,
}

impl KonnektorenJs {
    /// Creates a new instance of `KonnektorenJs`.
    ///
    /// This function initializes all the handlers and ensures that the `window.konnektoren`
    /// object exists in the JavaScript environment.
    ///
    /// # Arguments
    ///
    /// * `window` - A reference to the JavaScript `Window` object.
    ///
    /// # Returns
    ///
    /// A new instance of `KonnektorenJs`.
    pub fn new(window: &Window) -> Self {
        let window = window.clone();
        let static_window: &'static Window = Box::leak(Box::new(window.clone()));
        let js_executor: &'static JsExecutor = Box::leak(Box::new(JsExecutor::new(static_window)));

        let konnektoren_obj = js_executor.get_or_create_konnektoren_object();
        if js_executor.is_konnektoren_object_empty(&konnektoren_obj) {
            js_executor.create_konnektoren_object(&konnektoren_obj);
        }

        let challenge = ChallengeHandler::new(js_executor);
        let command = CommandHandler::new(static_window);
        let event = EventHandler::new(static_window);
        let i18n = I18nHandler::new(static_window);
        let result = ResultHandler::new(js_executor);

        Self {
            challenge,
            command,
            event,
            i18n,
            js_executor,
            result,
        }
    }

    /// Sets the challenge data in the JavaScript environment.
    ///
    /// # Arguments
    ///
    /// * `challenge_data` - The `Custom` challenge data to be set.
    pub fn set_challenge_data(&self, challenge_data: Custom) {
        self.challenge.set_challenge_data(challenge_data);
    }

    /// Sets the result data in the JavaScript environment.
    ///
    /// # Arguments
    ///
    /// * `result_data` - The `CustomChallengeResult` to be set.
    pub fn set_result_data(&self, result_data: CustomChallengeResult) {
        self.result.set_result_data(result_data);
    }

    /// Exposes a function to send events from JavaScript to Rust.
    ///
    /// # Arguments
    ///
    /// * `on_event` - A closure that will be called when an event is sent from JavaScript.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A closure type that takes a `JsValue` and returns nothing.
    pub fn expose_send_event<F>(&self, on_event: F)
    where
        F: 'static + FnMut(JsValue),
    {
        self.event.expose_send_event(on_event);
    }

    /// Exposes a function to execute commands from JavaScript in Rust.
    ///
    /// # Arguments
    ///
    /// * `on_command` - A closure that will be called when a command is executed from JavaScript.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A closure type that takes a `JsValue` and returns nothing.
    pub fn expose_execute_command<F>(&self, on_command: F)
    where
        F: 'static + FnMut(JsValue),
    {
        self.command.expose_execute_command(on_command);
    }

    /// Executes JavaScript code.
    ///
    /// # Arguments
    ///
    /// * `js_code` - A string slice containing the JavaScript code to be executed.
    pub fn execute_js(&self, js_code: &str) {
        self.js_executor.execute(js_code);
    }

    /// Sets the internationalization (i18n) data.
    ///
    /// # Arguments
    ///
    /// * `i18n_data` - A `serde_json::Value` containing the i18n data.
    pub fn set_i18n_data(&mut self, i18n_data: serde_json::Value) {
        self.i18n.set_i18n_data(i18n_data);
    }

    /// Finishes the current challenge with an optional result.
    ///
    /// # Arguments
    ///
    /// * `result` - An optional `CustomChallengeResult` representing the result of the challenge.
    pub fn finish_challenge(&self, result: Option<CustomChallengeResult>) {
        self.command.finish_challenge(result);
    }
}
