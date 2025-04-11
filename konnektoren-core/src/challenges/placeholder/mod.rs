use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Placeholder {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: PlaceholderType,
    pub image: Option<String>,
    pub estimated_time: Option<String>,
    pub text: Vec<InformativeText>,
}

impl Default for Placeholder {
    fn default() -> Self {
        let data = include_str!("../../../assets/placeholder_default.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlaceholderType {
    ComingSoon,
    Planned,
    UnderDevelopment,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InformativeText {
    pub language: String,
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
