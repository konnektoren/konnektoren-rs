use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChallengeEvent {
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    Started,
    Completed,
}
