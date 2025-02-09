use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MultipleChoice {
    pub id: String,
    pub name: String,
    pub lang: String,
    pub options: Vec<MultipleChoiceOption>,
    pub questions: Vec<Question>,
}

impl Default for MultipleChoice {
    fn default() -> Self {
        let data = include_str!("../../assets/multiple_choice_default.yml");
        serde_yaml::from_str(data).unwrap()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MultipleChoiceOption {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Question {
    pub question: String,
    pub help: String,
    pub image: Option<String>,
    pub option: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dataset() {
        let id = "123".to_string();
        let name = "Test".to_string();
        let lang = "de".to_string();
        let options = vec![
            MultipleChoiceOption {
                id: 1,
                name: "Option 1".to_string(),
            },
            MultipleChoiceOption {
                id: 2,
                name: "Option 2".to_string(),
            },
        ];
        let questions = vec![
            Question {
                question: "Question 1".to_string(),
                help: "Help 1".to_string(),
                option: 1,
                image: None,
            },
            Question {
                question: "Question 2".to_string(),
                help: "Help 2".to_string(),
                option: 2,
                image: None,
            },
        ];
        let dataset = MultipleChoice {
            id,
            name,
            lang,
            options,
            questions,
        };
        assert_eq!(dataset.name, "Test");
        assert_eq!(dataset.options.len(), 2);
        assert_eq!(dataset.questions.len(), 2);
    }
}
