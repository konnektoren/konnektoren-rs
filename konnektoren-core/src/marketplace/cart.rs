use super::Product;
use serde::{Deserialize, Serialize};

/// A cart that contains products.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Cart {
    pub products: Vec<Product>,
}

impl Cart {
    pub fn new() -> Self {
        Self { products: vec![] }
    }

    pub fn add_product(&mut self, product: Product) {
        if self.products.iter().any(|p| p.id == product.id) {
            return;
        }
        self.products.push(product);
    }

    pub fn remove_product(&mut self, product_id: &str) {
        self.products
            .retain(|product| product.id.as_deref() != Some(product_id));
    }

    pub fn total_price(&self) -> f64 {
        self.products
            .iter()
            .fold(0.0, |acc, product| acc + product.price.unwrap_or(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cart() {
        let cart = Cart::new();
        assert_eq!(cart.products.len(), 0);
    }

    #[test]
    fn test_add_product() {
        let mut cart = Cart::new();
        let product = Product::new("Test".to_string(), "Test".to_string());
        cart.add_product(product.clone());
        assert_eq!(cart.products.len(), 1);
        assert_eq!(cart.products[0], product);
    }

    #[test]
    fn test_remove_product() {
        let mut cart = Cart::new();
        let product = Product::new("Test".to_string(), "Test".to_string());
        cart.add_product(product.clone());
        cart.remove_product(product.id.as_deref().unwrap());
        assert_eq!(cart.products.len(), 0);
    }

    #[test]
    fn test_total_price() {
        let mut cart = Cart::new();
        let product1 = Product {
            id: Some("1".to_string()),
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: Some(10.0),
            image: None,
            tags: vec![],
            path: None,
        };
        let product2 = Product {
            id: Some("2".to_string()),
            name: "Test".to_string(),
            description: "Test".to_string(),
            price: Some(20.0),
            image: None,
            tags: vec![],
            path: None,
        };
        cart.add_product(product1);
        cart.add_product(product2);
        // Product with price 0
        cart.add_product(Product::default());
        assert_eq!(cart.total_price(), 30.0);
    }
}
