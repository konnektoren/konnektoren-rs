use crate::prelude::Metric;

pub trait AchievementStatistic: Metric {}

pub trait TotalChallenges: AchievementStatistic {
    fn name(&self) -> &str {
        "total_challenges"
    }
    fn description(&self) -> &str {
        "Total number of challenges completed"
    }

    fn total_challenges(&self) -> u32;
    fn value(&self) -> f64 {
        self.total_challenges() as f64
    }
}

pub trait AveragePerformance: AchievementStatistic {
    fn name(&self) -> &str {
        "average_performance"
    }
    fn description(&self) -> &str {
        "Average performance across all challenges"
    }
    fn average_performance(&self) -> f64;
    fn value(&self) -> f64 {
        self.average_performance()
    }
}

pub trait TotalXp: AchievementStatistic {
    fn name(&self) -> &str {
        "total_xp"
    }
    fn description(&self) -> &str {
        "Total experience points earned"
    }
    fn total_xp(&self) -> u32;
    fn value(&self) -> f64 {
        self.total_xp() as f64
    }
}

pub trait CompletedGamePaths: AchievementStatistic {
    fn name(&self) -> &str {
        "completed_game_paths"
    }
    fn description(&self) -> &str {
        "Number of game paths completed"
    }
    fn completed_game_paths(&self) -> u32;
    fn value(&self) -> f64 {
        self.completed_game_paths() as f64
    }
}

pub trait PerfectChallenges: AchievementStatistic {
    fn name(&self) -> &str {
        "perfect_challenges"
    }
    fn description(&self) -> &str {
        "Number of perfect challenges completed"
    }
    fn perfect_challenges(&self) -> u32;
    fn value(&self) -> f64 {
        self.perfect_challenges() as f64
    }
}

pub trait DifferentChallengeTypesCompleted: AchievementStatistic {
    fn name(&self) -> &str {
        "different_challenge_types_completed"
    }
    fn description(&self) -> &str {
        "Number of different challenge types completed"
    }
    fn different_challenge_types_completed(&self) -> u32;
    fn value(&self) -> f64 {
        self.different_challenge_types_completed() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Metric;

    struct DummyStats;

    impl Metric for DummyStats {
        fn name(&self) -> &str {
            "dummy"
        }
        fn value(&self) -> f64 {
            0.0
        }
        fn description(&self) -> &str {
            "dummy metric"
        }
    }

    impl AchievementStatistic for DummyStats {}

    impl TotalChallenges for DummyStats {
        fn total_challenges(&self) -> u32 {
            42
        }
    }
    impl AveragePerformance for DummyStats {
        fn average_performance(&self) -> f64 {
            88.5
        }
    }
    impl TotalXp for DummyStats {
        fn total_xp(&self) -> u32 {
            1234
        }
    }
    impl CompletedGamePaths for DummyStats {
        fn completed_game_paths(&self) -> u32 {
            3
        }
    }
    impl PerfectChallenges for DummyStats {
        fn perfect_challenges(&self) -> u32 {
            7
        }
    }
    impl DifferentChallengeTypesCompleted for DummyStats {
        fn different_challenge_types_completed(&self) -> u32 {
            5
        }
    }

    #[test]
    fn test_total_challenges_trait() {
        let stats = DummyStats;
        assert_eq!(TotalChallenges::name(&stats), "total_challenges");
        assert_eq!(
            TotalChallenges::description(&stats),
            "Total number of challenges completed"
        );
        assert_eq!(stats.total_challenges(), 42);
        assert_eq!(TotalChallenges::value(&stats), 42.0);
    }

    #[test]
    fn test_average_performance_trait() {
        let stats = DummyStats;
        assert_eq!(AveragePerformance::name(&stats), "average_performance");
        assert_eq!(
            AveragePerformance::description(&stats),
            "Average performance across all challenges"
        );
        assert_eq!(stats.average_performance(), 88.5);
        assert_eq!(AveragePerformance::value(&stats), 88.5);
    }

    #[test]
    fn test_total_xp_trait() {
        let stats = DummyStats;
        assert_eq!(TotalXp::name(&stats), "total_xp");
        assert_eq!(
            TotalXp::description(&stats),
            "Total experience points earned"
        );
        assert_eq!(stats.total_xp(), 1234);
        assert_eq!(TotalXp::value(&stats), 1234.0);
    }

    #[test]
    fn test_completed_game_paths_trait() {
        let stats = DummyStats;
        assert_eq!(CompletedGamePaths::name(&stats), "completed_game_paths");
        assert_eq!(
            CompletedGamePaths::description(&stats),
            "Number of game paths completed"
        );
        assert_eq!(stats.completed_game_paths(), 3);
        assert_eq!(CompletedGamePaths::value(&stats), 3.0);
    }

    #[test]
    fn test_perfect_challenges_trait() {
        let stats = DummyStats;
        assert_eq!(PerfectChallenges::name(&stats), "perfect_challenges");
        assert_eq!(
            PerfectChallenges::description(&stats),
            "Number of perfect challenges completed"
        );
        assert_eq!(stats.perfect_challenges(), 7);
        assert_eq!(PerfectChallenges::value(&stats), 7.0);
    }

    #[test]
    fn test_different_challenge_types_completed_trait() {
        let stats = DummyStats;
        assert_eq!(
            DifferentChallengeTypesCompleted::name(&stats),
            "different_challenge_types_completed"
        );
        assert_eq!(
            DifferentChallengeTypesCompleted::description(&stats),
            "Number of different challenge types completed"
        );
        assert_eq!(stats.different_challenge_types_completed(), 5);
        assert_eq!(DifferentChallengeTypesCompleted::value(&stats), 5.0);
    }
}
