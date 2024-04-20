use crate::challenges::multiple_choice::MultipleChoiceOption;
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