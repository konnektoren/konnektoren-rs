use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub price: Option<f64>,
    pub image: Option<String>,
    pub tags: Vec<String>,
    pub data_path: Option<String>,
}

impl Product {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: None,
            name,
            description,
            price: Option::from(0.0),
            image: None,
            tags: vec![],
            data_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_product() {
        let product = Product::new("Test".to_string(), "Test".to_string());
        assert_eq!(product.name, "Test");
        assert_eq!(product.description, "Test");
        assert_eq!(product.price, None);
        assert_eq!(product.tags.len(), 0);
    }
}
