use super::{ChallengeResult, ContextualChoice, Custom, Performance};
use crate::challenges::informative::Informative;
use crate::challenges::multiple_choice::MultipleChoice;
use crate::challenges::sort_table::SortTable;
use crate::challenges::task_pattern::TaskPattern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeType {
    #[serde(rename = "multiple-choice")]
    MultipleChoice(MultipleChoice),
    #[serde(rename = "contextual-choice")]
    ContextualChoice(ContextualChoice),
    #[serde(rename = "sort-table")]
    SortTable(SortTable),
    #[serde(rename = "informative")]
    Informative(Informative),
    #[serde(rename = "custom")]
    Custom(Custom),
}

impl Default for ChallengeType {
    fn default() -> Self {
        let data = include_str!("../assets/konnektoren.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

impl ChallengeType {
    pub fn of_tasks(&self, task_pattern: &TaskPattern) -> Self {
        match self {
            ChallengeType::MultipleChoice(dataset) => {
                let selected_questions = task_pattern.select_items(&dataset.questions);
                let mut new_dataset = dataset.clone();
                new_dataset.questions = selected_questions;
                ChallengeType::MultipleChoice(new_dataset)
            }
            ChallengeType::ContextualChoice(dataset) => {
                let selected_items = task_pattern.select_items(&dataset.items);
                let mut new_dataset = dataset.clone();
                new_dataset.items = selected_items;
                ChallengeType::ContextualChoice(new_dataset)
            }
            ChallengeType::SortTable(dataset) => {
                let selected_rows = task_pattern.select_items(&dataset.rows);
                let mut new_dataset = dataset.clone();
                new_dataset.rows = selected_rows;
                ChallengeType::SortTable(new_dataset)
            }
            ChallengeType::Informative(dataset) => ChallengeType::Informative(dataset.clone()),
            ChallengeType::Custom(dataset) => ChallengeType::Custom(dataset.clone()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.name,
            ChallengeType::ContextualChoice(dataset) => &dataset.name,
            ChallengeType::SortTable(dataset) => &dataset.name,
            ChallengeType::Informative(dataset) => &dataset.name,
            ChallengeType::Custom(dataset) => &dataset.name,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.id,
            ChallengeType::ContextualChoice(dataset) => &dataset.id,
            ChallengeType::SortTable(dataset) => &dataset.id,
            ChallengeType::Informative(dataset) => &dataset.id,
            ChallengeType::Custom(dataset) => &dataset.id,
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
            (
                ChallengeType::ContextualChoice(dataset),
                ChallengeResult::ContextualChoice(choices),
            ) => {
                let mut score = 0;
                for (item, choice) in dataset.items.iter().zip(choices.iter()) {
                    if item.choices.iter().zip(&choice.ids).all(|(c, &id)| {
                        c.options
                            .get(id)
                            .map_or(false, |selected| *selected == c.correct_answer)
                    }) {
                        score += 1;
                    }
                }
                100 * score / dataset.items.len() as u32
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
            (ChallengeType::Custom(_), ChallengeResult::Custom(result)) => {
                (100.0 * result.performance) as u32
            }
            _ => panic!("Invalid challenge type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::multiple_choice::MultipleChoiceOption;
    use crate::challenges::{ChallengeResult, ContextItemChoiceAnswers};

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
        let new_challenge = challenge.of_tasks(&TaskPattern::Exact(2));
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

    #[test]
    fn challenge_name() {
        let challenge = ChallengeType::default();
        let name = challenge.name();
        assert_eq!(name, "Konnektoren");
    }

    #[test]
    fn challenge_id() {
        let challenge = ChallengeType::default();
        let id = challenge.id();
        assert_eq!(id, "konnektoren");
    }

    #[test]
    fn contextual_choice_performance() {
        let challenge: ContextualChoice =
            serde_yaml::from_str(include_str!("../assets/konjunktiv-2.yml")).unwrap();
        let result = ChallengeResult::ContextualChoice(vec![
            ContextItemChoiceAnswers { ids: vec![0, 0] },
            ContextItemChoiceAnswers { ids: vec![1, 0] },
        ]);

        let performance = ChallengeType::ContextualChoice(challenge).performance(&result);
        assert_eq!(performance, 100);
    }
}
