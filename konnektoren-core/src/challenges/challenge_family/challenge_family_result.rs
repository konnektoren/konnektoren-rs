/// A response belonging to the corresponding branch of a challenge family.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeFamilyResult<A, B> {
    First(A),
    Second(B),
}
