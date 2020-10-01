#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate itertools;

use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket_contrib::databases::diesel::SqliteConnection;
use rocket_contrib::json::Json;
use serde::Serialize;

pub mod schema;
pub mod models;
mod recipes;

#[database("sqlite")]
#[derive(Copy)]
pub struct Storage(SqliteConnection);

#[derive(Serialize)]
struct Recipe {
    id: i32,
    name: String,
    description: Option<String>,
    image: Option<String>,
    price: f32,
    preparation_duration: i64,
    cooking_duration: i64,
    creation_date: NaiveDateTime,
    last_update_date: NaiveDateTime,
    ingredients: Vec<RecipeIngredient>,
    steps: Vec<String>,
}

#[derive(Serialize, Debug)]
struct RecipeIngredient {
    id: i32,
    name: String,
    image: String,
    quantity: String,
}

#[post("/ingredients", format = "json", data = "<ingredient_form>")]
fn post_ingredient(connection: Storage, ingredient_form: Json<models::IngredientForm>) -> Json<models::Ingredient> {
    use schema::ingredients::dsl::*;
    let new_ingredient = connection.transaction(|| {
        insert_into(ingredients).values(ingredient_form.into_inner()).execute(&*connection)?;
        ingredients.order(id.desc()).first::<models::Ingredient>(&*connection)
    }).expect("could not insert ingredient");
    Json(new_ingredient)
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_response("Apply CORS", |_, response| {
            response.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        }))
        .attach(Storage::fairing())
        .mount("/api", routes![
            recipes::routes::post_recipe,
            recipes::routes::post_ingredient,
            recipes::routes::list_recipes,
            post_ingredient
        ])
        .launch();
}