//! Game module.
#![allow(clippy::module_inception)]
pub mod game;
pub mod game_path;
pub mod game_state;

pub use game::Game;
pub use game_path::GamePath;
pub use game_state::GameState;
