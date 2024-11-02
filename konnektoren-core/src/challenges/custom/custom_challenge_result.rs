use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CustomChallengeResult {
    pub id: String,
    pub performance: f64,
    pub data: serde_json::Value,
}
