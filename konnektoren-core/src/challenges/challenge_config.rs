use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChallengeConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub challenge: String,
    pub questions: usize,
    pub unlock_points: usize,
}
