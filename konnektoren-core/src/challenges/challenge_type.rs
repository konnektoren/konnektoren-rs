use super::{ChallengeResult, ContextualChoice, Custom, Performance, Placeholder};
use crate::challenges::gap_fill::GapFill;
use crate::challenges::informative::Informative;
use crate::challenges::multiple_choice::MultipleChoice;
use crate::challenges::ordering::Ordering;
use crate::challenges::sort_table::SortTable;
use crate::challenges::task_pattern::TaskPattern;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumIter, IntoStaticStr)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ChallengeType {
    MultipleChoice(MultipleChoice),
    ContextualChoice(ContextualChoice),
    GapFill(GapFill),
    SortTable(SortTable),
    Informative(Informative),
    Ordering(Ordering),
    Custom(Custom),
    Placeholder(Placeholder),
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
            ChallengeType::GapFill(dataset) => {
                let selected_questions = task_pattern.select_items(&dataset.questions);
                let mut new_dataset = dataset.clone();
                new_dataset.questions = selected_questions;
                ChallengeType::GapFill(new_dataset)
            }
            ChallengeType::SortTable(dataset) => {
                let selected_rows = task_pattern.select_items(&dataset.rows);
                let mut new_dataset = dataset.clone();
                new_dataset.rows = selected_rows;
                ChallengeType::SortTable(new_dataset)
            }
            ChallengeType::Informative(dataset) => ChallengeType::Informative(dataset.clone()),
            ChallengeType::Ordering(dataset) => ChallengeType::Ordering(dataset.clone()),
            ChallengeType::Custom(dataset) => {
                let mut dataset = dataset.clone();
                let ids: Vec<_> = (0..100).collect();
                let task_ids = task_pattern.select_items(&ids);
                dataset.task_ids = Some(task_ids);
                ChallengeType::Custom(dataset)
            }
            ChallengeType::Placeholder(dataset) => ChallengeType::Placeholder(dataset.clone()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.name,
            ChallengeType::ContextualChoice(dataset) => &dataset.name,
            ChallengeType::SortTable(dataset) => &dataset.name,
            ChallengeType::GapFill(dataset) => &dataset.name,
            ChallengeType::Informative(dataset) => &dataset.name,
            ChallengeType::Ordering(dataset) => &dataset.name,
            ChallengeType::Custom(dataset) => &dataset.name,
            ChallengeType::Placeholder(dataset) => &dataset.name,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.id,
            ChallengeType::ContextualChoice(dataset) => &dataset.id,
            ChallengeType::GapFill(dataset) => &dataset.id,
            ChallengeType::SortTable(dataset) => &dataset.id,
            ChallengeType::Informative(dataset) => &dataset.id,
            ChallengeType::Ordering(dataset) => &dataset.id,
            ChallengeType::Custom(dataset) => &dataset.id,
            ChallengeType::Placeholder(dataset) => &dataset.id,
        }
    }
}

impl Performance for ChallengeType {
    fn performance(&self, result: &ChallengeResult) -> u32 {
        match (self, result) {
            (ChallengeType::MultipleChoice(dataset), ChallengeResult::MultipleChoice(options)) => {
                if dataset.questions.is_empty() {
                    return 0;
                }
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
                if dataset.items.is_empty() {
                    return 0;
                }
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
            (ChallengeType::GapFill(dataset), ChallengeResult::GapFill(answers)) => {
                if dataset.questions.is_empty() {
                    return 0;
                }
                let mut score = 0;
                for (question, answer) in dataset.questions.iter().zip(answers.iter()) {
                    if question
                        .gaps
                        .iter()
                        .zip(answer.answers.iter())
                        .all(|(gap, ans)| gap.correct == *ans)
                    {
                        score += 1;
                    }
                }
                100 * score / dataset.questions.len() as u32
            }
            (ChallengeType::SortTable(dataset), ChallengeResult::SortTable(rows)) => {
                if dataset.rows.is_empty() {
                    return 0;
                }
                let mut score = 0;
                for (row, option) in dataset.rows.iter().zip(rows.iter()) {
                    if row.values.eq(option.values.as_slice()) {
                        score += 1;
                    }
                }
                100 * score / dataset.rows.len() as u32
            }
            (ChallengeType::Informative(_), _) => 100,
            (ChallengeType::Ordering(dataset), ChallengeResult::Ordering(results)) => {
                if dataset.items.is_empty() {
                    return 0;
                }
                let mut score = 0;
                for (item, result) in dataset.items.iter().zip(results.iter()) {
                    if item.correct_order == result.order {
                        score += 1;
                    }
                }
                100 * score / dataset.items.len() as u32
            }
            (ChallengeType::Custom(_), ChallengeResult::Custom(result)) => {
                (100.0 * result.performance) as u32
            }
            (ChallengeType::Placeholder(_), _) => 0,
            _ => panic!("Invalid challenge type {:?}", self),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::challenges::{
        Challenge, ChallengeConfig, ChallengeResult, ContextItemChoiceAnswers,
        MultipleChoiceOption, Question,
    };
    use strum::IntoEnumIterator;

    use chrono::{TimeZone, Utc};

    // Base timestamp for tests
    pub const BASE_TIMESTAMP: i64 = 1700000000;

    // Helper functions for test challenges
    pub fn create_successful_challenge() -> Challenge {
        let challenge_type = ChallengeType::MultipleChoice(MultipleChoice {
            id: "test".to_string(),
            name: "Test".to_string(),
            lang: "en".to_string(),
            options: vec![
                MultipleChoiceOption {
                    id: 0,
                    name: "Correct".to_string(),
                },
                MultipleChoiceOption {
                    id: 1,
                    name: "Wrong".to_string(),
                },
            ],
            questions: vec![Question {
                question: "Test Question".to_string(),
                help: "Test Help".to_string(),
                option: 0,
                image: None,
            }],
        });

        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());

        // Set fixed timestamps
        challenge.start_time = Some(Utc.timestamp_opt(BASE_TIMESTAMP, 0).unwrap());

        // Add correct answer (id: 0)
        challenge.challenge_result = ChallengeResult::MultipleChoice(vec![MultipleChoiceOption {
            id: 0,
            name: "Correct".to_string(),
        }]);

        challenge.end_time = Some(Utc.timestamp_opt(BASE_TIMESTAMP + 3600, 0).unwrap()); // 1 hour later
        challenge
    }

    pub fn create_unsuccessful_challenge() -> Challenge {
        let challenge_type = ChallengeType::MultipleChoice(MultipleChoice {
            id: "test".to_string(),
            name: "Test".to_string(),
            lang: "en".to_string(),
            options: vec![
                MultipleChoiceOption {
                    id: 0,
                    name: "Correct".to_string(),
                },
                MultipleChoiceOption {
                    id: 1,
                    name: "Wrong".to_string(),
                },
            ],
            questions: vec![Question {
                question: "Test Question".to_string(),
                help: "Test Help".to_string(),
                option: 0,
                image: None,
            }],
        });

        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());

        // Set fixed timestamps
        challenge.start_time = Some(Utc.timestamp_opt(BASE_TIMESTAMP + 7200, 0).unwrap()); // 2 hours after base

        // Add wrong answer (id: 1)
        challenge.challenge_result = ChallengeResult::MultipleChoice(vec![MultipleChoiceOption {
            id: 1,
            name: "Wrong".to_string(),
        }]);

        challenge.end_time = Some(Utc.timestamp_opt(BASE_TIMESTAMP + 10800, 0).unwrap()); // 3 hours after base
        challenge
    }

    #[test]
    fn test_successful_challenge_performance() {
        let challenge = create_successful_challenge();
        assert_eq!(
            challenge.performance(&challenge.challenge_result),
            100,
            "Successful challenge should have 100% performance"
        );
    }

    #[test]
    fn test_unsuccessful_challenge_performance() {
        let challenge = create_unsuccessful_challenge();
        assert_eq!(
            challenge.performance(&challenge.challenge_result),
            0,
            "Unsuccessful challenge should have 0% performance"
        );
    }

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

    #[test]
    fn test_strum_into_static_str() {
        assert_eq!(
            <ChallengeType as Into<&'static str>>::into(ChallengeType::MultipleChoice(
                MultipleChoice::default()
            )),
            "multiple-choice"
        );
        assert_eq!(
            <ChallengeType as Into<&'static str>>::into(ChallengeType::ContextualChoice(
                ContextualChoice::default()
            )),
            "contextual-choice"
        );
    }

    #[test]
    fn test_variant_iteration() {
        let variants: Vec<ChallengeType> = ChallengeType::iter()
            .map(|v| match v {
                ChallengeType::MultipleChoice(_) => {
                    ChallengeType::MultipleChoice(MultipleChoice::default())
                }
                ChallengeType::ContextualChoice(_) => {
                    ChallengeType::ContextualChoice(ContextualChoice::default())
                }
                _ => v,
            })
            .collect();

        assert!(!variants.is_empty());
        assert!(variants
            .iter()
            .any(|v| matches!(v, ChallengeType::MultipleChoice(_))));
        assert!(variants
            .iter()
            .any(|v| matches!(v, ChallengeType::ContextualChoice(_))));
    }

    #[test]
    fn test_serde_serialization() {
        let challenge = ChallengeType::MultipleChoice(MultipleChoice::default());
        let serialized = serde_yaml::to_string(&challenge).unwrap();
        assert!(serialized.contains("multiple-choice"));
    }
}
