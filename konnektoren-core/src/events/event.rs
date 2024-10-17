use super::ChallengeEvent;
use super::GameEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Game(GameEvent),
    Challenge(ChallengeEvent),
}
