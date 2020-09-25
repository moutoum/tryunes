interface Recipe {
    id: number,
    name: string,
    description: string,
    image: string,
    price: number,
    preparation_duration: number,
    cooking_duration: number,
    creation_date: Date,
    last_update_date: Date,
    ingredients: Array<Ingredient>,
    steps: Array<string>,
}

interface Ingredient {
    id: number,
    name: string,
    image: string,
    quantity: string,
}