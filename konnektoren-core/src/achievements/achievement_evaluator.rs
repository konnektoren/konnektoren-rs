use super::achievement_definition::{AchievementDefinition, AchievementDefinitions};
use super::achievement_statistic::*;
use super::game_statistics::GameStatistics;
use crate::challenges::PerformanceScale;
use crate::game::Game;
use eval::{eval, to_value as eval_to_value};

pub struct AchievementEvaluator {
    definitions: Vec<AchievementDefinition>,
    scale: PerformanceScale,
}
impl AchievementEvaluator {
    /// Conditions may use `total_challenges`, `average_performance`, `total_xp`,
    /// `completed_game_paths`, `perfect_challenges`, `passed_challenges`,
    /// `mastered_challenges` and `different_challenge_types_completed`.
    pub fn new(yaml_content: &str) -> Result<Self, serde_yaml::Error> {
        let definitions: AchievementDefinitions = serde_yaml::from_str(yaml_content)?;
        Ok(AchievementEvaluator {
            definitions: definitions.achievements,
            scale: PerformanceScale::DEFAULT,
        })
    }

    /// Judge `passed_challenges` and `mastered_challenges` on the app's `scale`.
    pub fn with_scale(mut self, scale: PerformanceScale) -> Self {
        self.scale = scale;
        self
    }

    pub fn evaluate(&self, game: &Game) -> Vec<&AchievementDefinition> {
        let statistics = GameStatistics::new(game).with_scale(self.scale);
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
                "passed_challenges",
                &statistics.passed_challenges().to_string(),
            )
            .replace(
                "mastered_challenges",
                &statistics.mastered_challenges().to_string(),
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
    fn passed_and_mastered_conditions_follow_the_scale() {
        use crate::challenges::{
            Challenge, ChallengeConfig, ChallengeResult, ChallengeType, MultipleChoice,
            MultipleChoiceOption,
        };
        use crate::prelude::Question;
        const YAML: &str = r#"
        achievements:
          - id: passed
            name: Passed
            description: Pass one challenge
            icon: ✅
            condition: "passed_challenges >= 1"
          - id: mastered
            name: Mastered
            description: Master one challenge
            icon: ⭐
            condition: "mastered_challenges >= 1 & total_challenges >= 1"
        "#;
        let challenge_type = ChallengeType::MultipleChoice(MultipleChoice {
            id: "mc".into(),
            questions: vec![Question::default()],
            options: vec![MultipleChoiceOption::default()],
            ..Default::default()
        });
        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());
        challenge.challenge_result = Some(ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption::default(),
        ]));
        let mut game = Game::default();
        game.challenge_history.add_challenge(challenge);

        let ids = |evaluator: &AchievementEvaluator| -> Vec<String> {
            evaluator
                .evaluate(&game)
                .iter()
                .map(|a| a.id.clone())
                .collect()
        };
        let evaluator = AchievementEvaluator::new(YAML).unwrap();
        assert_eq!(ids(&evaluator), ["passed", "mastered"]);
        let strict = AchievementEvaluator::new(YAML)
            .unwrap()
            .with_scale(PerformanceScale::DEFAULT.with_mastered(101));
        assert_eq!(ids(&strict), ["passed"]);
    }

    #[test]
    fn test_achievement_evaluator() {
        let evaluator = AchievementEvaluator::new(TEST_YAML).unwrap();

        let mut game = Game::default();
        game.challenge_history = ChallengeHistory { challenges: vec![] };

        let achieved = evaluator.evaluate(&game);

        assert_eq!(achieved.len(), 0);
    }
}
