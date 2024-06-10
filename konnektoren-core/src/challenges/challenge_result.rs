use crate::challenges::multiple_choice::MultipleChoiceOption;
use crate::challenges::ChallengeInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeResult {
    MultipleChoice(Vec<MultipleChoiceOption>),
}

impl Default for ChallengeResult {
    fn default() -> Self {
        ChallengeResult::MultipleChoice(Vec::new())
    }
}

impl ChallengeResult {
    pub fn add_input(&mut self, input: ChallengeInput) -> anyhow::Result<()> {
        match self {
            ChallengeResult::MultipleChoice(options) => match input {
                ChallengeInput::MultipleChoice(option) => {
                    options.push(option);
                    Ok(())
                }
            },
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ChallengeResult::MultipleChoice(options) => options.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_challenge_result() {
        let challenge_result = ChallengeResult::default();
        match challenge_result {
            ChallengeResult::MultipleChoice(options) => {
                assert!(options.is_empty());
            }
        }
    }

    #[test]
    fn add_multiple_choice() {
        let mut challenge_result = ChallengeResult::default();
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption {
            id: 1,
            name: "Option 1".to_string(),
        });
        let result = challenge_result.add_input(input);
        assert!(result.is_ok());
        match challenge_result {
            ChallengeResult::MultipleChoice(options) => {
                assert_eq!(options.len(), 1);
            }
        }
    }
}
