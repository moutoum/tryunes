use crate::schema::{recipes, ingredients, recipe_ingredients, recipe_steps};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Debug)]
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

#[derive(Serialize, Queryable, Identifiable, Debug)]
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

#[derive(Queryable, Debug, Identifiable, Associations)]
#[belongs_to(Recipe)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: String,
}

#[derive(Serialize, Insertable, Debug)]
#[table_name = "recipe_ingredients"]
pub struct RecipeIngredientForm {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: String,
}


#[derive(Queryable, Debug, Associations, Identifiable)]
#[belongs_to(Recipe)]
pub struct RecipeStep {
    pub id: i32,
    pub recipe_id: i32,
    pub position: i32,
    pub step: String,
}

#[derive(Serialize, Insertable, Debug)]
#[table_name = "recipe_steps"]
pub struct RecipeStepForm {
    pub recipe_id: i32,
    pub position: i32,
    pub step: String,
}