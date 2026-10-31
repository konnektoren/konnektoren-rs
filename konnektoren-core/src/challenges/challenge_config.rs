use super::challenge_variant::ChallengeVariant;
use crate::challenges::task_pattern::TaskPattern;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ChallengeConfig<V = ChallengeVariant> {
    pub id: String,
    pub name: String,
    pub description: String,
    pub challenge: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<V>,
    /// Number or range of tasks to select (e.g. "10", "5:random", "1..5")
    #[cfg_attr(feature = "schema", schemars(with = "String"))]
    pub tasks: TaskPattern,
    pub unlock_points: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<(i32, i32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl<V> ChallengeConfig<V> {
    /// The same configuration with another presentation variant (or none).
    pub fn with_variant<W>(self, variant: Option<W>) -> ChallengeConfig<W> {
        ChallengeConfig {
            id: self.id,
            name: self.name,
            description: self.description,
            challenge: self.challenge,
            variant,
            tasks: self.tasks,
            unlock_points: self.unlock_points,
            position: self.position,
            icon: self.icon,
        }
    }

    /// The same configuration with its presentation variant converted.
    pub fn map_variant<W>(self, convert: impl FnOnce(V) -> W) -> ChallengeConfig<W> {
        let converted = self.variant.map(convert);
        ChallengeConfig {
            id: self.id,
            name: self.name,
            description: self.description,
            challenge: self.challenge,
            variant: converted,
            tasks: self.tasks,
            unlock_points: self.unlock_points,
            position: self.position,
            icon: self.icon,
        }
    }
}

impl Default for ChallengeConfig {
    fn default() -> Self {
        ChallengeConfig {
            id: "123".to_string(),
            name: "Konnektoren #1".to_string(),
            description: "Your first Konnektoren challenge!".to_string(),
            challenge: "konnektoren".to_string(),
            variant: None,
            tasks: 10.into(),
            unlock_points: 0,
            position: Some((0, 0)),
            icon: None,
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
        assert_eq!(challenge_config.tasks, 10.into());
        assert_eq!(challenge_config.unlock_points, 0);
        assert_eq!(challenge_config.icon, None);
    }
}
