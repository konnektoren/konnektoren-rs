use super::ChallengeResult;

pub trait Performance {
    /// Returns the performance in percentage.
    fn performance(&self, result: &ChallengeResult) -> i32;
}
