#[derive(Debug, Clone, PartialEq, Default)]
pub struct Payment {
    pub method: String,
    pub amount: f64,
}
