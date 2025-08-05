use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct MultipleChoice {
    /// Unique identifier for the challenge
    pub id: String,
    /// Display name of the challenge
    pub name: String,
    /// Language code (e.g., "en", "de")
    pub lang: String,
    /// Available answer options
    pub options: Vec<MultipleChoiceOption>,
    /// List of questions
    pub questions: Vec<Question>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct MultipleChoiceOption {
    /// Option identifier
    pub id: usize,
    /// Display text for the option
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, JsonSchema)]
pub struct Question {
    /// The question text
    pub question: String,
    /// Help text or context
    pub help: String,
    /// Optional image/icon identifier
    pub image: Option<String>,
    /// ID of the correct option
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
