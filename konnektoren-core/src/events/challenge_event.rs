use super::{EventType, event::EventTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ChallengeEvent {
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    #[default]
    Started,
    Completed,
    Error(String),
}

impl EventTrait for ChallengeEvent {
    fn get_type(&self) -> EventType {
        EventType::Challenge
    }

    fn get_action(&self) -> &str {
        match self {
            ChallengeEvent::SolvedCorrect(_) => "SolvedCorrect",
            ChallengeEvent::SolvedIncorrect(_) => "SolvedIncorrect",
            ChallengeEvent::Started => "Started",
            ChallengeEvent::Completed => "Completed",
            ChallengeEvent::Error(_) => "Error",
        }
    }
}
