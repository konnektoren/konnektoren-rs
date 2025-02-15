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
