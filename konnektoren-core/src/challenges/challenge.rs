use crate::challenges::challenge_config::ChallengeConfig;
use crate::challenges::challenge_result::ChallengeResult;
use crate::challenges::challenge_type::ChallengeType;
use serde::{Deserialize, Serialize};

use super::{Performance, Solvable};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Challenge {
    pub challenge_type: ChallengeType,
    pub challenge_config: ChallengeConfig,
    pub challenge_result: ChallengeResult,
}

impl Challenge {
    pub fn new(challenge_type: &ChallengeType, challenge_config: &ChallengeConfig) -> Self {
        Challenge {
            challenge_type: challenge_type.clone(),
            challenge_config: challenge_config.clone(),
            challenge_result: ChallengeResult::default(),
        }
    }
}

impl Solvable for Challenge {
    fn solve(&mut self, input: super::ChallengeInput) -> anyhow::Result<bool> {
        match self.challenge_result.add_input(input) {
            Ok(_) => {
                let index = self.challenge_result.len();
                let question = match self.challenge_type {
                    ChallengeType::MultipleChoice(ref mc) => mc.questions.get(index),
                    ChallengeType::SortTable(_) => None,
                };
                let result = match self.challenge_result {
                    ChallengeResult::MultipleChoice(ref mc) => mc.get(index),
                    ChallengeResult::SortTable(_) => None,
                };
                match (question, result) {
                    (Some(question), Some(result)) => Ok(question.option == result.id),
                    _ => Ok(false),
                }
            }
            Err(_) => Ok(false),
        }
    }
}

impl Performance for Challenge {
    fn performance(&self, result: &ChallengeResult) -> u32 {
        self.challenge_type.performance(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge_input::ChallengeInput;
    use crate::challenges::MultipleChoiceOption;

    #[test]
    fn new_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let challenge = Challenge::new(&challenge_type, &challenge_config);
        assert_eq!(challenge.challenge_type, challenge_type);
        assert_eq!(challenge.challenge_config, challenge_config);
        assert_eq!(challenge.challenge_result, ChallengeResult::default());
    }

    #[test]
    fn solve_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
    }
}
