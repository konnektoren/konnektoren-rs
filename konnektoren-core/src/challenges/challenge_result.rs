use crate::challenges::multiple_choice::MultipleChoiceOption;
use crate::challenges::sort_table::SortTableRow;
use crate::challenges::{ChallengeInput, ContextItemChoiceAnswers, CustomChallengeResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeResult {
    MultipleChoice(Vec<MultipleChoiceOption>),
    ContextualChoice(Vec<ContextItemChoiceAnswers>),
    SortTable(Vec<SortTableRow>),
    Informative,
    Custom(CustomChallengeResult),
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
                _ => panic!("Invalid challenge input"),
            },
            ChallengeResult::ContextualChoice(answers) => match input {
                ChallengeInput::ContextualChoice(answer) => {
                    answers.push(answer);
                    Ok(())
                }
                _ => panic!("Invalid challenge input"),
            },
            ChallengeResult::SortTable(rows) => match input {
                ChallengeInput::SortTable(row) => {
                    rows.push(row);
                    Ok(())
                }
                _ => panic!("Invalid challenge input"),
            },
            ChallengeResult::Informative => Ok(()),
            ChallengeResult::Custom(_) => Ok(()),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ChallengeResult::MultipleChoice(options) => options.len(),
            ChallengeResult::ContextualChoice(items) => items.len(),
            ChallengeResult::SortTable(rows) => rows.len(),
            ChallengeResult::Informative => 0,
            ChallengeResult::Custom(_) => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ChallengeResult::MultipleChoice(options) => options.is_empty(),
            ChallengeResult::ContextualChoice(items) => items.is_empty(),
            ChallengeResult::SortTable(rows) => rows.is_empty(),
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
}
