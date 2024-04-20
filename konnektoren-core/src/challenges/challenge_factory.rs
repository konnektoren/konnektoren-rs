use crate::challenges::challenge::Challenge;
use crate::challenges::challenge_config::ChallengeConfig;
use crate::challenges::challenge_type::ChallengeType;

#[derive(Debug, Default, Clone)]
pub struct ChallengeFactory {
    pub challenge_types: Vec<ChallengeType>,
}

impl ChallengeFactory {
    pub fn new() -> Self {
        ChallengeFactory {
            challenge_types: vec![],
        }
    }

    pub fn create_challenge(&self, challenge_config: &ChallengeConfig) -> Challenge {
        let challenge_type = self
            .challenge_types
            .iter()
            .find(|challenge_type| challenge_type.id() == challenge_config.challenge)
            .unwrap();
        Challenge::new(
            &challenge_type.of_questions(challenge_config.questions),
            challenge_config,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_challenge() {
        let mut challenge_factory = ChallengeFactory::new();
        let challenge_type = ChallengeType::default();
        challenge_factory.challenge_types.push(challenge_type);
        let challenge_config = ChallengeConfig {
            challenge: "konnektoren".to_string(),
            questions: 2,
            ..Default::default()
        };

        let challenge = challenge_factory.create_challenge(&challenge_config);
        match challenge.challenge_type {
            ChallengeType::MultipleChoice(dataset) => {
                assert_eq!(dataset.questions.len(), 2);
            }
        }
    }
}
