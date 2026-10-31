use super::ChallengeDefinition;
use super::error::{ChallengeError, Result};

/// What the game lifecycle needs from a challenge type to accept answers.
///
/// Implement this for an application's own challenge type to let commands such as
/// `NextTask` and `Solve` operate on it. Solving itself goes through
/// `Solvable<Self::Input>` on the matching `Challenge`.
pub trait Playable: ChallengeDefinition {
    /// A single answer to one task.
    type Input;

    /// Number of tasks that already have an answer in `result`.
    fn result_len(result: &Self::Result) -> usize;

    /// Answer recorded when a task is skipped, keeping result indices aligned with tasks.
    /// `Ok(None)` means skipped tasks stay unanswered.
    fn default_input(&self, task_index: usize) -> Result<Option<Self::Input>>;

    /// Append `input` to `result`, creating an empty result first when none exists yet.
    fn record_input(&self, result: &mut Option<Self::Result>, input: Self::Input) -> Result<()>;

    /// Translate the index of a presented option into an input.
    /// Only challenge types that present numbered options support this.
    fn option_input(&self, _option_index: usize) -> Result<Self::Input> {
        Err(ChallengeError::InvalidChallengeType)
    }
}
