use serde::{Deserialize, Serialize};

use super::Challenge;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChallengeHistory {
    pub challenges: Vec<Challenge>,
}

impl ChallengeHistory {
    pub fn new() -> Self {
        ChallengeHistory { challenges: vec![] }
    }

    pub fn add_challenge(&mut self, challenge: Challenge) {
        self.challenges.push(challenge);
    }

    pub fn len(&self) -> usize {
        self.challenges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge::Challenge;
    use crate::challenges::challenge_config::ChallengeConfig;
    use crate::challenges::challenge_type::ChallengeType;

    #[test]
    fn new_challenge_history() {
        let challenge_history = ChallengeHistory::new();
        assert!(challenge_history.challenges.is_empty());
    }

    #[test]
    fn add_challenge() {
        let mut challenge_history = ChallengeHistory::new();
        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge_history.add_challenge(challenge);
        assert_eq!(challenge_history.challenges.len(), 1);
    }
}
