use super::command::CommandTrait;
use crate::game::GameState;
use anyhow::Result;

pub enum GameCommand {
    NextChallenge,
    PreviousChallenge,
}

impl CommandTrait for GameCommand {
    fn execute(&self, state: &mut GameState) -> Result<()> {
        unimplemented!()
    }
}
