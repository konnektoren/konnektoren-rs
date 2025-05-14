mod app;
mod challenge_tabs;
mod challenge_widget;
mod error;
mod map_widget;
mod options_widget;
mod results_widget;
#[cfg(feature = "crossterm")]
mod tui;

pub mod prelude {
    pub use crate::app::App;
    #[cfg(feature = "crossterm")]
    pub use crate::tui::{init, restore, Tui};
}
