CREATE TABLE recipes (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    image TEXT,
    price REAL NOT NULL DEFAULT 0,
    preparation_duration BIGINT NOT NULL DEFAULT 0,
    cooking_duration BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE ingredients (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    image TEXT
);

CREATE TABLE recipe_ingredients (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    quantity TEXT NOT NULL,
    UNIQUE(recipe_id, ingredient_id) ON CONFLICT ROLLBACK,
    FOREIGN KEY (recipe_id) REFERENCES recipes (id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients (id)
);

CREATE TABLE recipe_steps (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
    step TEXT NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes (id)
);