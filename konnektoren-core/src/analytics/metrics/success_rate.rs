use super::super::Trend;
use super::metric::Metric;
use crate::challenges::ChallengeHistory;
use crate::challenges::Performance;
use chrono::{DateTime, Duration, Utc};

#[derive(Clone, PartialEq, Default)]
pub struct SuccessRateMetric {
    history: ChallengeHistory,
    reference_time: DateTime<Utc>,
}

impl SuccessRateMetric {
    pub fn new(history: ChallengeHistory) -> Self {
        // Use the most recent challenge end time as reference, or current time if no challenges
        let reference_time = history
            .challenges
            .iter()
            .filter_map(|c| c.end_time)
            .max()
            .unwrap_or_else(Utc::now);

        Self {
            history,
            reference_time,
        }
    }

    fn calculate(&self) -> f64 {
        let total_challenges = self.history.len();
        if total_challenges == 0 {
            return 0.0;
        }

        let successful_challenges = self
            .history
            .challenges
            .iter()
            .filter(|challenge| {
                challenge.performance(&challenge.challenge_result) >= 80 // Consider 80% as success
            })
            .count();

        (successful_challenges as f64 / total_challenges as f64) * 100.0
    }

    pub fn get_trend(&self, time_window: Duration) -> Trend {
        let window_start = self.reference_time - time_window;

        // Split challenges into recent and older, sorting by start time
        let mut challenges: Vec<_> = self.history.challenges.iter().collect();
        challenges.sort_by_key(|c| c.start_time);

        let (older_challenges, recent_challenges): (Vec<_>, Vec<_>) = challenges
            .into_iter()
            .partition(|c| c.start_time.is_none_or(|t| t < window_start));

        // If no recent challenges, return Stable
        if recent_challenges.is_empty() {
            return Trend::Stable;
        }

        // Calculate success rate for recent challenges
        let recent_success_count = recent_challenges
            .iter()
            .filter(|c| c.performance(&c.challenge_result) >= 80)
            .count();
        let recent_success_rate = recent_success_count as f64 / recent_challenges.len() as f64;

        // Calculate success rate for older challenges
        let older_success_rate = if older_challenges.is_empty() {
            recent_success_rate // If no older challenges, use recent rate as baseline
        } else {
            let older_success_count = older_challenges
                .iter()
                .filter(|c| c.performance(&c.challenge_result) >= 80)
                .count();
            older_success_count as f64 / older_challenges.len() as f64
        };

        // Calculate trend value as the difference between recent and older success rates
        let trend_value = recent_success_rate - older_success_rate;
        Trend::from_value(trend_value)
    }
}

impl Metric for SuccessRateMetric {
    fn name(&self) -> &str {
        "Success Rate"
    }

    fn value(&self) -> f64 {
        self.calculate()
    }

    fn description(&self) -> &str {
        "The percentage of successful challenges out of all completed challenges."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge_type::tests::{
        create_successful_challenge, create_unsuccessful_challenge, BASE_TIMESTAMP,
    };
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn test_success_rate_calculation() {
        let mut history = ChallengeHistory::new();

        // Add one successful and one unsuccessful challenge
        history.add_challenge(create_successful_challenge());
        history.add_challenge(create_unsuccessful_challenge());

        let metric = SuccessRateMetric::new(history);
        assert_eq!(
            metric.value(),
            50.0,
            "Should have 50% success rate with one successful and one failed challenge"
        );
    }

    #[test]
    fn test_all_successful() {
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_successful_challenge());
        history.add_challenge(create_successful_challenge());

        let metric = SuccessRateMetric::new(history);
        assert_eq!(
            metric.value(),
            100.0,
            "Should have 100% success rate with all successful challenges"
        );
    }

    #[test]
    fn test_all_unsuccessful() {
        let mut history = ChallengeHistory::new();
        history.add_challenge(create_unsuccessful_challenge());
        history.add_challenge(create_unsuccessful_challenge());

        let metric = SuccessRateMetric::new(history);
        assert_eq!(
            metric.value(),
            0.0,
            "Should have 0% success rate with all unsuccessful challenges"
        );
    }

    #[test]
    fn test_trend_calculation() {
        let mut history = ChallengeHistory::new();

        // Add an older unsuccessful challenge (3 days ago)
        let mut old_challenge = create_unsuccessful_challenge();
        old_challenge.start_time = Some(
            Utc.timestamp_opt(BASE_TIMESTAMP - 3 * 24 * 3600, 0)
                .unwrap(),
        );
        old_challenge.end_time = Some(
            Utc.timestamp_opt(BASE_TIMESTAMP - 3 * 24 * 3600 + 3600, 0)
                .unwrap(),
        );
        history.add_challenge(old_challenge);

        // Add recent successful challenges (within last day)
        let mut recent_challenge1 = create_successful_challenge();
        recent_challenge1.start_time =
            Some(Utc.timestamp_opt(BASE_TIMESTAMP - 12 * 3600, 0).unwrap());
        recent_challenge1.end_time =
            Some(Utc.timestamp_opt(BASE_TIMESTAMP - 11 * 3600, 0).unwrap());
        history.add_challenge(recent_challenge1);

        let mut recent_challenge2 = create_successful_challenge();
        recent_challenge2.start_time =
            Some(Utc.timestamp_opt(BASE_TIMESTAMP - 6 * 3600, 0).unwrap());
        recent_challenge2.end_time = Some(Utc.timestamp_opt(BASE_TIMESTAMP - 5 * 3600, 0).unwrap());
        history.add_challenge(recent_challenge2);

        let metric = SuccessRateMetric::new(history);
        let trend = metric.get_trend(Duration::days(2));
        assert_eq!(
            trend,
            Trend::Improving,
            "Trend should be improving with recent successful challenges"
        );
    }

    #[test]
    fn test_declining_trend() {
        let mut history = ChallengeHistory::new();

        // Add older successful challenge (3 days ago)
        let mut old_challenge = create_successful_challenge();
        old_challenge.start_time = Some(
            Utc.timestamp_opt(BASE_TIMESTAMP - 3 * 24 * 3600, 0)
                .unwrap(),
        );
        old_challenge.end_time = Some(
            Utc.timestamp_opt(BASE_TIMESTAMP - 3 * 24 * 3600 + 3600, 0)
                .unwrap(),
        );
        history.add_challenge(old_challenge);

        // Add recent unsuccessful challenges (within last day)
        let mut recent_challenge1 = create_unsuccessful_challenge();
        recent_challenge1.start_time =
            Some(Utc.timestamp_opt(BASE_TIMESTAMP - 12 * 3600, 0).unwrap());
        recent_challenge1.end_time =
            Some(Utc.timestamp_opt(BASE_TIMESTAMP - 11 * 3600, 0).unwrap());
        history.add_challenge(recent_challenge1);

        let mut recent_challenge2 = create_unsuccessful_challenge();
        recent_challenge2.start_time =
            Some(Utc.timestamp_opt(BASE_TIMESTAMP - 6 * 3600, 0).unwrap());
        recent_challenge2.end_time = Some(Utc.timestamp_opt(BASE_TIMESTAMP - 5 * 3600, 0).unwrap());
        history.add_challenge(recent_challenge2);

        let metric = SuccessRateMetric::new(history);
        let trend = metric.get_trend(Duration::days(2));
        assert_eq!(
            trend,
            Trend::Declining,
            "Trend should be declining with recent unsuccessful challenges"
        );
    }

    #[test]
    fn test_stable_trend() {
        let mut history = ChallengeHistory::new();
        let window = Duration::days(2);

        // First period - older challenges
        let mut old_successful = create_successful_challenge();
        let old_time = BASE_TIMESTAMP - 3 * 24 * 3600;
        old_successful.start_time = Some(Utc.timestamp_opt(old_time, 0).unwrap());
        old_successful.end_time = Some(Utc.timestamp_opt(old_time + 3600, 0).unwrap());
        history.add_challenge(old_successful);

        let mut old_unsuccessful = create_unsuccessful_challenge();
        old_unsuccessful.start_time = Some(Utc.timestamp_opt(old_time + 4 * 3600, 0).unwrap());
        old_unsuccessful.end_time = Some(Utc.timestamp_opt(old_time + 5 * 3600, 0).unwrap());
        history.add_challenge(old_unsuccessful);

        // Second period - recent challenges with same pattern
        let mut recent_successful = create_successful_challenge();
        let recent_time = BASE_TIMESTAMP - 12 * 3600;
        recent_successful.start_time = Some(Utc.timestamp_opt(recent_time, 0).unwrap());
        recent_successful.end_time = Some(Utc.timestamp_opt(recent_time + 3600, 0).unwrap());
        history.add_challenge(recent_successful);

        let mut recent_unsuccessful = create_unsuccessful_challenge();
        recent_unsuccessful.start_time =
            Some(Utc.timestamp_opt(recent_time + 4 * 3600, 0).unwrap());
        recent_unsuccessful.end_time = Some(Utc.timestamp_opt(recent_time + 5 * 3600, 0).unwrap());
        history.add_challenge(recent_unsuccessful);

        let metric = SuccessRateMetric::new(history);

        let trend = metric.get_trend(window);
        assert_eq!(
            trend,
            Trend::Stable,
            "Trend should be stable with same success ratio"
        );
    }

    #[test]
    fn test_empty_history() {
        let history = ChallengeHistory::new();
        let metric = SuccessRateMetric::new(history);
        assert_eq!(
            metric.value(),
            0.0,
            "Empty history should have 0% success rate"
        );
        assert_eq!(
            metric.get_trend(Duration::days(7)),
            Trend::Stable,
            "Empty history should have stable trend"
        );
    }
}
