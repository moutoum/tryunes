#![feature(proc_macro_hygiene, decl_macro)]

pub mod schema;
pub mod models;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use chrono::{Duration, Utc, NaiveDateTime};
use rocket::fairing::AdHoc;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::insert_into;

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
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}

#[derive(Deserialize)]
struct RecipeForm {
    name: String,
    description: Option<String>,
    image: Option<String>,
    price: f32,
    preparation_duration: i64,
    cooking_duration: i64,
    ingredients: Vec<RecipeIngredientRecord>,
    steps: Vec<String>,
}

#[derive(Deserialize)]
struct RecipeIngredientRecord {
    pub ingredient_id: i32,
    pub quantity: String,
}

#[derive(Serialize)]
struct Ingredient {
    id: i32,
    name: String,
    image: String,
    quantity: String,
}

#[post("/ingredients", format = "json", data = "<ingredient_form>")]
fn post_ingredient(ingredient_form: Json<models::IngredientForm>) -> Json<models::Ingredient> {
    use schema::ingredients::dsl::*;

    let connection = SqliteConnection::establish("./db/sqlite.db").expect("could not connect");
    let new_ingredient = connection.transaction(|| {
        insert_into(ingredients).values(ingredient_form.into_inner()).execute(&connection)?;
        ingredients.order(id.desc()).first::<models::Ingredient>(&connection)
    }).expect("could not insert ingredient");
    Json(new_ingredient)
}

#[get("/recipes/<recipe_id>")]
fn get_recipe(recipe_id: i32) -> Json<Recipe> {
    use schema::recipes::dsl::*;

    let connection = SqliteConnection::establish("./db/sqlite.db").expect("could not connect");
    let recipe = recipes
        .filter(id.eq(recipe_id))
        .first::<models::Recipe>(&connection)
        .expect("could not load recipe");

    Json(Recipe {
        id: recipe.id,
        name: recipe.name,
        description: recipe.description,
        image: recipe.image,
        price: recipe.price,
        preparation_duration: recipe.preparation_duration,
        cooking_duration: recipe.cooking_duration,
        creation_date: recipe.created_at,
        last_update_date: recipe.updated_at,
        ingredients: vec![],
        steps: vec![],
    })
}

#[post("/recipes", format = "json", data = "<recipe_form>")]
fn post_recipe(recipe_form: Json<RecipeForm>) -> Json<Recipe> {
    use schema::*;

    let connection = SqliteConnection::establish("./db/sqlite.db").expect("could not connect");
    let new_recipe = connection.transaction::<_, diesel::result::Error, _>(|| {
        /// Inserting recipe.
        let recipe = models::RecipeForm {
            name: recipe_form.name.clone(),
            description: recipe_form.description.clone(),
            image: recipe_form.image.clone(),
            price: recipe_form.price,
            preparation_duration: recipe_form.preparation_duration,
            cooking_duration: recipe_form.cooking_duration,
        };

        insert_into(recipes::table).values(&recipe).execute(&connection)?;
        let mut r = recipes::table
            .order(recipes::columns::id.desc())
            .first::<models::Recipe>(&connection)
            .expect("Could not find recipe");

        // Inserting ingredient records.
        let ingredients = recipe_form.ingredients
            .iter()
            .map(|record| models::RecipeIngredient {
                recipe_id: r.id,
                ingredient_id: record.ingredient_id,
                quantity: record.quantity.clone(),
            })
            .collect::<Vec<models::RecipeIngredient>>();

        insert_into(recipe_ingredients::table)
            .values(ingredients)
            .execute(&connection)
            .expect("Could not insert recipe ingredient record");

        // Fetch ingredients depending on the recipe ingredient records.
        let ingredients = ingredients::table
            .inner_join(recipe_ingredients::table)
            .filter(recipe_ingredients::columns::recipe_id.eq(r.id))
            .load::<(models::Ingredient, models::RecipeIngredient)>(&connection)
            .expect("Could not load ingredients")
            .into_iter()
            .map(|(i, ri)| {
                Ingredient {
                    id: i.id,
                    name: i.name,
                    image: i.image.unwrap_or_default(),
                    quantity: ri.quantity,
                }
            })
            .collect();

        return Ok(Recipe {
            id: r.id,
            name: r.name,
            description: r.description,
            image: r.image,
            price: r.price,
            preparation_duration: r.preparation_duration,
            cooking_duration: r.cooking_duration,
            creation_date: r.created_at,
            last_update_date: r.updated_at,
            ingredients,
            steps: vec![],
        });
    }).expect("could not insert recipe");

    Json(new_recipe)
}

#[get("/recipes")]
fn list_recipes() -> Json<Vec<Recipe>> {
    Json(vec![Recipe {
        id: 1,
        name: "Pâtes à la carbonara".to_string(),
        description: Some("Meilleurs pâtes importées tout droit d'Italie!".to_string()),
        image: Some("https://www.ptitchef.com/imgupl/recipe/pates-a-la-carbonara-la-vraie-recette--449563p702930.jpg".to_string()),
        price: 3.50,
        preparation_duration: Duration::minutes(5).num_seconds(),
        cooking_duration: Duration::minutes(15).num_seconds(),
        creation_date: Utc::now().naive_utc(),
        last_update_date: Utc::now().naive_utc(),
        ingredients: vec![
            Ingredient {
                id: 0,
                name: "Pâtes".to_string(),
                image: "./images/pasta.png".to_string(),
                quantity: "250g".to_string(),
            },
            Ingredient {
                id: 1,
                name: "Crème".to_string(),
                image: "./images/cream.png".to_string(),
                quantity: "20cl".to_string(),
            },
            Ingredient {
                id: 2,
                name: "Lardons".to_string(),
                image: "./images/lardons.png".to_string(),
                quantity: "200g".to_string(),
            }],
        steps: vec![
            "Faire bouillir de l'eau salée et verser les pâtes.".to_string(),
            "Faire revenir les lardons dans une poêle chaude.".to_string(),
            "Ajouter la crème aux lardons et laisser cuire 1-2 minutes".to_string(),
            "Egouter puis servir.".to_string(),
        ],
    }, Recipe {
        id: 2,
        name: "Tomates farcies".to_string(),
        description: Some("Les plus grosses tomates farcies trouvés sur le marché.".to_string()),
        image: Some("https://img-3.journaldesfemmes.fr/DSXwGCHAbEkrYQLG4TyHKs3IpnI=/750x/smart/2599468a50354b90bd2a9399a63456e0/recipe-jdf/390385.jpg".to_string()),
        price: 8.56,
        preparation_duration: Duration::minutes(20).num_seconds(),
        cooking_duration: Duration::minutes(45).num_seconds(),
        creation_date: Utc::now().naive_utc(),
        last_update_date: Utc::now().naive_utc(),
        ingredients: Vec::new(),
        steps: Vec::new(),
    }])
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_response("Apply CORS", |_, response| {
            response.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        }))
        .mount("/api", routes![get_recipe, post_recipe, list_recipes, post_ingredient]).launch();
}