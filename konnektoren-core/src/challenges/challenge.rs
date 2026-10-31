use crate::challenges::Timed;
use crate::challenges::error::{ChallengeError, Result};
use crate::challenges::{ChallengeConfig, ChallengeInput, ChallengeResult, ChallengeType};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use super::{Performance, Solvable};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Challenge<C = ChallengeType, R = ChallengeResult, Config = ChallengeConfig> {
    pub challenge_type: C,
    pub challenge_config: Config,
    pub challenge_result: Option<R>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl<C: Default, R, Config: Default> Default for Challenge<C, R, Config> {
    fn default() -> Self {
        Self {
            challenge_type: C::default(),
            challenge_config: Config::default(),
            challenge_result: None,
            start_time: None,
            end_time: None,
        }
    }
}

impl Challenge {
    pub fn new(challenge_type: &ChallengeType, challenge_config: &ChallengeConfig) -> Self {
        Challenge {
            challenge_type: challenge_type.clone(),
            challenge_config: challenge_config.clone(),
            challenge_result: None,
            start_time: None,
            end_time: None,
        }
    }

    pub(crate) fn result_mut(&mut self) -> &mut ChallengeResult {
        self.challenge_result
            .get_or_insert_with(|| self.challenge_type.empty_result())
    }
    pub fn get_id(&self) -> String {
        self.challenge_config.id.clone()
    }

    pub fn solved(&self) -> bool {
        self.challenge_result
            .as_ref()
            .is_some_and(|result| !result.is_empty())
    }
}

impl Solvable for Challenge {
    fn solve(&mut self, input: ChallengeInput, task_index: usize) -> Result<bool> {
        self.update_end_time();

        match self.result_mut().set_input(task_index, input.clone()) {
            Ok(_) => match (
                &self.challenge_type,
                self.challenge_result
                    .as_ref()
                    .ok_or(ChallengeError::InvalidChallengeType)?,
            ) {
                (ChallengeType::MultipleChoice(mc), ChallengeResult::MultipleChoice(results)) => {
                    if let (Some(question), Some(result)) =
                        (mc.questions.get(task_index), results.get(task_index))
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
                        (cc.items.get(task_index), results.get(task_index))
                    {
                        Ok(item.choices.iter().zip(&choice.ids).all(|(c, &id)| {
                            c.options
                                .get(id)
                                .is_some_and(|selected| *selected == c.correct_answer)
                        }))
                    } else {
                        Ok(false)
                    }
                }
                (ChallengeType::GapFill(gf), ChallengeResult::GapFill(results)) => {
                    if let (Some(question), Some(answer)) =
                        (gf.questions.get(task_index), results.get(task_index))
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
                    if let (Some(row), Some(result)) =
                        (st.rows.get(task_index), results.get(task_index))
                    {
                        Ok(row.values == result.values)
                    } else {
                        Ok(false)
                    }
                }
                (ChallengeType::Informative(_), ChallengeResult::Informative) => Ok(true),
                (ChallengeType::Ordering(ord), ChallengeResult::Ordering(results)) => {
                    if let (Some(item), Some(result)) =
                        (ord.items.get(task_index), results.get(task_index))
                    {
                        Ok(item.correct_order == result.order)
                    } else {
                        Ok(false)
                    }
                }
                (ChallengeType::Custom(_), ChallengeResult::Custom(_)) => Ok(true),
                (ChallengeType::Placeholder(_), ChallengeResult::MultipleChoice(_)) => Ok(true),
                (ChallengeType::Vocabulary(_), ChallengeResult::Vocabulary) => Ok(true),
                (ChallengeType::Dialog(dialog), ChallengeResult::Dialog(_)) => {
                    if let ChallengeInput::Dialog(answer) = &input {
                        match dialog.turns.get(answer.turn_index) {
                            Some(turn) => Ok(turn.correct_option == Some(answer.selected_option)),
                            None => Err(ChallengeError::InvalidInput(format!(
                                "turn index {} out of bounds",
                                answer.turn_index
                            ))),
                        }
                    } else {
                        Err(ChallengeError::InvalidInput(
                            "Expected Dialog input".to_string(),
                        ))
                    }
                }
                _ => Err(ChallengeError::InvalidChallengeType),
            },
            Err(_) => Ok(false),
        }
    }
}

impl<C: Performance<R>, R, Config> Performance<R> for Challenge<C, R, Config> {
    fn performance(&self, result: &R) -> u32 {
        self.challenge_type.performance(result)
    }
}

impl<C: Performance<R>, R, Config> Performance<Option<R>> for Challenge<C, R, Config> {
    fn performance(&self, result: &Option<R>) -> u32 {
        result
            .as_ref()
            .map_or(0, |result| self.challenge_type.performance(result))
    }
}

impl<C, R, Config> Timed for Challenge<C, R, Config> {
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
    use crate::challenges::*;

    #[test]
    fn new_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let challenge = Challenge::new(&challenge_type, &challenge_config);
        assert_eq!(challenge.challenge_type, challenge_type);
        assert_eq!(challenge.challenge_config, challenge_config);
        assert!(challenge.challenge_result.is_none());
    }

    #[test]
    fn solve_challenge() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input, 0); // ← Add task_index = 0
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn performance_with_timer() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        challenge.start();
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input, 0).unwrap(); // ← Add task_index = 0
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
        let result = challenge.solve(input, 0).unwrap(); // ← Add task_index = 0
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
        let result = challenge.solve(input, 0).unwrap(); // ← Add task_index = 0
        assert!(result);
        let end_time = challenge.end_time().unwrap();
        assert!(end_time > start_time);
    }

    #[test]
    fn test_get_id_and_solved() {
        let challenge_type = ChallengeType::default();
        let challenge_config = ChallengeConfig::default();
        let mut challenge = Challenge::new(&challenge_type, &challenge_config);
        assert_eq!(challenge.get_id(), challenge_config.id);
        assert!(!challenge.solved());
        challenge.challenge_result = Some(ChallengeResult::MultipleChoice(vec![
            MultipleChoiceOption::default(),
        ]));
        assert!(challenge.solved());
    }

    #[test]
    fn new_challenges_have_no_result() {
        use strum::IntoEnumIterator;
        for content in ChallengeType::iter() {
            let challenge = Challenge::new(&content, &ChallengeConfig::default());
            assert!(challenge.challenge_result.is_none());
            assert_eq!(challenge.performance(&challenge.challenge_result), 0);
            assert!(!challenge.solved());
        }
    }

    #[test]
    fn test_solve_dialog_observer() {
        // Observer: no quiz turns → always correct
        let mut observer = Dialog::default();
        for turn in &mut observer.turns {
            turn.options = None;
            turn.correct_option = None;
        }
        let mut challenge = Challenge::new(
            &ChallengeType::Dialog(observer),
            &ChallengeConfig::default(),
        );
        // Observer dialogs produce no answers; performance is 100 % by convention.
        // Calling solve with a valid DialogAnswer should still return Ok.
        let input = ChallengeInput::Dialog(DialogAnswer {
            turn_index: 0,
            selected_option: 0,
        });
        let result = challenge.solve(input, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_solve_dialog_quiz_correct() {
        // Default dialog: turn 1 is a quiz turn with correct_option = 0
        let dialog_type = ChallengeType::Dialog(Dialog::default());
        let mut challenge = Challenge::new(&dialog_type, &ChallengeConfig::default());
        let input = ChallengeInput::Dialog(DialogAnswer {
            turn_index: 1,
            selected_option: 0, // correct
        });
        let result = challenge.solve(input, 1);
        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "selecting correct_option should return true"
        );
    }

    #[test]
    fn test_solve_dialog_quiz_wrong() {
        // Default dialog: turn 1 correct_option = 0; option 2 is wrong
        let dialog_type = ChallengeType::Dialog(Dialog::default());
        let mut challenge = Challenge::new(&dialog_type, &ChallengeConfig::default());
        let input = ChallengeInput::Dialog(DialogAnswer {
            turn_index: 1,
            selected_option: 2, // wrong
        });
        let result = challenge.solve(input, 1);
        assert!(result.is_ok());
        assert!(
            !result.unwrap(),
            "selecting wrong option should return false"
        );
    }

    #[test]
    fn test_solve_dialog_quiz_out_of_bounds_turn() {
        let dialog_type = ChallengeType::Dialog(Dialog::default());
        let mut challenge = Challenge::new(&dialog_type, &ChallengeConfig::default());
        let input = ChallengeInput::Dialog(DialogAnswer {
            turn_index: 999, // no such turn
            selected_option: 0,
        });
        let result = challenge.solve(input, 0);
        assert!(result.is_err(), "out-of-bounds turn_index must be an error");
    }

    #[test]
    fn test_solve_dialog_accumulates_answers() {
        let dialog_type = ChallengeType::Dialog(Dialog::default());
        let mut challenge = Challenge::new(&dialog_type, &ChallengeConfig::default());

        // Answer turn 1 (quiz turn, correct_option = 0)
        let _ = challenge.solve(
            ChallengeInput::Dialog(DialogAnswer {
                turn_index: 1,
                selected_option: 0,
            }),
            1,
        );
        // Answer turn 3 (quiz turn, correct_option = 1)
        let _ = challenge.solve(
            ChallengeInput::Dialog(DialogAnswer {
                turn_index: 3,
                selected_option: 1,
            }),
            3,
        );

        assert_eq!(
            challenge
                .challenge_result
                .as_ref()
                .map_or(0, ChallengeResult::len),
            2
        );
        assert!(challenge.solved());
    }

    #[test]
    fn test_solve_contextual_choice() {
        let contextual_choice = ContextualChoice {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            items: vec![ContextItem {
                template: "Test {0} {1}".to_string(),
                choices: vec![
                    Choice {
                        id: 0,
                        options: vec!["correct".to_string(), "wrong".to_string()],
                        correct_answer: "correct".to_string(),
                    },
                    Choice {
                        id: 1,
                        options: vec!["right".to_string(), "incorrect".to_string()],
                        correct_answer: "right".to_string(),
                    },
                ],
            }],
        };

        let challenge_type = ChallengeType::ContextualChoice(contextual_choice);
        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());

        assert!(challenge.challenge_result.is_none());

        // Test correct answer
        let correct_input = ChallengeInput::ContextualChoice(ContextItemChoiceAnswers {
            ids: vec![0, 0], // Both correct (index 0 for both choices)
        });

        let result = challenge.solve(correct_input, 0);
        assert!(result.is_ok(), "Solve should not error: {:?}", result);
        assert!(result.unwrap(), "Should be correct");

        // Test incorrect answer
        let mut challenge2 = Challenge::new(&challenge_type, &ChallengeConfig::default());
        let incorrect_input = ChallengeInput::ContextualChoice(ContextItemChoiceAnswers {
            ids: vec![1, 0], // First wrong, second correct
        });

        let result = challenge2.solve(incorrect_input, 0);
        assert!(result.is_ok(), "Solve should not error: {:?}", result);
        assert!(!result.unwrap(), "Should be incorrect");
    }

    #[test]
    fn solve_ordering() {
        let ordering = Ordering {
            id: "ordering-1".to_string(),
            name: "Test".to_string(),
            description: "Description".to_string(),
            items: vec![OrderingItem {
                elements: vec!["A".to_string(), "B".to_string(), "C".to_string()],
                correct_order: vec![0, 1, 2],
            }],
        };
        let challenge_type = ChallengeType::Ordering(ordering);

        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());
        let correct_input = ChallengeInput::Ordering(OrderingResult {
            order: vec![0, 1, 2],
        });
        let result = challenge.solve(correct_input, 0);
        assert!(result.is_ok(), "Solve should not error: {:?}", result);
        assert!(result.unwrap(), "Should be correct");

        let mut challenge2 = Challenge::new(&challenge_type, &ChallengeConfig::default());
        let incorrect_input = ChallengeInput::Ordering(OrderingResult {
            order: vec![2, 1, 0],
        });
        let result = challenge2.solve(incorrect_input, 0);
        assert!(result.is_ok(), "Solve should not error: {:?}", result);
        assert!(!result.unwrap(), "Should be incorrect");
    }

    #[test]
    fn solve_placeholder() {
        let challenge_type = ChallengeType::Placeholder(Placeholder::default());
        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input, 0);
        assert!(result.is_ok(), "Solve should not error: {:?}", result);
        assert!(result.unwrap(), "Placeholder has no wrong answer");
    }

    #[test]
    fn solve_vocabulary() {
        let challenge_type = ChallengeType::Vocabulary(Vocabulary::default());
        let mut challenge = Challenge::new(&challenge_type, &ChallengeConfig::default());
        let input = ChallengeInput::MultipleChoice(MultipleChoiceOption::default());
        let result = challenge.solve(input, 0);
        assert!(result.is_ok(), "Solve should not error: {:?}", result);
        assert!(result.unwrap(), "Vocabulary has no wrong answer");
    }
}
