extern crate serde_json;

use std::io::{stdout, Write, stdin};
use std::error::Error;
use serde_json::{json, to_string_pretty};
use std::str::FromStr;

const BASE_URL: &str = "http://localhost:8000/api";

fn read<T: FromStr>(sentence: &str) -> Result<T, Box<dyn Error>> {
    let mut value = String::new();
    print!("{} ", sentence);
    stdout().flush()?;
    stdin().read_line(&mut value)?;
    Ok(value.trim().parse().or(Err("Could not parse data"))?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/recipes", BASE_URL).to_string();
    let ingredient = json!({
        "name": read::<String>("Name?")?,
        "description": read::<String>("Description?")?,
        "image": read::<String>("Image?")?,
        "price": read::<f32>("Price?")?,
        "preparation_duration": read::<i64>("Preparation duration?")?,
        "cooking_duration": read::<i64>("Cooking duration?")?,
        "ingredients": read::<u32>("Number of ingredients?")
            .and_then(|range| {
                let mut v = Vec::new();
                for i in 0..range {
                    println!("Ingredient {}/{}:", i, range);
                    v.push(json!({
                        "ingredient_id": read::<i32>("IngredientID?")?,
                        "quantity": read::<String>("Quantity?")?,
                    }));
                }
                Ok(v)
            })?,
        "steps": Vec::<String>::new(),
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(&url)
        .json(&ingredient)
        .send()?
        .error_for_status()?
        .json::<serde_json::Value>()?;

    println!("New recipe: {}", to_string_pretty(&res)?);
    Ok(())
}
