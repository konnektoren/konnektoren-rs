use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChallengeVariant {
    #[serde(rename = "multiple-choice")]
    #[default]
    MultipleChoice,
    #[serde(rename = "multiple-choice-circle")]
    MultipleChoiceCircle,
    #[serde(rename = "sort-table")]
    SortTable,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChallengeConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub challenge: String,
    pub variant: Option<ChallengeVariant>,
    pub tasks: usize,
    pub unlock_points: usize,
    pub position: Option<(i32, i32)>,
}

impl Default for ChallengeConfig {
    fn default() -> Self {
        ChallengeConfig {
            id: "123".to_string(),
            name: "Konnektoren #1".to_string(),
            description: "Your first Konnektoren challenge!".to_string(),
            challenge: "konnektoren".to_string(),
            variant: None,
            tasks: 10,
            unlock_points: 0,
            position: Some((0, 0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_challenge_config() {
        let challenge_config = ChallengeConfig::default();
        assert_eq!(challenge_config.id, "123");
        assert_eq!(challenge_config.name, "Konnektoren #1");
        assert_eq!(
            challenge_config.description,
            "Your first Konnektoren challenge!"
        );
        assert_eq!(challenge_config.challenge, "konnektoren");
        assert_eq!(challenge_config.tasks, 10);
        assert_eq!(challenge_config.unlock_points, 0);
    }
}
