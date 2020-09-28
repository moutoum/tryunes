extern crate serde_json;

use std::io::{stdout, Write, stdin};
use std::error::Error;
use serde_json::{json, to_string_pretty};

const BASE_URL: &str = "http://localhost:8000/api";

fn read(sentence: &str) -> Result<String, Box<dyn Error>> {
    let mut value = String::new();
    print!("{} ", sentence);
    stdout().flush()?;
    stdin().read_line(&mut value)?;
    Ok(value.trim().into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/ingredients", BASE_URL).to_string();
    let ingredient = json!({
        "name": read("Name?")?,
        "image": read("Image?")?,
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(&url)
        .json(&ingredient)
        .send()?
        .error_for_status()?
        .json::<serde_json::Value>()?;

    println!("New ingredient: {}", to_string_pretty(&res)?);
    Ok(())
}
