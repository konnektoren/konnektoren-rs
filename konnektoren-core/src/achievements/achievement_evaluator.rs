use super::achievement_definition::{AchievementDefinition, AchievementDefinitions};
use super::achievement_statistic::*;
use super::game_statistics::GameStatistics;
use crate::game::Game;

pub struct AchievementEvaluator {
    definitions: Vec<AchievementDefinition>,
}
impl AchievementEvaluator {
    pub fn new(yaml_content: &str) -> Result<Self, serde_yaml::Error> {
        let definitions: AchievementDefinitions = serde_yaml::from_str(yaml_content)?;
        Ok(AchievementEvaluator {
            definitions: definitions.achievements,
        })
    }

    pub fn evaluate<'a>(&self, game: &'a Game) -> Vec<&AchievementDefinition> {
        let statistics = GameStatistics::new(game);
        self.definitions
            .iter()
            .filter(|def| self.evaluate_condition(&def.condition, &statistics))
            .collect()
    }

    fn evaluate_condition(&self, condition: &str, statistics: &GameStatistics) -> bool {
        let parts: Vec<&str> = condition.split_whitespace().collect();
        if parts.len() != 3 {
            return false;
        }

        let statistic = parts[0];
        let operator = parts[1];
        let value: f64 = parts[2].parse().unwrap_or(0.0);

        let stat_value = match statistic {
            "total_challenges" => statistics.total_challenges() as f64,
            "average_performance" => statistics.average_performance(),
            "total_xp" => statistics.total_xp() as f64,
            "completed_game_paths" => statistics.completed_game_paths() as f64,
            "perfect_challenges" => statistics.perfect_challenges() as f64,
            "different_challenge_types_completed" => {
                statistics.different_challenge_types_completed() as f64
            }
            _ => return false,
        };

        match operator {
            ">" => stat_value > value,
            ">=" => stat_value >= value,
            "<" => stat_value < value,
            "<=" => stat_value <= value,
            "==" => (stat_value - value).abs() < f64::EPSILON,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::ChallengeHistory;
    use crate::game::Game;

    const TEST_YAML: &str = r#"
    achievements:
      - id: xp_master
        name: XP Master
        description: Earn 1000 XP
        icon: 🏆
        condition: "total_xp > 1000"
      - id: challenge_champion
        name: Challenge Champion
        description: Complete 50 challenges
        icon: 🏅
        condition: "total_challenges >= 50"
    "#;

    #[test]
    fn test_achievement_evaluator() {
        let evaluator = AchievementEvaluator::new(TEST_YAML).unwrap();

        let mut game = Game::default();
        game.challenge_history = ChallengeHistory { challenges: vec![] };

        let achieved = evaluator.evaluate(&game);

        assert_eq!(achieved.len(), 0);
    }
}
