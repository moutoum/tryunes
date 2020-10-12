use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::{models, Storage};
use crate::result::Result;

#[get("/ingredients")]
pub fn list_ingredients(connection: Storage) -> Result<Json<Vec<models::Ingredient>>> {
    use crate::schema::ingredients;

    ingredients::table
        .order_by(ingredients::name)
        .load::<models::Ingredient>(&*connection)
        .map_err(|e| e.into())
        .and_then(|is| Ok(Json(is)))
}