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
    Vocabulary,
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
            ChallengeResult::Vocabulary => Ok(()),
        }
    }

    /// Sets the input at a specific index, filling gaps with default values if needed
    pub fn set_input(&mut self, index: usize, input: ChallengeInput) -> Result<()> {
        match self {
            ChallengeResult::MultipleChoice(options) => match input {
                ChallengeInput::MultipleChoice(option) => {
                    // Ensure we have enough slots, fill with default if needed
                    while options.len() <= index {
                        options.push(MultipleChoiceOption {
                            id: 0,
                            name: String::new(),
                        });
                    }
                    options[index] = option;
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected MultipleChoice input".to_string(),
                )),
            },
            ChallengeResult::ContextualChoice(answers) => match input {
                ChallengeInput::ContextualChoice(answer) => {
                    while answers.len() <= index {
                        answers.push(ContextItemChoiceAnswers { ids: vec![] });
                    }
                    answers[index] = answer;
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected ContextualChoice input".to_string(),
                )),
            },
            ChallengeResult::GapFill(answers) => match input {
                ChallengeInput::GapFill(answer) => {
                    while answers.len() <= index {
                        answers.push(GapFillAnswer {
                            question_index: 0,
                            answers: vec![],
                        });
                    }
                    answers[index] = answer;
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected GapFill input".to_string(),
                )),
            },
            ChallengeResult::SortTable(rows) => match input {
                ChallengeInput::SortTable(row) => {
                    while rows.len() <= index {
                        rows.push(SortTableRow {
                            id: 0,
                            values: vec![],
                        });
                    }
                    rows[index] = row;
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected SortTable input".to_string(),
                )),
            },
            ChallengeResult::Ordering(results) => match input {
                ChallengeInput::Ordering(result) => {
                    while results.len() <= index {
                        results.push(OrderingResult { order: vec![] });
                    }
                    results[index] = result;
                    Ok(())
                }
                _ => Err(ChallengeError::InvalidInput(
                    "Expected Ordering input".to_string(),
                )),
            },
            ChallengeResult::Informative => Ok(()),
            ChallengeResult::Custom(_) => Ok(()),
            ChallengeResult::Vocabulary => Ok(()),
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
            ChallengeResult::Vocabulary => 0,
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
            ChallengeResult::Vocabulary => true,
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

    #[test]
    fn test_add_input_wrong_type() {
        let mut result = ChallengeResult::MultipleChoice(vec![]);
        let input = ChallengeInput::SortTable(SortTableRow::default());
        let err = result.add_input(input);
        assert!(err.is_err());
    }

    #[test]
    fn test_set_input_multiple_choice() {
        let mut challenge_result = ChallengeResult::default();
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption {
            id: 2,
            name: "Option 2".to_string(),
        });

        // Set at index 2 (will fill 0, 1 with defaults)
        let result = challenge_result.set_input(2, input);
        assert!(result.is_ok());

        match challenge_result {
            ChallengeResult::MultipleChoice(options) => {
                assert_eq!(options.len(), 3);
                assert_eq!(options[0].id, 0); // Default
                assert_eq!(options[1].id, 0); // Default
                assert_eq!(options[2].id, 2); // Our input
                assert_eq!(options[2].name, "Option 2");
            }
            _ => panic!("Invalid challenge result"),
        }
    }

    #[test]
    fn test_set_input_replace_existing() {
        let mut challenge_result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
        ]);

        let new_input = ChallengeInput::MultipleChoice(MultipleChoiceOption {
            id: 3,
            name: "Option 3".to_string(),
        });

        // Replace index 1
        let result = challenge_result.set_input(1, new_input);
        assert!(result.is_ok());

        match challenge_result {
            ChallengeResult::MultipleChoice(options) => {
                assert_eq!(options.len(), 2);
                assert_eq!(options[0].id, 1); // Unchanged
                assert_eq!(options[1].id, 3); // Replaced
                assert_eq!(options[1].name, "Option 3");
            }
            _ => panic!("Invalid challenge result"),
        }
    }

    #[test]
    fn test_set_input_wrong_type() {
        let mut result = ChallengeResult::MultipleChoice(vec![]);
        let input = ChallengeInput::SortTable(SortTableRow::default());
        let err = result.set_input(0, input);
        assert!(err.is_err());
    }

    #[test]
    fn test_set_input_ordering() {
        let mut challenge_result = ChallengeResult::Ordering(Vec::new());
        let input = ChallengeInput::Ordering(OrderingResult {
            order: vec![2, 0, 1],
        });

        let result = challenge_result.set_input(1, input);
        assert!(result.is_ok());

        match challenge_result {
            ChallengeResult::Ordering(results) => {
                assert_eq!(results.len(), 2);
                assert!(results[0].order.is_empty()); // Default
                assert_eq!(results[1].order, vec![2, 0, 1]);
            }
            _ => panic!("Invalid challenge result"),
        }
    }

    #[test]
    fn test_set_input_contextual_choice() {
        let mut result = ChallengeResult::ContextualChoice(Vec::new());
        let input = ChallengeInput::ContextualChoice(ContextItemChoiceAnswers { ids: vec![0, 1] });

        let res = result.set_input(0, input);
        assert!(res.is_ok());

        match result {
            ChallengeResult::ContextualChoice(answers) => {
                assert_eq!(answers.len(), 1);
                assert_eq!(answers[0].ids, vec![0, 1]);
            }
            _ => panic!("Expected ContextualChoice result"),
        }
    }

    #[test]
    fn test_set_input_fills_gaps_contextual_choice() {
        let mut result = ChallengeResult::ContextualChoice(Vec::new());
        let input = ChallengeInput::ContextualChoice(ContextItemChoiceAnswers { ids: vec![2] });

        // Set at index 2, should fill 0 and 1 with defaults
        let res = result.set_input(2, input);
        assert!(res.is_ok());

        match result {
            ChallengeResult::ContextualChoice(answers) => {
                assert_eq!(answers.len(), 3);
                assert!(answers[0].ids.is_empty()); // Default
                assert!(answers[1].ids.is_empty()); // Default
                assert_eq!(answers[2].ids, vec![2]);
            }
            _ => panic!("Expected ContextualChoice result"),
        }
    }

    #[test]
    fn test_set_input_wrong_type_contextual_choice() {
        let mut result = ChallengeResult::ContextualChoice(Vec::new());
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());

        let res = result.set_input(0, input);
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), ChallengeError::InvalidInput(_)));
    }
}
