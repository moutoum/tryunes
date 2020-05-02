use crate::ingredients::Ingredient;

pub struct Recipe {
    name: String,
    components: Vec<Component>,
    steps: Vec<String>,
}

pub struct Component {
    ingredient: Ingredient,
    amount: f64,
}

impl Recipe {
    fn new<S>(name: S, components: Vec<Component>, steps: Vec<String>) -> Recipe where
        S: ToString
    {
        Recipe { name: name.to_string(), components, steps }
    }

    fn get_price(&self) -> f64 {
        self.components.iter().fold(0f64, |price, component| price + component.get_price())
    }
}

impl Component {
    fn get_price(&self) -> f64 {
        self.ingredient.price_per_unit * self.amount
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::units::Unit;

    #[test]
    fn component_price() {
        let component = Component {
            ingredient: Ingredient::new("tomato", 0.30, Unit::None),
            amount: 5.0,
        };

        assert_eq!(component.get_price(), 1.5);
    }

    #[test]
    fn recipe_price() {
        let recipe = Recipe::new(
            "tomates farcies",
            vec![
                Component { ingredient: Ingredient::new("tomate", 0.30, Unit::None), amount: 6.0 },
                Component { ingredient: Ingredient::new("chaire à saucisse", 8.00, Unit::Kilograms), amount: 0.3 },
                Component { ingredient: Ingredient::new("riz", 5.00, Unit::Kilograms), amount: 0.2 },
                Component { ingredient: Ingredient::new("oignon", 0.50, Unit::None), amount: 1.0 },
            ],
            vec![],
        );

        assert_eq!((recipe.get_price() * 100.0).round() / 100.0, 5.70)
    }
}