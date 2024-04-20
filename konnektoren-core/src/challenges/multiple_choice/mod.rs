use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultipleChoice {
    pub name: String,
    pub options: Vec<MultipleChoiceOption>,
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultipleChoiceOption {
    id: usize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    question: String,
    help: String,
    option: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dataset() {
        let name = "Test".to_string();
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
            },
            Question {
                question: "Question 2".to_string(),
                help: "Help 2".to_string(),
                option: 2,
            },
        ];
        let dataset = MultipleChoice {
            name,
            options,
            questions,
        };
        assert_eq!(dataset.name, "Test");
        assert_eq!(dataset.options.len(), 2);
        assert_eq!(dataset.questions.len(), 2);
    }
}
