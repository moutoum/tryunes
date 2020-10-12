import {get, post} from "./common";
import {Recipe, RecipeCreationForm} from "../models/recipes";

export async function list() {
    return await get<Array<Recipe>>('/recipes');
}

export async function getById(id: number) {
    return await get<Recipe>(`/recipes/${id}`);
}

export async function create(recipe: RecipeCreationForm): Promise<Recipe> {
    const r = await post<RecipeCreationForm, Recipe>("/recipes", recipe)
    return r.data;
}