use crate::challenges::{ChallengeHistory, Performance, Timed};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ChallengeTimeMilliseconds = u64;
pub type ChallengeId = String;
pub type ChallengePercentage = u8;
pub type ChallengePerformance = (ChallengeId, ChallengePercentage, ChallengeTimeMilliseconds);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub struct PerformanceRecord {
    pub game_path_id: String,
    pub profile_name: String,
    pub challenges_performance: Vec<ChallengePerformance>,
    pub total_challenges: usize,
    pub performance_percentage: u8,
    pub date: DateTime<Utc>,
}

impl PerformanceRecord {
    pub fn new(
        game_path_id: String,
        user_profile_name: String,
        challenges_performance: Vec<ChallengePerformance>,
        total_challenges: usize,
        date: DateTime<Utc>,
    ) -> Self {
        let solved_challenges = challenges_performance.len();
        let performance_percentage =
            ((solved_challenges as f64 / total_challenges as f64) * 100.0) as u8;
        PerformanceRecord {
            game_path_id,
            profile_name: user_profile_name,
            challenges_performance,
            total_challenges,
            performance_percentage,
            date,
        }
    }

    pub fn new_from_history(
        game_path_id: String,
        user_profile_name: String,
        total_challenges: usize,
        challenge_history: ChallengeHistory,
    ) -> Self {
        let mut best_performance: HashMap<ChallengeId, ChallengePerformance> = HashMap::new();

        for challenge in challenge_history.challenges {
            let id = challenge.challenge_config.id.clone();
            let performance = challenge.performance(&challenge.challenge_result) as u8;
            let elapsed_time = challenge
                .elapsed_time()
                .map(|d| d.num_milliseconds() as ChallengeTimeMilliseconds)
                .unwrap_or(0);

            let current_performance = best_performance.get(&id).map_or(0, |(_, p, _)| *p);

            best_performance
                .entry(id.clone())
                .and_modify(|(_id, p, t)| {
                    if performance > *p {
                        *p = performance;
                        *t = elapsed_time;
                    } else if elapsed_time < *t && performance == current_performance {
                        *t = elapsed_time;
                    }
                })
                .or_insert((id, performance, elapsed_time));
        }

        let mut challenges_performance: Vec<ChallengePerformance> =
            best_performance.values().cloned().collect();

        // Sort by performance in descending order (highest performance first)
        challenges_performance.sort_by(|a, b| b.1.cmp(&a.1));

        // Sort by elapsed time in ascending order (lowest time first)
        challenges_performance.sort_by(|a, b| a.2.cmp(&b.2));

        let solved_challenges = challenges_performance.len();
        let performance_percentage =
            ((solved_challenges as f64 / total_challenges as f64) * 100.0) as u8;

        PerformanceRecord {
            game_path_id,
            profile_name: user_profile_name,
            challenges_performance,
            total_challenges,
            performance_percentage,
            date: Utc::now(),
        }
    }
}

impl Timed for PerformanceRecord {
    fn start(&mut self) {}
    fn update_end_time(&mut self) {}
    fn elapsed_time(&self) -> Option<Duration> {
        let mut elapsed_time = Duration::zero();
        for (_, _, time) in &self.challenges_performance {
            elapsed_time = elapsed_time + Duration::milliseconds(*time as i64);
        }
        Some(elapsed_time)
    }
    fn start_time(&self) -> Option<DateTime<Utc>> {
        Some(self.date)
    }
    fn end_time(&self) -> Option<DateTime<Utc>> {
        let end_time = self.date + self.elapsed_time().unwrap_or_default();
        Some(end_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::{Challenge, ChallengeConfig, ChallengeType};
    use chrono::offset::TimeZone;

    #[test]
    fn new_performance_record() {
        let date = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let performance_record = PerformanceRecord::new(
            "game_path_id".to_string(),
            "profile_name".to_string(),
            vec![("challenge_id".to_string(), 100, 1)],
            1,
            date,
        );
        assert_eq!(performance_record.game_path_id, "game_path_id");
        assert_eq!(performance_record.profile_name, "profile_name");
        assert_eq!(
            performance_record.challenges_performance,
            vec![("challenge_id".to_string(), 100, 1)]
        );
        assert_eq!(performance_record.total_challenges, 1);
        assert_eq!(performance_record.performance_percentage, 100);
        assert_eq!(performance_record.date, date);
    }

    #[test]
    fn new_performance_record_from_history() {
        let challenge = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        let mut challenge_history = ChallengeHistory::new();
        challenge_history.add_challenge(challenge);
        let performance_record = PerformanceRecord::new_from_history(
            "game_path_id".to_string(),
            "profile_name".to_string(),
            1,
            challenge_history,
        );
        assert_eq!(performance_record.game_path_id, "game_path_id");
        assert_eq!(performance_record.profile_name, "profile_name");
        assert_eq!(performance_record.total_challenges, 1);
        assert_eq!(performance_record.performance_percentage, 100);
    }

    #[test]
    fn new_performance_record_from_history_multiple_challenges() {
        let timestamp_0 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let timestamp_10 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 10).unwrap();
        let timestamp_20 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 20).unwrap();

        let mut challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        let mut challenge2 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge1.start_time = Some(timestamp_0);
        challenge1.end_time = Some(timestamp_10);
        challenge2.start_time = Some(timestamp_0);
        challenge2.end_time = Some(timestamp_20);

        let mut challenge_history = ChallengeHistory::new();
        challenge_history.add_challenge(challenge1);
        challenge_history.add_challenge(challenge2);
        let performance_record = PerformanceRecord::new_from_history(
            "game_path_id".to_string(),
            "profile_name".to_string(),
            1,
            challenge_history,
        );
        assert_eq!(performance_record.game_path_id, "game_path_id");
        assert_eq!(performance_record.profile_name, "profile_name");
        assert_eq!(performance_record.total_challenges, 1);
        assert_eq!(performance_record.performance_percentage, 100);
    }

    #[test]
    fn new_performance_record_from_history_multiple_challenges_with_different_elapsed_time() {
        let timestamp_0 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let timestamp_10 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 10).unwrap();
        let timestamp_20 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 20).unwrap();

        let mut challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        let mut challenge2 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge1.start_time = Some(timestamp_0);
        challenge1.end_time = Some(timestamp_10);
        challenge2.start_time = Some(timestamp_0);
        challenge2.end_time = Some(timestamp_20);

        let mut challenge_history = ChallengeHistory::new();
        challenge_history.add_challenge(challenge1);
        challenge_history.add_challenge(challenge2);
        let performance_record = PerformanceRecord::new_from_history(
            "game_path_id".to_string(),
            "profile_name".to_string(),
            1,
            challenge_history,
        );
        assert_eq!(performance_record.game_path_id, "game_path_id");
        assert_eq!(performance_record.profile_name, "profile_name");
        assert_eq!(performance_record.total_challenges, 1);
        assert_eq!(performance_record.performance_percentage, 100);
        assert_eq!(performance_record.challenges_performance[0].2, 10 * 1000);
    }

    #[test]
    fn elapsed_time() {
        let timestamp_0 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let timestamp_10 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 10).unwrap();
        let timestamp_20 = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 20).unwrap();

        let mut challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        let mut challenge2 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        challenge1.start_time = Some(timestamp_0);
        challenge1.end_time = Some(timestamp_10);
        challenge2.start_time = Some(timestamp_0);
        challenge2.end_time = Some(timestamp_20);

        let mut challenge_history = ChallengeHistory::new();
        challenge_history.add_challenge(challenge1);
        challenge_history.add_challenge(challenge2);
        let performance_record = PerformanceRecord::new_from_history(
            "game_path_id".to_string(),
            "profile_name".to_string(),
            1,
            challenge_history,
        );
        assert_eq!(
            performance_record
                .elapsed_time()
                .unwrap()
                .num_milliseconds(),
            10 * 1000
        );
    }
}
