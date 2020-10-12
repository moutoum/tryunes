#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate itertools;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket_contrib::databases::diesel::SqliteConnection;
use rocket_contrib::json::Json;
use rocket::http::{Method, ContentType, Status, Header};
use std::io::Cursor;

pub mod schema;
pub mod models;
pub mod recipes;
pub mod result;
pub mod ingredients;


#[database("sqlite")]
#[derive(Copy)]
pub struct Storage(SqliteConnection);

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
        .attach(AdHoc::on_response("Apply CORS", |request, response| {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            if request.method() == Method::Options {
                response.set_status(Status::Ok);
                response.set_header(ContentType::Plain);
                response.set_sized_body(Cursor::new(""));
            }
        }))
        .attach(Storage::fairing())
        .mount("/api", routes![
            recipes::routes::post_recipe,
            recipes::routes::post_ingredient,
            recipes::routes::list_recipes,
            ingredients::routes::list_ingredients,
            post_ingredient
        ])
        .launch();
}