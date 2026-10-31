use super::ChallengeType;
use super::task_pattern::TaskPattern;

/// Content identity and task selection, independent of a learner's result.
pub trait ChallengeDefinition: Sized {
    /// Response type associated with this content; no response is created here.
    type Result;

    fn id(&self) -> &str;
    fn of_tasks(&self, tasks: &TaskPattern) -> Self;
}

impl ChallengeDefinition for ChallengeType {
    type Result = super::ChallengeResult;

    fn id(&self) -> &str {
        self.id()
    }

    fn of_tasks(&self, tasks: &TaskPattern) -> Self {
        self.of_tasks(tasks)
    }
}
