use crate::challenges::multiple_choice::MultipleChoice;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeType {
    #[serde(rename = "multiple-choice")]
    MultipleChoice(MultipleChoice),
}

impl Default for ChallengeType {
    fn default() -> Self {
        let data = include_str!("../assets/konnektoren.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

impl ChallengeType {
    pub fn of_questions(&self, questions: usize) -> Self {
        match self {
            ChallengeType::MultipleChoice(dataset) => {
                let mut new_dataset = dataset.clone();
                new_dataset.questions = dataset.questions.iter().take(questions).cloned().collect();
                ChallengeType::MultipleChoice(new_dataset)
            }
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.name,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            ChallengeType::MultipleChoice(dataset) => &dataset.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_challenge() {
        let challenge = ChallengeType::default();
        match challenge {
            ChallengeType::MultipleChoice(dataset) => {
                assert_eq!(dataset.name, "Konnektoren");
                assert_eq!(dataset.options.len(), 5);
                assert!(dataset.questions.len() > 0);
            }
        }
    }

    #[test]
    fn new_challenge() {
        let challenge = ChallengeType::default();
        let new_challenge = challenge.of_questions(2);
        match new_challenge {
            ChallengeType::MultipleChoice(dataset) => {
                assert_eq!(dataset.questions.len(), 2);
            }
        }
    }
}
