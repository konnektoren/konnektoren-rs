use crate::challenges::challenge_config::ChallengeConfig;
use crate::challenges::challenge_result::ChallengeResult;
use crate::challenges::challenge_type::ChallengeType;
use crate::challenges::Timed;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use super::{Performance, Solvable};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Challenge {
    pub challenge_type: ChallengeType,
    pub challenge_config: ChallengeConfig,
    pub challenge_result: ChallengeResult,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Challenge {
    pub fn new(challenge_type: &ChallengeType, challenge_config: &ChallengeConfig) -> Self {
        Challenge {
            challenge_type: challenge_type.clone(),
            challenge_config: challenge_config.clone(),
            challenge_result: ChallengeResult::default(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn get_id(&self) -> String {
        self.challenge_config.id.clone()
    }

    pub fn solved(&self) -> bool {
        self.challenge_result.len() > 0
    }
}

impl Solvable for Challenge {
    fn solve(&mut self, input: super::ChallengeInput) -> anyhow::Result<bool> {
        self.update_end_time();

        match self.challenge_result.add_input(input.clone()) {
            Ok(_) => {
                let index = self.challenge_result.len() - 1; // Adjust index to be 0-based
                match (&self.challenge_type, &self.challenge_result) {
                    (
                        ChallengeType::MultipleChoice(mc),
                        ChallengeResult::MultipleChoice(results),
                    ) => {
                        if let (Some(question), Some(result)) =
                            (mc.questions.get(index), results.get(index))
                        {
                            Ok(question.option == result.id)
                        } else {
                            Ok(false)
                        }
                    }
                    (
                        ChallengeType::ContextualChoice(cc),
                        ChallengeResult::ContextualChoice(results),
                    ) => {
                        if let (Some(item), Some(choice)) =
                            (cc.items.get(index), results.get(index))
                        {
                            Ok(item.choices.iter().zip(&choice.ids).all(|(c, &id)| {
                                c.options
                                    .get(id)
                                    .map_or(false, |selected| *selected == c.correct_answer)
                            }))
                        } else {
                            Ok(false)
                        }
                    }
                    (ChallengeType::GapFill(gf), ChallengeResult::GapFill(results)) => {
                        if let (Some(question), Some(answer)) =
                            (gf.questions.get(index), results.get(index))
                        {
                            if question.gaps.len() != answer.answers.len() {
                                return Ok(false);
                            }

                            Ok(question
                                .gaps
                                .iter()
                                .zip(answer.answers.iter())
                                .all(|(gap, ans)| gap.correct == *ans))
                        } else {
                            Ok(false)
                        }
                    }
                    (ChallengeType::SortTable(st), ChallengeResult::SortTable(results)) => {
                        if let (Some(row), Some(result)) = (st.rows.get(index), results.get(index))
                        {
                            Ok(row.values == result.values)
                        } else {
                            Ok(false)
                        }
                    }
                    (ChallengeType::Informative(_), ChallengeResult::Informative) => Ok(true),
                    (ChallengeType::Custom(_), ChallengeResult::Custom(_)) => {
                        // Custom challenges might need special handling
                        Ok(true)
                    }
                    _ => Ok(false), // Mismatched challenge type and result type
                }
            }
            Err(_) => Ok(false),
        }
    }
}

impl Performance for Challenge {
    fn performance(&self, result: &ChallengeResult) -> u32 {
        self.challenge_type.performance(result)
    }
}

impl Timed for Challenge {
    fn start(&mut self) {
        self.start_time = Some(Utc::now());
    }

    fn update_end_time(&mut self) {
        self.end_time = Some(Utc::now());
    }

    fn elapsed_time(&self) -> Option<Duration> {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            Some(end - start)
        } else {
            None
        }
    }

    fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time
    }

    fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge_input::ChallengeInput;
    use crate::challenges::MultipleChoiceOption;

    #[test]
    fn new_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let challenge = Challenge::new(&challenge_type, &challenge_config);
        assert_eq!(challenge.challenge_type, challenge_type);
        assert_eq!(challenge.challenge_config, challenge_config);
        assert_eq!(challenge.challenge_result, ChallengeResult::default());
    }

    #[test]
    fn solve_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn performance_with_timer() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        challenge.start();
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input).unwrap();
        assert!(result);
        let performance = challenge.performance(&challenge.challenge_result);
        let time_difference = challenge.end_time.unwrap() - challenge.start_time.unwrap();
        assert!(performance >= time_difference.num_seconds() as u32);
    }

    #[test]
    fn elapsed_time() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        challenge.start();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input).unwrap();
        assert!(result);
        let elapsed_time = challenge.elapsed_time().unwrap();
        assert!(elapsed_time > Duration::zero());
    }

    #[test]
    fn start_and_end_time() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        challenge.start();
        let start_time = challenge.start_time().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input).unwrap();
        assert!(result);
        let end_time = challenge.end_time().unwrap();
        assert!(end_time > start_time);
    }
}
