//! This module contains the command pattern implementation for the game.
use anyhow::Result;

use crate::game::GameState;

pub mod game_commands;

pub trait GameCommand {
    fn execute(&self, state: &mut GameState) -> Result<()>;
}
