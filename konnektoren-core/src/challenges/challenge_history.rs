use serde::{Deserialize, Serialize};

use super::Challenge;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChallengeHistory {
    pub challenges: Vec<Challenge>,
}

impl ChallengeHistory {
    pub fn new() -> Self {
        ChallengeHistory { challenges: vec![] }
    }

    /// Add a challenge to the history if it is different from the last one.
    pub fn add_challenge(&mut self, challenge: Challenge) {
        if let Some(last) = self.challenges.last() {
            if last != &challenge {
                self.challenges.push(challenge);
            }
        } else {
            self.challenges.push(challenge);
        }
    }

    pub fn len(&self) -> usize {
        self.challenges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.challenges.is_empty()
    }

    pub fn extend(&mut self, other: &ChallengeHistory) {
        self.challenges.extend(other.challenges.iter().cloned());
    }

    pub fn merge(&mut self, other: &ChallengeHistory) {
        for challenge in &other.challenges {
            if !self.challenges.contains(challenge) {
                self.challenges.push(challenge.clone());
            }
        }
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

    #[test]
    fn test_len() {
        let mut challenge_history = ChallengeHistory::new();
        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge_history.add_challenge(challenge);
        assert_eq!(challenge_history.len(), 1);
    }

    #[test]
    fn test_is_empty() {
        let challenge_history = ChallengeHistory::new();
        assert!(challenge_history.is_empty());

        let mut challenge_history = ChallengeHistory::new();
        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge_history.add_challenge(challenge);
        assert!(!challenge_history.is_empty());
    }

    #[test]
    fn test_extend() {
        let mut challenge_history = ChallengeHistory::new();
        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge_history.add_challenge(challenge);

        let mut other_challenge_history = ChallengeHistory::new();
        let other_challenge =
            Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        other_challenge_history.add_challenge(other_challenge);

        challenge_history.extend(&mut other_challenge_history);
        assert_eq!(challenge_history.len(), 2);
    }

    #[test]
    fn test_merge() {
        let mut challenge_history1 = ChallengeHistory::new();
        let challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge_history1.add_challenge(challenge1.clone());

        let mut challenge_history2 = ChallengeHistory::new();
        challenge_history2.add_challenge(challenge1.clone());
        let challenge2 = Challenge::new(
            &ChallengeType::default(),
            &ChallengeConfig {
                id: "456".to_string(),
                ..ChallengeConfig::default()
            },
        );
        challenge_history2.add_challenge(challenge2.clone());

        challenge_history1.merge(&challenge_history2);
        assert_eq!(challenge_history1.len(), 2);
        assert!(challenge_history1.challenges.contains(&challenge1));
        assert!(challenge_history1.challenges.contains(&challenge2));
    }
}
