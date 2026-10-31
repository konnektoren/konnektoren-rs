/// An answer belonging to the corresponding branch of a challenge family.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeFamilyInput<A, B> {
    First(A),
    Second(B),
}
