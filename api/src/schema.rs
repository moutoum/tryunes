table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        image -> Nullable<Text>,
    }
}

table! {
    recipe_ingredients (id) {
        id -> Integer,
        recipe_id -> Integer,
        ingredient_id -> Integer,
        quantity -> Text,
    }
}

table! {
    recipe_steps (id) {
        id -> Integer,
        recipe_id -> Integer,
        position -> Integer,
        step -> Text,
    }
}

table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        image -> Text,
        price -> Float,
        preparation_duration -> BigInt,
        cooking_duration -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(recipe_ingredients -> ingredients (ingredient_id));
joinable!(recipe_ingredients -> recipes (recipe_id));
joinable!(recipe_steps -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_ingredients,
    recipe_steps,
    recipes,
);
