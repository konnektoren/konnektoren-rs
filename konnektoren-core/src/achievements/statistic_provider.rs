use crate::achievements::AchievementStatistic;

pub trait StatisticProvider {
    fn get_statistic(&self, name: &str) -> Option<Box<dyn AchievementStatistic>>;
    fn all_statistics(&self) -> Vec<Box<dyn AchievementStatistic>>;
}

pub struct StatisticProviderImpl {
    statistics: Vec<Box<dyn AchievementStatistic>>,
}
