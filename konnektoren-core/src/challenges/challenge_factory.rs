use crate::challenges::challenge::Challenge;
use crate::challenges::challenge_config::ChallengeConfig;
use crate::challenges::challenge_type::ChallengeType;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChallengeFactory {
    pub challenge_types: Vec<ChallengeType>,
}

impl Default for ChallengeFactory {
    fn default() -> Self {
        ChallengeFactory {
            challenge_types: vec![
                ChallengeType::default(),
                serde_yaml::from_str(include_str!("../assets/articles-1.yml")).unwrap(),
            ],
        }
    }
}

impl ChallengeFactory {
    pub fn new() -> Self {
        ChallengeFactory {
            challenge_types: vec![],
        }
    }

    pub fn create_challenge(&self, challenge_config: &ChallengeConfig) -> Result<Challenge> {
        let challenge_type = self
            .challenge_types
            .iter()
            .find(|challenge_type| challenge_type.id() == challenge_config.challenge)
            .ok_or_else(|| anyhow::anyhow!("Challenge type not found"))?;
        Ok(Challenge::new(
            &challenge_type.of_tasks(&challenge_config.tasks),
            challenge_config,
        ))
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
            tasks: 2.into(),
            ..Default::default()
        };

        let challenge = challenge_factory.create_challenge(&challenge_config);
        match challenge.unwrap().challenge_type {
            ChallengeType::MultipleChoice(dataset) => {
                assert_eq!(dataset.questions.len(), 2);
            }
            _ => panic!("Invalid challenge type"),
        }
    }
}
