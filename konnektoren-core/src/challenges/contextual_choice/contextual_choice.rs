use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ContextualChoice {
    pub id: String,
    pub name: String,
    pub description: String,
    pub items: Vec<ContextItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContextItem {
    pub template: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Choice {
    pub id: usize,
    pub options: Vec<String>,
    pub correct_answer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContextItemChoiceAnswers {
    pub ids: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_contextual_choice_deserialization() {
        let yaml = r#"
        id: "konjunktiv-2"
        name: "Konjunktiv II Exercise"
        description: "Fill in the gaps with the correct Konjunktiv II forms."
        items:
          - template: "Wenn ich reich {0}, {1} ich um die Welt reisen."
            choices:
              - id: 0
                options:
                  - "wäre"
                  - "würde sein"
                  - "sei"
                correct_answer: "wäre"
              - id: 1
                options:
                  - "würde"
                  - "werde"
                  - "wurde"
                correct_answer: "würde"
          - template: "Er {0} gerne Deutsch, wenn er mehr Zeit {1}."
            choices:
              - id: 0
                options:
                  - "lernte"
                  - "würde lernen"
                  - "lerne"
                correct_answer: "würde lernen"
              - id: 1
                options:
                  - "hätte"
                  - "habe"
                  - "würde haben"
                correct_answer: "hätte"
        "#;

        let contextual_choice: ContextualChoice = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(contextual_choice.id, "konjunktiv-2");
        assert_eq!(contextual_choice.name, "Konjunktiv II Exercise");
        assert_eq!(
            contextual_choice.description,
            "Fill in the gaps with the correct Konjunktiv II forms."
        );
        assert_eq!(contextual_choice.items.len(), 2);

        let first_item = &contextual_choice.items[0];
        assert_eq!(
            first_item.template,
            "Wenn ich reich {0}, {1} ich um die Welt reisen."
        );
        assert_eq!(first_item.choices.len(), 2);
        assert_eq!(first_item.choices[0].id, 0);
        assert_eq!(
            first_item.choices[0].options,
            vec!["wäre", "würde sein", "sei"]
        );
        assert_eq!(first_item.choices[0].correct_answer, "wäre");

        let second_item = &contextual_choice.items[1];
        assert_eq!(
            second_item.template,
            "Er {0} gerne Deutsch, wenn er mehr Zeit {1}."
        );
        assert_eq!(second_item.choices.len(), 2);
        assert_eq!(second_item.choices[1].id, 1);
        assert_eq!(
            second_item.choices[1].options,
            vec!["hätte", "habe", "würde haben"]
        );
        assert_eq!(second_item.choices[1].correct_answer, "hätte");
    }

    #[test]
    fn test_contextual_choice_file_deserialization() {
        let data = include_str!("../../assets/konjunktiv-2.yml");
        let contextual_choice: ContextualChoice = serde_yaml::from_str(data).unwrap();

        assert_eq!(contextual_choice.id, "konjunktiv-2");
        assert_eq!(contextual_choice.name, "Konjunktiv II Exercise");
        assert_eq!(
            contextual_choice.description,
            "Fill in the gaps with the correct Konjunktiv II forms."
        );
    }
}
