use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChallengeVariant {
    #[serde(rename = "multiple-choice")]
    #[default]
    MultipleChoice,
    #[serde(rename = "multiple-choice-circle")]
    MultipleChoiceCircle,
    #[serde(rename = "sort-table")]
    SortTable,
    #[serde(rename = "informative-text")]
    InformativeText,
    #[serde(rename = "informative-markdown")]
    InformativeMarkdown,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "custom-package")]
    CustomPackage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_variant() {
        let variant = ChallengeVariant::MultipleChoice;
        assert_eq!(variant, ChallengeVariant::MultipleChoice);
    }

    #[test]
    fn default_challenge_variant() {
        let challenge_variant = ChallengeVariant::default();
        assert_eq!(challenge_variant, ChallengeVariant::MultipleChoice);
    }

    #[test]
    fn default_challenge_variant_with_default_attribute() {
        let challenge_variant = ChallengeVariant::default();
        assert_eq!(challenge_variant, ChallengeVariant::MultipleChoice);
    }
}
