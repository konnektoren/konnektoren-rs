use super::{ChallengeFamily, ChallengeFamilyInput, ChallengeFamilyResult, ChallengeFamilyVariant};
use crate::challenges::error::{ChallengeError, Result};
use crate::challenges::{Attempt, Challenge, Playable};

/// Splits a family attempt into the matching branch attempt, runs `solve` on it, and
/// writes the outcome back. A mismatched branch is an error and leaves `challenge` untouched.
macro_rules! solve_branch {
    ($challenge:expr, $inner:expr, $branch:ident, $solve:expr) => {{
        let stored = match $challenge.challenge_result.take() {
            None => None,
            Some(ChallengeFamilyResult::$branch(result)) => Some(result),
            Some(other) => {
                $challenge.challenge_result = Some(other);
                return Err(ChallengeError::FamilyMismatch);
            }
        };
        let mut attempt = Challenge {
            challenge_type: $inner,
            challenge_config: {
                let variant = match &$challenge.challenge_config.variant {
                    Some(ChallengeFamilyVariant::$branch(variant)) => Some(variant.clone()),
                    _ => None,
                };
                $challenge.challenge_config.clone().with_variant(variant)
            },
            challenge_result: stored,
            start_time: $challenge.start_time,
            end_time: $challenge.end_time,
        };
        let outcome = $solve(&mut attempt);
        $challenge.challenge_result = attempt.challenge_result.map(ChallengeFamilyResult::$branch);
        $challenge.start_time = attempt.start_time;
        $challenge.end_time = attempt.end_time;
        outcome
    }};
}

impl<A: Playable + Clone, B: Playable + Clone> Playable for ChallengeFamily<A, B> {
    type Input = ChallengeFamilyInput<A::Input, B::Input>;
    type Variant = ChallengeFamilyVariant<A::Variant, B::Variant>;

    fn result_len(result: &Self::Result) -> usize {
        match result {
            ChallengeFamilyResult::First(result) => A::result_len(result),
            ChallengeFamilyResult::Second(result) => B::result_len(result),
        }
    }

    fn default_input(&self, task_index: usize) -> Result<Option<Self::Input>> {
        Ok(match self {
            Self::First(challenge) => challenge
                .default_input(task_index)?
                .map(ChallengeFamilyInput::First),
            Self::Second(challenge) => challenge
                .default_input(task_index)?
                .map(ChallengeFamilyInput::Second),
        })
    }

    fn record_input(&self, result: &mut Option<Self::Result>, input: Self::Input) -> Result<()> {
        match (self, input) {
            (Self::First(challenge), ChallengeFamilyInput::First(input)) => {
                let mut inner = match result.take() {
                    None => None,
                    Some(ChallengeFamilyResult::First(inner)) => Some(inner),
                    Some(other) => {
                        *result = Some(other);
                        return Err(ChallengeError::FamilyMismatch);
                    }
                };
                let outcome = challenge.record_input(&mut inner, input);
                *result = inner.map(ChallengeFamilyResult::First);
                outcome
            }
            (Self::Second(challenge), ChallengeFamilyInput::Second(input)) => {
                let mut inner = match result.take() {
                    None => None,
                    Some(ChallengeFamilyResult::Second(inner)) => Some(inner),
                    Some(other) => {
                        *result = Some(other);
                        return Err(ChallengeError::FamilyMismatch);
                    }
                };
                let outcome = challenge.record_input(&mut inner, input);
                *result = inner.map(ChallengeFamilyResult::Second);
                outcome
            }
            _ => Err(ChallengeError::FamilyMismatch),
        }
    }

    fn try_performance(&self, result: &Self::Result) -> Result<u32> {
        match (self, result) {
            (Self::First(challenge), ChallengeFamilyResult::First(result)) => {
                challenge.try_performance(result)
            }
            (Self::Second(challenge), ChallengeFamilyResult::Second(result)) => {
                challenge.try_performance(result)
            }
            _ => Err(ChallengeError::FamilyMismatch),
        }
    }

    fn solve_task(
        challenge: &mut Attempt<Self>,
        input: Self::Input,
        task_index: usize,
    ) -> Result<bool> {
        match (challenge.challenge_type.clone(), input) {
            (Self::First(inner), ChallengeFamilyInput::First(input)) => {
                solve_branch!(challenge, inner, First, |attempt: &mut Attempt<A>| {
                    A::solve_task(attempt, input, task_index)
                })
            }
            (Self::Second(inner), ChallengeFamilyInput::Second(input)) => {
                solve_branch!(challenge, inner, Second, |attempt: &mut Attempt<B>| {
                    B::solve_task(attempt, input, task_index)
                })
            }
            _ => Err(ChallengeError::FamilyMismatch),
        }
    }

    fn option_input(&self, option_index: usize) -> Result<Self::Input> {
        match self {
            Self::First(challenge) => challenge
                .option_input(option_index)
                .map(ChallengeFamilyInput::First),
            Self::Second(challenge) => challenge
                .option_input(option_index)
                .map(ChallengeFamilyInput::Second),
        }
    }
}
