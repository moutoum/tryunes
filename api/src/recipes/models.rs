use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: f32,
    pub preparation_duration: i64,
    pub cooking_duration: i64,
    pub creation_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub quantity: String,
}

pub mod forms {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Recipe {
        pub name: String,
        pub description: String,
        pub image: String,
        pub price: f32,
        pub preparation_duration: i64,
        pub cooking_duration: i64,
        pub ingredients: Vec<Ingredient>,
        pub steps: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Ingredient {
        pub ingredient_id: i32,
        pub quantity: String,
    }
}
