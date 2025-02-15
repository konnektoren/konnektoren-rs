use super::achievement_statistic::*;
use crate::analytics::Metric;
use crate::challenges::performance::Performance;
use crate::game::{Game, GamePath};
use std::collections::HashSet;

pub struct GameStatistics<'a> {
    game: &'a Game,
}

impl<'a> GameStatistics<'a> {
    pub fn new(game: &'a Game) -> Self {
        GameStatistics { game }
    }
}

impl<'a> TotalChallenges for GameStatistics<'a> {
    fn total_challenges(&self) -> u32 {
        self.game.challenge_history.len() as u32
    }
}

impl<'a> AveragePerformance for GameStatistics<'a> {
    fn average_performance(&self) -> f64 {
        if self.game.challenge_history.is_empty() {
            return 0.0;
        }
        let total_performance: u32 = self
            .game
            .challenge_history
            .challenges
            .iter()
            .map(|challenge| challenge.performance(&challenge.challenge_result))
            .sum();
        total_performance as f64 / self.game.challenge_history.len() as f64
    }
}

impl<'a> TotalXp for GameStatistics<'a> {
    fn total_xp(&self) -> u32 {
        self.game
            .challenge_history
            .challenges
            .iter()
            .map(|challenge| self.game.calculate_xp_reward(challenge))
            .sum()
    }
}

impl<'a> CompletedGamePaths for GameStatistics<'a> {
    fn completed_game_paths(&self) -> u32 {
        fn is_game_path_completed(game: &Game, path: &GamePath) -> bool {
            path.challenges.iter().all(|challenge_config| {
                game.challenge_history
                    .challenges
                    .iter()
                    .any(|completed_challenge| {
                        completed_challenge.challenge_config.id == challenge_config.id
                    })
            })
        }

        self.game
            .game_paths
            .iter()
            .filter(|path| is_game_path_completed(self.game, path))
            .count() as u32
    }
}

impl<'a> PerfectChallenges for GameStatistics<'a> {
    fn perfect_challenges(&self) -> u32 {
        self.game
            .challenge_history
            .challenges
            .iter()
            .filter(|challenge| challenge.performance(&challenge.challenge_result) == 100)
            .count() as u32
    }
}

impl<'a> DifferentChallengeTypesCompleted for GameStatistics<'a> {
    fn different_challenge_types_completed(&self) -> u32 {
        let unique_types: HashSet<_> = self
            .game
            .challenge_history
            .challenges
            .iter()
            .map(|challenge| challenge.challenge_type.id())
            .collect();
        unique_types.len() as u32
    }
}

// Implement Metric for GameStatistics
impl<'a> Metric for GameStatistics<'a> {
    fn name(&self) -> &str {
        "game_statistics"
    }

    fn value(&self) -> f64 {
        0.0
    }

    fn description(&self) -> &str {
        "Overall game statistics and metrics"
    }
}

// Implement AchievementStatistic for GameStatistics
impl<'a> AchievementStatistic for GameStatistics<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::sort_table::SortTableColumn;
    use crate::challenges::{
        Challenge, ChallengeConfig, ChallengeResult, ChallengeType, Choice, ContextItem,
        ContextItemChoiceAnswers, ContextualChoice, MultipleChoice, MultipleChoiceOption,
        SortTable, SortTableRow,
    };
    use crate::game::Game;
    use crate::prelude::Question;

    fn create_mock_game(num_challenges: usize, _performance: u32) -> Game {
        let mut game = Game::default();
        for i in 0..num_challenges {
            let (challenge_type, challenge_result) = match i % 3 {
                0 => (
                    ChallengeType::MultipleChoice(MultipleChoice {
                        id: "mc".to_string(),
                        questions: vec![Question::default()],
                        options: vec![MultipleChoiceOption::default()],
                        ..Default::default()
                    }),
                    ChallengeResult::MultipleChoice(vec![MultipleChoiceOption::default()]),
                ),
                1 => (
                    ChallengeType::ContextualChoice(ContextualChoice {
                        id: "cc".to_string(),
                        items: vec![ContextItem {
                            template: "".to_string(),
                            choices: vec![Choice {
                                id: 0,
                                options: vec!["".to_string()],
                                correct_answer: "".to_string(),
                            }],
                        }],
                        ..Default::default()
                    }),
                    ChallengeResult::ContextualChoice(vec![ContextItemChoiceAnswers::default()]),
                ),
                _ => (
                    ChallengeType::SortTable(SortTable {
                        id: "st".to_string(),
                        name: "".to_string(),
                        description: "".to_string(),
                        columns: vec![SortTableColumn {
                            id: "".to_string(),
                            title: "".to_string(),
                            description: "".to_string(),
                        }],
                        rows: vec![SortTableRow {
                            id: 0,
                            values: vec!["".to_string()],
                        }],
                    }),
                    ChallengeResult::SortTable(vec![SortTableRow::default()]),
                ),
            };

            let mut challenge = Challenge::new(
                &challenge_type,
                &ChallengeConfig {
                    id: format!("challenge_{}", i),
                    ..Default::default()
                },
            );
            challenge.challenge_result = challenge_result;
            game.challenge_history.add_challenge(challenge);
        }

        // Set performance for each challenge (this is a simplified version)
        for challenge in &mut game.challenge_history.challenges {
            match &mut challenge.challenge_result {
                ChallengeResult::MultipleChoice(options) => {
                    options[0].id = 0;
                }
                ChallengeResult::ContextualChoice(choices) => {
                    choices[0].ids = vec![0];
                }
                ChallengeResult::SortTable(rows) => {
                    rows[0].values = vec!["".to_string()];
                }
                _ => {}
            }
        }

        game
    }

    #[test]
    fn test_total_challenges() {
        let game = create_mock_game(5, 80);
        let stats = GameStatistics::new(&game);
        assert_eq!(stats.total_challenges(), 5);
    }

    #[test]
    fn test_average_performance() {
        let game = create_mock_game(3, 100);
        let stats = GameStatistics::new(&game);
        assert_eq!(stats.average_performance(), 100.0);
    }

    #[test]
    fn test_total_xp() {
        let game = create_mock_game(2, 100);
        let stats = GameStatistics::new(&game);
        assert_eq!(stats.total_xp(), 600);
    }

    #[test]
    fn test_perfect_challenges() {
        let game = create_mock_game(5, 100);
        let stats = GameStatistics::new(&game);
        assert_eq!(stats.perfect_challenges(), 5);
    }

    #[test]
    fn test_different_challenge_types_completed() {
        let game = create_mock_game(3, 80);

        let stats = GameStatistics::new(&game);
        assert_eq!(stats.different_challenge_types_completed(), 3);
    }
}
