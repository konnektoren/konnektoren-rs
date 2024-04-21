use super::ChallengeResult;

pub trait Performance {
    /// Returns the performance in percentage.
    fn performance(&self, result: &ChallengeResult) -> i32;

    /// Returns the number of stars based on the performance.
    /// 3 stars for 80% or more, 2 stars for 60% or more, 1 star for 40% or more, 0 stars otherwise.
    /// The performance is calculated by the `performance` method.
    fn stars(&self, result: &ChallengeResult) -> i32 {
        let performance = self.performance(result);
        if performance >= 80 {
            3
        } else if performance >= 60 {
            2
        } else if performance >= 40 {
            1
        } else {
            0
        }
    }
}
