use diesel::insert_into;
use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::{models, Storage};
use crate::recipes::{RecipeIngredient, IngredientForm, Recipe, RecipeForm};
use crate::recipes::helpers::{insert_recipe_ingredient, insert_recipe_steps, get_recipe_ingredients};
use itertools::Itertools;

#[post("/recipes", format = "json", data = "<recipe_form>")]
pub fn post_recipe(connection: Storage, recipe_form: Json<RecipeForm>) -> Json<Recipe> {
    use crate::schema::{recipes};

    let new_recipe = connection.transaction::<_, diesel::result::Error, _>(|| {
        let recipe = models::RecipeForm {
            name: recipe_form.name.clone(),
            description: recipe_form.description.clone(),
            image: recipe_form.image.clone(),
            price: recipe_form.price,
            preparation_duration: recipe_form.preparation_duration,
            cooking_duration: recipe_form.cooking_duration,
        };

        insert_into(recipes::table).values(&recipe).execute(&*connection)?;
        let recipe = recipes::table
            .order(recipes::columns::id.desc())
            .first::<models::Recipe>(&*connection)
            .expect("Could not find recipe");

        let ingredients = recipe_form.ingredients
            .iter()
            .map(|ingredient| insert_recipe_ingredient(&*connection, recipe.id, ingredient))
            .collect::<Vec<_>>();

        let steps = insert_recipe_steps(&*connection, recipe.id, &recipe_form.steps);

        return Ok(Recipe {
            id: recipe.id,
            name: recipe.name,
            description: recipe.description,
            image: recipe.image,
            price: recipe.price,
            preparation_duration: recipe.preparation_duration,
            cooking_duration: recipe.cooking_duration,
            creation_date: recipe.created_at,
            last_update_date: recipe.updated_at,
            ingredients,
            steps,
        });
    }).expect("could not insert recipe");

    Json(new_recipe)
}

#[post("/recipes/<recipe_id>/ingredients", format = "json", data = "<ingredient_form>")]
pub fn post_ingredient(connection: Storage, recipe_id: i32, ingredient_form: Json<IngredientForm>) -> Json<RecipeIngredient> {
    let new_ingredient = connection.transaction::<_, diesel::result::Error, _>(|| {
        Ok(insert_recipe_ingredient(&*connection, recipe_id, &ingredient_form))
    }).expect("could not create ingredient");
    Json(new_ingredient)
}

#[get("/recipes")]
pub fn list_recipes(connection: Storage) -> Json<Vec<Recipe>> {
    use crate::schema::{recipes, recipe_ingredients, recipe_steps, ingredients};

    let r = recipes::table.load::<models::Recipe>(&*connection).expect("could not load recipes");

    Json(r
        .into_iter()
        .map(|recipe| {
            let i = get_recipe_ingredients(&*connection, &recipe);

            Recipe {
                id: recipe.id,
                name: recipe.name,
                description: recipe.description,
                image: recipe.image,
                price: recipe.price,
                preparation_duration: recipe.preparation_duration,
                cooking_duration: recipe.cooking_duration,
                creation_date: recipe.created_at,
                last_update_date: recipe.updated_at,
                ingredients: i,
                steps: vec![],
            }
        })
        .collect()
    )
}