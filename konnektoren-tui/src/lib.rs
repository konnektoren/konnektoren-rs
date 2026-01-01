mod app;
mod challenge_tabs;
mod challenge_widget;
mod error;
mod map_widget;
mod options_widget;
mod results_widget;

#[cfg(feature = "crossterm")]
mod tui;

#[cfg(feature = "ssh")]
pub mod ssh_server;

pub mod prelude {
    pub use crate::app::App;

    #[cfg(feature = "crossterm")]
    pub use crate::tui::{Tui, init, restore};

    #[cfg(feature = "ssh")]
    pub use crate::ssh_server::SshServer;
}
