use super::achievement_definition::{AchievementDefinition, AchievementDefinitions};
use super::achievement_statistic::*;
use super::game_statistics::GameStatistics;
use crate::game::Game;
use eval::{eval, to_value as eval_to_value};

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
        let expression = self.prepare_expression(condition, statistics);
        let true_value = eval_to_value(true);
        match eval(&expression) {
            Ok(value) => value == true_value,
            Err(_) => false,
        }
    }

    fn prepare_expression(&self, condition: &str, statistics: &GameStatistics) -> String {
        condition
            .replace(
                "total_challenges",
                &statistics.total_challenges().to_string(),
            )
            .replace(
                "average_performance",
                &statistics.average_performance().to_string(),
            )
            .replace("total_xp", &statistics.total_xp().to_string())
            .replace(
                "completed_game_paths",
                &statistics.completed_game_paths().to_string(),
            )
            .replace(
                "perfect_challenges",
                &statistics.perfect_challenges().to_string(),
            )
            .replace(
                "different_challenge_types_completed",
                &statistics.different_challenge_types_completed().to_string(),
            )
            .replace("&", "&&")
            .replace("|", "||")
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
      - id: path_finder
        name: Path Finder
        description: Complete 3 game paths
        icon: 🧭
        condition: "completed_game_paths >= 3 && perfect_challenges >= 10"
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
