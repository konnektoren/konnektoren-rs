use crate::challenges::error::{ChallengeError, Result};
use crate::challenges::{
    ChallengeInput, ContextItemChoiceAnswers, CustomChallengeResult, GapFillAnswer,
    MultipleChoiceOption, OrderingResult, SortTableRow,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeResult {
    MultipleChoice(Vec<MultipleChoiceOption>),
    ContextualChoice(Vec<ContextItemChoiceAnswers>),
    GapFill(Vec<GapFillAnswer>),
    SortTable(Vec<SortTableRow>),
    Informative,
    Ordering(Vec<OrderingResult>),
    Custom(CustomChallengeResult),
}

impl Default for ChallengeResult {
    fn default() -> Self {
        ChallengeResult::MultipleChoice(Vec::new())
    }
}

impl ChallengeResult {
    pub fn add_input(&mut self, input: ChallengeInput) -> Result<()> {
        match self {
            ChallengeResult::MultipleChoice(options) => match input {
                ChallengeInput::MultipleChoice(option) => {
                    options.push(option);
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected MultipleChoice input".to_string(),
                )),
            },
            ChallengeResult::ContextualChoice(answers) => match input {
                ChallengeInput::ContextualChoice(answer) => {
                    answers.push(answer);
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected ContextualChoice input".to_string(),
                )),
            },
            ChallengeResult::GapFill(answers) => match input {
                ChallengeInput::GapFill(answer) => {
                    answers.push(answer);
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected GapFill input".to_string(),
                )),
            },
            ChallengeResult::SortTable(rows) => match input {
                ChallengeInput::SortTable(row) => {
                    rows.push(row);
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected SortTable input".to_string(),
                )),
            },
            ChallengeResult::Ordering(results) => match input {
                ChallengeInput::Ordering(result) => {
                    results.push(result);
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected Ordering input".to_string(),
                )),
            },
            ChallengeResult::Informative => Ok(()),
            ChallengeResult::Custom(_) => Ok(()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ChallengeResult::MultipleChoice(options) => options.len(),
            ChallengeResult::ContextualChoice(items) => items.len(),
            ChallengeResult::GapFill(answers) => answers.len(),
            ChallengeResult::SortTable(rows) => rows.len(),
            ChallengeResult::Ordering(results) => results.len(),
            ChallengeResult::Informative => 0,
            ChallengeResult::Custom(_) => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ChallengeResult::MultipleChoice(options) => options.is_empty(),
            ChallengeResult::ContextualChoice(items) => items.is_empty(),
            ChallengeResult::GapFill(answers) => answers.is_empty(),
            ChallengeResult::SortTable(rows) => rows.is_empty(),
            ChallengeResult::Ordering(results) => results.is_empty(),
            ChallengeResult::Informative => true,
            ChallengeResult::Custom(_) => true,
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
            _ => panic!("Invalid challenge result"),
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
            _ => panic!("Invalid challenge result"),
        }
    }

    #[test]
    fn add_sort_table() {
        let mut challenge_result = ChallengeResult::SortTable(Vec::new());
        let input = ChallengeInput::SortTable(SortTableRow {
            id: 1,
            values: vec!["Value 1".to_string()],
        });
        let result = challenge_result.add_input(input);
        assert!(result.is_ok());
        match challenge_result {
            ChallengeResult::SortTable(rows) => {
                assert_eq!(rows.len(), 1);
            }
            _ => panic!("Invalid challenge result"),
        }
    }

    #[test]
    fn test_is_empty() {
        let challenge_result = ChallengeResult::default();
        assert!(challenge_result.is_empty());
    }

    #[test]
    fn test_len() {
        let challenge_result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
        ]);
        assert_eq!(challenge_result.len(), 2);
    }

    #[test]
    fn add_ordering() {
        let mut challenge_result = ChallengeResult::Ordering(Vec::new());
        let input = ChallengeInput::Ordering(OrderingResult {
            order: vec![2, 0, 1],
        });
        let result = challenge_result.add_input(input);
        assert!(result.is_ok());
        match challenge_result {
            ChallengeResult::Ordering(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].order, vec![2, 0, 1]);
            }
            _ => panic!("Invalid challenge result"),
        }
    }

    #[test]
    fn test_ordering_len() {
        let challenge_result = ChallengeResult::Ordering(vec![
            OrderingResult {
                order: vec![0, 1, 2],
            },
            OrderingResult {
                order: vec![2, 1, 0],
            },
        ]);
        assert_eq!(challenge_result.len(), 2);
    }

    #[test]
    fn test_ordering_is_empty() {
        let challenge_result = ChallengeResult::Ordering(Vec::new());
        assert!(challenge_result.is_empty());
    }
}
