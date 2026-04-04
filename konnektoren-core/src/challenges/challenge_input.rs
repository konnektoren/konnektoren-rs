use crate::challenges::{
    ContextItemChoiceAnswers, DialogAnswer, GapFillAnswer, MultipleChoiceOption, OrderingResult,
    SortTableRow,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeInput {
    MultipleChoice(MultipleChoiceOption),
    ContextualChoice(ContextItemChoiceAnswers),
    GapFill(GapFillAnswer),
    SortTable(SortTableRow),
    Ordering(OrderingResult),
    Dialog(DialogAnswer),
}
