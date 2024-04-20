use serde::{Deserialize, Serialize};

use crate::challenges::multiple_choice::MultipleChoiceOption;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeInput {
    MultipleChoice(MultipleChoiceOption),
}
