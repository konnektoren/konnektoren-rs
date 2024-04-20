use crate::challenges::challenge_config::ChallengeConfig;
use crate::challenges::challenge_result::ChallengeResult;
use crate::challenges::challenge_type::ChallengeType;
use serde::{Deserialize, Serialize};

use super::{Performance, Solvable};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Challenge {
    pub challenge_type: ChallengeType,
    pub challenge_config: ChallengeConfig,
    pub challenge_result: ChallengeResult,
}

impl Challenge {
    pub fn new(challenge_type: &ChallengeType, challenge_config: &ChallengeConfig) -> Self {
        Challenge {
            challenge_type: challenge_type.clone(),
            challenge_config: challenge_config.clone(),
            challenge_result: ChallengeResult::default(),
        }
    }
}

impl Solvable for Challenge {
    fn solve(&mut self, input: super::ChallengeInput) -> anyhow::Result<bool> {
        match self.challenge_result.add_input(input) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl Performance for Challenge {
    fn performance(&self, result: &ChallengeResult) -> i32 {
        self.challenge_type.performance(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let challenge = Challenge::new(&challenge_type, &challenge_config);
        assert_eq!(challenge.challenge_type, challenge_type);
        assert_eq!(challenge.challenge_config, challenge_config);
        assert_eq!(challenge.challenge_result, ChallengeResult::default());
    }
}
