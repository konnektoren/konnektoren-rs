use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Ordering {
    pub id: String,
    pub name: String,
    pub description: String,
    pub items: Vec<OrderingItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct OrderingItem {
    pub elements: Vec<String>,
    pub correct_order: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct OrderingResult {
    pub order: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ordering() {
        let id = "123".to_string();
        let name = "Test".to_string();
        let description = "Description".to_string();
        let items = vec![
            OrderingItem {
                elements: vec!["A".to_string(), "B".to_string(), "C".to_string()],
                correct_order: vec![0, 1, 2],
            },
            OrderingItem {
                elements: vec!["D".to_string(), "E".to_string(), "F".to_string()],
                correct_order: vec![2, 1, 0],
            },
        ];
        let ordering = Ordering {
            id,
            name,
            description,
            items,
        };
        assert_eq!(ordering.id, "123");
        assert_eq!(ordering.name, "Test");
        assert_eq!(ordering.description, "Description");
        assert_eq!(ordering.items.len(), 2);
        assert_eq!(ordering.items[0].elements.len(), 3);
        assert_eq!(ordering.items[0].correct_order.len(), 3);
        assert_eq!(ordering.items[1].elements.len(), 3);
        assert_eq!(ordering.items[1].correct_order.len(), 3);
    }

    #[test]
    fn test_ordering_deserialization() {
        let yaml = r#"
        id: "ordering-example"
        name: "Ordering Example"
        description: "Order the elements correctly"
        items:
          - elements:
              - "First element"
              - "Second element"
              - "Third element"
            correct_order: [0, 1, 2]
          - elements:
              - "Apple"
              - "Banana"
              - "Orange"
            correct_order: [2, 0, 1]
        "#;

        let ordering: Ordering = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(ordering.id, "ordering-example");
        assert_eq!(ordering.name, "Ordering Example");
        assert_eq!(ordering.description, "Order the elements correctly");
        assert_eq!(ordering.items.len(), 2);

        let first_item = &ordering.items[0];
        assert_eq!(first_item.elements.len(), 3);
        assert_eq!(first_item.correct_order, vec![0, 1, 2]);

        let second_item = &ordering.items[1];
        assert_eq!(second_item.elements.len(), 3);
        assert_eq!(second_item.correct_order, vec![2, 0, 1]);
    }

    #[test]
    fn test_ordering_result() {
        let result = OrderingResult {
            order: vec![2, 0, 1],
        };
        assert_eq!(result.order, vec![2, 0, 1]);
    }

    #[test]
    fn test_ordering_result_default() {
        let result = OrderingResult::default();
        assert!(result.order.is_empty());
    }
}
