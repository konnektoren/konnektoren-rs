use serde::{Deserialize, Serialize};

pub mod multiple_choice;
pub use multiple_choice::MultipleChoice;
pub use multiple_choice::MultipleChoiceOption;
pub use multiple_choice::Question;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Challenge {
    #[serde(rename = "multiple-choice")]
    MultipleChoice(MultipleChoice),
}

impl Default for Challenge {
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
        let challenge = Challenge::default();
        match challenge {
            Challenge::MultipleChoice(dataset) => {
                assert_eq!(dataset.name, "Konnektoren");
                assert_eq!(dataset.options.len(), 5);
                assert!(dataset.questions.len() > 0);
            }
        }
    }
}
