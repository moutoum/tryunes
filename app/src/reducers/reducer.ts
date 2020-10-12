import {Action} from "./definitions";
import {Ingredient} from "../models/ingredients";
import {Recipe} from "../models/recipes";

type ListingState<T> =
    | { status: 'empty' }
    | { status: 'loading' }
    | { status: 'success', data: Array<T> }
    | { status: 'error', error: string };

export type RecipeState = ListingState<Recipe>;
export type IngredientState = ListingState<Ingredient>

export type State = {
    recipes: RecipeState,
    ingredients: IngredientState,
};

const initialRecipeState: RecipeState = {status: 'empty'};
const initialIngredientState: IngredientState = {status: "empty"};

export const initialState: State = {
    recipes: initialRecipeState,
    ingredients: initialIngredientState,
};

function recipesReducer(state: RecipeState = initialRecipeState, action: Action): RecipeState {
    switch (action.type) {
        case 'FETCH_RECIPES_REQUEST':
            return {status: 'loading'}

        case 'FETCH_RECIPES_SUCCESS':
            return {status: "success", data: action.recipes};

        case 'FETCH_RECIPES_FAILURE':
            return {status: "error", error: action.error};

        case "INSERT_RECIPE":
            if (state.status === "success") return {
                ...state,
                data: [...state.data, action.recipe].sort((rA, rB) => rA.name.localeCompare(rB.name))
            };
            return {status: "success", data: [action.recipe]};

        default:
            return state;
    }
}

function ingredientsReducer(state: IngredientState = initialIngredientState, action: Action): IngredientState {
    switch (action.type) {
        case 'FETCH_INGREDIENTS_REQUEST':
            return {status: 'loading'}

        case 'FETCH_INGREDIENTS_SUCCESS':
            return {status: "success", data: action.ingredients};

        case 'FETCH_INGREDIENTS_FAILURE':
            return {status: "error", error: action.error};

        default:
            return state;
    }
}

export function reducer(state: State, action: Action): State {
    return {
        recipes: recipesReducer(state.recipes, action),
        ingredients: ingredientsReducer(state.ingredients, action),
    }
}