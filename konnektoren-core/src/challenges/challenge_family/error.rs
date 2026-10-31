/// A response was supplied for a different family branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("challenge and result belong to different family branches")]
pub struct FamilyResultMismatch;
