pub trait AchievementStatistic {
    fn name(&self) -> &str;
    fn value(&self) -> f64;
}

pub trait TotalChallenges: AchievementStatistic {
    fn name(&self) -> &str {
        "total_challenges"
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
    fn average_performance(&self) -> f64;
    fn value(&self) -> f64 {
        self.average_performance()
    }
}

pub trait TotalXp: AchievementStatistic {
    fn name(&self) -> &str {
        "total_xp"
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
    fn completed_game_paths(&self) -> u32;
    fn value(&self) -> f64 {
        self.completed_game_paths() as f64
    }
}

pub trait PerfectChallenges: AchievementStatistic {
    fn name(&self) -> &str {
        "perfect_challenges"
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
    fn different_challenge_types_completed(&self) -> u32;
    fn value(&self) -> f64 {
        self.different_challenge_types_completed() as f64
    }
}
