#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use serde::Serialize;
use rocket_contrib::json::Json;
use chrono::{DateTime, Duration, Utc};
use chrono::serde::{ts_milliseconds};
use rocket::fairing::AdHoc;

#[derive(Serialize)]
struct Recipe {
    id: u64,
    name: String,
    description: String,
    image: String,
    price: f64,
    preparation_duration: i64,
    cooking_duration: i64,
    #[serde(with = "ts_milliseconds")]
    creation_date: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    last_update_date: DateTime<Utc>,
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}

#[derive(Serialize)]
struct Ingredient {
    id: u64,
    name: String,
    image: String,
    quantity: String,
}

#[get("/recipes")]
fn list_recipes() -> Json<Vec<Recipe>> {
    Json(vec![Recipe {
        id: 1,
        name: "Pâtes à la carbonara".to_string(),
        description: "Meilleurs pâtes importées tout droit d'Italie!".to_string(),
        image: "https://www.ptitchef.com/imgupl/recipe/pates-a-la-carbonara-la-vraie-recette--449563p702930.jpg".to_string(),
        price: 3.50,
        preparation_duration: Duration::minutes(5).num_seconds(),
        cooking_duration: Duration::minutes(15).num_seconds(),
        creation_date: Utc::now(),
        last_update_date: Utc::now(),
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
        description: "Les plus grosses tomates farcies trouvés sur le marché.".to_string(),
        image: "https://img-3.journaldesfemmes.fr/DSXwGCHAbEkrYQLG4TyHKs3IpnI=/750x/smart/2599468a50354b90bd2a9399a63456e0/recipe-jdf/390385.jpg".to_string(),
        price: 8.56,
        preparation_duration: Duration::minutes(20).num_seconds(),
        cooking_duration: Duration::minutes(45).num_seconds(),
        creation_date: Utc::now(),
        last_update_date: Utc::now(),
        ingredients: Vec::new(),
        steps: Vec::new(),
    }])
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_response("Apply CORS", |_, response| {
            response.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        }))
        .mount("/api", routes![list_recipes]).launch();
}