use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomChallengeState {
    pub current_index: usize,
    pub correct_answers: usize,
    pub total_questions: usize,
    pub is_finished: bool,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Default for CustomChallengeState {
    fn default() -> Self {
        Self {
            current_index: 0,
            correct_answers: 0,
            total_questions: 0,
            is_finished: false,
            start_time: None,
            end_time: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_default_custom_challenge_state() {
        let state = CustomChallengeState::default();

        assert_eq!(state.current_index, 0);
        assert_eq!(state.correct_answers, 0);
        assert_eq!(state.total_questions, 0);
        assert_eq!(state.is_finished, false);
        assert_eq!(state.start_time, None);
        assert_eq!(state.end_time, None);
    }
}
