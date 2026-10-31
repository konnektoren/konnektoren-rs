use super::{ChallengeCommand, Command};
use crate::challenges::{ChallengeEmbedding, GameChallenge};

impl<Inner: GameChallenge> ChallengeCommand<Inner> {
    /// Express a command for an included challenge type as one for the larger type.
    pub fn embed<Outer: GameChallenge>(
        self,
        embedding: &ChallengeEmbedding<Inner, Outer>,
    ) -> ChallengeCommand<Outer> {
        match self {
            Self::Start(attempt) => ChallengeCommand::Start(embedding.attempt(attempt)),
            Self::NextTask => ChallengeCommand::NextTask,
            Self::PreviousTask => ChallengeCommand::PreviousTask,
            Self::SolveOption(index) => ChallengeCommand::SolveOption(index),
            Self::Solve(input) => ChallengeCommand::Solve(embedding.input(input)),
            Self::Finish(result) => ChallengeCommand::Finish(result.map(|r| embedding.result(r))),
        }
    }
}

impl<Inner: GameChallenge> Command<Inner> {
    /// Express a command for an included challenge type as one for the larger type.
    pub fn embed<Outer: GameChallenge>(
        self,
        embedding: &ChallengeEmbedding<Inner, Outer>,
    ) -> Command<Outer> {
        match self {
            Self::Game(command) => Command::Game(command),
            Self::Challenge(command) => Command::Challenge(command.embed(embedding)),
        }
    }
}
