use super::super::Trend;
use super::Metric;
use crate::challenges::{ChallengeHistory, Timed};
use chrono::{DateTime, Duration, Utc};

#[derive(Clone, PartialEq, Default, Debug)]
pub struct AverageTimeTakenMetric {
    history: ChallengeHistory,

    reference_time: DateTime<Utc>,
}

impl AverageTimeTakenMetric {
    pub fn new(history: ChallengeHistory) -> Self {
        let reference_time = history
            .challenges
            .iter()
            .filter_map(|c| c.end_time())
            .max()
            .unwrap_or_else(Utc::now);

        AverageTimeTakenMetric {
            history,
            reference_time,
        }
    }

    // Helper function to calculate the average time.
    fn calculate_average_time(&self) -> Option<f64> {
        let completed_challenges: Vec<_> = self
            .history
            .challenges
            .iter()
            .filter(|c| c.end_time().is_some() && c.start_time().is_some())
            .collect();

        if completed_challenges.is_empty() {
            return None;
        }

        let total_duration: Duration = completed_challenges
            .iter()
            .filter_map(|challenge| challenge.elapsed_time())
            .sum();

        let average_duration = total_duration / (completed_challenges.len() as i32);
        Some(average_duration.num_seconds() as f64)
    }

    pub fn get_trend(&self, time_window: Duration) -> Trend {
        let window_start = self.reference_time - time_window;

        // Split challenges into recent and older, sorting by start time
        let mut challenges: Vec<_> = self.history.challenges.iter().collect();
        challenges.sort_by_key(|c| c.start_time());

        let (older_challenges, recent_challenges): (Vec<_>, Vec<_>) = challenges
            .into_iter()
            .partition(|c| c.start_time().is_none_or(|t| t < window_start));

        // If no recent challenges, return Stable
        if recent_challenges.is_empty() {
            return Trend::Stable;
        }

        // Calculate average time for recent challenges
        let recent_avg_time = if recent_challenges.is_empty() {
            0.0
        } else {
            let recent_total_duration: Duration = recent_challenges
                .iter()
                .filter_map(|c| c.elapsed_time())
                .sum();
            recent_total_duration.num_seconds() as f64 / recent_challenges.len() as f64
        };

        // Calculate average time for older challenges
        let older_avg_time = if older_challenges.is_empty() {
            recent_avg_time // If no older challenges, use recent rate as baseline
        } else {
            let older_total_duration: Duration = older_challenges
                .iter()
                .filter_map(|c| c.elapsed_time())
                .sum();
            older_total_duration.num_seconds() as f64 / older_challenges.len() as f64
        };

        // Calculate trend value as the difference between recent and older success rates
        let trend_value = older_avg_time - recent_avg_time;
        Trend::from_value(trend_value)
    }
}

impl Metric for AverageTimeTakenMetric {
    fn name(&self) -> &str {
        "Average Time Taken (Seconds)"
    }

    fn value(&self) -> f64 {
        self.calculate_average_time().unwrap_or(0.0)
    }

    fn description(&self) -> &str {
        "The average time (in seconds) taken to complete a challenge, across all completed challenges."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::{Challenge, ChallengeConfig, ChallengeHistory, ChallengeType};
    use chrono::{Duration, Utc};

    #[test]
    fn test_average_time_taken_no_challenges() {
        let history = ChallengeHistory::new();
        let metric = AverageTimeTakenMetric::new(history);
        assert_eq!(metric.value(), 0.0);
    }

    #[test]
    fn test_average_time_taken_one_challenge() {
        let mut history = ChallengeHistory::new();
        let mut challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge.start();
        challenge.update_end_time();
        history.add_challenge(challenge);
        let metric = AverageTimeTakenMetric::new(history);
        assert_eq!(metric.value(), 0.0); // Because start and end are set at the same time.
    }

    #[test]
    fn test_average_time_taken_multiple_challenges() {
        let mut history = ChallengeHistory::new();

        // Challenge 1: 10 seconds
        let mut challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge1.start_time = Some(Utc::now() - Duration::seconds(10));
        challenge1.end_time = Some(Utc::now());
        history.add_challenge(challenge1);

        // Challenge 2: 20 seconds
        let mut challenge2 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge2.start_time = Some(Utc::now() - Duration::seconds(20));
        challenge2.end_time = Some(Utc::now());
        history.add_challenge(challenge2);

        let metric = AverageTimeTakenMetric::new(history);
        assert_eq!(metric.value(), 15.0); // (10 + 20) / 2 = 15
    }

    #[test]
    fn test_trend_calculation_improving() {
        let mut history = ChallengeHistory::new();

        // Older challenge: 20 seconds
        let mut challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge1.start_time = Some(Utc::now() - Duration::days(1) - Duration::seconds(20));
        challenge1.end_time = Some(Utc::now() - Duration::days(1));
        history.add_challenge(challenge1);

        // Recent challenge: 10 seconds
        let mut challenge2 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge2.start_time = Some(Utc::now() - Duration::seconds(10));
        challenge2.end_time = Some(Utc::now());
        history.add_challenge(challenge2);

        let metric = AverageTimeTakenMetric::new(history);
        let trend = metric.get_trend(Duration::days(1));
        assert_eq!(trend, Trend::Improving);
    }
}
