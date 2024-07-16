use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SortTable {
    pub id: String,
    pub name: String,
    pub description: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Column {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Row {
    pub id: usize,
    pub values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dataset() {
        let id = "123".to_string();
        let name = "Test".to_string();
        let description = "Test".to_string();
        let columns = vec![
            Column {
                id: "column1".to_string(),
                title: "Column 1".to_string(),
                description: "Description 1".to_string(),
            },
            Column {
                id: "column2".to_string(),
                title: "Column 2".to_string(),
                description: "Description 2".to_string(),
            },
        ];
        let rows = vec![
            Row {
                id: 1,
                values: vec!["Value 1".to_string(), "Value 2".to_string()],
            },
            Row {
                id: 2,
                values: vec!["Value 3".to_string(), "Value 4".to_string()],
            },
        ];
        let dataset = SortTable {
            id,
            name,
            description,
            columns,
            rows,
        };

        assert_eq!(dataset.id, "123");
        assert_eq!(dataset.name, "Test");
        assert_eq!(dataset.description, "Test");
        assert_eq!(dataset.columns.len(), 2);
        assert_eq!(dataset.rows.len(), 2);
    }

    #[test]
    fn from_yaml() {
        let yaml_data = include_str!("../../assets/personal_pronouns.yml");
        let dataset: SortTable = serde_yaml::from_str(yaml_data).unwrap();
        assert_eq!(dataset.id, "personal_pronouns");
        assert_eq!(dataset.name, "Personal Pronouns");
        assert_eq!(dataset.description, "Match the personal pronouns in their correct case (Nominativ, Akkusativ, Dativ)");
        assert_eq!(dataset.columns.len(), 3);
        assert_eq!(dataset.rows.len(), 6);
    }
}
