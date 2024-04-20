use crate::challenges::challenge_config::ChallengeConfig;
use crate::challenges::challenge_type::ChallengeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Challenge {
    pub challenge_type: ChallengeType,
    pub challenge_config: ChallengeConfig,
}

impl Challenge {
    pub fn new(challenge_type: &ChallengeType, challenge_config: &ChallengeConfig) -> Self {
        Challenge {
            challenge_type: challenge_type.clone(),
            challenge_config: challenge_config.clone(),
        }
    }
}
