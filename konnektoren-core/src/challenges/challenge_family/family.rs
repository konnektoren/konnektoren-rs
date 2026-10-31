use super::{ChallengeFamilyResult, FamilyResultMismatch};
use crate::challenges::{
    ChallengeDefinition, ChallengeEmbedding, Performance, Playable, task_pattern::TaskPattern,
};

/// A challenge from either of two independently defined groups.
///
/// Families can be nested: `ChallengeFamily<ChallengeType, ChallengeFamily<A, B>>`
/// includes core's built-in challenges and two external groups.
/// Applications choose their own stable serialized challenge type names.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeFamily<A, B> {
    First(A),
    Second(B),
}

impl<A: ChallengeDefinition, B: ChallengeDefinition> ChallengeFamily<A, B> {
    /// Apply a branch-specific operation only when the challenge and result match.
    /// Nested families can call this again for their inner branch.
    pub fn try_with_result<T>(
        &self,
        result: &ChallengeFamilyResult<A::Result, B::Result>,
        first: impl FnOnce(&A, &A::Result) -> T,
        second: impl FnOnce(&B, &B::Result) -> T,
    ) -> Result<T, FamilyResultMismatch> {
        match (self, result) {
            (Self::First(challenge), ChallengeFamilyResult::First(result)) => {
                Ok(first(challenge, result))
            }
            (Self::Second(challenge), ChallengeFamilyResult::Second(result)) => {
                Ok(second(challenge, result))
            }
            _ => Err(FamilyResultMismatch),
        }
    }

    /// Score a matching result without treating a mismatched branch as zero.
    pub fn try_performance(
        &self,
        result: &ChallengeFamilyResult<A::Result, B::Result>,
    ) -> Result<u32, FamilyResultMismatch>
    where
        A: Performance<A::Result>,
        B: Performance<B::Result>,
    {
        self.try_with_result(result, Performance::performance, Performance::performance)
    }
}

impl<A: ChallengeDefinition, B: ChallengeDefinition> ChallengeDefinition for ChallengeFamily<A, B> {
    type Result = ChallengeFamilyResult<A::Result, B::Result>;

    fn id(&self) -> &str {
        match self {
            Self::First(challenge) => challenge.id(),
            Self::Second(challenge) => challenge.id(),
        }
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        match self {
            Self::First(challenge) => Self::First(challenge.of_tasks(tasks)),
            Self::Second(challenge) => Self::Second(challenge.of_tasks(tasks)),
        }
    }
}

impl<A: Playable + Clone + 'static, B: Playable + Clone + 'static> ChallengeFamily<A, B>
where
    A::Result: Clone + 'static,
    A::Input: 'static,
    A::Variant: Clone + 'static,
    B::Result: Clone + 'static,
    B::Input: 'static,
    B::Variant: Clone + 'static,
{
    /// Embeds the first branch's challenges, results, and inputs.
    pub fn first_embedding() -> ChallengeEmbedding<A, Self> {
        ChallengeEmbedding::new(
            Self::First,
            ChallengeFamilyResult::First,
            super::ChallengeFamilyInput::First,
            super::ChallengeFamilyVariant::First,
        )
        .with_extraction(
            |family| match family {
                Self::First(challenge) => Some(challenge.clone()),
                Self::Second(_) => None,
            },
            |result| match result {
                ChallengeFamilyResult::First(result) => Some(result.clone()),
                ChallengeFamilyResult::Second(_) => None,
            },
            |variant| match variant {
                super::ChallengeFamilyVariant::First(variant) => Some(variant.clone()),
                super::ChallengeFamilyVariant::Second(_) => None,
            },
        )
    }

    /// Embeds the second branch's challenges, results, and inputs.
    pub fn second_embedding() -> ChallengeEmbedding<B, Self> {
        ChallengeEmbedding::new(
            Self::Second,
            ChallengeFamilyResult::Second,
            super::ChallengeFamilyInput::Second,
            super::ChallengeFamilyVariant::Second,
        )
        .with_extraction(
            |family| match family {
                Self::Second(challenge) => Some(challenge.clone()),
                Self::First(_) => None,
            },
            |result| match result {
                ChallengeFamilyResult::Second(result) => Some(result.clone()),
                ChallengeFamilyResult::First(_) => None,
            },
            |variant| match variant {
                super::ChallengeFamilyVariant::Second(variant) => Some(variant.clone()),
                super::ChallengeFamilyVariant::First(_) => None,
            },
        )
    }
}
