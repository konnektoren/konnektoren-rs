use crate::challenges::challenge_input::ChallengeInput;
use crate::challenges::error::ChallengeError;

/// A challenge that can accept inputs and report whether each task was solved.
pub trait Solvable<I = ChallengeInput, E = ChallengeError> {
    /// Attempts to solve a part of the challenge with the given input.
    /// Returns `true` if the part is successfully solved, otherwise `false`.
    fn solve(&mut self, input: I, task_index: usize) -> std::result::Result<bool, E>;
}
