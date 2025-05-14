use crate::challenges::challenge_input::ChallengeInput;
use crate::challenges::error::Result;

pub trait Solvable {
    /// Attempts to solve a part of the challenge with the given input.
    /// Returns `true` if the part is successfully solved, otherwise `false`.
    fn solve(&mut self, input: ChallengeInput) -> Result<bool>;
}
