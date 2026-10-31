/// The names an enum uses for its variants when it is saved.
///
/// Challenge families save their branches without a marker, so loading finds the right
/// branch by these names. Names must therefore differ between the included types; see
/// [`ChallengeFamily::duplicate_names`](super::ChallengeFamily::duplicate_names).
///
/// For an enum that derives `strum::VariantNames` with the same naming as its serde
/// attributes, implement it with `Self::VARIANTS.to_vec()`.
pub trait SerializedNames {
    fn serialized_names() -> Vec<&'static str>;
}

impl SerializedNames for super::ChallengeType {
    fn serialized_names() -> Vec<&'static str> {
        <Self as strum::VariantNames>::VARIANTS.to_vec()
    }
}

impl SerializedNames for super::ChallengeResult {
    fn serialized_names() -> Vec<&'static str> {
        <Self as strum::VariantNames>::VARIANTS.to_vec()
    }
}
