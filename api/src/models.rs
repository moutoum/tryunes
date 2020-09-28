use crate::schema::{recipes, ingredients, recipe_ingredients};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub price: f32,
    pub preparation_duration: i64,
    pub cooking_duration: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "recipes"]
pub struct RecipeForm {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub price: f32,
    pub preparation_duration: i64,
    pub cooking_duration: i64
}

#[derive(Serialize, Queryable, Identifiable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "ingredients"]
pub struct IngredientForm {
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Insertable, Queryable)]
pub struct RecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: String,
}