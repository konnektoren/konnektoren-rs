use std::collections::HashMap;
use crate::challenges::{ChallengeHistory, Performance};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerformanceRecord {
    pub game_path_id: String,
    pub profile_name: String,
    pub challenges_performance: Vec<(String, u8)>,
    pub total_challenges: usize,
    pub performance_percentage: u8,
    pub date: DateTime<Utc>,
}

impl PerformanceRecord {
    pub fn new(
        game_path_id: String,
        user_profile_name: String,
        challenges_performance: Vec<(String, u8)>,
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
        let mut best_performance: HashMap<String, u8> = HashMap::new();

        for challenge in challenge_history.challenges {
            let id = challenge.challenge_config.id.clone();
            let performance = challenge.performance(&challenge.challenge_result) as u8;
            best_performance
                .entry(id)
                .and_modify(|e| {
                    if performance > *e {
                        *e = performance;
                    }
                })
                .or_insert(performance);
        }

        let mut challenges_performance: Vec<(String, u8)> = best_performance.into_iter().collect();

        challenges_performance.sort_by(|a, b| b.1.cmp(&a.1));

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
            vec![("challenge_id".to_string(), 100)],
            1,
            date,
        );
        assert_eq!(performance_record.game_path_id, "game_path_id");
        assert_eq!(performance_record.profile_name, "profile_name");
        assert_eq!(
            performance_record.challenges_performance,
            vec![("challenge_id".to_string(), 100)]
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
        let challenge1 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
        let challenge2 = Challenge::new(&ChallengeType::default(), &ChallengeConfig::default());
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
}
