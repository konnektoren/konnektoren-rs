use super::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProductCatalog {
    pub id: String,
    pub products: Vec<Product>,
}

impl ProductCatalog {
    pub fn new(id: String) -> Self {
        Self {
            id,
            products: vec![],
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_product_catalog() {
        let product_catalog = ProductCatalog::new("Test".to_string());
        assert_eq!(product_catalog.id, "Test");
        assert_eq!(product_catalog.products.len(), 0);
    }

    #[test]
    fn test_add_product() {
        let mut product_catalog = ProductCatalog::new("Test".to_string());
        let product = Product::new("Test".to_string(), "Test".to_string());
        product_catalog.add_product(product.clone());
        assert_eq!(product_catalog.products.len(), 1);
        assert_eq!(product_catalog.products[0].name, product.name);
    }

    #[test]
    fn test_from_yaml() {
        let yaml = r#"
            id: "Test"
            products:
              - name: "Test"
                description: "Test"
                price: 0.0
                tags: []
        "#;
        let product_catalog = ProductCatalog::from_yaml(yaml).unwrap();
        assert_eq!(product_catalog.id, "Test");
        assert_eq!(product_catalog.products.len(), 1);
        assert_eq!(product_catalog.products[0].name, "Test");
    }
}
