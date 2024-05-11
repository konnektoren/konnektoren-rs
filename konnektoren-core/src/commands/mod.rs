use anyhow::Result;

use crate::game::GameState;

pub mod game_commands;

pub trait GameCommand {
    fn execute(&self, state: &mut GameState) -> Result<(), String>;
}
