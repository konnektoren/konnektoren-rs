use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(
    Default, Debug, Serialize, Deserialize, Clone, PartialEq, EnumIter, IntoStaticStr, Eq, Hash,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ChallengeVariant {
    #[default]
    MultipleChoice,
    ContextualChoice,
    MultipleChoiceCircle,
    MultipleChoice4,
    SortTable,
    InformativeText,
    InformativeMarkdown,
    Custom,
    CustomPackage,
}

impl fmt::Display for ChallengeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChallengeVariant::MultipleChoice => "Multiple Choice",
            ChallengeVariant::ContextualChoice => "Contextual Choice",
            ChallengeVariant::MultipleChoiceCircle => "Multiple Choice Circle",
            ChallengeVariant::MultipleChoice4 => "Multiple Choice (4 Options)",
            ChallengeVariant::SortTable => "Sort Table",
            ChallengeVariant::InformativeText => "Informative Text",
            ChallengeVariant::InformativeMarkdown => "Informative Markdown",
            ChallengeVariant::Custom => "Custom",
            ChallengeVariant::CustomPackage => "Custom Package",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_serde_serialization() {
        let variant = ChallengeVariant::MultipleChoice;
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, "\"multiple-choice\"");

        let variant = ChallengeVariant::ContextualChoice;
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, "\"contextual-choice\"");

        let variant = ChallengeVariant::MultipleChoice4;
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, "\"multiple-choice4\"");
    }

    #[test]
    fn test_serde_deserialization() {
        let variant: ChallengeVariant = serde_json::from_str("\"multiple-choice\"").unwrap();
        assert_eq!(variant, ChallengeVariant::MultipleChoice);

        let variant: ChallengeVariant = serde_json::from_str("\"contextual-choice\"").unwrap();
        assert_eq!(variant, ChallengeVariant::ContextualChoice);

        let variant: ChallengeVariant = serde_json::from_str("\"multiple-choice4\"").unwrap();
        assert_eq!(variant, ChallengeVariant::MultipleChoice4);
    }

    #[test]
    fn test_strum_into_static_str() {
        assert_eq!(
            <ChallengeVariant as Into<&'static str>>::into(ChallengeVariant::MultipleChoice),
            "multiple-choice"
        );
        assert_eq!(
            <ChallengeVariant as Into<&'static str>>::into(ChallengeVariant::ContextualChoice),
            "contextual-choice"
        );
        assert_eq!(
            <ChallengeVariant as Into<&'static str>>::into(ChallengeVariant::MultipleChoice4),
            "multiple-choice4"
        );
    }

    #[test]
    fn test_variant_iteration() {
        let variants: Vec<ChallengeVariant> = ChallengeVariant::iter().collect();
        assert_eq!(variants.len(), 9);
        assert!(variants.contains(&ChallengeVariant::MultipleChoice));
        assert!(variants.contains(&ChallengeVariant::MultipleChoice4));
        assert!(variants.contains(&ChallengeVariant::CustomPackage));
    }

    #[test]
    fn test_variant_display() {
        assert_eq!(
            ChallengeVariant::MultipleChoice.to_string(),
            "Multiple Choice"
        );
        assert_eq!(
            ChallengeVariant::ContextualChoice.to_string(),
            "Contextual Choice"
        );
        assert_eq!(
            ChallengeVariant::MultipleChoice4.to_string(),
            "Multiple Choice (4 Options)"
        );
    }

    #[test]
    fn test_default_variant() {
        assert_eq!(
            ChallengeVariant::default(),
            ChallengeVariant::MultipleChoice
        );
    }
}
