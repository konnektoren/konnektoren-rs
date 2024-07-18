use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Informative {
    pub id: String,
    pub name: String,
    pub description: String,
    pub text: Vec<InformativeText>,
}

impl Default for Informative {
    fn default() -> Self {
        let data = include_str!("../../assets/personal_pronouns_info.yml");
        serde_yaml::from_str(data).unwrap()
    }
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
    fn from_yaml() {
        let yaml_data = include_str!("../../assets/personal_pronouns_info.yml");
        let dataset: Informative = serde_yaml::from_str(yaml_data).unwrap();
        assert_eq!(dataset.id, "personal_pronouns_info");
        assert_eq!(dataset.name, "Personal Pronouns Info");
    }
}
