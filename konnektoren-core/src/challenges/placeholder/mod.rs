use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Placeholder {
    /// Unique identifier for the placeholder
    pub id: String,
    /// Display name of the placeholder
    pub name: String,
    /// Description of what's coming
    pub description: String,
    /// Type of placeholder
    #[serde(rename = "type")]
    pub type_: PlaceholderType,
    /// Optional image identifier
    pub image: Option<String>,
    /// Estimated time for completion
    pub estimated_time: Option<String>,
    /// Informative text content
    pub text: Vec<InformativeText>,
}

impl Default for Placeholder {
    fn default() -> Self {
        let data = include_str!("../../../assets/placeholder_default.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum PlaceholderType {
    ComingSoon,
    Planned,
    UnderDevelopment,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InformativeText {
    /// Language code
    pub language: String,
    /// The text content
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let dataset = Placeholder::default();
        assert_eq!(dataset.id, "placeholder-adjective-declension");
        assert_eq!(dataset.name, "Coming Soon: Adjective Declension");
        assert_eq!(dataset.type_, PlaceholderType::ComingSoon);
    }
}
