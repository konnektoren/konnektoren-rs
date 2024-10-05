use crate::challenges::{ChallengeConfig, ChallengeHistory, Performance};

pub trait ChallengeStats {
    fn challenges(&self) -> usize;

    fn completed_challenges(&self) -> usize;

    fn stars(&self) -> u32;

    fn performance(&self) -> u32;

    fn solved(&self) -> bool;
}

impl ChallengeStats for ChallengeHistory {
    fn challenges(&self) -> usize {
        self.len()
    }

    fn completed_challenges(&self) -> usize {
        self.challenges.iter().filter(|c| c.solved()).count()
    }

    fn stars(&self) -> u32 {
        let performance = self.performance();
        if performance >= 80 {
            3
        } else if performance >= 60 {
            2
        } else if performance >= 40 {
            1
        } else {
            0
        }
    }

    fn performance(&self) -> u32 {
        let challenges = self.challenges();
        if challenges == 0 {
            return 0;
        }
        let completed_challenges = self.completed_challenges();
        (completed_challenges as f32 / challenges as f32 * 100.0) as u32
    }

    fn solved(&self) -> bool {
        self.challenges.iter().all(|c| c.solved())
    }
}

impl ChallengeStats for (&ChallengeConfig, &ChallengeHistory) {
    fn challenges(&self) -> usize {
        self.1
            .challenges
            .iter()
            .filter(|c| c.challenge_config.id == self.0.id)
            .count()
    }

    fn completed_challenges(&self) -> usize {
        self.1
            .challenges
            .iter()
            .filter(|c| {
                c.challenge_config.id == self.0.id
                    && c.challenge_result.len() == c.challenge_config.tasks.len()
            })
            .count()
    }

    fn stars(&self) -> u32 {
        let performance = self.performance();
        if performance >= 80 {
            3
        } else if performance >= 60 {
            2
        } else if performance >= 40 {
            1
        } else {
            0
        }
    }

    fn performance(&self) -> u32 {
        let challenges = self.challenges();
        if challenges == 0 {
            return 0;
        }
        let total_performance: u32 = self
            .1
            .challenges
            .iter()
            .map(|c| c.performance(&c.challenge_result))
            .sum();
        total_performance / challenges as u32
    }

    fn solved(&self) -> bool {
        let challenge_id = self.0.id.clone();
        self.1
            .challenges
            .iter()
            .filter(|c| c.challenge_config.id == challenge_id)
            .count()
            > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge::Challenge;
    use crate::challenges::challenge_config::ChallengeConfig;
    use crate::challenges::challenge_type::ChallengeType;
    use crate::challenges::{ChallengeResult, MultipleChoiceOption};

    #[test]
    fn test_challenges() {
        let mut challenge_history = ChallengeHistory::new();
        assert_eq!(challenge_history.challenges(), 0);

        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge_history.add_challenge(challenge);
        assert_eq!(challenge_history.challenges(), 1);
    }

    #[test]
    fn test_completed_challenges() {
        let mut challenge_history = ChallengeHistory::new();
        assert_eq!(challenge_history.completed_challenges(), 0);

        let mut config = ChallengeConfig::default();
        config.tasks = 3.into();
        let challenge = Challenge::new(&ChallengeType::default(), &config);
        challenge_history.add_challenge(challenge);
        assert_eq!(challenge_history.completed_challenges(), 0);

        let mut challenge = Challenge::new(&ChallengeType::default(), &config);
        challenge.challenge_result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
            MultipleChoiceOption {
                id: 3,
                name: "Option 3".to_string(),
            },
        ]);
        challenge_history.add_challenge(challenge);
        assert_eq!(challenge_history.completed_challenges(), 1);
    }

    #[test]
    fn test_stars() {
        let mut challenge_history = ChallengeHistory::new();
        assert_eq!(challenge_history.stars(), 0);

        let mut config = ChallengeConfig::default();
        config.tasks = 3.into();

        let mut challenge = Challenge::new(&ChallengeType::default(), &config);
        challenge.challenge_result = ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
            MultipleChoiceOption {
                id: 3,
                name: "Option 3".to_string(),
            },
        ]);
        challenge_history.add_challenge(challenge);

        assert!(challenge_history.stars() > 0);
    }
}
