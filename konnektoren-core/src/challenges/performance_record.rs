use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PerformanceRecord {
    pub game_path_id: String,
    pub user_profile_name: String,
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
        let performance_percentage = ((solved_challenges as f64 / total_challenges as f64) * 100.0) as u8;
        PerformanceRecord {
            game_path_id,
            user_profile_name,
            challenges_performance,
            total_challenges,
            performance_percentage,
            date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;

    #[test]
    fn new_performance_record() {
        let date = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let performance_record = PerformanceRecord::new(
            "game_path_id".to_string(),
            "user_profile_name".to_string(),
            vec![("challenge_id".to_string(), 100)],
            1,
            date,
        );
        assert_eq!(performance_record.game_path_id, "game_path_id");
        assert_eq!(performance_record.user_profile_name, "user_profile_name");
        assert_eq!(performance_record.challenges_performance, vec![("challenge_id".to_string(), 100)]);
        assert_eq!(performance_record.total_challenges, 1);
        assert_eq!(performance_record.performance_percentage, 100);
        assert_eq!(performance_record.date, date);
    }
}
