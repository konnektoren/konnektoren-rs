//! This module defines the core command structure and traits for the game.

use super::CommandType;
use super::challenge_command::ChallengeCommand;
use super::error::Result;
use super::game_command::GameCommand;
use crate::game::GameState;

/// A trait that defines the basic behavior for all commands in the game.
pub trait CommandTrait {
    /// Executes the command on the given game state.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the command execution failed.
    fn execute(&self, state: &mut GameState) -> Result<()>;

    /// Gets the type of the command.
    fn get_type(&self) -> CommandType;
}

/// An enum representing all possible commands in the game.
///
/// This enum serves as a unified interface for both game-level and challenge-level commands.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Represents a game-level command.
    Game(GameCommand),
    /// Represents a challenge-level command.
    Challenge(ChallengeCommand),
}

impl CommandTrait for Command {
    /// Executes the command on the given game state.
    ///
    /// This implementation delegates the execution to either the game command
    /// or the challenge command based on the variant.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the current game state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing an error if the command execution failed.
    fn execute(&self, state: &mut GameState) -> Result<()> {
        match self {
            Command::Game(cmd) => cmd.execute(state),
            Command::Challenge(cmd) => cmd.execute(state),
        }
    }

    /// Gets the type of the command.
    fn get_type(&self) -> CommandType {
        match self {
            Command::Game(_) => CommandType::Game,
            Command::Challenge(_) => CommandType::Challenge,
        }
    }
}
