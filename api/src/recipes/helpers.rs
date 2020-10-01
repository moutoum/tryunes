use diesel::*;
use rocket_contrib::databases::diesel::SqliteConnection;

use crate::models;
use crate::recipes::{RecipeIngredient, IngredientForm};

pub fn insert_recipe_ingredient(connection: &SqliteConnection, recipe_id: i32, ingredient_form: &IngredientForm) -> RecipeIngredient
{
    use crate::schema::{recipe_ingredients, ingredients};

    let ingredient = models::RecipeIngredientForm {
        recipe_id,
        ingredient_id: ingredient_form.ingredient_id,
        quantity: ingredient_form.quantity.clone(),
    };

    println!("Inserting ingredient for {:?}", ingredient);

    insert_into(recipe_ingredients::table)
        .values(&ingredient)
        .execute(connection)
        .expect("could not insert recipe ingredient");

    let (ingredient, recipe_ingredient) = ingredients::table
        .inner_join(recipe_ingredients::table)
        .filter(recipe_ingredients::columns::ingredient_id.eq(ingredient_form.ingredient_id))
        .filter(recipe_ingredients::columns::recipe_id.eq(recipe_id))
        .first::<(models::Ingredient, models::RecipeIngredient)>(connection)
        .expect("could not load ingredient");

    RecipeIngredient {
        id: ingredient.id,
        name: ingredient.name,
        image: ingredient.image.unwrap_or_default(),
        quantity: recipe_ingredient.quantity,
    }
}

pub fn insert_recipe_steps(connection: &SqliteConnection, recipe_id: i32, steps: &Vec<String>) -> Vec<String> {
    use crate::schema::recipe_steps;

    let steps = steps.iter()
        .enumerate()
        .map(|(i, step)| models::RecipeStepForm {
            recipe_id,
            position: i as i32,
            step: step.clone(),
        })
        .collect::<Vec<_>>();

    insert_into(recipe_steps::table)
        .values(&steps)
        .execute(&*connection)
        .expect("could not insert recipe steps");

    recipe_steps::table
        .filter(recipe_steps::columns::recipe_id.eq(recipe_id))
        .order(recipe_steps::columns::position.asc())
        .select(recipe_steps::columns::step)
        .load::<String>(&*connection)
        .expect("could not select recipe steps")
}