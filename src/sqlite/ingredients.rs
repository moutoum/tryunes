use std::convert::TryFrom;

use rusqlite::{Connection, NO_PARAMS, Row};

use crate::errors::Result;
use crate::ingredients::Ingredient;

pub struct IngredientManager<'a>(&'a Connection);

impl<'a> IngredientManager<'a> {
    pub fn new(connection: &'a Connection) -> IngredientManager {
        IngredientManager(connection)
    }

    pub fn init(&self) -> Result<()> {
        self.0.execute(
            "CREATE TABLE ingredients (
                id  INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                price_per_unit REAL NOT NULL,
                unit INTEGER NOT NULL
            )",
            NO_PARAMS,
        )?;
        Ok(())
    }

    pub fn insert(&self, ingredient: &Ingredient) -> Result<()> {
        self.0.execute_named(
            "INSERT INTO ingredients (name, price_per_unit, unit) VALUES (:name, :price_per_unit, :unit)",
            named_params! {
                ":name": ingredient.name,
                ":price_per_unit": ingredient.price_per_unit,
                ":unit": ingredient.unit,
            },
        )?;
        Ok(())
    }

    pub fn get<S: AsRef<str>>(&self, name: S) -> Result<Ingredient> {
        let ingredient = self.0.query_row(
            "SELECT name, price_per_unit, unit FROM ingredients WHERE name = ?1",
            params![name.as_ref()],
            |row| Ingredient::try_from(row))?;

        Ok(ingredient)
    }
}

impl TryFrom<&Row<'_>> for Ingredient {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            name: row.get(0)?,
            price_per_unit: row.get(1)?,
            unit: row.get(2)?,
        })
    }
}

#[cfg(test)]
mod test {
    use rusqlite::Connection;

    use crate::ingredients::Ingredient;
    use crate::units::Unit;

    use super::*;

    #[test]
    fn ingredients() {
        let connection = Connection::open_in_memory().unwrap();
        let ingredient_manager = IngredientManager::new(&connection);
        let ingredient = Ingredient::new("tomate", 0.30, Unit::Kilograms);
        let fetched_ingredient = ingredient_manager
            .init()
            .and_then(|_| ingredient_manager.insert(&ingredient))
            .and_then(|_| ingredient_manager.get("tomate"))
            .unwrap();

        assert_eq!(ingredient, fetched_ingredient);
    }
}