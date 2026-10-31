use super::{ChallengeDefinition, Playable};
use std::fmt::Debug;

/// Everything the game lifecycle (state, commands, controller, plugins) needs from a
/// challenge type. It is implemented automatically for every type that qualifies.
pub trait GameChallenge:
    Playable<Input: Clone + Debug + PartialEq + Send + Sync + 'static>
    + ChallengeDefinition<Result: Clone + Debug + PartialEq + Send + Sync + 'static>
    + Clone
    + Debug
    + PartialEq
    + Send
    + Sync
    + 'static
{
}

impl<T> GameChallenge for T where
    T: Playable<Input: Clone + Debug + PartialEq + Send + Sync + 'static>
        + ChallengeDefinition<Result: Clone + Debug + PartialEq + Send + Sync + 'static>
        + Clone
        + Debug
        + PartialEq
        + Send
        + Sync
        + 'static
{
}
