#[macro_use]
extern crate rusqlite;

use rusqlite::Connection;

use crate::errors::Result;
use crate::sqlite::ingredients::IngredientManager;

mod recipes;
mod ingredients;
mod units;
mod sqlite;
mod errors;

pub struct Tryunes<'a> {
    pub ingredients: IngredientManager<'a>,
}

impl<'a> Tryunes<'a> {
    pub fn with_connection(connection: &'a Connection) -> Tryunes<'a> {
        Tryunes {
            ingredients: IngredientManager::new(connection),
        }
    }

    pub fn init(&self) -> Result<()> {
        self.ingredients.init()
    }
}