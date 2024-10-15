use super::command::CommandTrait;
use crate::game::GameState;
use anyhow::Result;

pub enum ChallengeCommand {
    NextTask,
    PreviousTask,
    SolveOption(usize),
}

impl CommandTrait for ChallengeCommand {
    fn execute(&self, state: &mut GameState) -> Result<()> {
        unimplemented!()
    }
}
