use super::{ChallengeDefinition, Playable};
use std::fmt::Debug;

/// Everything the game lifecycle (state, commands, controller, plugins) needs from a
/// challenge type. It is implemented automatically for every type that qualifies.
pub trait GameChallenge:
    Playable<
        Input: Clone + Debug + PartialEq + Send + Sync + 'static,
        Variant: Clone + Debug + PartialEq + Send + Sync + 'static,
    > + ChallengeDefinition<Result: Clone + Debug + PartialEq + Send + Sync + 'static>
    + Clone
    + Debug
    + PartialEq
    + Send
    + Sync
    + 'static
{
}

impl<T> GameChallenge for T where
    T: Playable<
            Input: Clone + Debug + PartialEq + Send + Sync + 'static,
            Variant: Clone + Debug + PartialEq + Send + Sync + 'static,
        > + ChallengeDefinition<Result: Clone + Debug + PartialEq + Send + Sync + 'static>
        + Clone
        + Debug
        + PartialEq
        + Send
        + Sync
        + 'static
{
}

/// An attempt at a challenge of type `C`, configured with that type's own variants.
pub type Attempt<C> = super::Challenge<
    C,
    <C as ChallengeDefinition>::Result,
    super::ChallengeConfig<<C as Playable>::Variant>,
>;

/// A [`GameChallenge`] whose content, results, and variants can be saved and loaded.
/// Implemented automatically.
pub trait SavedChallenge:
    GameChallenge
    + serde::Serialize
    + serde::de::DeserializeOwned
    + ChallengeDefinition<Result: serde::Serialize + serde::de::DeserializeOwned>
    + Playable<Variant: serde::Serialize + serde::de::DeserializeOwned>
{
}

impl<T> SavedChallenge for T where
    T: GameChallenge
        + serde::Serialize
        + serde::de::DeserializeOwned
        + ChallengeDefinition<Result: serde::Serialize + serde::de::DeserializeOwned>
        + Playable<Variant: serde::Serialize + serde::de::DeserializeOwned>
{
}
