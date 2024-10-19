use super::event::EventTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ChallengeEvent {
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    #[default]
    Started,
    Completed,
}

impl EventTrait for ChallengeEvent {
    fn get_type(&self) -> &str {
        "Challenge"
    }

    fn get_action(&self) -> &str {
        match self {
            ChallengeEvent::SolvedCorrect(_) => "SolvedCorrect",
            ChallengeEvent::SolvedIncorrect(_) => "SolvedIncorrect",
            ChallengeEvent::Started => "Started",
            ChallengeEvent::Completed => "Completed",
        }
    }
}
