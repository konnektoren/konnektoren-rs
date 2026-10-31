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

/// Whether no tag appears twice. Usable in constants, which is how `challenge_family!`
/// rejects duplicate tags while compiling.
pub const fn tags_are_distinct(tags: &[&str]) -> bool {
    let mut first = 0;
    while first < tags.len() {
        let mut second = first + 1;
        while second < tags.len() {
            if same(tags[first].as_bytes(), tags[second].as_bytes()) {
                return false;
            }
            second += 1;
        }
        first += 1;
    }
    true
}

const fn same(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut index = 0;
    while index < a.len() {
        if a[index] != b[index] {
            return false;
        }
        index += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::tags_are_distinct;

    #[test]
    fn repeated_tags_are_found() {
        assert!(tags_are_distinct(&[]));
        assert!(tags_are_distinct(&["melody", "quiz", "melodies"]));
        assert!(!tags_are_distinct(&["melody", "quiz", "melody"]));
        assert!(!tags_are_distinct(&["same", "same"]));
    }
}
