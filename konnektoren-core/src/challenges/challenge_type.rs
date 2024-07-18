use super::{ChallengeResult, Performance};
use crate::challenges::informative::Informative;
use crate::challenges::multiple_choice::MultipleChoice;
use crate::challenges::sort_table::SortTable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeType {
    #[serde(rename = "multiple-choice")]
    MultipleChoice(MultipleChoice),
    #[serde(rename = "sort-table")]
    SortTable(SortTable),
    #[serde(rename = "informative")]
    Informative(Informative),
}

impl Default for ChallengeType {
    fn default() -> Self {
        let data = include_str!("../assets/konnektoren.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

impl ChallengeType {
    pub fn of_tasks(&self, tasks: usize) -> Self {
        match self {
            ChallengeType::MultipleChoice(dataset) => {
                let mut new_dataset = dataset.clone();
                new_dataset.questions = dataset.questions.iter().take(tasks).cloned().collect();
                ChallengeType::MultipleChoice(new_dataset)
            }
            ChallengeType::SortTable(dataset) => {
                let mut new_dataset = dataset.clone();
                new_dataset.rows = dataset.rows.iter().take(tasks).cloned().collect();
                ChallengeType::SortTable(new_dataset)
            }
            ChallengeType::Informative(dataset) => ChallengeType::Informative(dataset.clone()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.name,
            ChallengeType::SortTable(dataset) => &dataset.name,
            ChallengeType::Informative(dataset) => &dataset.name,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.id,
            ChallengeType::SortTable(dataset) => &dataset.id,
            ChallengeType::Informative(dataset) => &dataset.id,
        }
    }
}

impl Performance for ChallengeType {
    fn performance(&self, result: &ChallengeResult) -> u32 {
        match (self, result) {
            (ChallengeType::MultipleChoice(dataset), ChallengeResult::MultipleChoice(options)) => {
                let mut score = 0;
                for (question, option) in dataset.questions.iter().zip(options.iter()) {
                    if question.option == option.id {
                        score += 1;
                    }
                }
                100 * score / dataset.questions.len() as u32
            }
            (ChallengeType::SortTable(dataset), ChallengeResult::SortTable(rows)) => {
                let mut score = 0;
                for (row, option) in dataset.rows.iter().zip(rows.iter()) {
                    if row.values.eq(option.values.as_slice()) {
                        score += 1;
                    }
                }
                100 * score / dataset.rows.len() as u32
            }
            (ChallengeType::Informative(_), ChallengeResult::Informative) => 100,
            _ => panic!("Invalid challenge type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::multiple_choice::MultipleChoiceOption;
    use crate::challenges::ChallengeResult;

    #[test]
    fn default_challenge() {
        let challenge = ChallengeType::default();
        match challenge {
            ChallengeType::MultipleChoice(dataset) => {
                assert_eq!(dataset.name, "Konnektoren");
                assert_eq!(dataset.options.len(), 5);
                assert!(!dataset.questions.is_empty());
            }
            _ => panic!("Invalid challenge type"),
        }
    }

    #[test]
    fn new_challenge() {
        let challenge = ChallengeType::default();
        let new_challenge = challenge.of_tasks(2);
        match new_challenge {
            ChallengeType::MultipleChoice(dataset) => {
                assert_eq!(dataset.questions.len(), 2);
            }
            _ => panic!("Invalid challenge type"),
        }
    }

    #[test]
    fn challenge_performance() {
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
        let performance = challenge.performance(&result);
        assert_eq!(performance, 0);
    }
}
