use super::{ChallengeDefinition, task_pattern::TaskPattern};

/// A challenge from either of two independently defined groups.
///
/// Families can be nested: `ChallengeFamily<ChallengeType, ChallengeFamily<A, B>>`
/// includes core's built-in challenges and two external groups.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeFamily<A, B> {
    First(A),
    Second(B),
}

/// A response belonging to the corresponding branch of a challenge family.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeFamilyResult<A, B> {
    First(A),
    Second(B),
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
