use super::challenge_command::ChallengeCommand;
use super::game_command::GameCommand;
use crate::game::GameState;
use anyhow::Result;

pub trait CommandTrait {
    fn execute(&self, state: &mut GameState) -> Result<()>;
}

pub enum Command {
    Game(GameCommand),
    Challenge(ChallengeCommand),
}

impl CommandTrait for Command {
    fn execute(&self, state: &mut GameState) -> Result<()> {
        match self {
            Command::Game(cmd) => cmd.execute(state),
            Command::Challenge(cmd) => cmd.execute(state),
        }
    }
}
