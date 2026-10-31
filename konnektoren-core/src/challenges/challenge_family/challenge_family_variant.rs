use serde::{Deserialize, Serialize};

/// A presentation variant belonging to one branch of a challenge family.
///
/// It is stored without a branch marker, so saved paths and attempts only contain the
/// variant names themselves. When both branches would accept the same name, the first
/// branch wins; choose distinct names across included types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChallengeFamilyVariant<A, B> {
    First(A),
    Second(B),
}
