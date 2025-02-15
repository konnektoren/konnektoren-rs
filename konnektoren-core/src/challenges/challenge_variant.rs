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
    }

    #[test]
    fn test_serde_deserialization() {
        let variant: ChallengeVariant = serde_json::from_str("\"multiple-choice\"").unwrap();
        assert_eq!(variant, ChallengeVariant::MultipleChoice);

        let variant: ChallengeVariant = serde_json::from_str("\"contextual-choice\"").unwrap();
        assert_eq!(variant, ChallengeVariant::ContextualChoice);
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
    }

    #[test]
    fn test_variant_iteration() {
        let variants: Vec<ChallengeVariant> = ChallengeVariant::iter().collect();
        assert_eq!(variants.len(), 8);
        assert!(variants.contains(&ChallengeVariant::MultipleChoice));
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
    }

    #[test]
    fn test_default_variant() {
        assert_eq!(
            ChallengeVariant::default(),
            ChallengeVariant::MultipleChoice
        );
    }
}
