mod challenge;
mod command;
mod event;
mod i18n;
mod js_executor;
mod konnektoren_js_impl;
mod result;

pub const KONNEKTOREN_NAMESPACE: &str = "konnektoren";

pub use challenge::ChallengeHandler;
pub use command::CommandHandler;
pub use event::EventHandler;
pub use i18n::I18nHandler;
pub use js_executor::JsExecutor;
pub use konnektoren_js_impl::KonnektorenJs;
pub use result::ResultHandler;

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests;
