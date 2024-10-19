use super::{event::EventTrait, EventType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChallengeEvent {
    SolvedCorrect(usize),
    SolvedIncorrect(usize),
    Started,
    Completed,
}

impl Default for ChallengeEvent {
    fn default() -> Self {
        ChallengeEvent::Started
    }
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
        }
    }
}
