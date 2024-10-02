use crate::challenges::multiple_choice::MultipleChoiceOption;
use crate::challenges::sort_table::SortTableRow;
use crate::challenges::ContextItemChoiceAnswers;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeInput {
    MultipleChoice(MultipleChoiceOption),
    SortTable(SortTableRow),
    ContextualChoice(ContextItemChoiceAnswers),
}
