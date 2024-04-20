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
}
