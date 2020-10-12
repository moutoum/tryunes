import {Ingredient} from "../models/ingredients";
import { Recipe } from "../models/recipes";

export type Action =
    | { type: 'FETCH_RECIPES_REQUEST' }
    | { type: 'FETCH_RECIPES_SUCCESS', recipes: Array<Recipe> }
    | { type: 'FETCH_RECIPES_FAILURE', error: string }
    | { type: 'FETCH_INGREDIENTS_REQUEST' }
    | { type: 'FETCH_INGREDIENTS_SUCCESS', ingredients: Array<Ingredient> }
    | { type: 'FETCH_INGREDIENTS_FAILURE', error: string }
    | { type: 'INSERT_RECIPE', recipe: Recipe };
