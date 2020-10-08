import {Action} from "./definitions";

export type RecipeState =
    | { status: 'empty' }
    | { status: 'loading' }
    | { status: 'success', data: Array<Recipe> }
    | { status: 'error', error: string };

export type State = {
    recipes: RecipeState
};

const initialRecipeState: RecipeState = {status: 'empty'};

export const initialState: State = {
    recipes: initialRecipeState,
};

function recipesReducer(state: RecipeState = initialRecipeState, action: Action): RecipeState {
    switch (action.type) {
        case 'FETCH_RECIPES_REQUEST':
            return {status: 'loading'}

        case 'FETCH_RECIPES_SUCCESS':
            return {status: "success", data: action.recipes};

        case 'FETCH_RECIPES_FAILURE':
            return {status: "error", error: action.error};

        default:
            return state;
    }
}

export function reducer(state: State, action: Action): State {
    return {
        recipes: recipesReducer(state.recipes, action),
    }
}