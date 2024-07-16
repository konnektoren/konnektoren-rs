use serde::{Deserialize, Serialize};

use crate::challenges::multiple_choice::MultipleChoiceOption;
use crate::challenges::sort_table::SortTableRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeInput {
    MultipleChoice(MultipleChoiceOption),
    SortTable(SortTableRow),
}
