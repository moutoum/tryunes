use diesel::insert_into;
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

#[post("/ingredients", format = "json", data = "<ingredient_form>")]
pub fn create_ingredient(connection: Storage, ingredient_form: Json<models::IngredientForm>) -> Result<Json<models::Ingredient>> {
    use crate::schema::ingredients::dsl::*;
    connection.0
        .transaction(|| {
            insert_into(ingredients)
                .values(ingredient_form.into_inner())
                .execute(&connection.0)
                .and_then(|_|
                    ingredients
                        .order(id.desc())
                        .first::<models::Ingredient>(&connection.0)
                )
        })
        .map_err(|e| e.into())
        .and_then(|i| Ok(Json(i)))
}