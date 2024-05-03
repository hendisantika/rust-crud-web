use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub price: f32,
}

#[derive(Default, PartialEq)]
pub struct ItemFormData {
    pub name: String,
    pub price: String,
}