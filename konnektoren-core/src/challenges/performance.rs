use super::ChallengeResult;

pub trait Performance {
    /// Returns the performance in percentage.
    fn performance(&self, result: &ChallengeResult) -> u32;

    /// Returns the number of stars based on the performance.
    /// 3 stars for 80% or more, 2 stars for 60% or more, 1 star for 40% or more, 0 stars otherwise.
    /// The performance is calculated by the `performance` method.
    fn stars(&self, result: &ChallengeResult) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::multiple_choice::MultipleChoiceOption;
    use crate::challenges::ChallengeResult;
    use crate::challenges::ChallengeType;

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
