use super::{ChallengeResult, PerformanceScale};

/// Scores a challenge result. External challenges can supply their own result type.
pub trait Performance<R = ChallengeResult> {
    /// Returns the performance in percentage.
    fn performance(&self, result: &R) -> u32;

    /// Stars on the [default scale](PerformanceScale::DEFAULT): three from 80 %, two
    /// from 70 %, one from 60 %.
    fn stars(&self, result: &R) -> u32 {
        self.stars_with(result, &PerformanceScale::DEFAULT)
    }

    /// Stars on the app's `scale`.
    fn stars_with(&self, result: &R, scale: &PerformanceScale) -> u32 {
        scale.stars(self.performance(result))
    }
}

/// The stars a performance earns on the [default scale](PerformanceScale::DEFAULT):
/// three from 80, two from 70, one from 60.
pub fn stars_for(performance: u32) -> u32 {
    PerformanceScale::DEFAULT.stars(performance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::ChallengeResult;
    use crate::challenges::ChallengeType;
    use crate::challenges::multiple_choice::MultipleChoiceOption;

    #[test]
    fn test_performance() {
        let challenge = ChallengeType::default();
        let result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
        ]);
        let performance = challenge.performance(&result);
        assert_eq!(performance, 0);
    }

    #[test]
    fn test_stars() {
        let challenge = ChallengeType::default();
        let result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
            MultipleChoiceOption {
                id: 3,
                name: "Option 3".to_string(),
            },
            MultipleChoiceOption {
                id: 4,
                name: "Option 4".to_string(),
            },
            MultipleChoiceOption {
                id: 5,
                name: "Option 5".to_string(),
            },
        ]);
        let stars = challenge.stars(&result);
        assert_eq!(stars, 0);
    }
}
