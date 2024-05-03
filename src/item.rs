use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub price: f32,
}