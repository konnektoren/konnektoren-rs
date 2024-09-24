use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Custom {
    pub id: String,
    pub name: String,
    pub description: String,
    pub html: String,
    pub results_html: Option<String>,
    pub css: String,
    pub js: String,
    pub i18n: Option<String>,
    pub data: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dataset() {
        let id = "123".to_string();
        let name = "Test".to_string();
        let data = serde_json::json!({
            "key": "value"
        });
        let dataset = Custom {
            id: id.clone(),
            name: name.clone(),
            description: "".to_string(),
            html: "".to_string(),
            results_html: None,
            css: "".to_string(),
            js: "".to_string(),
            i18n: None,
            data: data.clone(),
        };

        assert_eq!(dataset.id, id);
        assert_eq!(dataset.name, name);
        assert_eq!(dataset.data, data);
    }

    #[test]
    fn serialize_dataset() {
        let json_str = r#"{"id":"123","name":"Test","description":"","html":"","results_html":null,"css":"","js":"","i18n":null,"data":{"key":"value"}}"#;
        let dataset = Custom {
            id: "123".to_string(),
            name: "Test".to_string(),
            description: "".to_string(),
            html: "".to_string(),
            results_html: None,
            css: "".to_string(),
            js: "".to_string(),
            i18n: None,
            data: serde_json::json!({
                "key": "value"
            }),
        };

        let serialized = serde_json::to_string(&dataset).unwrap();
        assert_eq!(serialized, json_str);
    }
}
