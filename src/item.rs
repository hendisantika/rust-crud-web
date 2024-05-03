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

#[derive(Debug, PartialEq)]
pub struct ValidatedItem {
    name: String,
    price: String,
}

#[derive(Debug, PartialEq)]
pub enum ItemValidationErr {
    InvalidName,
    InvalidPrice,
}

impl ItemFormData {
    pub fn validate(form_data: &ItemFormData) -> Result<ValidatedItem, Vec<ItemValidationErr>> {
        let mut errors = vec![];

        let name = ItemFormData::validate_name(String::from(&form_data.name))
            .unwrap_or_else(|e| {
                errors.push(e);
                String::from("")
            });

        let price = ItemFormData::validate_price(String::from(&form_data.price))
            .unwrap_or_else(|e| {
                errors.push(e);
                String::from("")
            });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ValidatedItem { name, price })
    }

    fn validate_name(name: String) -> Result<String, ItemValidationErr> {
        if name.len() > 1 {
            Ok(name)
        } else {
            Err(ItemValidationErr::InvalidName)
        }
    }
}