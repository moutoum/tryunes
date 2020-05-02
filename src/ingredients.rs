use crate::units::Unit;

#[derive(Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub price_per_unit: f64,
    pub unit: Unit,
}

impl Ingredient {
    pub fn new<S>(name: S, price_per_unit: f64, unit: Unit) -> Ingredient where
        S: ToString
    {
        Ingredient {
            name: name.to_string(),
            price_per_unit,
            unit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ingredient() {
        assert_eq!(Ingredient::new("tomato", 0.30, Unit::None), Ingredient { name: "tomato".to_string(), price_per_unit: 0.30, unit: Unit::None });
    }
}