use serde::{Deserialize, Serialize};

/// A single record stored by the application.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// `0` means "not persisted yet" — the store assigns the real id on save.
    pub id: usize,
    pub name: String,
    pub price: f64,
}

impl Item {
    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    /// Price rendered for display, e.g. `12.50`.
    pub fn formatted_price(&self) -> String {
        format!("{:.2}", self.price)
    }
}

/// The raw, still untrusted values coming from the form inputs.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ItemFormData {
    pub name: String,
    pub price: String,
}

/// Form values that passed every validation rule.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedItem {
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemValidationErr {
    InvalidName,
    InvalidPrice,
}

impl ItemValidationErr {
    pub fn message(&self) -> &'static str {
        match self {
            ItemValidationErr::InvalidName => "Name is required and must be at least 2 characters.",
            ItemValidationErr::InvalidPrice => "Price must be a number greater than or equal to 0.",
        }
    }
}

impl ItemFormData {
    /// Validates every field, collecting *all* errors instead of failing on the first one.
    pub fn validate(&self) -> Result<ValidatedItem, Vec<ItemValidationErr>> {
        let mut errors = Vec::new();

        let name = Self::validate_name(&self.name).unwrap_or_else(|e| {
            errors.push(e);
            String::new()
        });

        let price = Self::validate_price(&self.price).unwrap_or_else(|e| {
            errors.push(e);
            0.0
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(ValidatedItem { name, price })
    }

    fn validate_name(name: &str) -> Result<String, ItemValidationErr> {
        let name = name.trim();

        if name.chars().count() > 1 {
            Ok(name.to_owned())
        } else {
            Err(ItemValidationErr::InvalidName)
        }
    }

    fn validate_price(price: &str) -> Result<f64, ItemValidationErr> {
        match price.trim().parse::<f64>() {
            Ok(price) if price.is_finite() && price >= 0.0 => Ok(price),
            _ => Err(ItemValidationErr::InvalidPrice),
        }
    }
}

impl From<(String, String)> for ItemFormData {
    fn from((name, price): (String, String)) -> Self {
        Self { name, price }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form(name: &str, price: &str) -> ItemFormData {
        (name.to_owned(), price.to_owned()).into()
    }

    #[test]
    fn accepts_a_valid_form() {
        let valid = form("Keyboard", "49.9").validate().unwrap();

        assert_eq!(valid.name, "Keyboard");
        assert_eq!(valid.price, 49.9);
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let valid = form("  Mouse  ", "  10  ").validate().unwrap();

        assert_eq!(valid.name, "Mouse");
        assert_eq!(valid.price, 10.0);
    }

    #[test]
    fn rejects_a_name_that_is_too_short() {
        let errors = form("K", "10").validate().unwrap_err();

        assert_eq!(errors, vec![ItemValidationErr::InvalidName]);
    }

    #[test]
    fn rejects_a_blank_name() {
        let errors = form("   ", "10").validate().unwrap_err();

        assert_eq!(errors, vec![ItemValidationErr::InvalidName]);
    }

    #[test]
    fn rejects_a_price_that_is_not_a_number() {
        let errors = form("Keyboard", "free").validate().unwrap_err();

        assert_eq!(errors, vec![ItemValidationErr::InvalidPrice]);
    }

    #[test]
    fn rejects_a_negative_price() {
        let errors = form("Keyboard", "-1").validate().unwrap_err();

        assert_eq!(errors, vec![ItemValidationErr::InvalidPrice]);
    }

    #[test]
    fn reports_every_broken_field_at_once() {
        let errors = form("", "").validate().unwrap_err();

        assert_eq!(
            errors,
            vec![
                ItemValidationErr::InvalidName,
                ItemValidationErr::InvalidPrice
            ]
        );
    }

    #[test]
    fn formats_the_price_with_two_decimals() {
        let item = Item {
            id: 1,
            name: "Keyboard".to_owned(),
            price: 49.5,
        };

        assert_eq!(item.formatted_price(), "49.50");
        assert!(!item.is_new());
        assert!(Item::default().is_new());
    }
}
